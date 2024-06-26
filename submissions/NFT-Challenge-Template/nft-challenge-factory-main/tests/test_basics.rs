use std::str::FromStr;

use near_sdk::{AccountId, NearToken};
use nft_challenger_generator::NFTTokenMetadata;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChallengeMetaData {
    // The owner of this NFT Challenge
    pub owner_id: String,
    // The name for this challenge.
    pub name: String,
    // Free-form description of this challenge.
    pub description: String,
    // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_link: Option<String>,
    // The id of the reward NFT.
    pub reward_nft_id: String,
    // Metadata for the reward token NFT. Only necessary if we mint the nft.
    pub reward_nft_metadata: NFTTokenMetadata,
    // Ids of the challenge nfts that are part of this challenge.
    pub challenge_nft_ids: Vec<String>,
    // The expiration date of this challenge, expressed as a nano second timestamp.
    pub expiration_date_in_ns: u64,
    // Maximum number of winners for this challenge.
    pub winner_limit: u64,
    // Number of winners for this challenge.
    pub winners_count: u64,
    // Whether the challenge is completed or not.
    pub challenge_completed: bool,
    // Whether the creator of this challenge can update the challenge status.
    pub creator_can_update: bool,
}

#[tokio::test]
async fn test_can_create_challenge() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let contract = sandbox.dev_deploy(&contract_wasm).await?;

    let user_account = sandbox.dev_create_account().await?;

    let challenge_creation_outcome = user_account
        .call(contract.id(), "create_challenge")
        .args_json(json!({
            "id_prefix": "test-challenge",
            "name": "Test Challenge!",
            "description": "A test description",
            "media_link": "A fake media link",
            "reward_nft_id": "reward-nft-id",
            "challenge_nft_ids": vec!["challenge-nft-id"],
            "burn_challenge_piece_on_claim": vec![true],
            "_expiration_date_in_ns": "9007199254740991",
            "_winner_limit": "100",
            "creator_can_update": true,
            "reward_nft_metadata": NFTTokenMetadata{
                title: Some("Test Token".to_string()),
                description: Some("Test Token Desc".to_string()),
                media: Some("media link".to_string()),
                copies: Some(1),
                media_hash: None,
                expires_at: None,
                starts_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            },
        }))
        .max_gas()
        .deposit(NearToken::from_near(1))
        .transact()
        .await?;

    assert!(challenge_creation_outcome.is_success());

    let outcome_challenge_exists = contract
        .view("challenge_exists")
        .args_json(json!({
            "store_id":"test-challenge"
        }))
        .await?;

    assert!(outcome_challenge_exists.json::<bool>().unwrap());
    let mut id_prefix = "test-challenge.".to_string();
    id_prefix.push_str(contract.id().as_str());
    let metadata_call = sandbox
        .view(
            &AccountId::from_str(id_prefix.as_str()).unwrap(),
            "get_challenge_metadata",
        )
        .await?;
    let metadata: ChallengeMetaData = metadata_call.json().unwrap();
    assert_eq!(metadata.owner_id, user_account.id().to_string());
    assert_eq!(metadata.name, "Test Challenge!");
    assert_eq!(metadata.description, "A test description");
    assert_eq!(metadata.media_link, Some("A fake media link".to_string()));
    assert_eq!(metadata.reward_nft_id, "reward-nft-id");
    assert_eq!(metadata.challenge_nft_ids, vec!["challenge-nft-id"]);
    assert_eq!(metadata.expiration_date_in_ns, 9007199254740991);
    assert_eq!(metadata.winner_limit, 100);
    assert_eq!(metadata.winners_count, 0);
    assert_eq!(metadata.challenge_completed, false);
    assert_eq!(metadata.creator_can_update, true);
    assert_eq!(
        metadata.reward_nft_metadata.title,
        Some("Test Token".to_string())
    );
    assert_eq!(
        metadata.reward_nft_metadata.description,
        Some("Test Token Desc".to_string())
    );
    assert_eq!(
        metadata.reward_nft_metadata.media,
        Some("media link".to_string())
    );

    Ok(())
}
