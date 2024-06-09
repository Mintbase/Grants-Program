# Milestone Delivery Guidelines

These are the guidelines to be followed for milestones submitted for evaluation.

## Content

The submission should contain the following information:

- zkOracle Framework for Real-world Data
- **Open-source code/delivery**:
  - X (Twitter) NFTs Game: [https://github.com/usherlabs/x-twitter-nfts](https://github.com/usherlabs/x-twitter-nfts)
  - - Framework (currently moved into the X NFTs Repo): [https://github.com/usherlabs/x-twitter-nfts/tree/main/src/zkaf](https://github.com/usherlabs/x-twitter-nfts/tree/main/src/zkaf)
- **[License](#license)**
- **[Documentation](#documentation)**
- **[Formatted code](#formatted-code), according to a set of guidelines**
- **[Testing Guide](#testing-guide)**
- **A list of the [milestone deliverables](#milestone-deliverables)**
- **Any [additional information](#additional-information)**

## License

Licensed under Apache 2.0.

## Documentation

This milestone encapsulates the interation of zkProofs into the Near Blockchain, using Aurora as a proxy for EVM (Groth16) SNARK verification.
An EVM platform is necessary, as currently, the ZK Prover natively generates parameters for Groth16 SNARKs on EVM blockchains.
While zkProof verification directly on Near is viable, and is currently being researched, this is yet to be implemented successfully.
Therefore, as per the original milestone deliverables, the Aurora EVM Blockchain that includes an SDKs for Aurora <> Near communication is used as a means to verify zkProofs, and establish a registry for verified zkProofs that encapsulate verification of social data from X (Twitter).

While the full documentation, detailing deployment of the various contracts and operation of the ZK Prover can be found in the [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/main/README.md), in this documentation we will highlight steps to:

1. generate a zkProof of X (Twitter) data
2. verify the zkProof in an Aurora transaction
3. cross-reference this verification in a Near Smart Contract transaction, in order to authenticate subsequent steps - such as minting an NFT as per standards established by Mintbase.

### Execution

1. Navigate to the zkaf repo
   Moved into X (Twitter) NFTs — https://github.com/usherlabs/twitter_notary/blob/feature/integrate-smart-contracts/src/zkaf
  
2. (Optional) [Follow the deployment process outlined in the README.md](https://github.com/usherlabs/x-twitter-nfts/blob/main/README.md#smart-contracts)

3. Install pre-requisite software libraries and technologies
   1. Install foundry [here](https://book.getfoundry.sh/getting-started/installation)
   2. Install the [necessary tool-chain to build the program](https://dev.risczero.com/api/zkvm/install)
   3. Install Docker
      *To deterministically build the ZK Circuit / Guest, Docker must be running — [Learn more](https://dev.risczero.com/terminology#deterministic-builds)*
    
4. Provide all environment variables and install prerequisites.
    1. See all requires env variables: https://github.com/usherlabs/x-twitter-nfts/blob/main/src/zkaf/.env.sample
    2. If you have not deployed the Smart Contracts using your own wallets, and do not have access to RiscZero's Bonsai managed ZK compute environment, Usher Labs can provide testnet Near and Aurora private keys and associated env variables on an ad-hoc basis, or provide a live demo of this execution.
    
5. Run the publisher to generate and verify the proof.
 
    ```jsx
    cargo run --bin publisher
    ```

## Formatted code

Originally, the [ZKAF R0 (Framework) codebase](https://github.com/usherlabs/zkaf-r0) and the [X (Twitter) NFTs Game](https://github.com/usherlabs/x-twitter-nfts) were separate repositories, establishing the fact that the framework for processing ZK Attestation is modularised and allow applications like the X (Twitter) NFT Game to adopt and use these modules to deliver a end-to-end verifiable data protocol that verifies social data within the Near Blockchain. 

However, for the sake of simplified development, the ZK AF has temporarily been merged into the X (Twitter) NFTs repository allowing for streamlined dependency management and operation. The goal of separating the framework into a standalone dependency of the X (Twitter) NFT Game will be achieved in a future milestone.

**The code distribution:**

1. Solidity Smart Contracts are designed to deploy to Aurora to manage the registry of verified zkProofs 
2. Near Smart Contracts, and the ZK AF logic are programmed in strictly typed Rust. 
3. Comments are included among various steps in the logic to aid in understand the flow and processing of data.

## Testing Guide

### Testing Near Smart Contracts

1. `cd ./src/near/integration-tests`
2. `cargo test`

### Testing Aurora Smart Contracts

1. `cd ./src/zkaf`
2. `cargo build`
3. `forge build`
4. `forge test`

### A step-by-step guide demonstrating how your code achieves the milestones

A video has been created to walk you through execution of the project for this milestone. 
Explaination is provided on how the code/logic achieves the outlined deliverables.

[👉 See Demonstration]()

### Unit tests

Unit test have been developed for both the Near and Aurora Smart Contracts.
Follow the [Testing Guide](#testing-guide) to execute these unit tests.

## Milestone Deliverables



| Number | Deliverable | Specification | Links & Notes |
| --- | --- | --- | --- |
| 0a. | License | Apache 2.0 | [LICENSE](https://github.com/usherlabs/zkaf-r0/blob/8f54c1c563220db80bf07620d11d46fa1a8fe988/LICENSE) |
| 0b. | Documentation | Documentation on deploying NEAR and Aurora Smart Contracts to respective testnets and their integration using Aurora Contract SDK. | [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/main/README.md) |
| 0c. | Testing Guide | Testing guide for Smart Contract functionality, with a focus on Docker operation for ZK Proof generation. | ZKAF Testing Guide is outlined in the [README.md](https://github.com/usherlabs/x-twitter-nfts/blob/main/README.md) both for unit tests of Smart Contracts across Near and Aurora, the deployment guide for this Smart Contracts, and TL;DR covers execution of logic that generates zkProof, verifies it, and uses verification as authentication for subsequent onchain process within Near. |
| 0d. | Docker | No updates in Docker setup; existing environment used for test proof generation. | Unlike the deliverable specification, Docker is now reincorporated as a primary mechanism through which zkProofs are built, as it ensures deterministic builds. This is a [paradigm set by RiscZero](https://dev.risczero.com/terminology#deterministic-builds), and this limitation is specific to the RiscZero ZK Co-processor. |
| 1. | Verification Smart Contracts | ZK Verification Smart Contracts leveraging the existing EVM ~~Placeholder~~ Proof System Verification | Placeholder replace with [R0's Groth16 Verification](https://github.com/usherlabs/x-twitter-nfts/blob/main/src/zkaf/contracts/Verifier.sol) |
| 2. | NEAR Contract Integration | Integration of Aurora Smart Contract functionality through a NEAR Smart Contract entry point. | [Near Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/blob/main/src/near/contract/src/lib.rs) |
| 3. | Smart Contract Deployment | Deployment of ZK Verification Smart Contracts to Aurora Testnet, and deployment of NEAR Smart Contracts to NEAR Testnet | The ["Publisher"](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/apps/src/bin/publisher.rs#L11) encapsulates the zkProof generatation, [verification on Aurora](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/apps/src/aurora.rs), and [cross-referencing the verification within Near](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/zkaf/apps/src/near.rs). Deployment of EVM Contracts is managed by [Forge](https://book.getfoundry.sh/). Near Smart Contracts are deployed using the Near CLI. This is detailed in the documentation.  |
| 4. | Testing Framework | Development of a test-driven development framework for Smart Contract functionality. | Unit Tests are developed for both [Aurora Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/blob/main/src/zkaf/tests/Verifier.t.sol), where execution is managed by [Forge](https://book.getfoundry.sh/), and [Near Smart Contracts](https://github.com/usherlabs/x-twitter-nfts/blob/9a10a819d982c6ff0d90238f636051cf8d069f13/src/near/integration-tests/src/lib.rs), where unit tests are essentially Rust Unit Tests |

## Additional Information
