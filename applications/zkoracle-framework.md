# Mintbase Grant Proposal

- **Project Name:** zkOracle Framework for Real-world Data
- **Team Name:** Usher Labs
- **Payment Address:** 5fd9395dbfffbbf9388724cf65a9f3d918e75b5115c36d2c9ac964a1a3f7bcd6
- **Level:** 2

## Project Overview 📄

**Tag Line:** Streamlining Real-world Data Verification for Smart Contracts

**Brief Description:**
Usher Labs is embarking on an innovative project to develop a zkOracle Framework, designed to facilitate the creation of tailored ZKOracles that authenticate real-world data. This framework aims to empower Aurora and NEAR Smart Contracts to reliably verify cryptographic proofs pertaining to private data sourced from centralised operations or TLS-enabled HTTP endpoints. To showcase the framework's capabilities, we plan to implement a simple NFT project on Mintbase, where Tweets are minted with cryptographic assurance of uniqueness, preventing duplicate minting of the same tweet.

**Integration into the Mintbase / NEAR Ecosystem:**
Our project is intrinsically linked to the Mintbase and NEAR ecosystem. By enabling the minting of unique Tweet-based NFTs on Mintbase, we demonstrate the practical utility of our zkOracle Framework within the NEAR environment. This integration showcases how real-world data, verified via our framework, can enhance the authenticity and uniqueness of NFTs, adding tangible value to the Mintbase platform.

**Reason for Interest:**
Our motivation stems from Usher Labs' ongoing efforts in the Transparency Network. Our Transparency (T) Network addresses transparency challenges in centralised operations through the T Node, which enriches off-chain data with cryptographic proofs. The T Node functions as a Database, HTTP request Prover, and Proxy Interface, operating in tandem with our T (Notary) Network. It securely processes first-party data, private API/HTTP data, and third-party partner data. With our interest in zero-knowledge architecture, we aim to extend this functionality, enabling the proof of diverse real-world data on-chain, such as KYC/Identity and financial information. This project is a natural extension of our existing work and aligns with our vision to enhance data integrity and privacy on the blockchain.

### Project Details

**UI/Architecture Documentation:**

