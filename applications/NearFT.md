# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** NearFT - The first AMM for NFT on NEAR Ecosystem
- **Team Name:** DeGa Labs
- **Payment Address:** nearftamm.near
- **[Level](../README.md#level_slider-levels):**  2
- **Approved for Grant:** 30,000$

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview 

NearFT - The first AMM for NFT on NEAR Ecosystem

### Overview

Most NFT marketplaces nowadays operate in an order book manner and in an auction manner. To determine a final price on a marketplace such as Opensea, Mintbase or Paras, the NFT usually goes through a bidding process where the seller sets a floor price, and the cost of the NFT changes as users bid on the NFTs. In addition, the owner must manually change the price of the NFT based on the demand of the market to have their NFT sold. 

On such order book marketplaces, users need to adjust their NFTs’ prices manually following the market trend to sell the NFTs at higher or lower current prices. This model is, however, inefficient and borderline intractable to attract liquidity for NFTs. All this can take time and be frustrating for traders and content creators. Users might sell an NFT at a price lower than the market price if their manual adjustment to the NFT price is slower than the change in the NFT market price, which has been showing that it can quickly increase or decrease in a short time. 

Furthermore, most current NFT marketplaces have a very high trading fees percentage, i.e. around 4-7% for both sellers and buyers. This trading fee is very high compared to what it is in fungible trading markets. We believe that this is also one of the reasons why NFT trading volume is very low compared to fungible asset trading volume.


### Project Details

The NearFT AMM is designed to facilitate a new way to trade NFTs. The goal is to ensure that anyone can add liquidity to an NFT collection and earn trading fees. The AMM will use the supported linear curves to solve the NFT liquidity problem. There will be 3 types of pools in NearFT: 

* NFT pool: Pool creators deposit only their NFT and specify a token P that they are willing to receive as payment token (can be native token like Near or Stable coin, or even any other token on the chain) when users buy their NFTs from the pool. The creators will receive the specified token P as others swap their tokens for the NFTs in the pools. The creators can set an initial floor price in the token P. The price of NFTs in the pool will be automatically increased when users trade P for NFTs. This is a great way for content creators to monetize their artworks by creating NFT pools.
* Token pool: Pool creators deposit only their specified payment token P and denote the NFT they are willing to buy. The pool creator can set an initial price in P for NFTs that the pool owner is willing to buy. Any user that owns an NFT specified by the pool can sell the NFT for an amount of token P.
* TRADING pool: Pool creators can create a pool by:
Specifying a payment token P and an NFT collection contract
Deposit a list of NFTs from the collection to the pool
Deposit an initial liquidity in P for the pool
It is similar to a UniSwap liquidity pool. The creator can set the initial price for NFTs in the pool. 
Anyone can buy/sell NFTs using NEAR. Creators of these pools will mostly earn trading fees from the pools.

#### Unique features in TRADING pools:
* Pool creators can set a time period that specifies a locked period for their initial liquidity. A pool creator then can only withdraw their provided initial liquidity after the locked period. The locked period will be then fully on-chain and transparent to all users
* Liquidity provision: Any one can provide liquidity to a trading by providing a list of NFT tokens from the same collection specified by the pool and a corresponding amount of the payment token specified by the pool. The amount of the payment token to provide is equal to the number of provided NFTs multiplied by the NFT price in the pool at the time the user provides liquidity. Once provided, the user will be given Liquidity Pool (LP) token for that pool that is proportional to the pool share they have in the pool

#### Bonding curves
The NearFT AMM is designed to facilitate a new way to trade NFTs. The goal is to ensure that anyone can add liquidity to an NFT collection and earn trading fees. The AMM will use the following supported price bonding curves to adjust NFT price in the pools when there are swaps:

* Linear Curve: The linear curve performs an additive operation to update the price. If a user buys an NFT from a pool, the price of an NFT will be the current price plus a delta. On the other hand, if a user sells an NFT to the pool, the price of an NFT will be the current price minus a delta.

* Exponential Curve: The exponential curve makes a multiplicative operation on NFT token price. The delta is evaluated as a multiplier or percentage with 10^8 as 1. For example, if delta is 1e8 + 1e7, this represents a 10% change in price each time. If a user buys an NFT from a pool, the next price will be multiplicatively delta more. Conversely, if a user sells an NFT to a pool, the next price it will quote to purchase NFTs will be multiplicatively delta less.

More curves will be researched and developed to NearFT over time.

#### Liquidity Pool (LP) token and liquidity handling

This works somewhat similar to how users provide and remove liquidity from Uniswap pools. However there are a few unique features that come from the uniqueness nature of NFTs
* A pool creator provides NFTs and an initial liquidity (i.e. in P token that can be a native token or any token on the supported chain) for trading, and receiving an initial amount of LP token for that pool, which basically represents 100% of the liquidity of the pool at the pool creation time
* The pool creator specifies whether the initial liquidity in the pool is locked till a certain specified time or not, until which the pool creator cannot burn the LP token to withdraw the pool liquidity
* Any liquidity addition after the pool creation time happens as follows:
  * Any one can provide liquidity including a list of NFT tokens and the corresponding liquidity in P. The amount of P needed to provide is equal to the number of provided NFTs multiplied by the current NFT price specified by the bonding curve in the pool
  * The liquidity provider will receive an amount of LP token of the pool that is corresponding to the pool share the added liquidity represents in the pool
  * The LP token can be transferred to anyone or can be used in any farming applications that have been widely used in all DeFi applications to incentivize users for providing liquidity. This practice has been shown to be very efficient in building up the liquidity pool for fungible assets in Uniswap-like AMMs. NearFT thus aims to greatly increase the liquidity and trading volume for NFT markets which is low compared to the trading volume of fungible assets.
  * Users with the LP token can withdraw/remove liquidity to retrieve NFTs and liquidity in P any time. The only exception is that the pool creator cannot transfer their LP token or withdraw liquidity before the locked period ends. 

#### NFT swap types
	
There are 4 types of swaps 
* Token-to-NFTs: buy a random list of NFTs or a specified list of NFTs from a pool with the payment token specified in the pool
* NFTs-to-Token: sell a list of NFTs to Token.
* NFTs-to-NFTs: This is an arbitration or FlashSwap for NFTs on NearFT. Any user can make this swap when there is a difference/spread in the prices of NFTs of the same collection in different liquidity pools. Users can swap from a list of NFTs for the specified payment token on a pool, then from the payment token to NFTs on another pool, which can be followed by swaps on any number of pools to get the desired NFTs. The owners of pools in this swap type earn fees, just like liquidity providers on Uniswap-like AMMs.
* Token-to-Token: this is another arbitration or FlashSwap where traders can also earn Token when there is a spread of the same NFT collection in different pools. 


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

NearFi team already launched our NearFi wallet on mainnet and is integrated into the wallet selector. NearFi Wallet will support the growth of NearFT AMM, for instance, NearFT will be integrated into NearFi to provide ease of access to NFT AMM for mobile users.

A similar model has already been done on Ethereum, the Sudoswap. Which is a proven successful NFT AMM model. An article below depicts the advantages of NFT AMM over the traditional order book market 
https://bheau.substack.com/p/sudoswap-the-nft-amm-explained?sd=pf

NearFT is implemented on Near, and extends with extensive features such as LP token and liquidity incentives that aim to greatly improve liquidity and trading volume for NFTs.

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
