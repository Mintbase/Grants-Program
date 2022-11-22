# Mintbase Grant Proposal

- **Project Name:** RentVerse
- **Team Name:** RentVerse Team
- **Payment Address:** libos.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

<!-- If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section. -->

Our project is a follow-up of our winning project in NEARCON 2022 IRL Hackathon: [RentVerse](https://devpost.com/software/nft-renting-platform). A significant amount of new features and designs have been added to support our application for this grant.

### Overview

RentVerse is a collateral-free utility NFT rental service built on NEAR.

**About RentVerse**
  
  We propose to continue building an easy-to-use tool for users to rent/hire utility NFTs (e.g. in-game assets, virtual passes) in a collateral-free approach. We plan to use this grant to support implementing important features (such as creator's royalty, multi FTs payment), optimizing contracts, improving service UX, and potentially integrating with real-life applications. Ideally, we will build our service to be production ready. 

**Relevance to Mintbase and NEAR** 
  - Mintbase's NFT indexer is used in our project for retrieving NFT information, which helps to provide a smooth user experience.
  - Our NFT rental function will benefit Mintbase by creating a widely needed rental market derived from Mintbase created NFTs. 
  - NEAR smart contracts are used to handle the core rental business logic. The team also has plans to leverage the experience of building our rental product to propose a high-quality rental protocol on NEAR, which will benefit the ecosystem. 
  - Our service and SDK will enable NEAR dapps and games to seamlessly integrate NFT Rental functions with miniumal code intrusion. No code change is needed from original NFT contracts (as long as they are in line with the [Near NFT standard](https://nomicon.io/Standards/Tokens/NonFungibleToken/)).
 

**Team's Motivation**
  - Both team members are seasoned engineering professionals with a strong interest in blockchain and web3 technologies. We have been closely following blockchain technology since ~2016 and have participated in several related projects in our spare time (details in the Team section). We are committed to building truly useful products for an open web world.
  - We believe that renting is an essential feature of a sustainable NFT market. It enables NFT owners to make more efficient use of their digital belongings and allows borrowers to experience the NFT inexpensively. Rental service will also help develop more use cases for NFT and help to adopt more users to NFT and Metaverse.
  - The constant expanding adoption of NFTs in and beyond NEAR ecosystem provides a solid market space for this derived rental market. NEAR native solution is currently limited. The team is aiming to provide highly functional NEAR native solutions.


### Project Details

- The current UI and designs are covered in our MVP demo and deployed on testnet. (links attached below) 
- Data models / API specifications of the core functionality
  - Rental contract:
    - Uses `nft_on_approve` callback to initiate a new lease after the lender approves the contract for their NFT.
    - A function `lending_accept` will be called (with the sufficient amount of rent paid) by the lender to accept and start the lease.
    - A function `claim_back` will be called by the lender after the lease expires to retrieve the asset.
    - A function `get_borrower` can be called by anyone to check who is the current rightful borrower of a given NFT.
  - A marketplace design is WIP.
- Technology Stack:
  - Smart Contracts: Rust, NEAR SDK, Mintbase API
  - Front End: React
  - Hosting: Static CDN (e.g. Netlify)
- PoC/MVP or other relevant prior work or research on the topic
  - NEARCON IRL Hackathon submission: https://devpost.com/software/nft-renting-platform
  - Demo vedio: https://vimeo.com/749514302
  - Testnet MVP app: https://rentverse.netlify.app/



### Ecosystem Fit

**Where and how does our project fit into the ecosystem?**
  - First, our project **allows users to experience NFT-enabled Web3 features affordably**, by removing the upfront capital barrier, which can be significant for new users.
  - Second, it benefits NFT owners by giving them a **reliable way to earn passive income in Web3**, creating more financial incentives.
  - Finally, it helps developers/creators to **gain revenue through royalty** (via [NEP-199](https://github.com/near/NEPs/blob/master/neps/nep-0199.md)) and explore **more sustainable business models**.
  - In general, our NFT rental solution helps to open a derived rental market, boost both NFT economy and utilities on NEAR, and ultimately adopt more users to Web3.


**Who is our target audience?**
  - The developers of dapps and games who want to add NFT rental function to their app.
  - Users who want to rent/hire NFTs.


**What need(s) does our project meet?**
  - For utility NFT owners, we provide them with a safe and convenient way to earn revenue from renting their NFT assets.
  - For people who are interested in certain utility NFTs, we offer them an affordable way to rent the NFT to experience.
  - For developers and creators, we enable them to explore new business models through renting.


**Are there any other projects similar to ours in the Mintbase / NEAR ecosystem?**
  - If so, how is our project different?
    - PawnNFT: As the name suggested, it works like a pawn shop rather than, for example, a car rental service. It is a finance service where the NFTs are used as collateral. 

  - If not, are there similar projects in related ecosystems?
    - https://double.one/: They require the NFT contracts conform to a special NFT standard they issued, i.e. don't support the standard ERC-721 NFTs.
    - ReNFT on Ethereum and Avalanche: Lenders in their platform have no way to trade their leasing assets until the lease expires.


  - Our project provides a NEAR native solution through a non-collateral approach.It also focuses on prividing a smooth user and developer experience, by providing SDK for existing projects to integreate seamlessly. 
## Team :busts_in_silhouette:

### Team members

- Names of team members
  - **Libo Shen**
  - **Steven Yu**

### Contact

- **Contact Name:** Libo Shen
- **Contact Email:** liboooshen@gmail.com
- **Website:**

### Legal Structure

- **Registered Address:** N/a
- **Registered Legal Entity:** N/a

### Team's experience

<!-- Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful. -->

<!-- If anyone on your team has applied for a grant at the Mintbase previously, please list the name of the project and legal entity here. -->

- **Libo Shen**: 
  - Full-stack engineer, worked at Google London for 6 years, Head of Engineering of a tech startup.
  - GitHub: https://github.com/LiboShen
  - LinkedIn: https://www.linkedin.com/in/libo-shen/
  - Email: liboooshen@gmail.com
  
- **Steven Yu**: 
  - Experienced machine learning engineer worked in both front-office trading and a cybersecurity startup. Strong software engineering professional with an MSc in Computer Science from the University of Oxford.
  - GitHub: https://github.com/stevenyu530
  - LinkedIn: https://www.linkedin.com/in/tianlin-steven-yu/
  - Email: stevenyu530@gmail.com 

The team has participated in and won the IRL hackathon in Nearcon 2022: https://devpost.com/software/nft-renting-platform.

In the past, we also worked on Solana to build our NFT store: https://github.com/stevenyu530/metaplex


### Team Code Repos

- https://github.com/LiboShen/nft-rental (To be moved to an org account)
- https://github.com/LiboShen/mooncake-nft


## Development Status :open_book:

As mentioned above, RentVerse originally started during NEARCON 2022 IRL Hackathon. The code currently locates at https://github.com/LiboShen/nft-rental. A significant amount of new features and designs have been added to the initial idea to support the application for this grant.


## Development Roadmap :nut_and_bolt:

**Note**: All of our milestones do not deliver any server-side programs. We will continuously deploy our smart contracts to testnet and our frontend to hosting platforms (like Netlify). As such, there is no need for Dockerfiles to test our deliverables.


### Overview

- **Total Estimated Duration:** 3 months
- **Full-Time Equivalent (FTE):**  2 FTE
- **Total Costs:** 45,000 USD

### Milestone 1 - Royalty

- **Estimated duration:** 1 month
- **FTE:**  2 (plus UX, editing and potential extra dev contractors)
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can lend and borrow NFTs and how the NFT creators will get their royalty split. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0e. | Article | We will publish an **article/document** that explains why NEAR dapp and games should integrate with our service to enable NFT rental (hint: royalty). |
| 1. | Gas Optimisation | Improves data structure and algorithms in the existing code to make all functions gas-efficient when there are significant number of renting items. |  
| 2. | Royalty | Develop the royalty payout mechanism respectin NEP-199 standard. With it, **the NFT creators can get a split of the rent revenue.** |  
| 3. | Integration tests | Develop integration tests to ensure the contract software quality and facilitate fast iteration |
| 4. | UX Improvements | Re-design the UX to match the expectation of the potential userbase. Various polishments of the UX. |


### Milestone 2 - FTs (Fungible Tokens) Payment & SDK

- **Estimated duration:** 1 month
- **FTE:**  2 (plus UX, editing and potential extra dev contractors)
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** about how to use the SDK to integrate with our project. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0e. | Article | We will publish an **article/document** that explains why users should use our service to lend and borrow NFTs (hint: easy-to-use, affordable, and safe). |
| 1. | Support NEP-141 FT Payment | **The rental contract supports NEP-141 FTs as rent payment.** The web app allows users to choose popular FT as the rent currency. |  
| 2. | Owner's IOU NFT | An NFT will be issued to the lender when the lease starts. The NFT represents that ownership of the rented NFT and rent revenue. **It enables the owner to trade their assets while being rented.** |  
| 3. | JavaScript Integration SDK | A package to make dapp and web game developers easily integrate with our service. |
<!-- | 4. | Allow Early Lease Termination | If both the lender and the borrower agree, the NFT can be returned before the expiration date with an agreed percentage of rent refunded to the borrower. |   -->

### Milestone 3 - Rental Marketplace for NFTs

- **Estimated duration:** 1 month
- **FTE:**  2 (plus UX, editing and potential dev contractors)
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a lender can use the marketplace to list their NFTs and a borrower can take the offer. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0e. | Article | We will publish an **article** to introduce the our marketplace and shopfront for renting NFTs. |
| 1. | Marketplace contract | Develop a marketplace contract where NFT owners can list their assets as rental offers, and other users can choose to accept the offers to start renting.  |  
| 2. | Marketplace frontend | Build a web app to allow users to use the marketplace. |
| 3. | Marketplace Shopfront | Provide a solution to allow users setup customised market shopfront for certain NFTs to be listed. |
| 4. | Example game integration | Provide a simple open sourced web game to demostrate both how users can lend and borrow NFTs adn how developers can integrate with our service |


## Future Plans

The team has designed an ample amount of features and integrations for the product which we plan to implement if resources are allowed. Part of the plans are covered here

In the short-term:
  - Content marketing to promote the importance of NFT rental functions
  - Collaborate with gaming, social projects and DAOs on NEAR to promote NFT utilities and rental.
  - Integrate with existing NEAR dapps such as Roketo to enable streaming rent payment

In the long-term:
  - Hosting Game Jams (i.e. Hackathon for game dev) to encourage game devs to explore NFT-based web games.
  - Design an effective governance approach and fee structure.
  - Leverage our experience to contribute to high-quality NEP standards and facilitate mass adoption in the ecosystem.


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** 
- Personal recommendation via Maria Magdalena