- **UI Mockups**: We have prepared detailed mockups for the dApp interface, designed for minting Tweet NFTs. These mockups provide a clear and intuitive user experience.
    
    ![151763928-ac7e9eb7-dc62-4b24-b9b4-1316e9bcbe39.png](https://user-images.githubusercontent.com/63192115/151763928-ac7e9eb7-dc62-4b24-b9b4-1316e9bcbe39.png)
    
- **Solution Architecture**: A comprehensive architectural diagram is available, illustrating the integration of various components within our solution.
    
    https://miro.com/app/board/uXjVN-U5nDI=/?share_link_id=681340344130
    
- **Video Walkthrough**: For a deeper understanding, we offer a video walkthrough that explains the solution’s architecture and its functionalities.
    
    https://www.loom.com/share/2e9d3738568446a3a8511a44e35d30ca?sid=783a0a02-d77c-422f-99b7-7d26b8945827
    

**Technology Stack:**

- **Transparency (T) Node and Network**: At the core of our project is the T Node, an advanced version of our already deployed Log Store Node. This technology specialises in generating cryptographic proofs for data, both published to and retrieved from, the network and HTTP endpoints.
- **Notary Proof for Twitter Data**: A key feature includes generating Notary Proofs for data obtained from Twitter's V2 API, ensuring the authenticity of NFTs representing Twitter data and their uniqueness based on Tweet IDs.
- **zkOracle Framework**: The framework will utilise ZKLLVM, a circuit compiler enabling Rust programming language to be compiled into ZK Circuits. We will leverage the Placeholder Proof System for verification within EVM environments.
- **Smart Contract Integration**: Verification Smart Contracts will be deployed on Aurora, with integration into NEAR Smart Contracts for NFT minting and management via the Aurora Contracts SDK.
- **User Interface dApp**: A user-friendly Next.js based dApp will be developed, using existing open-source libraries for swift implementation. This dApp will facilitate the minting process of Twitter-based NFTs.

**Proof of Concept and Prior Work:**
Usher Labs has a strong track record, evident in our open-source libraries showcasing expertise in Rust programming, Smart Contract deployment across various blockchains, and Node and Network architecture development, exemplified by the Log Store.

**Project Scope and Expectations:**

- **Primary Focus**: The project is primarily aimed at providing developers with a streamlined zkOracle to authenticate real-world data on-chain.
- **Demonstration via Twitter and Mintbase**: While the creation of unique Twitter NFTs on Mintbase serves as an intriguing use case, it is primarily a means to demonstrate the broader utility of bridging real-world data with the NEAR blockchain.
- **Limitations**: It's important to note that while our project might catalyse a novel NFT collection on Mintbase, our main focus is on enhancing developer tools for real-world data verification in the NEAR ecosystem.

### Ecosystem Fit

**Integration into the Mintbase and NEAR Ecosystem:**
Our project is strategically positioned to enhance the NEAR blockchain, aligning with its goal of evolving into a comprehensive Blockchain Operating System. By enabling the verification of real-world data on-chain, our solution bridges the gap between familiar web2 services and the innovative capabilities of web3. Our initial demonstration via Mintbase, involving the creation of unique Twitter-derived NFTs, is a testament to this integration. This approach not only introduces an element of novelty and scarcity in the NFT space but also paves the way for advanced crypto payments and exchange functionalities integrated with social media platforms like Twitter.

**Target Audience:**
While our project might initially appeal to NFT enthusiasts intrigued by the novelty of Twitter-based NFTs, our core target audience extends far beyond. We are primarily focused on Web3 developers and enterprises looking to integrate real-world data verification within their blockchain applications. Our technology serves as a bridge for these entities to incorporate public and private data from traditional web2 platforms, enhancing their blockchain applications' functionality and security.

**Meeting Market Needs:**
Our project addresses a crucial need in the blockchain ecosystem – the ability to verify real-world data in a transparent, secure, and cryptographically sound manner. This opens up new avenues for businesses to securely interact with traditional platforms like KYC, banking, finance, and ticketing, all while ensuring data integrity and security. Such capabilities are pivotal in advancing the practical use cases of blockchain technology in everyday business operations.

**Comparison with Existing Projects:**
To our knowledge, there are no direct counterparts within the NEAR or Mintbase ecosystems that offer similar functionalities. However, in the broader blockchain and Web3 domain, projects like HyperOracle, PadoLabs, and ZKPass have ventured into related areas. What sets our project apart is our specific focus on the NEAR ecosystem and our unique approach to integrating Twitter data into NFTs, providing a practical demonstration of our technology's potential. Our solution not only adds value to the NEAR blockchain in terms of data verification capabilities but also innovates in how social media interactions can be transformed into valuable, verifiable digital assets.

## Team 👥

### Team members

**Team Lead:** Ryan Soury — Solutions Architect, Engineer, Founder

**Team Members:**

- Victor Shevtsov — DevOps, Infrastructure, Senior Engineer
- Shuaibu Alexander — Smart Contracts, On-chain, Research Engineer
- Raffael Campos — Full-Stack, Infrastructure, Wildcard Engineer

### Contact

- **Contact Name:** Ryan Soury
- **Contact Email:** ryan@usher.so
- **Website: https://www.usher.so** *(due for an update)*

### Legal Structure

- **Registered Address:** 8 The Green STE R, Dover, DE 19901, USA
- **Registered Legal Entity:** Usher Labs Inc.

### Team's experience

**Introduction to the Team:**
Our project is spearheaded by a talented and geographically diverse team, hailing predominantly from Australia but with a global outreach and influence. Each team member contributes a unique blend of skills and experiences, essential for the multifaceted demands of our innovative project. Below is an overview of our core team members:

1. **Ryan Soury**:
    - **Role**: Team Lead and Solutions Architect
    - **Experience**: Ryan is the creative force behind the development of the Log Store, a decentralised time-series database crucial for securely storing and serving event data. His expertise in blockchain technology and a history of successful project implementations make him an ideal leader for our project. Ryan’s innovative approach is the driving force behind our solution's architecture and strategic direction.
2. **Victor**:
    - **Role**: DevOps and Infrastructure Specialist
    - **Experience**: Victor’s journey in the Web3 realm has been marked by a passion for Rust programming, which he has adeptly applied to Smart Contract development. His work includes a proprietary project that utilised the capabilities of Solana and Wormhole, showcasing his skills in infrastructure operation, uptime management, and data engineering. His expertise in DevOps is vital for the robust and efficient infrastructure of our project.
3. **Alex**:
    - **Role**: Smart Contract Developer and Web3 Expert
    - **Experience**: With a rich background in Web3 startups, including Push Protocol (formerly EPNS), Alex brings comprehensive expertise in Smart Contract development, review, and testing. His involvement extends to dApp development, providing invaluable insights into the Web3 landscape. Alex’s holistic understanding of Smart Contracts and dApp ecosystems is instrumental for the technical execution of our project.
4. **Raffael**:
    - **Role**: Full-Stack Developer and DevOps Engineer
    - **Experience**: Raffael’s proficiency spans across application engineering, DevOps, and cloud infrastructure. His rapid adaptation to Web3 technologies, combined with direct experience in protocol and cryptographic schemes at Usher Labs, makes him a versatile and key contributor to our project. His ability to manage complex systems ensures smooth operation and deployment.

**Team's Collective Expertise**:
Our team boasts over three years of dedicated experience in the Web3 industry, enriched by a broader background in traditional Web2 development. This diverse expertise enables us to navigate the intricate requirements of our project effectively. Our deep understanding of cryptographically verifiable data engineering, coupled with extensive experience in Smart Contract development across both EVM and Rust-based chains, uniquely positions us to tackle the challenges of this project and drive it towards success.

### Team Code Repos

- https://github.com/usherlabs
- https://github.com/usherlabs/logstore/
- https://github.com/usherlabs/logstore-node/
- https://github.com/usherlabs/ccamp/
- https://github.com/usherlabs/indexer-relayer
- https://github.com/usherlabs/verifiable-data-streams

**GitHub accounts of all team members**

- https://github.com/rsoury/
- https://github.com/victorshevtsov/
- https://github.com/Xand6r/
- https://github.com/outerlook

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/ryan-soury-904072100/
- https://www.linkedin.com/in/victor-shevtsov-3a772258
- https://www.linkedin.com/in/alex-ander-shuaibu-28889b17a/

## Development Status 📖

- SIPs for Streamr Network:
  - [SIP-19](https://vote.streamr.network/#/proposal/0x615d603fd1f4e5c541f282081679ec3ae359c93b533ab9874e335c3e212c3af3)
  - [SIP-13](https://vote.streamr.network/#/proposal/0x87a023191e4525265c0f1c3a68b5b63e4dd758e8dbbdec2cb3ec721828773f1c)
- [Documentation](https://docs.logstore.usher.so/)
- [Tutorial](https://docs.logstore.usher.so/network/quick-start/tutorial)
- [Research Papers](https://docs.logstore.usher.so/network/overview/paper)
- [T Node for Stellar](https://www.loom.com/share/54611c7b984d412db9e3c96cde0e5281?sid=644e8832-acac-4ba7-9ddb-aeb71790732b)
- [Discussion on Asset Management in ICP](https://forum.dfinity.org/t/introducing-ccamp-unlocking-cross-chain-defi-aggregation-on-the-internet-computer/24643)
- [Discussion on Verifiability of TheGraph](https://forum.thegraph.com/t/empowering-subgraphs-with-verifiable-confirmations/4738/5)

**Investors Materials**:
Investor Deck and Company Overview Brief can be shared.

## Development Roadmap 🔩

### Overview

- **Total Estimated Duration:** 5 months
- **Full-Time Equivalent (FTE):** 2
- **Total Costs:** $50,000 USD

### Milestone 1: Initial zkProof Generation and Verification — $10,000

- **Estimated Duration:** 1 month
- **FTE:** 2
- **Costs:** $10,000

| Number | Deliverable | Specification |
| --- | --- | --- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | Inline documentation and a basic tutorial explaining how to set up the ZK Proof Generation environment and generate sub-proofs using the Transparency Node. |
| 0c. | Testing Guide | Guide on setting up and running unit tests for the initial zkProof generation logic. |
| 0d. | Docker | Dockerfile for setting up the ZK Proof Generation environment. Docker Compose environment for zkProof Generation, including separate Dev and Prod Compose Files. The Prod Compose File includes a service for the Transparency Node. |
| 1. | ZK Proof Generation Logic | Development of boilerplate logic for ZK Proof Generation of sub-proofs produced by the Transparency Node. |
| 2. | Notary Proofs from Twitter API | Implementation of logic to generate Notary Proofs for data fetched from the Twitter API. |
| 3. | ZK Circuit Verification | Development of a ZK Circuit for verifying the Notary Proofs (Sub-proofs). |

### Milestone 2: Smart Contract Development and Testing — $10,000

- **Estimated Duration:** 1 month
- **FTE:** 2
- **Costs:** $10,000

| Number | Deliverable | Specification |
| --- | --- | --- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | Documentation on deploying NEAR and Aurora Smart Contracts to respective testnets and their integration using Aurora Contract SDK. |
| 0c. | Testing Guide | Testing guide for Smart Contract functionality, with a focus on Docker operation for ZK Proof generation. |
| 0d. | Docker | No updates in Docker setup; existing environment used for test proof generation. |
| 1. | Verification Smart Contracts | ZK Verification Smart Contracts leveraging the existing EVM Placeholder Proof System Verification |
| 2. | NEAR Contract Integration | Integration of Aurora Smart Contract functionality through a NEAR Smart Contract entry point. |
| 3. | Smart Contract Deployment | Deployment of ZK Verification Smart Contracts to Aurora Testnet, and deployment of NEAR Smart Contracts to NEAR Testnet |
| 4. | Testing Framework | Development of a test-driven development framework for Smart Contract functionality. |

### Milestone 3: **Mintbase Integration and Metadata Research** — $10,000

- **Estimated Duration:** 1 month
- **FTE:** 2
- **Costs:** $10,000

| Number | Deliverable | Specification |
| --- | --- | --- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | Documentation on NEAR Smart Contract callback functionality for Mintbase NFT minting integration and metadata handling research. |
| 0c. | Testing Guide | Expanded testing suite incorporating NFT creation callback mechanisms. |
| 0d. | Docker | No Docker updates required for this milestone. |
| 1. | Callback Mechanism & Mintbase Module Integration | Expansion of NEAR Smart Contract callback functionality for NFT creation on Mintbase. |
| 2. | Metadata Handling Research | Research on data availability options for NFT metadata, including on-chain, in ZK Proof, and Arweave storage. |
| 3. | Unit Testing | Comprehensive development of unit tests for the integrated NFT creation process. |

### Milestone 4: Refinement and Modularisation — $10,000

- **Estimated Duration:** 1 month
- **FTE:** 2
- **Costs:** $10,000

| Number | Deliverable | Specification |
| --- | --- | --- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | Documentation detailing the separation of the zkOracle Framework and Twitter NFTs Example, including Docker service separation. |
| 0c. | Testing Guide | Distinguishing testing procedures for zkOracle Framework and Twitter NFTs Example. |
| 0d. | Docker | Docker setup update to separate services by their respective purposes. |
| 1. | ZK Circuit Refinement | Refinement of the ZK Circuit for verifying Twitter Data Sub-proofs. |
| 2. | Framework Modularisation | Modularisation of the zkOracle Framework to abstract from the Twitter API Integration. |
| 3. | Orchestration Layer Development | Development of an Orchestration Layer for the zkOracle Framework, facilitating frontend feedback. |

### Milestone 5: NFT Minting dApp Deployment and Testing — $10,000

- **Estimated Duration:** 1 month
- **FTE:** 2
- **Costs:** $10,000

| Number | Deliverable | Specification |
| --- | --- | --- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | Comprehensive documentation on the modified NFT minting dApp, including integration details with NEAR/Mintbase libraries. Reference to third-party packages used. |
| 0c. | Testing Guide | End-to-end testing guide for the NFT minting process with zkProof verification. |
| 0d. | Docker | Deployment of Docker environment for the final dApp. |
| 0e. | Article | Publication detailing the project's achievements, impact on the Mintbase, NEAR, and wider blockchain ecosystems. |
| 1. | dApp Modification and Integration | Modification and deployment of an existing open-source NFT Minting dApp, incorporating NEAR/Mintbase libraries for minting NFTs with zkProof verification from the zkOracle. Reference to third-party packages like https://github.com/AnishDe12020/twnft. |
| 2. | Smart Contract Deployment | Deployment of the complete suite of Smart Contracts to Mainnets. |
| 3. | E2E Testing | Conducting end-to-end tests to ensure unique Twitter Data minting and listing on the Mintbase Marketplace. |

## Future Plans
In the short term, Usher Labs plans to utilise and promote the zkOracle Framework as a foundational tool for enterprises integrating blockchain technology, particularly focusing on the verification of real-world data on-chain. Our promotion strategy will involve direct sales, partnerships, and highlighting the potential of NEAR as a Layer 1 blockchain. We also aim to promote the Twitter NFTs collection through co-marketing efforts with Mintbase and other NFT communities, exploring revenue-sharing models with experienced marketers. In the long term, we are committed to continually enhancing the framework, fostering a community around it, and staying at the forefront of blockchain technology advancements. We will explore expanding the integration of social, financial, or identity data into blockchain, depending on the success of our initial project with Mintbase.

## Additional Information ➕
Usher Labs learned about the Mintbase Grants Program through a referral from a previous NEAR Foundation Lead. As a capital-backed research and development lab, we specialise in addressing transparency challenges in centralised operations. Our team has a strong track record of successful partnerships and grant acquisitions from notable organisations in the blockchain space, such as Arweave, Streamr, Stellar, DFINITY, and Kyve. This history highlights our commitment to innovation and our capability to contribute significantly to blockchain ecosystems, including NEAR.
