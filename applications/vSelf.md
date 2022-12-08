# Mintbase Grant Proposal
 
- **Project Name:** vSelf + Mintbase
- **Team Name:** vSelf
- **Payment Address:** vself.near
- **[Level](../README.md#level_slider-levels):** 2
 
## Project Overview :page_facing_up:
 
The following proposal considers possible integration path and vSelf effort to facilitate Mintbase users’ onboarding and access to Mintbase API features.
 
### Overview
vSelf is a web3 identity wallet with rewards for data sharing. 
 
vSelf is an Identity-as-a-Service project enhanced by gamification and rewards for data sharing. Our goal is to guarantee full ownership of personal data to the users and bring benefits of the blockchain world to a broader audience. In vSelf, we create a user-centric solution with several key features:  management of identities and credentials, encrypted distributed storage for personal data, and the ability to trade the data with other participants and service providers.
 
vSelf platform is an autonomous agent with user-facing mobile and web applications. It facilitates a flow between issuance and verifying various users’ credentials. On one side, it allows to set up collections of rewards or certificates that can be assigned for particular actions in the digital space.  On the other side, it helps users to collect a self-sovereign web3 profile with their personal provenance and accompanied data. 
 
### Project Details
 
vSelf is built on top of modern web technologies and consists of the following components:
 
The web application (runs in the browser) acts as a secure digital wallet and handles all client-side secrets, credentials and cryptography. It's currently based on the T3 (Typescript/Tailwind CSS/tRPC) tech stack and is delivered through Google-owned CDN (Firebase Hosting).
 
Mobile application developed in Unity and published through Google Play and App Store. It uses a web application (wallet) to pass authentication via deep link mechanism or directly using Unity NEAR SDK. Mobile application sends authenticated requests to API services and communicates with web applications using web sockets.
 
All frontend applications use vSelf API for data fetching, session management, and interacting with other vSelf subsystems. As we are using the Next.js framework, it allows us to implement server side logic in Typescript and use it for typesafe endpoints implementation. 
 
The main part and business logic involving any value transfer is executed and verified on-chain. For this, we develop a set of smart-contracts written in Rust (using NEAR SDK).
 
Currently, we have released our beta version of the products (web/mobile/contracts), so we could onboard people and be exposed to public scrutiny. This MVP focuses on non-transferable NFTs as a basic standard for personal provenance. We are rolling out our initial set of features around profile management: services to collect NFT rewards and present them in a public profile (see an example for vSelf https://t.co/uCJXFFa6NM). This proposal focuses specifically on public profiles and NFT management, which we believe can be significantly improved by building on top of Mintbase. It incorporates an advanced UX for privacy management and the ability to choose which NFTs to expose, adding transferable NFT collections and increasing the quality of represented graphical data. 
 
### Ecosystem Fit
 
Our value proposition is that we want to integrate Mintbase inside a bigger scope of blockchain-agnostic identity management. As a lot of users in the NEAR ecosystem already use Mintbase for their NFTs management, we want to benefit them by meaningfully using them inside vSelf. Be it public profile enhancement, minting/trading capabilities or even something sophisticated like selective disclosure of ownership in zero-knowledge. 
 
## Team :busts_in_silhouette:
 
### Team members
 
- Tatiana Yakushkina (CEO)
- Inna Kalinina (CTO)
- Roxana Balan (CMO)
- Ilya Eriklintsev (Techlead)
- Sergey Kozlov (Backend Developer)
- Vasily Kharlamov (Frontend Developer)
 
### Contact
 
- **Contact Name:** Tatiana Yakushkina
- **Contact Email:** ty@vself.app
- **Website:** [vself.app](https://vself.app/about)
 
### Legal Structure
 
- **Registered Address:** 128 City Road, London, EC1V 2NX.
- **Registered Legal Entity:** vSelf Ltd.
 
### Team's experience
 
Most of us have a background in tech and have been working together on blockchain projects. Previously, most of the team worked in the e-government startup Nuland and developed a side-project, Lemonade.Care, utilising web3 to address the issues in healthcare management. Co-founders Tatiana and Inga both have PhDs in computer science. With our dev team, we have experience in building projects on the Web3 tech stack, working with Ethereum and NEAR frameworks, and developing web and mobile applications. Our team has a deep understanding of the industry and underlying complex problems. E.g., Tatiana specialized in game theory and teaches graduate courses on blockchain applications. Inga brings expertise in game development and entrepreneurship, at the same time being an experienced mathematician. Roxana has a strong background in behavioral science and is driven by purposeful projects.
 
Together at vSelf, we can deal with a broad spectrum of problems: design of the IT architecture, tokenomics, gamification, product development, and more. The current project was started in Feb 2021 and brought to its current state after half a year of R&D and further full-time development. 
 
### Team Code Repos
 
Our project codebase live on Github:
 
- https://github.com/vself-project
- https://github.com/vself-project/vself-beta
- https://github.com/vself-project/vself-dao
 
Our teammates personal Github profiles:
 
- https://github.com/ilerik 
- https://github.com/mrpejker
- https://github.com/caesai 
- https://github.com/sergantche 
- https://github.com/isenilova 
 
### Team LinkedIn Profiles (if available)
 
- https://www.linkedin.com/in/tyakushkina/
- https://www.linkedin.com/in/roxanabalan/
- https://www.linkedin.com/in/inga-kalinina-b5083a236/
- https://www.linkedin.com/in/ilya-eriklintsev-3a296852/ 
 
## Development Status :open_book:
 
Our MVP is currently in beta and deployed to production at [vself.app](https://vself.app). Code base is open source and exposed to public scrutiny. Currently vSelf provides a few features:
1. Public profiles (linktree-like) e.g. https://vself.app/linktree/ilerik.near
1. Free onboarding to NEAR at https://vself.app/onboard
1. Authentication using several NEAR native wallets
 
## Development Roadmap :nut_and_bolt:
 
We aim to deliver to our users the ability to link one's Mintbase account and show off her collection as a part of vSelf on-chain profiles.
 
### Overview
 
- **Total Estimated Duration:** 1 months
- **Full-Time Equivalent (FTE):**  4 FTE
- **Total Costs:** 20,000 USD
 
### Milestone 1 vSelf public profile integration
 
- **Estimated duration:** 1 month
- **FTE:**  4 FTE
- **Costs:** 20,000 USD
 
| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | vSelf UX/UI improvements | vSelf profile page overall experience remastering. We will provide an updated version of the public profile page which enables Mintbase identity and collection management. |  
| 2. | Mintbase API integration | vSelf server side application and API updates + Apollo Client and Mintbase Wallet on the frontend side |  
 
## Future Plans
 
In the near future we will focus on bringing more of Mintbase capabilities to the table, at the same time maintaining codebase and following up closely API / SDK updates.
 
On the other side our long term vision includes a number of privacy enhancing techniques. To name a few use cases that we think should be useful for NFT owners:
 
1. Passwordless authentication via zero-knowledge proof ensuring one possess a certain set of NFTs / SBTs (e.g. to protect limited access content).
1. Consensual airdrops using public commitment schemes and ZKPs to verify eligibility criteria without revealing recipients identity to the public.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Through the community