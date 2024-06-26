use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract,
    json_types::Base64VecU8,
    serde::{Deserialize, Serialize},
    AccountId, PromiseOrValue,
};

use std::collections::HashMap;

// Mintbase's TokenMetadata structure.
#[derive(Clone, Debug, Deserialize, Serialize, BorshDeserialize, BorshSerialize)]
pub struct NFTTokenMetadata {
    /// The Title for this token. ex. "Arch Nemesis: Mail Carrier" or "Parcel 5055"
    pub title: Option<String>,
    /// Free-form description of this token.
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

/// https://github.com/near/NEPs/blob/master/specs/Standards/NonFungibleToken/Core.md
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenCompliant {
    pub token_id: String,
}

pub type SplitBetweenUnparsed = HashMap<AccountId, u32>;

#[derive(Deserialize, Serialize)]
pub struct RoyaltyArgs {
    pub split_between: SplitBetweenUnparsed,
    pub percentage: u32,
}

#[ext_contract(mintbase_nft)]
pub trait MintbaseNft {
    fn check_is_minter(&self, account_id: near_sdk::AccountId) -> bool;

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<String>,
        limit: Option<u32>,
    ) -> Vec<TokenCompliant>;

    fn nft_batch_mint(
        &mut self,
        owner_id: near_sdk::AccountId,
        metadata: NFTTokenMetadata,
        num_to_mint: u64,
        royalty_args: Option<RoyaltyArgs>,
        split_owners: Option<SplitBetweenUnparsed>,
    ) -> PromiseOrValue<()>;

    fn nft_batch_burn(&mut self, token_ids: Vec<near_sdk::json_types::U64>);

    fn nft_approval_id(
        &self,
        token_id: near_sdk::json_types::U64,
        account_id: AccountId,
    ) -> Option<String>;

    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: near_sdk::json_types::U64,
        approval_id: u64,
        memo: Option<String>,
    );
}
