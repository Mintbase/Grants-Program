# Mintbase Grant Proposal

- **Project Name:** vSelf ZKLogin
- **Team Name:** vSelf
- **Payment Address:** vself.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

For mass adoption of Web3, it's crucial to balance user-friendly experiences with trust and data compliance, especially in B2B products. The vSelf team is dedicated to developing Web2.5 solutions, where our primary product is a no-code, Web3-powered platform designed to assist sales, marketing, and service teams in customer engagement and growth hacking.

In this proposal, we're honing in on a key aspect of the user journey: a straightforward yet privacy-centric authorization process embodied in our ZKLogin project. This approach is intended to simplify user experience while maintaining high privacy and security standards.

### Overview

- Privacy Meets Simplicity in Authentication: Introducing ZKLogin for the NEAR Ecosystem.
- ZKLogin represents a significant leap in the authentication domain, blending ease of use with robust privacy protections. Our solution uses conventional Web2 authorization services, like Google Sign-In and Telegram Login, to provide access to DApps in the NEAR Ecosystem without compromising the user’s identity. It blends an authentication token with a NEAR address (ephemeral or provided by the user) and passes it through a zero-knowledge proof (ZKP) service to create a unique identification for each DApp.

  ZKLogin offers a secure and private way of authentication, which is crucial for the mass adoption of the NEAR blockchain. In the NEAR ecosystem, our     
development focuses on creating an SDK for streamlined authorization processes alongside a Telegram bot designed to generate secure authorization passcodes. We aim to go beyond mere account ownership verification, incorporating an assessment of user status, such as evaluating Soulbound Tokens (SBTs). This holistic approach ensures a more comprehensive and secure authentication system tailored to the evolving needs of Web3 users.

