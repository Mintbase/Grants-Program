use std::str::FromStr;

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::Base64VecU8,
    log, near,
    serde::{Deserialize, Serialize},
    serde_json::json,
    AccountId, Gas, NearToken, Promise,
};

use near_sdk::{env, store::LookupSet};

// Cost per byte of storage
pub const YOCTO_PER_BYTE: u128 = 10_000_000_000_000_000_000;

const fn bytes_to_stake(bytes: u128) -> u128 {
    (bytes as u128) * YOCTO_PER_BYTE
}

#[derive(Clone, Debug, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
pub struct NFTTokenMetadata {
    /// the Title for this token. ex. "Arch Nemesis: Mail Carrier" or "Parcel 5055"
    pub title: Option<String>,
    /// free-form description of this token.
    pub description: Option<String>,
    /// URL to associated media, preferably to decentralized, content-addressed storage
    pub media: Option<String>,
    /// Base64-encoded sha256 hash of content referenced by the `media` field.
    /// Required if `media` is included.
    pub media_hash: Option<Base64VecU8>,
    /// number of copies of this set of metadata in existence when token was minted.
    pub copies: Option<u16>,
    /// ISO 8601 datetime when token expires.
    pub expires_at: Option<String>,
    /// ISO 8601 datetime when token starts being valid.
    pub starts_at: Option<String>,
    /// When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>,
    /// URL to an off-chain JSON file with more info. The Mintbase Indexer refers
    /// to this field as `thing_id` or sometimes, `meta_id`.
    pub reference: Option<String>,
    /// Base64-encoded sha256 hash of JSON from reference field. Required if
    /// `reference` is included.
    pub reference_hash: Option<Base64VecU8>,
}

#[near(contract_state)]
pub struct ChallengeFactory {
    pub challenges: LookupSet<String>,
}

impl Default for ChallengeFactory {
    fn default() -> Self {
        Self {
            challenges: LookupSet::new(b"a".to_vec()),
        }
    }
}

// Implement the contract structure
#[near]
impl ChallengeFactory {
    /// If a `Challenge` with `challenge_id` has been produced by this `Factory`, return `true`.
    pub fn challenge_exists(&self, store_id: String) -> bool {
        self.challenges.contains(&store_id)
    }

    /// Panics if a store with the requested ID already exists
    pub fn assert_no_challenge_with_id(&self, store_id: String) {
        assert!(
            !self.challenge_exists(store_id),
            "Challenge with that ID already exists"
        );
    }

    #[payable]
    pub fn create_challenge(
        &mut self,
        id_prefix: String,
        name: String,
        description: String,
        media_link: String,
        reward_nft_id: String,
        challenge_nft_ids: std::vec::Vec<String>,
        burn_challenge_piece_on_claim: Vec<bool>,
        _expiration_date_in_ns: String,
        _winner_limit: String,
        creator_can_update: bool,
        // only necessary if contract will be minting the reward nft
        reward_nft_metadata: NFTTokenMetadata,
    ) -> Promise {
        log!("Creating challenge: {}", name);
        assert!(
            env::attached_deposit().as_yoctonear() >= bytes_to_stake(100_000),
            "To cover the storage required for your store, you need to attach at least {} yoctoNEAR to this transaction.",
            bytes_to_stake(100_000)
        );
        self.assert_no_challenge_with_id(name.clone());
        let formatted_challenge_id = format!("{}.{}", id_prefix, env::current_account_id());
        let challenge_account_id = AccountId::from_str(&formatted_challenge_id).unwrap();
        let winner_limit: u64 = _winner_limit.parse().unwrap();
        let expiration_date_in_ns: u64 = _expiration_date_in_ns.parse().unwrap();

        self.challenges.insert(id_prefix.clone());
        Promise::new(challenge_account_id.clone())
            .create_account()
            .transfer(NearToken::from_yoctonear(bytes_to_stake(800_000)))
            .deploy_contract(include_bytes!("../wasm/nft-challenge.wasm").to_vec())
            .function_call(
                String::from("new"),
                json!({
                "owner_id": env::predecessor_account_id(),
                "name":name,
                "description":description,
                "media_link":media_link,
                "reward_nft_id":reward_nft_id,
                "_challenge_nft_ids": challenge_nft_ids,
                "_burn_challenge_piece_on_claim": burn_challenge_piece_on_claim,
                "expiration_date_in_ns": expiration_date_in_ns,
                "winner_limit": winner_limit,
                "creator_can_update": creator_can_update,
                "reward_nft_metadata": reward_nft_metadata
                 })
                .to_string()
                .into_bytes(),
                // TODO: Get better gas estimates
                NearToken::from_near(0),
                Gas::from_tgas(5),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::from_tgas(5))
                    .create_challenge_callback(id_prefix),
            )
    }

    #[private]
    pub fn create_challenge_callback(
        &mut self,
        id_prefix: String,
        #[callback_result] call_result: Result<(), near_sdk::PromiseError>,
    ) {
        if call_result.is_err() {
            panic!("There was an error creating the challenge");
        } else {
            self.challenges.insert(id_prefix);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "Challenge with that ID already exists")]
    fn get_default_greeting() {
        let mut contract = ChallengeFactory::default();
        contract.assert_no_challenge_with_id("test".to_string());
        contract.challenges.insert("test".to_string());
        contract.assert_no_challenge_with_id("test".to_string());
    }
}
