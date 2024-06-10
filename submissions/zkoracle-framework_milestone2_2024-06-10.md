# Milestone Delivery

## Content

The submission includes the following information:

- zkOracle Framework for Real-World Data
- **Open-source code/delivery**:
  - X (Twitter) NFTs Game: [https://github.com/usherlabs/x-twitter-nfts](https://github.com/usherlabs/x-twitter-nfts)
  - Framework (integrated into the X NFTs Repo): [https://github.com/usherlabs/x-twitter-nfts/tree/main/src/zkaf](https://github.com/usherlabs/x-twitter-nfts/tree/main/src/zkaf)
- **[License](#license)**
- **[Documentation](#documentation)**
- **[Formatted Code](#formatted-code), adhering to set guidelines**
- **[Testing Guide](#testing-guide)**
- **List of [Milestone Deliverables](#milestone-deliverables)**

## License

Licensed under Apache 2.0.

## Documentation

This milestone encapsulates the integration of zkProofs into the Near Blockchain, using Aurora as a proxy for EVM (Groth16) SNARK verification. An EVM platform is necessary because the ZK Prover currently generates parameters for Groth16 SNARKs on EVM blockchains.

While zkProof verification directly on Near is feasible and under research, it has not yet been successfully implemented. Therefore, as per the original milestone deliverables, the Aurora EVM Blockchain, which includes SDKs for Aurora <> Near communication, is used to verify zkProofs and establish a registry for verified zkProofs that encapsulate the verification of social data from X (Twitter).

The full documentation, detailing the deployment of various contracts and the operation of the ZK Prover, can be found in the [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/README.md). In this documentation, we will highlight the steps to:

1. Generate a zkProof of X (Twitter) data.
2. Verify the zkProof in an Aurora transaction.
3. Cross-reference this verification in a Near Smart Contract transaction to authenticate subsequent steps, such as minting an NFT according to standards established by Mintbase.

### Execution

1. Navigate to the zkaf repo
   - Moved into X (Twitter) NFTs — [src/zkaf](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf)

2. *(Optional)* [Follow the deployment process outlined in the README.md](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/README.md#smart-contracts)

3. Install pre-requisite software libraries and technologies:
   1. Install Foundry [here](https://book.getfoundry.sh/getting-started/installation)
   2. Install the [necessary toolchain to build the program](https://dev.risczero.com/api/zkvm/install)
   3. Install Docker:
      - To deterministically build the ZK Circuit / Guest, Docker must be running — [Learn more](https://dev.risczero.com/terminology#deterministic-builds)
    
4. Provide all environment variables
   1. See all required environment variables: [src/zkaf/.env.sample](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/.env.sample)
   2. If you have not deployed the Smart Contracts using your own wallets, and do not have access to RiscZero's Bonsai managed ZK compute environment, Usher Labs can provide testnet Near and Aurora private keys and associated environment variables on an ad-hoc basis, or provide a live demo of this execution.
    
5. Run the publisher to generate and verify the proof:
 
    ```sh
    cargo run --bin publisher
    ```

## Formatted Code

Originally, the [ZKAF R0 (Framework) codebase](https://github.com/usherlabs/zkaf-r0) and the [X (Twitter) NFTs Game](https://github.com/usherlabs/x-twitter-nfts) were separate repositories, demonstrating that the framework for processing ZK Attestation is composable. This allows applications like the X (Twitter) NFT Game to adopt and use these modules to deliver an end-to-end verifiable data protocol that verifies social data within the Near Blockchain. 

However, for simplified development, the [ZKAF has temporarily been merged into the X (Twitter) NFTs repository](https://github.com/usherlabs/x-twitter-nfts/tree/main/src/zkaf), allowing for streamlined dependency management and operation. The goal of separating the framework into a standalone dependency of the X (Twitter) NFT Game will be achieved in a future milestone.

**The code distribution:**

1. Solidity Smart Contracts are designed to deploy to Aurora to manage the registry of verified zkProofs.
2. Near Smart Contracts and the ZKAF logic are programmed in strictly typed Rust.
3. Comments are included among various steps in the logic to aid in understanding the flow and processing of data.

## Testing Guide

### Testing Near Smart Contracts

1. `cd ./src/near/integration-tests`
2. `cargo test`

### Testing Aurora Smart Contracts

1. `cd ./src/zkaf`
2. `cargo build`
3. `forge build`
4. `forge test`

### A Step-by-Step Guide Demonstrating How Code Achieves the Milestone

A video has been created to walk you through the execution of the project for this milestone. The explanation is provided on how the code/logic achieves the outlined deliverables.

[👉 See Demonstration](https://www.loom.com/share/09ab894833584744aefb58349b7659f4?sid=e56d8cc7-1722-4a02-bc32-e119b5263b9d)

### Unit Tests

Unit tests have been developed for both the Near and Aurora Smart Contracts. Follow the [Testing Guide](#testing-guide) to execute these unit tests.

## Milestone Deliverables

| Number | Deliverable | Specification | Links & Notes |
| --- | --- | --- | --- |
| 0a. | License | Apache 2.0 | [LICENSE](https://github.com/usherlabs/zkaf-r0/blob/8f54c1c563220db80bf07620d11d46fa1a8fe988/LICENSE) |
| 0b. | Documentation | Documentation on deploying NEAR and Aurora Smart Contracts to respective testnets and their integration using Aurora Contract SDK. | [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/README.md) |
| 0c. | Testing Guide | Testing guide for Smart Contract functionality, with a focus on Docker operation for ZK Proof generation. | The ZKAF Testing Guide is outlined in the [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/README.md) and covers unit tests for Smart Contracts on NEAR and Aurora, the deployment guide for these Smart Contracts, and a TL;DR on executing logic to generate zkProofs, verify them, and use verification as authentication for subsequent on-chain processes within NEAR. |
| 0d. | Docker | No updates in Docker setup; existing environment used for test proof generation. | Docker is now reincorporated as a primary mechanism through which zkProofs are built, ensuring deterministic builds. This is a [paradigm set by RiscZero](https://dev.risczero.com/terminology#deterministic-builds) and is specific to the RiscZero ZK Co-processor. |
| 1. | Verification Smart Contracts | ZK Verification Smart Contracts leveraging the existing EVM Proof System Verification. | Placeholder replaced with [R0's Groth16 Verification](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/contracts/Verifier.sol) |
| 2. | NEAR Contract Integration | Integration of Aurora Smart Contract functionality through a NEAR Smart Contract entry point. | [NEAR Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/near/contract/src/lib.rs) |
| 3. | Smart Contract Deployment | Deployment of ZK Verification Smart Contracts to Aurora Testnet and deployment of NEAR Smart Contracts to NEAR Testnet. | The ["Publisher"](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/apps/src/bin/publisher.rs#L11) encapsulates zkProof generation, [verification on Aurora](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/apps/src/aurora.rs), and [cross-referencing the verification within NEAR](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/apps/src/near.rs). Deployment of EVM Contracts is managed by [Forge](https://book.getfoundry.sh/). NEAR Smart Contracts are deployed using the NEAR CLI, as detailed in the documentation. |
| 4. | Testing Framework | Development of a test-driven development framework for Smart Contract functionality. | Unit tests are developed for both [Aurora Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/tests/Verifier.t.sol), where execution is managed by [Forge](https://book.getfoundry.sh/), and [NEAR Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/near/integration-tests/src/lib.rs), where unit tests are written in Rust. |
