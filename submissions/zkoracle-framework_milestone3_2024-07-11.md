# Milestone Delivery

## Content

The submission includes the following information:

- Mintbase Integration and Metadata Research
- **Open-source code/delivery**:
  - X (Twitter) NFTs Game: [https://github.com/usherlabs/x-twitter-nfts](https://github.com/usherlabs/x-twitter-nfts)
  - Framework (integrated into the X NFTs Repo): [https://github.com/usherlabs/x-twitter-nfts/tree/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/zkaf](https://github.com/usherlabs/x-twitter-nfts/tree/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/zkaf)
  - Near NFT Contract: [https://github.com/usherlabs/x-twitter-nfts/tree/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/near_nft](https://github.com/usherlabs/x-twitter-nfts/tree/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/near_nft)
  - Near Verifier Contract: [https://github.com/usherlabs/x-twitter-nfts/tree/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/near_verifier](https://github.com/usherlabs/x-twitter-nfts/tree/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/near_verifier)
- **[License](#license)**
- **[Documentation](#documentation)**
- **[Formatted Code](#formatted-code), adhering to set guidelines**
- **[Testing Guide](#testing-guide)**
- **List of [Milestone Deliverables](#milestone-deliverables)**

## License

Licensed under Apache 2.0.

## Documentation

This milestone focuses on the integration of Mintbase for NFT creation and the research on metadata handling. The NEAR Smart Contract has been expanded to include callback functionality for Mintbase NFT minting. Additionally, research has been conducted on various data availability options for NFT metadata, including on-chain storage, ZK Proof, and Arweave.

The full documentation, detailing the deployment of various contracts and the operation of the ZK Prover, can be found in the [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/README.md). In this documentation, we will highlight the steps to:

1. Integrate verification for NFT creation.
2. Handle metadata using ZK enabled hash check verification, with on-chain metadata storage. Media to be stored off-chain.
3. Verify ZK Proof and metadata hash before minting NFT
4. Integration and unit testing.

### Execution

1. Navigate to the zkaf repo - [src/zkaf](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/zkaf)

2. *(Optional)* [Follow the deployment process outlined in the README.md](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/README.md#smart-contracts)

3. Install pre-requisite software libraries and technologies:
   1. Install Foundry [here](https://book.getfoundry.sh/getting-started/installation)
   2. Install the [necessary toolchain to build the program](https://dev.risczero.com/api/zkvm/install)
   3. Install Docker:
      - To deterministically build the ZK Circuit / Guest, Docker must be running — [Learn more](https://dev.risczero.com/terminology#deterministic-builds)
    
4. Provide all environment variables
   1. See all required environment variables: [src/zkaf/.env.sample](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/zkaf/.env.sample)
   2. If you have not deployed the Smart Contracts using your own wallets, and do not have access to RiscZero's Bonsai managed ZK compute environment, Usher Labs can provide testnet Near and Aurora private keys and associated environment variables on an ad-hoc basis, or provide a live demo of this execution.
    
5. Run the publisher to generate and verify the proof:

   ```shell
   cargo run --bin publisher
   ```

6. Response will indicate a ZK verification and NFT token ID.


## Formatted Code

**The code distribution:**

1. Solidity Smart Contracts are designed to deploy to Aurora to manage the registry of verified zkProofs.
2. Near Smart Contracts and the ZK logic are programmed in strictly typed Rust.
3. Comments are included among various steps in the logic to aid in understanding the flow and processing of data.

## Testing Guide

### Testing Near NFT Smart Contract

1. `cd ./src/near_nft`
2. `cargo test`

### Testing Near Verifier Smart Contract

1. `cd ./src/near_verifier/integration-tests`
2. `cargo test`

### Testing Aurora Smart Contracts

1. `cd ./src/zkaf`
2. `cargo build`
3. `forge build`
4. `forge test`

### A Step-by-Step Guide Demonstrating How Code Achieves the Milestone

A video has been created to walk you through the execution of the project for this milestone. The explanation is provided on how the code/logic achieves the outlined deliverables.

- [🌁 See Architecture Walkthrough](https://www.loom.com/share/846468ac308d4fe5badc96ca9d93c6ea?sid=9e635dd5-08eb-428d-bc1d-4c67a87db760)  
- [🤖 See Code Walkthrough](https://www.loom.com/share/4f77e44f93d6487fa28fac13d90c2cec?sid=8d823fde-a9e9-4f50-8141-c093e5f8bc36)  
- [🦾 See Terminal Demonstration](https://www.loom.com/share/05daa7ac3996442082405ac05db31dec?sid=6b3e246b-d8c9-4bb7-bd65-9192c6a1532d)  

### Unit Tests

Unit tests have been developed for both the Near and Aurora Smart Contracts. Follow the [Testing Guide](#testing-guide) to execute these unit tests.

## Milestone Deliverables

| Number | Deliverable | Specification | Links & Notes |
| --- | --- | --- | --- |
| 0a. | License | Apache 2.0 | [LICENSE](https://github.com/usherlabs/zkaf-r0/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/LICENSE) |
| 0b. | Documentation | Documentation on NEAR Smart Contract callback functionality for Mintbase NFT minting integration and metadata handling research. | [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/README.md?tab=readme-ov-file#near-smart-contract-callback-functionality-for-nfts) |
| 0c. | Testing Guide | Expanded testing suite incorporating NFT creation callback mechanisms. | The ZKAF Testing Guide is outlined in the [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/README.md) and covers unit tests for Smart Contracts on NEAR and Aurora, the deployment guide for these Smart Contracts, and a TL;DR on executing logic to generate zkProofs, verify them, and use verification as authentication for subsequent on-chain processes within NEAR. |
| 0d. | Docker | No Docker updates required for this milestone. | Docker is now reincorporated as a primary mechanism through which zkProofs are built, ensuring deterministic builds. This is a [paradigm set by RiscZero](https://dev.risczero.com/terminology#deterministic-builds) and is specific to the RiscZero ZK Co-processor. |
| 1. | Callback Mechanism & Mintbase Module Integration | Expansion of NEAR Smart Contract callback functionality for NFT creation on Mintbase. | [NEAR Verifier Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/tree/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/near_verifier) |
| 2. | Metadata Handling Research | Please refer to the ZK proof of metadata hash and hash check verification to understand how metadata is directly and verifiably associated with the X Post from the API. This metadata is deemed appropriate for on-chain storage according to this verified paradigm, and Near storage supports such a paradigm. Furthermore, all media items will remain off-chain with references to Arweave or IPFS, to be re-incorporated in the final milestone. | [Research Outcome](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/near_verifier/contract/src/lib.rs#L63) |
| 3. | Unit Testing | Comprehensive development of unit tests for the integrated NFT creation process. | Unit tests are developed for both [Aurora Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/zkaf/tests/Verifier.t.sol), where execution is managed by [Forge](https://book.getfoundry.sh/), and [NEAR Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/blob/1b5f84536baf1de04b4cb58090ba762cdbbead8a/src/near/integration-tests/src/lib.rs), where unit tests are written in Rust. |