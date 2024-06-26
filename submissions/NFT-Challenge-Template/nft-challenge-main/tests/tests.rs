use near_sdk::{json_types::Base64VecU8, log};
use near_sdk::{AccountId, Gas, NearToken};
use near_workspaces::error::Error;
use near_workspaces::network::Sandbox;
use near_workspaces::result::ExecutionFinalResult;
use near_workspaces::{Contract, Worker};
use nft_challenge::{
    ChallengeMetaData, NFTTokenMetadata, RoyaltyArgs, SplitBetweenUnparsed, TokenCompliant,
};

use serde_json::json;
use std::time::SystemTime;

const SECONDS_IN_DAY: u64 = 86400;
const NS_IN_SECONDS: u64 = 1000000000;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct NFTContractMetadata {
    /// a version like "nft-1.0.0"
    pub spec: String,
    /// Subaccount of this `Store`. `Factory` is the super-account.
    pub name: String,
    /// Symbol of the Store. Up to 6 chars.
    pub symbol: String,
    /// a small image associated with this `Store`.
    pub icon: Option<String>,
    /// Centralized gateway known to have reliable access to decentralized storage
    /// assets referenced by `reference` or `media` URLs
    pub base_uri: Option<String>,
    /// URL to a JSON file with more info
    pub reference: Option<String>,
    /// Base64-encoded sha256 hash of the JSON file pointed at by the reference
    /// field. Required if `reference` is included.
    pub reference_hash: Option<Base64VecU8>,
}

async fn create_challenge(
    challenge_nft_ids: Vec<String>,
    burn_challenge_nft: Vec<bool>,
    reward_nft_id: String,
    winner_limit: u64,
    owner_id: AccountId,
    sandbox: &Worker<Sandbox>,
) -> Result<Contract, Box<dyn std::error::Error>> {
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    let duration_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let timestamp_nanos = duration_since_epoch.as_nanos() as u64 + SECONDS_IN_DAY * NS_IN_SECONDS;

    let outcome = user_account
        .call(contract.id(), "new")
        .args_json(json!({
            "owner_id":owner_id.to_string(),
            "name": "Test challenge".to_string(),
            "description": "A description for a test chalenge",
            "media_link": "A link to an image!",
            "reward_nft_id": reward_nft_id,
            "_challenge_nft_ids": challenge_nft_ids,
            "_burn_challenge_piece_on_claim":burn_challenge_nft,
            "expiration_date_in_ns": timestamp_nanos as u64,
            "winner_limit": winner_limit,
            "creator_can_update": true,
            "reward_nft_metadata": NFTTokenMetadata{
                title: Some("Reward NFT".to_string()),
                description: Some("A description for a reward NFT".to_string()),
                media: Some("A link to an image!".to_string()),
                media_hash: None,
                copies: None,
                expires_at: None,
                starts_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            },
        }))
        .max_gas()
        .transact()
        .await?;
    log!(
        "Failures after creating challenge: {:?}",
        outcome.failures()
    );

    let metadata_call = contract.view("get_challenge_metadata").await?;
    let metadata: ChallengeMetaData = metadata_call.json().unwrap();
    log!("Challenge Metadata: {:?}", metadata);
    assert!(metadata.owner_id == owner_id.to_string());
    assert!(metadata.name == "Test challenge");
    assert!(metadata.description == "A description for a test chalenge");
    assert!(metadata.media_link == Some("A link to an image!".to_string()));
    assert!(metadata.reward_nft_id == reward_nft_id);
    assert!(metadata.challenge_nft_ids == challenge_nft_ids);
    assert!(metadata.expiration_date_in_ns == timestamp_nanos as u64);
    assert!(metadata.winner_limit == winner_limit);
    assert!(metadata.reward_nft_metadata.title == Some("Reward NFT".to_string()));
    assert!(
        metadata.reward_nft_metadata.description
            == Some("A description for a reward NFT".to_string())
    );
    assert!(metadata.reward_nft_metadata.media == Some("A link to an image!".to_string()));
    assert!(metadata.creator_can_update);

    assert!(outcome.is_success());
    log!("Created challenge successfully!");
    Ok(contract)
}

