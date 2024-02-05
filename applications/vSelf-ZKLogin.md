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

ZKLogin offers a secure and private way of authentication, which is crucial for the mass adoption of the NEAR blockchain. In the NEAR ecosystem, our development focuses on creating an SDK for streamlined authorization processes alongside a Telegram bot designed to generate secure authorization passcodes. We aim to go beyond mere account ownership verification, incorporating an assessment of user status, such as evaluating Soulbound Tokens (SBTs). This holistic approach ensures a more comprehensive and secure authentication system tailored to the evolving needs of Web3 users.

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
  ![vPassBot](./vPassBot.png)
  ![vPassBot](./vPassBot2.png)
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

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
- What need(s) does your project meet?
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems?

## Team :busts_in_silhouette:

### Team members

- Name of team leader
- Names of team members

### Contact

- **Contact Name:** Full name of the contact person in your team
- **Contact Email:** Contact email (e.g. john@duo.com)
- **Website:**

### Legal Structure

- **Registered Address:** Address of your registered legal entity, if available. Please keep it in a single line. (e.g. High Street 1, London LK1 234, UK)
- **Registered Legal Entity:** Name of your registered legal entity, if available. (e.g. Duo Ltd.)

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Mintbase previously, please list the name of the project and legal entity here.

### Team Code Repos

- https://github.com/<your_organisation>
- https://github.com/<your_organisation>/<project_1>
- https://github.com/<your_organisation>/<project_2>

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/<team_member_1>
- https://github.com/<team_member_2>

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/<person_1>
- https://www.linkedin.com/<person_2>

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),
- academic publications relevant to the problem,
- links to your research diary, blog posts, articles, forum discussions or open GitHub issues,
- references to conversations you might have had related to this project with anyone from the Mintbase Foundation,
- previous interface iterations, such as mock-ups and wireframes.

## Development Roadmap :nut_and_bolt:

This section should break the development roadmap down into milestones and deliverables. To assist you in defining it, we have created a document with examples for some grant categories [here](../docs/grant_guidelines_per_category.md). Since these will be part of the agreement, it helps to describe _the functionality we should expect in as much detail as possible_, plus how we can verify and test that functionality. Whenever milestones are delivered, we refer to this document to ensure that everything has been delivered as expected.

Below we provide an **example roadmap**. In the descriptions, it should be clear how your project is related to Mintbase. We _recommend_ that teams structure their roadmap as 1 milestone ≈ 1 month.

For each milestone,

- make sure to include a specification of your software. _Treat it as a contract_; the level of detail must be enough to later verify that the software meets the specification.
- include the amount of funding requested _per milestone_.
- include documentation (tutorials, API specifications, architecture diagrams, whatever is appropriate) in each milestone. This ensures that the code can be widely used by the community.
- provide a test suite, comprising unit and integration tests, along with a guide on how to set up and run them.
- commit to providing Dockerfiles for the delivery of your project.
- indicate milestone duration as well as number of full-time employees working on each milestone.
- **Deliverables 0a-0d are mandatory for all milestones**, and deliverable 0e at least for the last one. If you do not intend to deliver one of these, please state a reason in its specification (e.g. Milestone X is research oriented and as such there is no code to test).

> :zap: If any of your deliverables is based on somebody else's work, make sure you work and publish _under the terms of the license_ of the respective project and that you **highlight this fact in your milestone documentation** and in the source code if applicable! **Teams that submit others' work without attributing it will be immediately terminated.**

### Overview

- **Total Estimated Duration:** Duration of the whole project (e.g. 2 months)
- **Full-Time Equivalent (FTE):** Average number of full-time employees working on the project throughout its duration (see [Wikipedia](https://en.wikipedia.org/wiki/Full-time_equivalent), e.g. 2 FTE)
- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in USN, and potentially NEAR and nUSDC, the amount will be calculated according to the exchange rate at the time of payment.

### Milestone 1 Example — Implement Mintbase Modules

- **Estimated duration:** 1 month
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
