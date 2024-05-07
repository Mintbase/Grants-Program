# Milestone Delivery Guidelines

These are the guidelines to be followed for milestones submitted for evaluation.

## Content

The submission should contain the following information:

- zkOracle Framework for Real-world Data
- **Open-source code/delivery**:
  - Framework: [https://github.com/usherlabs/zkaf-r0](https://github.com/usherlabs/zkaf-r0)
  - X (Twitter) NFTs Game: [https://github.com/usherlabs/x-twitter-nfts](https://github.com/usherlabs/x-twitter-nfts)
  - Framework (on legacy proving system): [https://github.com/usherlabs/zkaf](https://github.com/usherlabs/zkaf)
- **[License](#license)**
- **[Documentation](#documentation)**
- **[Formatted code](#formatted-code), according to a set of guidelines**
- **[Testing Guide](#testing-guide)**
- **A list of the [milestone deliverables](#milestone-deliverables)**
- **Any [additional information](#additional-information)**

## License

Licensed under Apache 2.0.

## Documentation

1. Instructions for generating TLS Proofs are found in [https://github.com/usherlabs/x-twitter-nfts](https://github.com/usherlabs/x-twitter-nfts)
   1. Configure X (Twitter) API v2 Keys and Conversation/Tweet ID in `./src/twitter/.env`
   2. Start the Notary Server - *This server runs locally for testing purposes, but will be offered by Usher Labs' decentralised data security network for production environments.*
      ```shell
       ./start_notary_server.sh
      ```
   3. Generate Twitter TLS Proof
      ```shell
       ./generate_twitter_proof.sh
      ```
2. Clone the framework - `git clone https://github.com/usherlabs/zkaf-r0`
3. If you've created a new TLS Proof, load it into the `./host/fixtures/` directory
4. Execute the ZK Circuit locally with `RISC0_DEV_MODE=1 cargo run --release`

## Formatted code

The [ZKAF R0 (Framework) codebase](https://github.com/usherlabs/zkaf-r0) and the X (Twitter) NFTs Game which involves the TLS Proof generation are both programmed in strictly typed Rust, for the most part. Comments are included among various steps in the logic to aid in understand the flow and processing of data.

The currently archived/deprecated [ZKAF (on zkLLVM) codebase](https://github.com/usherlabs/zkaf) includes Typescript for handling Smart Contract dependencies issues, however, is no longer relevant to demonstrate the working deliverables for this milestone.

## Testing Guide

Currently, tests are performed rudamentarily by executing the programs and ensure that there is a valid response through human evaluation.

### A step-by-step guide demonstrating how your code achieves the milestones

Two videos have been created to walk you through the project, and how it achieves the outlined milestones.

1. [A quick overview](https://www.loom.com/share/e5875772d9dd460facacd8daf5f7f8bf?sid=ad222b18-a9a7-42b4-a845-76c8b7a9bd65)
2. [An in-depth explainer](https://www.loom.com/share/5cdf166a728f43a995eecd79fe1e716d?sid=e2ee0a55-180e-40e8-b7ca-58629b25cfe0)
   1. The in-depth video explains why we switched from zkLLVM to RiscZero, for now.

### Unit tests

Unit test boilerplate has been established and leveraged where components are complex - specifically [within the custom `tls-substrings-verifier` module](https://github.com/usherlabs/zkaf-r0/blob/8f54c1c563220db80bf07620d11d46fa1a8fe988/tlsn-substrings-verifier/src/proof/substrings.rs#L331) that we developed to be compatibile with the ZK proving systems.

## Milestone Deliverables

Please provide a list of milestone deliverables. This list should closely reflect the list of deliverables agreed on in the Pull Request for the public **Grants Program** application or in Annex 1 of the grant contract for the private applications.

Each item in the list should include a link to the deliverable itself, e.g.:

- Google Doc link - make sure anyone with the link has View access
- GitHub repository - include the appropriate file/folder in the link

**Please highlight anything that deviates from the contract** and include further information that you think is relevant to the deliverable, either alongside the appropriate deliverable or under [Additional Information](#additional-information).

Please ensure the repo has the correct open-source license.

| Number | Deliverable | Links & Notes |
| --- | --- | --- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | [LICENSE](https://github.com/usherlabs/zkaf-r0/blob/master/LICENSE)  |
| 0c. | Testing Guide | [ZKAF Testing Guide](https://github.com/usherlabs/zkaf-r0/blob/master/README.md) and [TLS Proof Guide](https://github.com/usherlabs/x-twitter-nfts/blob/main/README.md) |
| 0d. | Docker | [Docker setup guide for deprecated ZKAF](https://github.com/usherlabs/zkaf?tab=readme-ov-file#build-the-image). Docker is incompatible with newer ZK proving systems due to GPU accelerated hardware requirements. |
| 1. | ZK Proof Generation Logic | [Development of boilerplate logic for ZK Proof Generation](https://github.com/usherlabs/zkaf-r0/blob/master/host/src/main.rs) of [sub-proofs produced by the Transparency Node](https://github.com/usherlabs/zkaf-r0/blob/master/methods/guest/src/main.rs). |
| 2. | Notary Proofs from Twitter API | [Implementation of logic](https://github.com/usherlabs/x-twitter-nfts/blob/main/src/twitter/src/main.rs) to generate [Notary Proofs](https://github.com/usherlabs/zkaf-r0/blob/master/host/fixtures/twitter_proof.json) for data fetched from the Twitter API. |
| 3. | ZK Circuit Verification | [Development of a ZK Circuit](https://github.com/usherlabs/zkaf-r0/blob/master/methods/guest/src/main.rs) for [verifying the Notary Proofs (Sub-proofs)](https://github.com/usherlabs/zkaf-r0/blob/master/host/src/main.rs#L39). |

## Additional Information

1. The project has been renamed to ZK Attestation Framework, from ZKOracle Framework, as this better depicts the value and purpose of the technology - in that it works to succinctly prove processing over verified attestations of real-world data.
2. The architecture of the project was designed with zkLLVM proving system in mind, but limitations of this nascent technology have forced us to explore alternatives to deliering on the same zero-knowledge protocol to converting proofs about the real-world into verified data points for verification on the Near / Aurora blockchains.