async fn create_nfts(
    owner_id: AccountId,
    nft_ids: Vec<&str>,
    sandbox: &Worker<Sandbox>,
) -> Result<Vec<Contract>, Box<dyn std::error::Error>> {
    let mut deployed_contracts: Vec<Contract> = Vec::new();
    let user_account = sandbox.dev_create_account().await?;
    for el in nft_ids.iter() {
        let contract_wasm = sandbox
            .dev_deploy(include_bytes!("./mb-nft-v1/mb-nft-v1.wasm"))
            .await?;

        let outcome = user_account
            .call(contract_wasm.id(), "new")
            .args_json(json!({
               "metadata":NFTContractMetadata {
                    spec: "nft-1.0.0".to_string(),
                    name: (*el).to_string(),
                    symbol: "CH0".to_string(),
                    icon: None,
                    base_uri: None,
                    reference: None,
                    reference_hash: None,
                },
                "owner_id":owner_id,
            }))
            .max_gas()
            .transact()
            .await?;
        assert!(outcome.is_success());
        deployed_contracts.push(contract_wasm);
    }
    log!("Deployed NFTs: {:?}", nft_ids);
    Ok(deployed_contracts)
}

#[tokio::test]
async fn test_complete_challenge_without_all_pieces() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let user_account = sandbox.dev_create_account().await?;
    // Don't need to make reward nft since we won't be successfully completing
    // a challenge here.
    let nft_ids = vec!["challenge-nft-1", "challenge-nft-2"];
    let nfts = create_nfts(user_account.id().clone(), nft_ids, &sandbox).await?;
    let mut challenge_nft_ids: Vec<String> = vec![];

    for nft in nfts.iter() {
        challenge_nft_ids.push(nft.id().to_string());
    }

    let challenge_contract = create_challenge(
        challenge_nft_ids,
        vec![false, false],
        "reward-nft".to_string(),
        1,
        user_account.id().clone(),
        &sandbox,
    )
    .await?;

    let outcome_with_none = user_account
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    log!("The original outcome: {:?}", outcome_with_none.logs());

    assert!(outcome_with_none
        .logs()
        .last()
        .unwrap()
        .contains("Account does not own any of the challenge nfts at index 0"));

    // Try with some but not all pieces.
    let mint_outcome = user_account
        .call(nfts[0].id(), "nft_batch_mint")
        .args_json(json!({
            "owner_id": user_account.id().clone(),
            "metadata":NFTTokenMetadata {
                title: Some("Challenge NFT 1".to_string()),
                description: Some("A description for a challenge NFT".to_string()),
                media: Some("A link to an image!".to_string()),
                media_hash: None,
                copies: None,
                expires_at: None,
                starts_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            },
            "num_to_mint": 1,
            "royalty_args": None::<RoyaltyArgs>,
            "split_owners": None::<SplitBetweenUnparsed>,
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;
    assert!(mint_outcome.is_success());

    let outcome_with_some_challenge_pieces = user_account
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;
    assert!(outcome_with_some_challenge_pieces
        .logs()
        .last()
        .unwrap()
        .contains("Account does not own any of the challenge nfts at index 1"));
    Ok(())
}

#[tokio::test]
async fn test_complete_challenge_with_max_potential_winners(
) -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let user_account0 = sandbox.dev_create_account().await?;
    let user_account1 = sandbox.dev_create_account().await?;
    let nft_ids = vec!["challenge-nft-1", "reward-nft"];
    let mut nfts = create_nfts(user_account0.id().clone(), nft_ids, &sandbox).await?;
    let mut challenge_nft_ids: Vec<String> = vec![];
    let reward_nft_id = nfts.pop().unwrap().id().to_string();
    for nft in nfts.iter() {
        challenge_nft_ids.push(nft.id().to_string());
    }

    let challenge_contract = create_challenge(
        challenge_nft_ids,
        vec![false],
        reward_nft_id,
        1,
        user_account0.id().clone(),
        &sandbox,
    )
    .await?;

    let metadata_call = challenge_contract.view("get_challenge_metadata").await?;
    let metadata: ChallengeMetaData = metadata_call.json().unwrap();

    // Mint challenge nfts.
    let mint_outcome = user_account0
        .call(nfts[0].id(), "nft_batch_mint")
        .args_json(json!({
            "owner_id": user_account0.id().clone(),
            "metadata":metadata.reward_nft_metadata,
            "num_to_mint": 1,
            "royalty_args": None::<RoyaltyArgs>,
            "split_owners": None::<SplitBetweenUnparsed>,
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    assert!(mint_outcome.is_success());

    let mint_outcome = user_account0
        .call(nfts[0].id(), "nft_batch_mint")
        .args_json(json!({
            "owner_id": user_account1.id().clone(),
            "metadata":metadata.reward_nft_metadata,
            "num_to_mint": 1,
            "royalty_args": None::<RoyaltyArgs>,
            "split_owners": None::<SplitBetweenUnparsed>,
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    assert!(mint_outcome.is_success());

    let promise_for_account0 = user_account0
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact();

    let promise_for_account1 = user_account1
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact();

    // Create a race condition to ensure only one account wins.
    let joint_promise: (
        Result<ExecutionFinalResult, Error>,
        Result<ExecutionFinalResult, Error>,
    ) = futures::join!(promise_for_account0, promise_for_account1);
    let (outcome_for_account0, outcome_for_account1) =
        (joint_promise.0.unwrap(), joint_promise.1.unwrap());

    log!(
        " Outcome for account 0: {:?}",
        outcome_for_account0.failures()
    );
    log!("Logs for account 0: {:?}", outcome_for_account0.logs());

    log!(
        " Outcome for account 1: {:?}",
        outcome_for_account1.failures()
    );
    log!("Logs for account 1: {:?}", outcome_for_account1.logs());

    assert!(outcome_for_account0.is_success() || outcome_for_account1.is_success());
    assert!(!(outcome_for_account0.is_success() && outcome_for_account1.is_success()));

    let (winner_account, loser_account) = if outcome_for_account0.is_success() {
        (user_account0, user_account1)
    } else {
        (user_account1, user_account0)
    };

    let winner_status_call = challenge_contract
        .view("is_account_winner")
        .args_json(json!({
            "account_id": winner_account.id()
        }))
        .await?;
    let winner_status: bool = winner_status_call.json().unwrap();

    assert!(winner_status);

    let winner_status_call = challenge_contract
        .view("is_account_winner")
        .args_json(json!({
            "account_id": loser_account.id()
        }))
        .await?;

    let winner_status: bool = winner_status_call.json().unwrap();
    assert!(!winner_status);

    let outcome_for_loser = loser_account
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    // Since challenge at max winners
    assert!(outcome_for_loser.is_failure());

    Ok(())
}

#[tokio::test]
async fn test_complete_challenge_already_complete() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let user_account0 = sandbox.dev_create_account().await?;
    let user_account1 = sandbox.dev_create_account().await?;
    let nft_ids = vec!["challenge-nft-1", "reward-nft"];
    let mut nfts = create_nfts(user_account0.id().clone(), nft_ids, &sandbox).await?;
    let mut challenge_nft_ids: Vec<String> = vec![];
    let reward_nft_id = nfts.pop().unwrap().id().to_string();
    log!("NFTs: {:?}", challenge_nft_ids);
    for nft in nfts.iter() {
        challenge_nft_ids.push(nft.id().to_string());
    }

    let challenge_contract = create_challenge(
        challenge_nft_ids,
        vec![false],
        reward_nft_id,
        2,
        user_account0.id().clone(),
        &sandbox,
    )
    .await?;

    let metadata_call = challenge_contract.view("get_challenge_metadata").await?;
    let metadata: ChallengeMetaData = metadata_call.json().unwrap();

    let mint_outcome = user_account0
        .call(nfts[0].id(), "nft_batch_mint")
        .args_json(json!({
            "owner_id": user_account0.id().clone(),
            "metadata":metadata.reward_nft_metadata,
            "num_to_mint": 1,
            "royalty_args": None::<RoyaltyArgs>,
            "split_owners": None::<SplitBetweenUnparsed>,
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    assert!(mint_outcome.is_success());

    let mint_outcome = user_account0
        .call(nfts[0].id(), "nft_batch_mint")
        .args_json(json!({
            "owner_id": user_account1.id().clone(),
            "metadata":metadata.reward_nft_metadata,
            "num_to_mint": 1,
            "royalty_args": None::<RoyaltyArgs>,
            "split_owners": None::<SplitBetweenUnparsed>,
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    assert!(mint_outcome.is_success());

    let outcome_for_account0 = user_account0
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    assert!(outcome_for_account0.is_success());

    let account_0_winner_status_call = challenge_contract
        .view("is_account_winner")
        .args_json(json!({
            "account_id": user_account0.id()
        }))
        .await?;
    let account_0_winner_status: bool = account_0_winner_status_call.json().unwrap();

    assert!(account_0_winner_status);

    let outcome_for_account0_again = user_account0
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    assert!(outcome_for_account0_again.is_failure());

    let end_challenge_outcome = user_account0
        .call(
            challenge_contract.id(),
            "update_challenge_completion_status",
        )
        .args_json(json!({
            "is_complete":true
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(end_challenge_outcome.is_success());

    let outcome_for_account1 = user_account1
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;
    assert!(outcome_for_account1.is_failure());
    Ok(())
}

#[tokio::test]
async fn test_end_challenge() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let user_account0 = sandbox.dev_create_account().await?;
    let user_account1 = sandbox.dev_create_account().await?;

    let challenge_contract = create_challenge(
        vec!["challenge-nft-1".to_string()],
        vec![false],
        "reward-nft".to_string(),
        1,
        user_account0.id().clone(),
        &sandbox,
    )
    .await?;

    let is_challenge_complete = challenge_contract.view("is_challenge_complete").await?;
    let is_complete: bool = is_challenge_complete.json().unwrap();
    assert!(!is_complete);

    let end_challenge_outcome = user_account1
        .call(
            challenge_contract.id(),
            "update_challenge_completion_status",
        )
        .args_json(json!({
            "is_complete":true
        }))
        .max_gas()
        .transact()
        .await?;
    assert!(end_challenge_outcome.is_failure());

    let end_challenge_outcome = user_account0
        .call(
            challenge_contract.id(),
            "update_challenge_completion_status",
        )
        .args_json(json!({
            "is_complete":true
        }))
        .max_gas()
        .transact()
        .await?;
    assert!(end_challenge_outcome.is_success());
    let is_complete_outcome = challenge_contract.view("is_challenge_complete").await?;
    assert!(is_complete_outcome.json::<bool>().unwrap());
    Ok(())
}

#[tokio::test]
async fn test_mint_nft() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let user_account0 = sandbox.dev_create_account().await?;
    let nft_ids = vec!["challenge-nft-1", "reward-nft"];
    let mut nfts = create_nfts(user_account0.id().clone(), nft_ids, &sandbox).await?;
    let mut challenge_nft_ids: Vec<String> = vec![];
    let reward_nft = nfts.pop().unwrap();

    for nft in nfts.iter() {
        challenge_nft_ids.push(nft.id().to_string());
    }

    let challenge_contract = create_challenge(
        challenge_nft_ids,
        vec![false],
        reward_nft.id().to_string(),
        1,
        user_account0.id().clone(),
        &sandbox,
    )
    .await?;

    let metadata_call = challenge_contract.view("get_challenge_metadata").await?;
    let metadata: ChallengeMetaData = metadata_call.json().unwrap();

    let mint_outcome = user_account0
        .call(nfts[0].id(), "nft_batch_mint")
        .args_json(json!({
            "owner_id": user_account0.id().clone(),
            "metadata":metadata.reward_nft_metadata,
            "num_to_mint": 1,
            "royalty_args": None::<RoyaltyArgs>,
            "split_owners": None::<SplitBetweenUnparsed>,
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    assert!(mint_outcome.is_success());

    let outcome_for_account0 = user_account0
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    assert!(outcome_for_account0.is_success());

    let account_0_status_call = challenge_contract
        .view("is_account_winner")
        .args_json(json!({
            "account_id": user_account0.id()
        }))
        .await?;
    let account_0_status: bool = account_0_status_call.json().unwrap();

    assert!(account_0_status);

    let change_minters_outcome = user_account0
        .call(reward_nft.id(), "batch_change_minters")
        .args_json(json!({
            "grant": vec![challenge_contract.id()],
            "revoke":None::<Vec<AccountId>>,
        }))
        .deposit(NearToken::from_yoctonear(1))
        .max_gas()
        .transact()
        .await?;

    assert!(change_minters_outcome.is_success());

    let outcome_for_challenge_mint = user_account0
        .call(challenge_contract.id(), "mint_nft")
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;
    assert!(outcome_for_challenge_mint.is_success());

    let outcome_for_nfts_owned_by_user = user_account0
        .call(reward_nft.id(), "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": user_account0.id().clone(),
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(outcome_for_nfts_owned_by_user.is_success());
    assert!(
        outcome_for_nfts_owned_by_user
            .json::<Vec<TokenCompliant>>()
            .unwrap()
            .len()
            == 1
    );

    Ok(())
}

#[tokio::test]
async fn test_burn_nfts() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let user_account0 = sandbox.dev_create_account().await?;
    let nft_ids = vec!["challenge-nft-1", "challenge-nft-2", "reward-nft"];
    let mut nfts = create_nfts(user_account0.id().clone(), nft_ids, &sandbox).await?;
    let mut challenge_nft_ids: Vec<String> = vec![];
    let reward_nft = nfts.pop().unwrap();

    for nft in nfts.iter() {
        challenge_nft_ids.push(nft.id().to_string());
    }

    let challenge_contract = create_challenge(
        challenge_nft_ids,
        vec![true, true],
        reward_nft.id().to_string(),
        1,
        user_account0.id().clone(),
        &sandbox,
    )
    .await?;

    let metadata_call = challenge_contract.view("get_challenge_metadata").await?;
    let metadata: ChallengeMetaData = metadata_call.json().unwrap();

    let mut mint_outcome = user_account0
        .call(nfts[0].id(), "nft_batch_mint")
        .args_json(json!({
            "owner_id": user_account0.id().clone(),
            "metadata":metadata.reward_nft_metadata,
            "num_to_mint": 1,
            "royalty_args": None::<RoyaltyArgs>,
            "split_owners": None::<SplitBetweenUnparsed>,
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    assert!(mint_outcome.is_success());

    mint_outcome = user_account0
        .call(nfts[1].id(), "nft_batch_mint")
        .args_json(json!({
            "owner_id": user_account0.id().clone(),
            "metadata":metadata.reward_nft_metadata,
            "num_to_mint": 1,
            "royalty_args": None::<RoyaltyArgs>,
            "split_owners": None::<SplitBetweenUnparsed>,
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    assert!(mint_outcome.is_success());

    let outcome_for_nfts_owned_by_user = user_account0
        .call(nfts[0].id(), "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": user_account0.id().clone(),
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(outcome_for_nfts_owned_by_user.is_success());

    let mut token_id = outcome_for_nfts_owned_by_user
        .json::<Vec<TokenCompliant>>()
        .unwrap()[0]
        .token_id
        .clone();

    let mut give_approval_outcome = user_account0
        .call(nfts[0].id(), "nft_approve")
        .args_json(json!({
            "token_id":     token_id.clone(),
            "account_id":challenge_contract.id(),
        }))
        .deposit(NearToken::from_millinear(8))
        .max_gas()
        .transact()
        .await?;

    assert!(give_approval_outcome.is_success());

    let mut outcome_for_account0 = user_account0
        .call(challenge_contract.id(), "initiate_claim")
        .max_gas()
        .deposit(NearToken::from_yoctonear(4))
        .transact()
        .await?;

    assert!(outcome_for_account0.is_success());

    let mut account_0_status_call = challenge_contract
        .view("is_account_winner")
        .args_json(json!({
            "account_id": user_account0.id()
        }))
        .await?;
    let mut account_0_status: bool = account_0_status_call.json().unwrap();
    // Should not be a winner since second challenge piece was not approved for
    // transfer.
    assert!(!account_0_status);

    let mut outcome_owning_challenge_pieces = user_account0
        .call(nfts[0].id(), "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": user_account0.id().clone(),
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(outcome_owning_challenge_pieces.is_success());

    outcome_owning_challenge_pieces = user_account0
        .call(nfts[1].id(), "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": user_account0.id().clone(),
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(outcome_owning_challenge_pieces.is_success());

    token_id = outcome_owning_challenge_pieces
        .json::<Vec<TokenCompliant>>()
        .unwrap()[0]
        .token_id
        .clone();

    give_approval_outcome = user_account0
        .call(nfts[1].id(), "nft_approve")
        .args_json(json!({
            "token_id":     token_id.clone(),
            "account_id":challenge_contract.id(),
        }))
        .deposit(NearToken::from_millinear(8))
        .max_gas()
        .transact()
        .await?;

    assert!(give_approval_outcome.is_success());

    outcome_for_account0 = user_account0
        .call(challenge_contract.id(), "initiate_claim")
        .gas(Gas::from_tgas(300))
        .deposit(NearToken::from_yoctonear(4))
        .transact()
        .await?;

    assert!(outcome_for_account0.is_success());

    account_0_status_call = challenge_contract
        .view("is_account_winner")
        .args_json(json!({
            "account_id": user_account0.id()
        }))
        .await?;
    account_0_status = account_0_status_call.json().unwrap();

    assert!(account_0_status);

    outcome_owning_challenge_pieces = user_account0
        .call(nfts[0].id(), "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": user_account0.id().clone(),
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(!outcome_owning_challenge_pieces.is_success());

    outcome_owning_challenge_pieces = user_account0
        .call(nfts[1].id(), "nft_tokens_for_owner")
        .args_json(json!({
            "account_id": user_account0.id().clone(),
        }))
        .max_gas()
        .transact()
        .await?;

    assert!(!outcome_owning_challenge_pieces.is_success());
    Ok(())
}
