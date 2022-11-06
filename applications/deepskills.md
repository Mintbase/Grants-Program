# Mintbase Grant Proposal

- **Project Name:** Deep Skills
- **Team Name:** Deep Skills Ltd
- **Payment Address:** kischiman.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### Overview

- Deep Skills is automating contributor recruitment and decentralising project- and payroll management for DAOs and online-native cooperatives.
- The application allows contributors to create and complete collaborative projects, get paid, earn skill attestations and build up their professional reputation on-chain.
- At NEAR, Deep Skills will act as an extension to the proposal platform AstroDAO, helping DAOs coordinate. Mintbase will provide the infrastructure to issue on-chain credentials for completed projects and skills.
- Our team has been designing and building digital products for the web3 ecosystem for the last four years and simultaneously innovated on a way to organize as a freelancer collective. After working on over 50 different projects, we are excited to bring our innovative ways of working to other industries.### Project Details

### Project Details

- Mockups/designs of any UI components:
    - [https://www.figma.com/proto/BxJXCmpy83E4fIQRRuEXSc/Deep-Skills---App-and-Working-Files?node-id=7622%3A96014&scaling=scale-down-width&page-id=7439%3A90920&starting-point-node-id=7622%3A96014&show-proto-sidebar=1](https://www.figma.com/proto/BxJXCmpy83E4fIQRRuEXSc/Deep-Skills---App-and-Working-Files?node-id=7622%3A96014&scaling=scale-down-width&page-id=7439%3A90920&starting-point-node-id=7622%3A96014&show-proto-sidebar=1)
    (Choose the desired user journey in the left-hand side bar)
- Data models / API specifications of the core functionality:
    - Core API specifications: [https://dsp-core.herokuapp.com/docs/](https://dsp-core.herokuapp.com/docs/)
    - Backend: [https://github.com/Deep-Skills/dsp-core](https://github.com/Deep-Skills/dsp-core)
    - Frontend: [https://github.com/Deep-Skills/dsp-web](https://github.com/Deep-Skills/dsp-web)
- An overview of the technology stack to be used
    - Languages:
        - Backend :
            - Node js (express framework)
        - Frontend:
            - React framework
            - Integration with Metamask + NEAR web wallet/ sender wallet: [https://senderwallet.io/](https://senderwallet.io/)
        - Ongoing work, which will be part of this project:
            - Mintbase, NEAR SDK for smart contract development
    - Database:
        - MongoDB
    - Backend and front-end hosting is on a cloud provider
    - Scaling:
        - Initial use of nodes/scaling solution provided by the cloud hosting providers first and consider specifically loadbalancer setup if/when it will be required.
    - CI/CD:
        - Front-end connected/integrated with Vercel pipeline at Gtihub
        - For backend we are considering Github actions (but proper set of unit and integration tests already in place and will be maintained continuously)
        - Moving forward we will add `near-smart-contracts` to the backend and frontend repos above.
- Documentation of core components, protocols, architecture, etc. to be deployed
    - For this project, we plan to integrate project payments on the NEAR blockchain and build a profile on decentralized storage. Portfolio and achievements will be stored on the Ceramic network and also be issued as Mintbase NFT badges upon completion of the project.
- PoC/MVP or other relevant prior work or research on the topic
    - The current version of the app (without NEAR payment support) can be accessed here:
        - [https://dsp-web.vercel.app/](https://dsp-web.vercel.app/)
    - Our research is based on working with many web3 teams:
        - [https://deepwork.studio/case-studies](https://deepwork.studio/case-studies)
    - Part of the research comes from [Google’s Project Aristotle](https://www.nytimes.com/2016/02/28/magazine/what-google-learned-from-its-quest-to-build-the-perfect-team.html), which explores how the most efficient teams collaborate.
    - We also refer to the research on [Stafford Beer’s Viable System Model for DAOs](https://medium.com/block-science/applying-stafford-beers-viable-system-model-to-decentralized-organization-ac42e17bf73) by our friends from Blockscience.
    - An off-chain version of a similar project by our team can be accessed here:
        - [https://app.deepskills.io/](https://app.deepskills.io/)
        - The design studio [Deep Work](https://deepwork.studio) has been operating on these principles for the last 3 years, using a combination of no-code tools rather than custom software.
- What your project is *not* or will *not* provide or implement
    - The project does not (yet) include multisig or smart contracts which lock up funds. Instead, we are relying on UX patterns which don’t require custom contracts.
    - Deep Skills does not handle proposals or governance.
    - Deep Skills’ UX does not focus on one-off bounties or tasks. It also encourages collaborative work, rather than solitary work.

### Ecosystem Fit

- Where and how does your project fit into the ecosystem?
    - At NEAR, Deep Skills will act as a job-board and extension to the proposal platform AstroDAO. DAOs can create proposals on AstroDAO, start tracking their collaborative projects on Deep Skills and the on-chain project attestations can be seen on [stats.gallery](http://stats.gallery) as part of each contributor’s profile. This connection is based on our conversations with Jordan from AstroDAO and Jacob from Stats.gallery
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
    - Existing DAOs on NEAR who need better oversight of the collaborative work and project teams who want to become DAOs by working together on a common goal.
    - On the other side, Deep Skills presents work opportunities for digital nomads in organizational roles and makes it easier to become part of a team.
- What need(s) does your project meet?
    - Most web3 projects have trouble coordinating and learning from their experience as a collective. Current tools to store projects and reputation are clunky or permissioned. Deep Skills helps teams hold each other accountable on collaborative projects, keep an on-chain record of their collaborative work and store the exact contribution of each individual.
    - The trust in CVs, job-board profiles, online course credentials is low due to easy falsification. By showing rich reputation profiles from actual projects (verified on-chain), Deep Skills makes it easier to trust claims about previous work, find skilled freelancers and discover skill-based work opportunities.
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
    - If so, how is your project different?
        - AstroDAO is similar, but focuses on governance proposals and one-off bounties.
    - If not, are there similar projects in related ecosystems?
        - A tool that is often compared to Deep Skills is [dework.xyz](http://dework.xyz), which focuses on individual bounty-driven work and delegation of one-off tasks. 
        Deep Skills has been designed with a focus on collaboration - to help teams develop innovative collectives by holding each other accountable for the work.

## Team :busts_in_silhouette:

### Team members

- Andrej Berlin (CEO, Product Design, UX)
- Denis Popov (CTO, web3, backend)
- Oleksandr Lazarchuk (Frontend)

### Contact

- **Contact Name:** Andrej Berlin
- **Contact Email:** andrej@deepwork.studio
- **Website:** deepskills.io

### Legal Structure

- **Registered Address:** Intershore Chambers, Road Town, Tortola, British Virgin Islands
- **Registered Legal Entity:** Deep Skills Ltd.

### Team's experience

**Denis Popov**

**Publications:**

Cryptography and SSI-related topics:

[https://denispopovengineer.medium.com/](https://denispopovengineer.medium.com/)

Publications at Affinidi space:

[https://medium.com/affinidi/using-decentralized-identifiers-dids-without-a-digital-wallet-34646074ba42](https://medium.com/affinidi/using-decentralized-identifiers-dids-without-a-digital-wallet-34646074ba42)

This received some traction and has been picked up by the Identosphere newsletter :

[https://newsletter.identosphere.net/p/identosphere-61-remembering-kim-cameron](https://newsletter.identosphere.net/p/identosphere-61-remembering-kim-cameron)

[https://academy.affinidi.com/using-did-as-a-second-factor-authentication-198630db4a1c](https://academy.affinidi.com/using-did-as-a-second-factor-authentication-198630db4a1c)

**ETH Hackathons:**

ETHDenver:

[https://dorahacks.io/buidl/2054](https://dorahacks.io/buidl/2054)

Won bounties from Ceramic and LitProtocol:

[https://twitter.com/ceramicnetwork/status/1498289896204689413?t=3rglBgA528Mp5Mexc1rzGA&s=19](https://twitter.com/ceramicnetwork/status/1498289896204689413?t=3rglBgA528Mp5Mexc1rzGA&s=19)
ETHLisbon:

[https://devpost.com/software/deep-skills-protocol](https://devpost.com/software/deep-skills-protocol)

**Engineering:**

Active participant of Affinidi ecosystem

[https://academy.affinidi.com/](https://academy.affinidi.com/)

One of the core contributors of the Affinidi SDK:

[https://github.com/affinityproject/affinidi-core-sdk](https://github.com/affinityproject/affinidi-core-sdk) (open sourced)
Participated as a team member at a number of well-known web2 projects for clients like HBO, Siemens, Petronas. Projects are under NDA.

**Andrej Berlin**

**Organizational Design**

- Co-founder and CEO of [Deep Work Studio](https://deepwork.studio/) - a permissionless cooperative design studio

**Product and UX Design**

- Creative director on over [50 different projects for the web3 ecosystem](https://deepwork.studio/case-studies). Among those are the Ethereum Foundation, ConsenSys, Arbitrum, Maple Finance, Metacartel and many more. All web3 design case studies are open source.
- Previously worked as a product designer at the agency [AJ&Smart](https://www.ajsmart.com/) on projects for BCG, Google, Udacity, and many more. Web2 projects are under NDA.

**Publications**

- [Eth2 Launchpad design case study](https://medium.com/deep-work-studio/eth2-deposit-launchpad-an-interface-for-the-first-world-computer-3e089138b264)
- [Decentralizing organizations](https://medium.com/@kischiman/decentralizing-organizations-c7acdcaa9fdd)

**Oleksandr Lazarchuk**

Frontend and Rust development experience, all client projects are under NDA.

### Team Code Repos

Deep Skills:

- [https://github.com/Deep-Skills](https://github.com/Deep-Skills)

Recent project showcasing portable reputation for the Denver hackathon:

- https://github.com/Deep-Skills/denver-eth-2022

Denis’ GitHub:

- [https://github.com/DenisPopov15](https://github.com/DenisPopov15)

Andrej’s GitHub:

- [https://github.com/kischiman](https://github.com/kischiman)

### Team LinkedIn Profiles (if available)

- [https://www.linkedin.com/in/andrejberlin/](https://www.linkedin.com/in/andrejberlin/)



## Development Status :open_book:

(also see above in the project details section)

- The current version of the app (without NEAR payment support) can be accessed here:
    - [https://dsp-web.vercel.app/](https://dsp-web.vercel.app/)
- Backend: [https://github.com/Deep-Skills/dsp-core](https://github.com/Deep-Skills/dsp-core)
- Frontend: [https://github.com/Deep-Skills/dsp-web](https://github.com/Deep-Skills/dsp-web)
- Mockups of integration with AstroDAO and stats.gallery: [https://www.figma.com/proto/BxJXCmpy83E4fIQRRuEXSc/Deep-Skills---App-and-Working-Files?node-id=8133%3A98030&scaling=min-zoom&page-id=8133%3A98030&starting-point-node-id=8133%3A98035](https://www.figma.com/proto/BxJXCmpy83E4fIQRRuEXSc/Deep-Skills---App-and-Working-Files?node-id=8133%3A98030&scaling=min-zoom&page-id=8133%3A98030&starting-point-node-id=8133%3A98035)
- Our research is based on working with many web3 teams:
    - [https://deepwork.studio/case-studies](https://deepwork.studio/case-studies)
- Additional research comes from [Google’s Project Aristotle](https://www.nytimes.com/2016/02/28/magazine/what-google-learned-from-its-quest-to-build-the-perfect-team.html), which explores how the most efficient teams collaborate.
- We also refer to the research on [Stafford Beer’s Viable System Model for DAOs](https://medium.com/block-science/applying-stafford-beers-viable-system-model-to-decentralized-organization-ac42e17bf73) by our friends from Blockscience.
- An off-chain version of a similar project our team has built last year can be accessed here:
    - [https://app.deepskills.io/](https://app.deepskills.io/)
    - The design studio [Deep Work](https://deepwork.studio) has been operating on these principles for the last 3 years, using a combination of no-code tools.

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 2.5 months
- **Full-Time Equivalent (FTE):** 3 FTE
- **Total Costs:** 50,000 USD

We are also happy to split the two milestones into two separate grant proposals if that approach is more favorable.

### Milestone 1 — Mint Mintbase NFTs when projects complete

- **Estimated duration:** 1.5 months
- **FTE:** 3
- **Costs:** 25,000 USD

| Number | Deliverable | Specification |
| --- | --- | --- |
| 0a. | License | ISC https://en.wikipedia.org/wiki/ISC_license |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains the functionality. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Video walkthrough | We will publish a video walkthrough that explains how to use Deep Skills on NEAR to create projects and mint Mintbase NFTs to issue attestations for project participation. 
| 1. | Research in Mintbase smart contracts, NEAR NFT standards, and Mintbase SDK and tooling. |  |
| 2. | Design userflow to login to Deep Skills with near wallet, participate and complete a project, issue NFTs upon completion. |  |
| 3. | Write Mintbase smart contracts to mint NFTs. |  |
| 4. | Mintbase implementation for our use case | Integrating user flow to issue a proof/certification NFT that would read the metadata from the Deep Skills app and mint NFTs upon project completion. |
| 5. | Frontend and UI updates | Frontend and UI to support the issuance of NFTs with a summary of each team member’s contribution. |

## Future Plans
- NEAR chain integration. As seen on the prototype, it will be possible to select NEAR payments on project creation and send payments in NEAR native tokens upon completion.
- Implement multisig for each project in order to automate payments
- Connect project data to AstroDAO and export to [Stats.gallery](http://Stats.gallery) on NEAR

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** personal recommendation and from NEARcon Alpha in Lisbon

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- So far, all work has been self-funded or paid for by Deep Work Studio
