# Mintbase Grant Proposal

- **Project Name:** Metronomo
- **Team Name:** Metronomo Team
- **Payment Address:** metronomo.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### Overview
[Metronomo](https://metronomo.xyz/) is a cross-chain user growth infrastructure

#### Problem
We asked a bunch of web3 projects about their biggest challenges. All of them mentioned user acquisition as an urgent issue.
Our first approach was to build "HubSpot for web3". But from the very beginning, we faced two problems:
1. On-chain data is fragmented and poorly structured to create user profiles 
2. ETL and mapping are resource intensive and expensive

#### Solution
To resolve these challenges, we decided to create a basic data layer for marketing teams and web3 builders. It will:
- structure and combine on-chain data into users, projects, interactions, and other relevant dimensions
- provide API to access and manage data
- give fundamental technologies for building martech products (e.g., NFT recommendation and personalization system, matching web2 and web3 user data, zero, 1st, 2nd & 3rd party data system, knowledge and social graph, and others)

#### Why we do
- Our team has 10+ years of experience in digital marketing, data science, development, and data engineering.
- We are highly confident that on-chain data has a significant role in scaling web3 products and the ecosystem as a whole.
- So we see our mission as **make web3 martech great by the next bull run**

### Project Details

We will deliver easy to use API, which allows functional integration in different products (ex., Mintbase UI, Telegram Bot, and others) 
- API specification: https://www.notion.so/metronomo/Projects-with-potential-power-users-7982033da4424787993fed3001a7f54a
- MVP: Telegram bot: [@metronomo_bot](https://t.me/metronomo_bot)
- Technology stack to be used: Google Cloud Functions, Google Cloud Storage, Python

### Ecosystem Fit
#### Where and how does your project fit into the ecosystem?
We see Metronomo as the user growth module of Mintbase now and part of marketplace contract 2.0 in the next iteration.

#### Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
Mintbase ecosystem projects

#### What need(s) does your project meet?
As part of the marketplace contract 2.0, allow merchants to create product selections based on user profiles and similarity to other projects to grow revenue.

#### Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
All existing NEAR products cover a single aspect of user growth but are not open-source infrastructure.
(e.g., Wombi - dashboards, Datrics - creating ML models, Plex - user profiling and engaging)

#### If so, how is your project different?
We build an open-source user growth infrastructure protocol.

#### If not, are there similar projects in related ecosystems?
0xscope, The Graph, Permission.io, and other infrastructure products are close but not entirely similar.
They are focused on general indexing issues but not on marketing data needs.

## Team :busts_in_silhouette:

### Team members

- Vladimir  Gusev (team leader)
- Yaroslav Bondarchuk
- Vadim Smirnov

### Contact

- **Contact Name:** Vladimir  Gusev
- **Contact Email:** vladimir@metronomo.xyz
- **Website:** https://metronomo.xyz/

### Legal Structure

- **Registered Address:** -
- **Registered Legal Entity:** -

### Team's experience
**Vladimir Gusev**:

- co-Founder and CEO at [Metronomo](https://metronomo.xyz/)
- ex-Partner & VP of Growth and later COO at [GigAnt - largest Workforce-as-a-Service marketplace in CIS](https://gigwork.ru/)
    - Company was successfully acquired by [Avito - World's Most-visited Classified Ads Site by Similarweb](https://www.yahoo.com/lifestyle/avito-named-world-most-visited-190600803.html)
- ex-Co-founder and CMO at Russia's TOP10 Digital Marketing Agency
    - Company was successfully acquired by the TOP1 digital marketing group
- ex-Professional online poker player from 2005 till 2010


**Yaroslav Bondarchuk**

8+ years in analytics, data science, and data engineering:
- co-Founder and CPO at [Metronomo](https://metronomo.xyz/)
- ex-Head of analytics at [GigAnt - largest Workforce-as-a-Service marketplace in CIS](https://gigwork.ru/)
- ex-Product owner at [Retail Rocket](https://retailrocket.net/) (recommender system product)
- ex-Head of analytics at [Skillbox - leading edtech platform in Eastern Europe](https://skillbox.com/)

**Vadim Smirnov**

9+ years in marketing and product growth:
- co-Founder and CMO at [Metronomo](https://metronomo.xyz/)
- ex-Head of growth at [GigAnt - largest Workforce-as-a-Service marketplace in CIS](https://gigwork.ru/)
- Advisor at [Sailplay - international marketing automation platform](https://sailplay.com/)
- ex-Performance marketing group head at [Skillbox - leading edtech platform in Eastern Europe](https://skillbox.com/)

### Team Code Repos
https://github.com/Metronomo-xyz

### Team LinkedIn Profiles (if available)

- [Vladimir Gusev](https://www.linkedin.com/in/gusevv1987/)
- [Yaroslav Bondarchuk](https://www.linkedin.com/in/bondarchukyaroslav/) 
- [Vadim Smirnov](https://www.linkedin.com/in/vadim-smirnov-71a66b189/)

## Development Status :open_book:

We are building an infrastructure protocol for creating user growth tools for web3. Currently, we are aiming to develop the first valuable tools, infrastructure, and technology they need.

We believe MVP will help us provide value for projects in the Mintbase ecosystem and create a foundation for user acquisition infrastructure protocol.

As for now, we've already done:
1. [Telegram Bot: @metronomo_bot](https://t.me/metronomo_bot) to test hypotheses on NEARCON and research projects' pains and value requests. We found out that the user acquisition part of the ecosystem lacks the most familiar tools and techniques.
2. For now, we consider that the primary method to find power users will be RFM-segmentation https://en.wikipedia.org/wiki/RFM_(market_research)
3. We created basic projects' and users' similarity analysis tools. We found out that it's possible to compute on not very expensive servers and in a reasonable time for creating a product.

During this project, we are going to get three main results:
- create a module to find power users who interacted with a given smart contract.
- create a module to calculate users similarity measures and find potential power users
- create a module to find a smart-contract which potential power users interact with

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 3,5 months
- **Full-Time Equivalent (FTE):**  5 FTE
- **Total Costs:** 25,000 USD

### Milestone 1 — Implement power-users search module

- **Estimated duration:** 1 month
- **FTE:**  0.5
- **Costs:** 2,500 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic tutorial on deploying and running created module.
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 1. | Power-users-search module | We will create a module that will search for power users of stores build on Mintbase, given the indexed blockchain data in described format.

We will create only mapping of Mintbase smart contracts methods and arguments to users engagement model 
We will create users engagement model (to measure user engagement and identify the most valuable users)

### Milestone 2 - Implement look-a-like module

- **Estimated Duration:** 1,5 months
- **FTE:**  1
- **Costs:** 7,500 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | proprietary
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic tutorial on how to deploy and run created module.
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 1. | Look-a-like module | We will create a module that will create a look-a-like (embedded vector) representation of power users in user space. 

We will create a module that will create a look-a-like (embedded vector) representation of all NEAR users in user space.
We will implement single frequency-recency engagement evaluation to create user vector representation (however, this method should be quite accurate)

### Milestone 3 - Implement the users' similarity module and the users' activity module

- **Estimated Duration:** 1 month
- **FTE:**  1
- **Costs:** 5,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Users similarity module, Similar contracts search module - Apache 2.0. API - poprietary, free for Mintbase ecosystem (proprietary for others)
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic tutorial on how to deploy and run created module.
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** and perform a workshop explaining how to use created API and receive value from this product.
| 1. | Users similarity module | We will create a module that will find the most similar users in the whole NEAR blockchain given a specific set of look-a-likes
| 2. | Similar contracts search module | We will create a module that will find and count how many potential similar to look-a-like users interacted with each smart contract (with at least one interaction during a given past period)
| 3. | API setup | We will run an API, which Mintbase users or Mintbase itself can use or integrate into Mintbase UI

## Future Plans

In the future, our primary goal is to develop protocol architecture and economics concepts based on user research.
The protocol should allow building tools like (but lot limited to):
- Recommender system for NFT marketplaces
- Personalized user incentives
- Affinity analysis
- Product analytics tools
- Open-source zero-party tracker
- Competitor analysis
- Cohort analysis
- etc.

Our roadmap includes delivering:
- General power-users analysis module that can be used as a separate open-source product
- Rewrite data extraction module (not covered by this grant application) to user NEAR Lake data (for Google Cloud) instead of NEAR Indexer for Explorer data
- Blend web2 & web3 data to enhance user acquisition
- Provide API to access and manage marketing data

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** 

from Paul Kuveke at NEARCON 2022
