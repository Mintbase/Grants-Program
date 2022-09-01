# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** NearFT - The first AMM for NFT on NEAR Ecosystem
- **Team Name:** DeGa Labs
- **Payment Address:** nearftamm.near
- **[Level](../README.md#level_slider-levels):**  2

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview 

NearFT - The first AMM for NFT on NEAR Ecosystem

### Overview

Most NFT marketplaces nowadays operate in an order book manner, meaning that it works in an auction manner. To determine a final price on a marketplace such as Opensea, Mintbase or Paras the NFT usually go through a bidding process where the seller sets a floor price, and the cost of the NFT changes as users bid on the NFTs. In addition, the owner can change the price of the NFT based on the demand of the market to have their NFT sold. On an order book marketplace, users need to adjust their prices manually. This model is inefficient and borderline intractable to attract liquidity for NFTs. All this can take time and be frustrating for traders and content creators.

We propose a solution to this problem, we are proposing to create an NFT AMM where users can instantly sell their NFTs or buy certain NFTs. Our mission is to create a mechanism to enrich the liquidity for NFTs, triggering the trading demand by creating more arbitrage opportunities, and increasing the ease of trading as well as trading volume. This leads to more incentives and benefits for NFT content creators, attracting more content creators from the traditional content market to NFTs and Web3. NearFT will be also integrated into the NearFi mobile wallet, making the trading experience much smoother for all users. The integration also creates the best experience for content creators and/or start-ups to raise funds through creating an NFT pool.

### Project Details

The NearFT AMM is designed to facilitate a new way to trade NFTs. The goal is to ensure that anyone can add liquidity to an NFT collection and earn trading fees. The AMM will use the supported linear curves to solve the NFT liquidity problem. There will be 3 types of pools in NearFT: 
- NFT pool: The creators deposit only their NFT.  They will receive NEAR tokens as others swap their NEAR tokens for the deposited NFTs. The creators can set an initial floor price, which will increase when users trade NEAR for NFTs. This is a great way for content creators to monetize their artworks by creating NFT pools.
- NEAR pool: The creators deposit only their NEAR token. The pool owner can set an initial price for NFTs that the pool owner is willing to buy. Any user that owns an NFT specified by the pool can sell the NFTs for NEAR.
- TRADING pool:  The creators deposit both NEAR tokens and NFTs to the pool. It is similar to a UniSwap liquidity pool. The creator can set the initial price for NFTs in the pool. Anyone can buy/sell NFTs using NEAR. 

In the future, we may support more bonding curves for liquidity pools such as the exponential curve. 
Some other unique features of NearFT AMM:
- Once a transaction is executed, the price changes depending on the curve set by the pool creator.
- No bidding and easily set price parameters for seller’s collection.
- No royalty fees, only exchange fees (0.5%).
- Creating specific pools for individual collections.
- There are 4 types of swaps : 
  - NEAR-to-NFTs: buy a random list of NFTs or a specified list of NFTs from a pool with NEAR as a payment token.
  - NFTs-to-NEAR: sell a list of NFTs to NEAR.
  - NFTs-to-NFTs: this is an arbitration for NFTs on NearFT. Any user can make this swap when there is a difference in the prices of NFTs of the same collection in different liquidity pools. Users can swap from a list of NFTs for NEAR on a pool, then from NEAR to NFTs on another different pool, which can be followed by swaps on any number of pools to get the desired NFTs. The owners of the pools in this swap type earn fees, just like liquidity providers on Uniswap-like AMMs.
  - NEAR-to-NEAR: this is another arbitration where traders can also earn NEAR when there is a spread of the same NFT collection in different pools. 

An overview of the technology stack we will use in NearFT:
- Front end
  - React
  - CSS
  - HTML
- Back end
  - Rust
  - Nodejs
  - Subgraph
  - APIs
- Hosting
  - IPFS
  - VPS
Some of our front end:

https://www.figma.com/file/eZsWEigdq41sbCB9tyXRKk/NearFT?node-id=0%3A1

### Ecosystem Fit

Current NFT trading volume on Near ecosystem mostly occurs via order book marketplace like Paras or Mintbase, this is inefficient and ease of trading NFTs. By providing NFT AMM, NearFT aims to create a mechanism to enrich the liquidity for NFTs, triggering the trading demand by creating more arbitrage opportunities, and increasing the ease of trading as well as trading volume. This leads to more incentives and benefits for NFT content creators, attracting more content creators from the traditional content market to NFTs and Web3.

We are thinking that the concentrated liquidity mechanism can be integrated into Mintbase’s swap section. This way users can immediately list their NFTs and swap them for tokens. Furthermore, the NearFT application will be natively integrated into the NearFi wallet which would connect users on mobile and allow them to mint and trade NFTs right on their phones. 

## Team :busts_in_silhouette:

### Team members

- Co-founder/CEO: Nguyen Bui
- Project manager: Duong Phan
- Backend Dev: Khai Do
- Backend Dev: Tony Dong
- Fullstack Dev: Hoang Anh
- Marketing lead: David Dang
- Technical Advisor: Cam Pham

### Contact

- **Contact Name:** Nguyen Bui
- **Contact Email:** nguyen@degalabs.fi
- **Website:** https://degalabs.fi/

### Legal Structure

- **Registered Address:** P.O. Box 4302, Intershore Chambers, Tortola Islands, British Virgin Islands
- **Registered Legal Entity:** Tazo Studio Ltd

### Team's experience

Nguyen Bui - Co-founder/CEO 
Experience: Ex CMO Tomochain, advisor C98, 15 years of experience in Tech marketing and financial trading

Duong Phan - Project manager
Experience: Former full stack Developer at FPT Software, 4 years of experience in blockchain development.

Khai Do - Backend Dev
Experience: Fullstack developer at Tomochain, 6 years of experience in the distributed system.

Tony Dong - Backend Dev
Experience: 4 years of experience in blockchain development and cryptocurrency. Specialized in NEAR protocol and Ethereum blockchain development.

Hoang Anh - Fullstack Dev
Experience:  Ex-full-stack developer at Tomochain and insight studio. Specialized in Web application and Web3 integration for decentralized applications. Experience in UI/UX design.  

David Dang - Marketing lead
Experience: 2 years in marketing and BD for Gamefi and Defi projects.

Cam Pham - Technical Advisor 
Experience: Ex-lead blockchain researcher at Tomochain, lead dev and smart contract auditor at Arcadia Group. Over 13 years in Software Engineering, and over 5 years in blockchain development and cryptocurrency. Ph.D. in Software Engineering, France.

### Team Code Repos

https://github.com/DegaLabs

Members' GitHub 

https://github.com/khaihkd
https://github.com/duongpad
https://github.com/hoangkianh
https://github.com/bendegalab
https://github.com/tonititi/

## Development Status :open_book:

NearFi team already launched our NearfFi wallet on mainnet and is integrated into the wallet selector. NearFi Wallet will support the growth of NearFT AMM, for instance, NearFT will be integrated into NearFi to provide ease of access to NFT AMM for mobile users.

A similar model has already been done on Ethereum, the Sudoswap. Which is a proven successful NFT AMM model. An article below depicts the advantages of NFT AMM over the traditional order book market 
https://bheau.substack.com/p/sudoswap-the-nft-amm-explained?sd=pf


## Development Roadmap :nut_and_bolt:

The project will go through 4  following milestones:
- Testnet launch
- Mainnet launch
- Support more tokens and bonding curves
- Generate LP tokens for liquidity providers.

### Overview

- **Total Estimated Duration:** 3 months
- **Full-Time Equivalent (FTE):**  4 full-time dev
- **Total Costs:** 50,000 USD

### Milestone 1  — Testnet launch

- **Estimated duration:** 1 month
- **FTE:**  4
- **Costs:** 20,000 USD

Details of what will be delivered in the milestone:

- Launch the testnet version for an AMM that supports the NEP-171 NFT standard on the NEAR ecosystem.
- Launch the testnet version in that users can create 3 types of pools on NearFT: NFT pool, NEAR pool or Trading pool.
- Launch the testnet version that completes the integration of the NearFi mobile wallet and  NearFT AMM.
- Launch the testnet version. Users can make 4 types of swaps: NEAR to NFT, NFT to NEAR, NFT to NFT and NEAR to NEAR.

### Milestone 2 — Mainnet launch

- **Estimated Duration:** 1 month
- **FTE:**  4
- **Costs:** 20,000 USD

Details of what will be delivered in the milestone:

- Launch the mainnet version for an AMM that supports the NEP-171 NFT standard on the NEAR ecosystem.
- Launch the mainnet version in that users can create 3 types of pools on NearFT: NFT pool, NEAR pool or Trading pool.
- Launch the mainnet version that completes the integration of the NearFi mobile wallet and  NearFT AMM.
- Launch the mainnet version. Users can make 4 types of swaps: NEAR to NFT, NFT to NEAR, NFT to NFT and NEAR to NEAR.

### Milestone 3 — Support more tokens and bonding curves

- **Estimated Duration:** 2 weeks
- **FTE:**  4
- **Costs:** 5,000 USD

Details of what will be delivered in the milestone:
- Together with the NEAR token, the NearFT will support other tokens that are implemented in Near Ecosystem.
- Together with linear curves, the NearFT will support other bonding curves such as exponential curves.


### Milestone 4 — Generate LP token for liquidity pool providers

- **Estimated Duration:** 2 weeks
- **FTE:**  4
- **Costs:** 5,000 USD

Details of what will be delivered in the milestone:
- Generate LP token for the project. LP tokens will be minted to the liquidity providers and will be burned when the liquidity was withdrawn.
- LP tokens can be used as an NFT and can be traded between users. 
- Users also can stake LP tokens to get the governance token of the project

## Future Plans

- We want to continually scale NearFT alongside Near Ecosystem, integrate with more wallets, improve user experience and partner with other projects to bring the best possible product and benefits to the NEAR ecosystem and Web3.
- Increase the TVL of NearFT that …. increase the TVL of Near Ecosystem.
- Token and NFT launch.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website

Being well integrated with the NEAR blockchain ecosystem through the NearFi mobile wallet, we understand the need for an NFT AMM in the NEAR ecosystem. We strongly believe that the NearFT project will bring a lot of users, TVL and contribute a lot to the development of the NEAR blockchain. 
