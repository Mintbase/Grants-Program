# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** QSTN
- **Team Name:** QSTN
- **Payment Address:** realorrin.near
- **[Level](../README.md#level_slider-levels):** 1

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- QSTN is a web application where users participate in surveys, quizzes and short questionnaires to earn virtual credit. 
- QSTN is a blockchain enabled market research portal where users answer questions, earn credit and spend these rewards in our digital marketplace for tokenized goods (e.g. NFTs, metaverse wearables). 
- The key to our platform is allowing users to monetize their personal information in a low gas, scalable environment and NEAR Protocol meets both of these requirements. By supporting this initiative, Mintbase is investing in data infrastructure for the entire NEAR ecosystem as well as creating a new “Play-to-Earn” model for data aggregation which can be monetized in the form of NFTs, minted on Mintbase. 

For scope, QSTN can be broken down into three user experiences:
			1. General experience - general users create an account, complete on-boarding process and answer 5 daily questions in return for credit which can be spent for an NFT reward 

			2. Business experience - businesses and corporate entities (e.g. DAOs and protocols) can establish an account to create their own questionnaire portal, reward users for their participation in NFTs or NEP tokens and track response rate in the QSTN business dashboard

			3. Commission experience - businesses, brands, celebrities and corporate entities pay a fee to have questions slipped into the "User experience” and glean insights from our audience for market research purposes
			
We aim to bring all three user experiences to the NEAR ecosystem, supporting the general user in scope 1, businesses and protocols who want to manage their own questionnaire in scope 2 and the opportunity for more traditional Web2 businesses to collect market research from Web3 audiences in scope 3.

In terms of integration, we want to use the exposed Mintbase API to offer Mintbase as the default NFT marketplace for media minted in our platform. This grant will allow us to create the following functionalities: 

Mintbase integration - 
* We will create a Mintbase store where users can make “requests to mint” and tokenize their media copy (up to 100 copies per NFT) 

Front end integration - 
* We will adapt the metadata from our smart contract to fit the Mintbase NEP standard 

Back end integration - 
* We create a function (grantMinter) that passes account ID as well as contract name so users have permission to mint in our Mintbase store 
* We create a function (mint) that accepts the amount and contract name to mint the actual media as an NFT in our Mintbase store

The goal is for our users to get exposure to Mintbase, NFTs and liquidity through our store - offering them the chance to further learn about the NEAR ecosystem and trade our unique, 3D collection in the secondary market  

- Founded by NYU graduate Orrin Campbell, QSTN was birthed by a recorded conversation with astrophysicist Neil deGrasse Tyson. During the interview, Neil argued that even if software is ‘free’, companies still profit from the sale, transfer, and storage of our information. 

For example, did you know Google makes almost $120 million dollars a day in advertising revenue (e.g. targeted ads, placements)? Orrin thought about how to creatively address this problem and deliver his “promise to save the world.”

Orrin has released NFTs with Mintbase, spoken on Cryptovoxels and Somnium panels as well as created a large social following behind his afrofuturistic aesthetic. 

Completing the QSTN team triad are the Suriel brothers, who Orrin met at NYU. Jose Suriel is an NYU graduate who handles project management at Forge AI — bringing this expertise to assisting the development and timeline of the QSTN marketplace.

Anibal Suriel is Jose’s twin brother and our head of legal, he is registered in New York and Miami; leveraging his time at KARM to create the legal framework and guidelines around data, compliance, and overall best practices.

With a desire to build meaningful products and experiences, Orrin founded QSTN with the aim to empower internet users and provide alternative means of passive income, aided by the invaluable expertise of the notorious Suriel duo.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
- Data models / API specifications of the core functionality
- An overview of the technology stack to be used
- Documentation of core components, protocols, architecture, etc. to be deployed
- PoC/MVP or other relevant prior work or research on the topic
- What your project is _not_ or will _not_ provide or implement
  - This is a place for you to manage expectations and to clarify any limitations that might not be obvious

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- QSTN is Web3 infrastructure to accelerate the adoption of data monetization. Since users can earn without a Web3 wallet, but have the option to create one later on, shows we are designed to be a Web2 on-ramp into the world of Web3.

We do this by taking advantage of the low gas, scalable environment offered on NEAR to allow users to earn credits, mint NFTs and transfer assets without exorbitant transactional fees. 

Our design is a centralized system “wrapped” with Web3 extensions so that users can sign up using Twitter or Facebook.

- Right now, our target audience can be broken down in the following groups:
1. Data privacy enthusiasts (Web2 and Web3)
2. High school and college students seeking additional income 
3. Web3 DAOs and Protocols which need to capture market research (e.g. DAO governance) 
4. Web2 users wishing to matriculate to Web3 but no upfront capital or resources to get into the space

Currently, we have over 2,000 sign-ups for the waitlist and confirmed private test sessions with Encode Club, Octopus Network and a couple projects on NEAR Protocol.

Since we also have a desire to partner with Web2 organizations to help demonstrate the value of our platform, we are currently in talks with the following educational institutions to run private sessions.
- Jack & Jill of America, Inc.
- New York University
- The Gray Matter Experience
			
- Our platform is able to act as a Web3 on-ramp because our design allows for Web2 social login. This is important because in our initial feedback, we learned a lot of users want to get into blockchain but lack the tools, resources and general knowledge.

We were inspired by the Pi app, as well as Sweatcoin which are two “Play-to-Earn” models where users get rewarded for daily, micro tasks based on actions taken in-platform.

QSTN builds on their concept by offering more exciting, engaging rewards, increasing the educational value behind our mission statement as well as creating a cohesive community for participants in our network (e.g. NFTs, metaverse).

-  There has been an increase in the “Play-to-Earn” model and a couple players who are making noise in the space would include Rabbit Hole (educational), NEAR Crowd (micro task), Axie Infinity (gaming), Pi (validation), and Sweat Coin (exercise). 

We see a market opportunity to use this model for data aggregation, gamifying marketing research and then selling this information to corporate sponsors who offer digital goods or discounts to our users. 
			
  - There are no other Web3 data market research portals but similar Web2 solutions include Google Form, Nielsen and Survey Monkey; the problem is these agencies do not have blockchain infrastructure currently in place.

We will be using game mechanics and "play-to-earn" models to accelerate user adoption and data collection as we find the perfect incentive model using tokenized rewards. 

Our unique offering is the ability for Web2 users to answer questions, earn credits and buy NFTs within a "gas-less" environment but having the option to connect their wallet and mint once familiar with the technology.

We spent the past year participating in over 5 accelerators to help drive brand awareness within the Web3 space and ensure we become the "go-to" market research portal in blockchain. 

## Team :busts_in_silhouette:

### Team members

- Orrin Campbell – QSTN founder, CEO and blockchain architect who will be designing the workflow and user experience **team leader**

Twitter - https://twitter.com/realorrin

Instagram - https://instagram.com/realorrin

Linkedin - https://linkedin.com/in/realorrin

GitHub - https://github.com/@qstnus
			
- Anibal Suriel - QSTN Head of Legal **team member**
			
Linkedin - https://linkedin.com/in/anibal-suriel-esq

Lafi Raed – QSTN Lead Developer **team member** 

GitHub - https://github.com/shadow111 

Phillip Coleman - Entreprenuer In Residence **team member**
			
Linkedin - https://www.linkedin.com/in/phillip-coleman-791b1475

### Contact

- **Contact Name:** Orrin Vincent Campbell
- **Contact Email:** qstnus@gmail.com
- **Website:**

### Legal Structure

- **Registered Address:** 67 Edgehill Drive, Wappingers Falls, New York, 12590, USA
- **Registered Legal Entity:** QSTN, LLC

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Web3 Foundation previously, please list the name of the project and legal entity here.

### Team Code Repos

- https://github.com/qstnus 
- https://github.com/qstn-ansr 
- https://github.com/<your_organisation>/<project_2>

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/shadow111
- https://github.com/<team_member_2>

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/realorrin
- https://www.linkedin.com/in/anibal-suriel-esq/
- https://www.linkedin.com/in/phillip-coleman-791b1475/ 

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),
- Here are links relevant to the problem - Article 1: https://www.npr.org/2021/02/25/971460327/tiktok-to-pay-92-million-to-settle-class-action-suit-over-theft-of-personal-data 
Article 2: https://www.reuters.com/technology/metas-facebook-pay-90-million-settle-privacy-lawsuit-over-user-tracking-2022-02-15/
Article 3: https://news.bloomberglaw.com/litigation/google-plus-7-5-million-privacy-settlement-gets-final-nod
- Here is Medium where we post on data, De-Fi and NFTs - https://medium.com/@qstnus, stay up-to-date on our Twitter - https://twitter.com/qstnus 
- Here is a link to the conversation I had with Neil deGrasse Tyson about data rights: https://youtu.be/ImdmYU6ykCo?t=1568
- Please reference these two presentations for early iterations on the NEAR Protocol blockchain - Video 1: https://youtu.be/XnHjM4lQiZM, Video 2: https://youtu.be/dn49lQPibwU, 

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
- **Full-Time Equivalent (FTE):**  Average number of full-time employees working on the project throughout its duration (see [Wikipedia](https://en.wikipedia.org/wiki/Full-time_equivalent), e.g. 2 FTE)
- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in Bitcoin, the amount will be calculated according to the exchange rate at the time of payment.

### Milestone 1 Example — Implement Mintbase Modules

- **Estimated duration:** 2 months
- **FTE:**  3 FTE
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense | We will open source the smart contract functions used within our platform to grant permission and eventually mint NFTs within our store on Mintbase 
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can purchase an NFT from our platform and mint it within Mintbase as their first “NEAR” marketplace touchpoint 
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** on Medium that announces the integration, how users can now mint their NFTs within Mintbase and why we decided to use Mintbase as our technological partner (e.g. smart contract royalties, creator marketplaces, grant program, audio NFTs) 
| 1. | Mintbase module: X | We will create a Mintbase / NEAR module that will grant permission for QSTN media owners to mint a copy of their media as an NFT in our Mintbase store [grantMinter function]
| 2. | Mintbase module: Y | We will create a Mintbase / NEAR module that will mint copies of media as an NFT in our Mintbase store [mint function] once the user has been granted permission by our smart contract 
| 3. | Mintbase module: Z | We will create a Mintbase / NEAR module that will redesign our UI to incorporate a “mint on Mintbase” function within the media dashboard as well as making sure our smart contract meets Mintbase NEP standards
| 4. | NEAR chain integration | Modules X, Y & Z of our custom chain will interact in such a way that users will purchase media within our marketplace and request to mint [Z], this request calls our backend which confirms the user has a purchase history of this media. If so, then our backend smart contract grants that specific user permission to mint their media [X]. Once permission has been granted, the smart contract then mints the actual NFT [Y] and the user has the option to then sell or auction within the greater Mintbase marketplace. 

### Milestone 2 Example — Additional features

- **Estimated Duration:** 1 month
- **FTE:**  1
- **Costs:** 4,000 USD

...
## Future Plans

Please include here

-  In the short term, we are competing in hackathons to better our understanding of Web3 products and services we can use to scale out the QSTN market research portal. 

Particularly, we will be participating in the Filecoin Sustainable Blockchain Summit Hackathon, Encode x Chainlink Hackathon and submitting for a couple accelerator programs which include VC pitching. 

In conjunction with our hacking and education, we will be testing our existing application privately with different companies ranging from Encode Club, Filecoin, Octopus Network, NEAR Protocol to Ocean Protocol. 

We would love the chance to test with Mintbase and potentially even demo our upcoming social widget!

- Looking towards the future, we are excited to host our first NFT launch event on NEAR Protocol (September) and work with more traditional Web2 businesses in music, entertainment and market research to onboard the next million users into Web3. 

We intend to use this NFT launch as chance to demonstrate our Mintbase integration by allowing all users (including NFT buyers) to mint their NFTs in the Mintbase marketplace for them to transact in a secondary market.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Twitter

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- NEAR grants program 