- vSelf team has experience in designing privacy-preserving solutions, e.g., we developed a ZK-powered community membership toolkit [vStudio](https://github.com/vself-project/vstudio-metabuild). Besides, we went through [Outlier Ventures ZK Cohort](https://outlierventures.io/article/outlier-ventures-announces-first-cohort-for-zero-knowledge-base-camp/), where we explored cutting-edge privacy tech with our mentors.

### Project Details

- **PoC/MVP and Prior Work:**
  The project is inspired by a recent release of the [zkLogin](https://sui.io/zklogin) in the Sui ecosystem. The original work presented a concept of web2.5 authorization that was executed for the Sui blockchain and [audited](https://www.zksecurity.xyz/blog/posts/zklogin/).

  In the current proposal, we expand on this solution while continuing our own previous zkp-related work: [vStudio tools for private membership.](https://devpost.com/software/vstudio-private-onboarding-to-web3-communities) It focused on the idea of smooth onboarding of the users into communities and Web3-powered campaigns (like DAO activations, voting, or SMM engagement) without compromising the account data with which the user verified its membership. [To build it](https://github.com/vself-project/vstudio-metabuild), we used Bulletproofs, which is a well-established ZK-proof system without a trusted setup (unlike some zkSNARKs). On top of Bulletproofs, we built our R1CS circuit to prove set membership in zero-knowledge, so on-chain data doesn't reveal members’ identities. This was quite an experimental approach: we used MiMC hash under the hood (as it has low multiplicative complexity). The produced proof contains zero-knowledge attestation of the fact that one knows a secret behind the member’s public commitment and simultaneously belongs to a set of members.

- **Deliverables**

  1. **ZKLogin SDK npm package** (partially compiled from Rust to WebAssembly).

  NEAR ecosystem members can use it to generate & verify proofs of account ownership. It is limited to two types of Web2 accounts (Google and Telegram) and NEAR Web3 addresses.

  2. **vPass** Telegram bot

  - It allows users to seamlessly authorize in dApps and gated Telegram groups and channels, using their combination of NEAR address and Telegram account and generating ZKLogin for each request. vPass utilizes ZKLogin SDK to run proof verification and connects to NEAR infrastructure for on-chain data.
  - vPass bot can be set up by the admin to check more specific credentials of the users, e.g. SBTs or NFTs of particular kind and conditions of ownership, that could be derived from on-chain data.
  - vPass TMA (Telegram mini application) which manages user identity (including SSO with Google) and enabled to produce zk-proofs for future logins (using same SDK).
  - Testnet gated Telegram channel which is moderated by vPass bot and only accessible on customizable conditions (like having specific SBT).

- **UI Components**
  The project will include intuitive and user-friendly interface designs in two forms: Telegram bots and web plugin. These mockups will showcase the login process through Telegram vPass bot using NEAR account, and the user journey involving ephemeral and personal account management.

  <img src="./vPassBot.png" width="200"  alt="vPassBot">
  <img src="./vPassBot2.png" width="200" alt="vPassBot2">

- **Project Limitations**
  vSelf ZKLogin will not provide a full-fledged identity management system beyond the scope of user authentication and status verification, described in this proposal. It will focus primarily on the use casessuitable for the NEAR and Mintbase ecosystem.

  The initial vPass release will be limited in terms of UX and feature set on the admin side. Future versions will aim to expand these capabilities. This detailed breakdown of deliverables and milestones sets clear expectations and provides a roadmap for the development of vSelf ZKLogin.

#### Technology Stack Overview

- **Frontend**: **Next.js 14 T3 stack** (Typescript/Tailwind CSS) for dynamic and responsive UI, Telegram bot (Telegraf.js), Telegram Web App.
- **Backend**: Node.js service for handling API requests, integration with NEAR blockchain.
- **Database**: **Firebase** & **Google Cloud** are used for hosting and content delivery and identity management.
- **Auth**: The Single Sign-On protocol supported by is **[OAuth 2.0](https://oauth.net/2/)**. In which a user can log into a trusted third party (Google, Facebook, etc.) and get a signed token attesting that they logged in in the form of a signed **[JSON Web Token](https://jwt.io/)** (JWT).
- **Blockchain**: NEAR protocol and Rust smart contracts for blockchain-based interactions, particularly for handling authentication and NFT-related processes. For NEAR integration, we use **NEAR JS SDK** & **WalletSelector.**
- **Crypto**: Zero-knowledge proofs **Bulletproofs** & **R1CS circuit** implementation in Rust ensures user privacy and safety.

### Ecosystem Fit
- Where and how does your project fit into the ecosystem?
  We are natively building on NEAR Protocol as we proponents of NEAR tech, particularly account abstraction, and its good fit for Web2.5 adoption. We see wallet-type of authorization as a big friction point for non-crypto native customers. Besides, existing alternative solutions are often unsuitable to meet business needs for data management regulation. We aim to provide a new open-source tool for forter adoption process and bridge current gaps in the market.
  
- Who is your target audience?
    This project targets NEAR-compatible or NEAR-native DApps and their Telegram communities, which could benefit from integration of our bot and/or SDK for private or gated access. Besides, all their users, whether they’re allready NEAR adopters or newcommers, will  leverage these tools.
    
- What need(s) does your project meet?
    There is a need for seamless onboarding techniques (including Web2 ways), as well as a reduction of on-chain footprint (we prefer to limit the exposure of the data to only what is necessary.).
    
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
    None we are aware of, though we have examples on other chains (like Sui). In the NEAR ecosystem, there are other solutions facilitating authorization like FastAuth (email key recovery) and Keypom (linkdrop mechanics), but they are complimentary and can be used along with zkLogin as a last-mile solution.

## Team :busts_in_silhouette:

### Team members

- Tatiana Yakushkina (CEO, product manager, blockchain architect)
- Roxana Balan (CMO, customer journey manager)
- Sergey Kozlov (Full Stack Dev)
- Anttoni Viital (Full Stack Dev)

### Contact

- **Contact Name:** Tatiana Yakushkina
- **Contact Email:** ty@vself.app
- **Website:** https://vself.app/

### Legal Structure

- **Registered Address:** 68 CIRCULAR ROAD 02-01 SINGAPORE (049422)
- **Registered Legal Entity:** VSELF PTE. LTD.

### Team's experience

The vSelf team has been contributing to the NEAR ecosystem since the end of 2021, focusing on identity applications and data management. We successfuly delivered a project for a grant from Human Guild (NEAR Foundation [grant report](https://gov.near.org/t/proposal-vself-learn-to-earn-app-for-participation-in-events-meetups-and-courses/11368)) and later won the Best Design Prize at the MetaBUILD Hackathon ([our demo](https://devpost.com/software/vself)). Besides, we completed a ZK cohort of the Outlier Ventures acceleration program with less than 3% of applicants accepted.  
Tatiana holds a PhD in Applied mathematics and spent ten years in academia, doing research in computer science and teaching at business informatics department (including courses on blockchain business applications). Together with our full-stack developer Sergey, she had worked on previous startup Nuland in the blockchain space: a Web3 app to allow tokenisation of rewards for user engagement and feedback based on geolocation. During 2020, it secured cooperation with MIT PathCheck ([collaborative pilot](https://github.com/nuland-project/safe-path-sm)) intended to customise their digital contract tracing (DCT) technology and deploy it to production in the San Marino Republic. Roxana has extensive experience in customer behaviour research and incorpoorating marketing and cusdev insights into bespoke user journeys. Anttoni is a seasoned blockchain and full-stack dev, bringing experience of working with the most modern tech stack accross platforms. This powerful combination of skills enables them to navigate the complexities of the web3 ecosystem effectively, driving innovation and creating cutting-edge solutions that cater to the evolving needs of the digital landscape.

Previously, [the team attempted to apply](https://www.notion.so/Mintbase-2024-be66e748b26940d08aca0fc619587dcd?pvs=4#4b4e2acb0a974439bc18614a54a24c32) for a Mintbase grant (but didn’t submit it correctly). Earlier, an [application for identity management was supported by the NEAR Foundation](https://gov.near.org/t/proposal-vself-learn-to-earn-app-for-participation-in-events-meetups-and-courses/11368).

### Team Code Repos

- https://github.com/vself-project
- https://github.com/vself-project/vself-dao
- https://github.com/vself-project/vstudio-metabuild
- https://github.com/vself-project/vself-beta
- https://github.com/vself-project/vself-brands

GitHub accounts of team members:

- https://github.com/sergantche
- https://github.com/legendaryangelist
- https://github.com/mrpejker

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/tyakushkina/
- https://www.linkedin.com/in/roxanabalan/
- https://www.linkedin.com/in/sergey-kozlov-7b3520127/
- https://www.linkedin.com/in/anttoni-viitala-69296124a/

## Development Status :open_book:

ADD

## Development Roadmap :nut_and_bolt:

### Overview

- **Estimated duration:** 6 months
- **FTE:** 2
- **Costs:** 8,000 USD

| Number | Deliverable            | Specification                                                                                                                                                                                                                                |
| -----: | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|    0a. | License                | Apache 2.0 / GPLv3 / MIT / Unlicense                                                                                                                                                                                                         |
|    0b. | Documentation          | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
|    0c. | Testing Guide          | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests.                                                                                            |
|    0d. | Docker                 | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone.                                                                                                                                |
|    0e. | Article                | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)                                               |
|     1. | Mintbase module: X     | We will create a Mintbase / NEAR module that will... (Please list the functionality that will be implemented for the first milestone)                                                                                                        |
|     2. | Mintbase module: Y     | We will create a Mintbase / NEAR module that will...                                                                                                                                                                                         |
|     3. | Mintbase module: Z     | We will create a Mintbase / NEAR module that will...                                                                                                                                                                                         |
|     4. | NEAR chain integration | Modules X, Y & Z of our custom chain will interact in such a way... (Please describe the deliverable here as detailed as possible)                                                                                                           |

### Milestone 2 Example — Additional features

- **Estimated Duration:** 1 month
- **FTE:** 1
- **Costs:** 4,000 USD

...

## Future Plans

Please include here

- how you intend to use, enhance, promote and support your project in the short term, and
- the team's long-term plans and intentions in relation to it.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.
