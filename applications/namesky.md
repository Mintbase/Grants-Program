# Mintbase Grant Proposal

- **Project Name:** NameSky
- **Team Name:** NameSky Team
- **Payment Address:** liaa.near
- **[Level](../README.md#level_slider-levels):** 1

## Project Overview 📄

NameSky is an NFT project on the NEAR blockchain. It’s built with utilities to make buying & selling NEAR accounts easy and secure.

### Overview

Please provide the following:

- NameSky is the first marketplace to buy and sell NEAR accounts as NFTs.
- We allow users to trade near accounts (through NFT) on Mintbase.
- Buying & selling a NEAR account is never an easy thing. So we started the NameSky project with a mission to make NEAR account trading easy and secure.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

* The current UI and designs are covered in our MVP demo and deployed on the testnet. (links attached below)
* Data models / API specifications of the core functionality

  * NFT Contract:

    * Mint

    > Alice has an account `love.near` and her main account is `alice.near`. Alice turns `love.near` into an NFT owned by `alice.near`  through the NameSky app.
    >

    - Transfer

    > Alice transfers the NFT `love.near` to her friend Celia (celia.near).

    - Burn

    > Celia burns the NFT `love.near` and gets a  **New Seed Phrase** to control account `love.near` again.
    >
  * NFT Marketplace Contrat:

    - Buy & Sell NFT
    - Make an offer & Accept the Offer of NFT

  - Other functionalities:
    - Search Near account NFT by account name
    - Search user by user name
    - Like an NFT
    - Notification for NFT buy, sell and offer

* Technology Stack:
  * Smart Contracts: Rust, NEAR SDK, NEAR SYS
  * Frontend: React, Typescript, Graphql
  * Server: Node.js
  * Indexer: The Graph

* PoC/MVP or other relevant prior work or research on the topic

  * Demo vedio: [https://youtu.be/71JUKrTWSPA](https://youtu.be/71JUKrTWSPA)
  * Testnet MVP app: [https://testnet.namesky.app](https://rentverse.netlify.app/)

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

Q: Where and how does your project fit into the ecosystem?
In the NEAR blockchain, there is no good way to trade NEAR accounts on a scale. NameSky is solving this problem by leveraging the power of the NFT protocol and making it possible to trade NEAR accounts easily and safely in scale. Also, NameSky is a utility-heavy NFT project that can show the good use case of the NFT protocol. 



Q: Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?

1. First of all, **myself**, **Demo Chen**. I desperately want to have the NEAR account `demo.near` as my identity on the NEAR blockchain, but that account is already registered by someone else. I have no way to even make an offer to him to buy `demo.near`.

2. **NEAR account collectors and investors**. It's a proven business both in the web2 and the web3 world. It's very rewarding when you invested early in web domain names or ENS names.
3. **Founders and Bussiness Owners**. They always want a good NEAR account for people to remember their brand and identity. NameSky is the place for them to find the NEAR accounts they want. 




Q: What need(s) does your project meet?
- NameSky makes it possible to turn a NEAR account into an NFT and vice versa
- NameSky has a dedicated marketplace front to buy/sell NEAR accounts (as NFTs)
- NameSky makes NEAR account sales discoverable through a Twitter bot [NEAR accounts sales bot](https://twitter.com/nearaccountsbot)

Q: Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?

There are few prior projects that to trade NEAR accounts, NameSky is the first one that takes advantage of the NFT protocol and also provides an easy-to-use interface to make NEAR account trading approachable, scalable and reliable.

Similar projects:
- https://gonear.name
- https://nomenmarket.com
- https://nearnames.io

## Team 👥

### Team members

- Name of team leader
    - Demo Chen
- Names of team members
    - Bob Xu
    - Coffee
    - Clemens

### Contact

- **Contact Name:** Demo Chen
- **Contact Email:** kidliaa@gmail.com

### Legal Structure

We don't have a formally registered company at the moment.

### Team's experience

- **Demo Chen**, product and frontend developer. 10 years of experience in web development and loves building things. The founding team of workstream.us.
- **Bob Xu**, contract developer. A big fan of NEAR since early 2021, and an active member in the NEAR ecosystem. 5 years of experience in backend development and 1.5 years experience of in NEAR contract development.
- **Coffee**, contract developer. Familiar with the NEAR ecosystem and active in the NEAR community.
- **Clemens**, product designer. https://clemensmorris.com


### Team Code Repos

- https://github.com/nameskyteam

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/hsxyl
- https://github.com/hash0000ff
- https://github.com/hanakannzashi


### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/liang-chen-3b5855132/

## Development Status 📖


- GitHub repos (we now in private mode, let us know to add you as a member): https://github.com/nameskyteam
- testnet MVP version: https://testnet.namesky.app
- Near account NFT sales bot: https://twitter.com/nearaccountsbot
- previous interface iterations, such as mock-ups and wireframes. [Figma design link](https://www.figma.com/file/3PgMxMQmsttOdccg32H7bq/NameSky?node-id=22759%3A2639&t=ceAaZHDHZeKUM70n-0)

## Development Roadmap 🔩

### Overview

- **Total Estimated Duration:** 6 months
- **Full-Time Equivalent (FTE):**  3 FTE
- **Total Costs:** 20,000 USD

### Milestone 1 — Audit

- **Costs:** 20,000 USD
- We have contacted some security companies, and they have feedback us about their price:
  - BlockSec: ~80,000 USD
  - Pessimistic Security: ~30,000 USD

## Future Plans

- At the moment, our first priority is to apply funds for auditing our Near contracts then we can go mainnet.
- In the long term, We will collaborate with other NFT projects on NEAR to integrate near account NFT in their project. And we'll keep optimizing and developing new features based on user feedback.

## Additional Information ➕

**How did you hear about the Grants Program?**  Announcement by another team

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.

  > We have finished Milestone 1.
  
- If any other teams have already contributed (financially) to the project.

  > No.
  
- Previous grants you may have applied for.

  > We haven't applied for funds.
  
