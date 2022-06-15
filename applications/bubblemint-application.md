# Mintbase Grant Proposal

![banner](./img/bubblemint_banner.png)

---

- **Project Name:** Bubblemint
- **Team Name:** AktaryTech
- **Payment Address:** shardeez.near
- **[Level](../README.md#level_slider-levels):** 3

## Project Overview :page_facing_up:

Bubblemint is a "lazy minting" or "candy machine" strategy that will be open-source asset for anyone wanting to launch a collection with Mintbase.io contracts.

The initial user will be our own NFT project, Shardeez. Subsequent users will be anyone with pre-generated assets and wants to use Mintbase contracts. These users will handle the upload and hosting of the assets and minting page. In subsequent milestones, Bubblemint tools will be extended into a no-code launchpad, which will make Mintbase a major source of large generative art projects.

### Overview

AktaryTech is a Web3 development service company. We normally build dApps for clients, including DeFi and NFT Marketplaces, but we wanted to create our own product as well and we started with Shardeez. We have an NFT artist on the team who does AI generation, and we chose NEAR as the ecosystem to grow with. So Shardeez are "Sharded Blocks", an homage to the sharded blockchain NEAR, which we believe can scale properly for the future.

#### Why Bubblemint?

Tagline: Gumball Machine Minting System for NEAR using Mintbase.io

We found that TenK DAO has a good open source "lazy minting" platform and forked it. It has a frontend site for the wallet connection and data on remaining NFTs to mint, and has different configurations based on whether your wallet is whitelisted and at what period of the mint.

We ran into issues getting it set up, including Apple M1 compatibility, smart contract issues, static site generation problems, and there are wallet mishaps under heavy use. But we got it working! All said, we gained familiarity with the frontend and their TypeScript / Rust smart contracts. Which gave us the confidence to remake it.

Mintbase.io is the perfect technology for us to remake this lazy minting / candy machine system. From the advanced revenue sharing, to 3XR VR galleries (with cool features like sound getting louder as you approach a music NFT), to the general quality of code from Mintbase's extended time in development of NFTs and indexers. We believe in your flexible approach to the technology that allows anyone to experiment with new NFT ideas.

#### Why Shardeez?

Tagline: Shardeez are "Sharded Blocks" on an eco-mission.

1. They look crazy cool. NEAR needs something like this.
2. Our team agreed on utility: an ecological mission. The Shardeez DAO will fund environmental projects.
3. Sharded blockchains are the future for settlement layers. Shardeez are their mascot.
4. We love our story and rarity trait formation. The 13 Clans are in groups of Eco-Defenders, Ecocidal Maniacs, and Degens. Each item has a Genesis Block, a Shardness Level, and a Clan. Each Clan has a King and Queen.

### Project Details

Screenshot of the mint page demo on testnet, showing the slider, minting allowance, and remaining in the collection.

![](./img/shardeez-mint-page-screenshot.png)

The first major milestone has three parts:

1. Smart Contracts: Mintbase.io contracts.
3. Bulk Arweave Uploads.
2. Mint Page: Frontend, Wallet, User, Reveal, etc. functionality.

Most of the work will be making a frontend that communicates with the smart contracts about the features of the NFT. We want to make a smooth and customizable experience that will be desired by anyone launching a collection on NEAR. Therefore, it will become a de-facto implementation; widely forked and repurposed. The first repurpose will be our second major milestone: a no-code launchpad.

Technology Stack:

- Frontend: React, Next.js
- Backend:
- Hosting:  
- Smart Contracts: NEAR SDK, Mintbase API, Rust
- Storage: Arweave

-----------------------------
We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
- Data models / API specifications of the core functionality
- An overview of the technology stack to be used
- Documentation of core components, protocols, architecture, etc. to be deployed
- PoC/MVP or other relevant prior work or research on the topic
- What your project is _not_ or will _not_ provide or implement
  - This is a place for you to manage expectations and to clarify any limitations that might not be obvious
-----------------------------

### Ecosystem Fit

-----------------------------
Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
- What need(s) does your project meet?
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems?
-----------------------------

## Team :busts_in_silhouette:

### Team members

Our US team is based mainly in Portland, Austin, and L.A.

### Contact

- **Contact Name:** Shane Neeley
- **Contact Email:** sneeley@aktarytech.com
- **Website:** [https://AktaryTech.com](AktaryTech.com)

### Legal Structure

- **Registered Address:** 701 Brazos St #500, Austin, Texas 78701, USA
- **Registered Legal Entity:** Aktary Enterprises LLC.

### Team's Experience, Code Repos and LinkedIn

**Shane Neeley — Dev / Artist / Experimenter —** Web3 dev, NFT artist and community manager. Previously 10 years experience in machine learning for healthcare. Author of two books on AI art. Launched and sold out an NFT with generative art, built backend for token reservations. Skilled in technical writing, marketing, Python, Tensorflow, Node.js, and is learning Rust.

[linkedin.com/in/neeley1](https://linkedin.com/in/neeley1) || [github.com/shane-neeley](https://github.com/shane-neeley)

**Adam Kecskes — Projects Orchestrator —** 25 years tech veteran with a background in project management, customer advocacy, operations improvement, frontend and backend architecture (C/C++, JS/React, Rust, more), M&A software due diligence, technical writing. Degrees in Math and CS.

[linkedin.com/in/adamkecskes](https://linkedin.com/in/adamkecskes) || [github.com/adamk72](https://github.com/adamk72)

**Geoffrey Ballard — Senior Engineer —** Design savvy senior software engineer with experience building and delivering complex, scalable, and robust applications for web, mobile, and desktop. Skilled in Graphic Design, Node.js, React, Next.js, TypeScript, AWS, Google Cloud, and GitHub CI/CD and Solidity.

[linkedin.com/in/gaballard](https://linkedin.com/in/gaballard)  || [github.com/gaballard](https://github.com/gaballard)

**Luke — Backend Engineer —** C.S. degree, experience in data analytics, DevOps and deployment with AWS, Google Cloud, DigitalOcean. Skilled in Python, Node.js, and smart contract programming with Solidity and Hardhat.

**Jared Childers — Blockchain Engineer —** A Gitcoin KERNEL Fellow and a Gitcoin + Filecoin APOLLO Fellow. Collector of metaverse assets. Skilled in Solidity, Hardhat, IPFS, Web3.py, Ethers.js, Chai testing and GitHub CI/CD.

[linkedin.com/in/jared-childers-343b68195](https://linkedin.com/in/jared-childers-343b68195)  || [github.com/JaredChilders](https://github.com/JaredChilders)

**Paul Giordano — Asset Manager —** Chief Investment Officer of Keccak Capital. Thirty-nine years of experience in finance and trading. Skilled in treasury management.

[linkedin.com/in/paul-giordano-25b8618a](https://linkedin.com/in/paul-giordano-25b8618a)

**David Aktary — CEO —** Founder of AktaryTech and three other companies. Founded ERC dEX, one of the first Ethereum token trading platforms in 2017. Currently leading Keccak Capital.

[linkedin.com/in/aktary](https://www.linkedin.com/in/aktary/)

**Kim Albee — Director of Marketing —** As the Director of Marketing, Kim brings a wealth of startup and marketing expertise to AktaryTech. She is the founder of the B2B Online Marketing Group on LinkedIn, and is also on faculty at CEOSpace. She has helped businesses attract high quality leads, and establish the engagement necessary for sustained growth. She was named a Top 40 Digital Marketer in 2020.

[linkedin.com/in/kimalbee](https://www.linkedin.com/in/kimalbee/)

**Jill Love — Operations Manager —** Manager for AktaryTech handling day-to-day operations tasks across departments.

## 🗃️ Team's experience 🚧

Below are some highlights of our service delivery, with some names obfuscated of not-yet-released work.

**Client A**

We built a smart contract education for a developer-incentivized blockchain. It’s a free interactive code school that teaches developers how to create their own CosmWasm contracts and deploy dApps to the network. This app trains devs on the Rust programming language.

**Renderz**

We built the backend and frontend of an NFT marketplace that is focused on anime-themed NFTs. The app is a full marketplace, where artists can create and mint NFTs, and sell them on the primary and secondary markets. We used ImmutableX, a zk-STARKS layer 2 of Ethereum for the source for minting and the secondary market provider.

**0x_Nodes**

A unique automated cross-chain yield aggregator. We built the interconnecting architecture for their liquidity terminal. This architecture was a transport layer for interactions between chains. This comprised of Solidity smart contracts and relayers via Node.js, TypeScript and Redis.

**Hummingbot Connectors**

Hummingbot is an open source algorithmic trading bot for CEXs and DEXs. We built connectors for clients that link Hummingbot's internal trading algorithms with live information from different exchanges. They interact with an exchange's REST API and through real-time interactions via Websockets; such as by gathering order book data and sending and canceling trades. We built connectors for speed in Python, Cython and AsyncIO. The projects we’ve done so far:

- **IDEX**  — connector for a hybrid liquidity platform with an off-chain order book + on-chain AMM
- **System9**  —  a crypto market maker who needed an OSL Exchange connector.

**Umee.cc**

We built a beautiful dApp for a cross-chain DeFi hub for lending, borrowing, rates, staking, and interoperability. We built the frontend in React and managed many backend smart contracts and integrations. Our contracts are currently running on a Cosmos-based chain as well as Ethereum Goerli and Kovan testnets. The interface was recently featured on CoinBureau.

**Client B**

An on-chain game, modeled after a roulette wheel where bets are placed in BNB tokens on the Binance Smart Chain. For game logic, we used Chainlink VRF (Verified Random Function), and Chainlink Keepers for trustless spinning of the wheel. We used the Graph Protocol for indexed user information and Tailwind CSS Framework for the UI theme.

**UniTrade**

We built and designed an system to place limit orders on Uniswap v2. Our React + Web3.js front-end provided an intuitive, user-friendly interface that interacted with the Uniswap contracts and submitted orders to the server process for processing.

**Panvala**

We helped build a protocol for cryptocurrency donation-matching on L2s (Polygon and xDai) using React, EthersJS, Web3.js and Gatsby. This matching fund allows for small contributions with low fees and integrates to existing donation sites.

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
- **Full-Time Equivalent (FTE):**  Average number of full-time employees working on the project throughout its duration (see [Wikipedia](https://en.wikipedia.org/wiki/Full-time_equivalent), e.g. 2 FTE)
- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in Bitcoin, the amount will be calculated according to the exchange rate at the time of payment.

### Milestone 1 Example — Implement Mintbase Modules

- **Estimated duration:** 1 month
- **FTE:**  2
- **Costs:** 8,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Mintbase module: X | We will create a Mintbase / NEAR module that will... (Please list the functionality that will be implemented for the first milestone) |  
| 2. | Mintbase module: Y | We will create a Mintbase / NEAR module that will... |  
| 3. | Mintbase module: Z | We will create a Mintbase / NEAR module that will... |  
| 4. | NEAR chain integration | Modules X, Y & Z of our custom chain will interact in such a way... (Please describe the deliverable here as detailed as possible) |  


### Milestone 2 Example — Additional features

- **Estimated Duration:** 1 month
- **FTE:**  1
- **Costs:** 4,000 USD

...
## Future Plans

Please include here

- how you intend to use, enhance, promote and support your project in the short term, and
- the team's long-term plans and intentions in relation to it.


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Web3 Foundation Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.
