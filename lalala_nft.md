# Mintbase Grant Proposal

- **Project Name:** LaLaLa
- **Team Name:** PickleLabs
- **Payment Address:** picklelabs.near (To Be Taken)
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

LaLaLa lets you easily mint and redeem NFTs with encrypted information. It also provide a whitelabel marketplace powered by Mintbase to
enable easy trading of NFTs for its customers.

### Overview

LaLaLa is a platform for anyone to easily mint NFTs with encrypted information, allowing tradability of the corresponding NFTs and their redemption. The usecase focuses on e-commerce brands publishing their NFTs with utilities e.g. promotional / discount codes or physical item backed NFTs, allowing customers to trade and redeem them with the brands. This enables promotional codes to be tradable in the open market implying better redemption of customer rewards and improved incentivization to loyal customers. Furthermore, LaLaLa also enables brands to efficiently create and manage NFT marketplaces on their e-commerce platforms for peer to peer trading of brand's NFTs. This provides certain benefits to a business in the form of increased redemption of vouchers, higher customer engagement, value extraction from the secondary sales, and potential newer channels of reward distribution generating higher brand outreach. On the other hand, liquid vouchers provide newer ways for redemption to extract value for the customers.

LaLaLa builds on top of Mintbase SDK and APIs, as it simplifies the core work of creation of NFTs and corresponding whitelabeled marketplaces, and the project focuses on providing the encryption of information in the NFTs and their redemption.

### Motivation

Based on a very positive market uptake of NFTs and web3 tech by big brands in the recent 12-15 months, we foresee a big rise in users and brands moving to web3 stack. This requires a low friction entry point for web2 brands and users to benefit from web3 features. Currently, e-commerce primarily revolves around brand and customer interactions, and there is no aspect of customer to customer interactions. Web3 is based around communities, and focuses on community value extraction. This could be fruitful to the web2 players as well, for instance, enabling customer to customer interaction channels could open up new redemption mechanisms for loyalty rewards, new ways of getting customers (e.g. distributing NFT vouchers in games, metaverses).

LALALA focuses on applying web3 tech and philosophy of open finance onto promotional vouchers. The brands currently distribute digital voucher codes for different marketing purposes, however, the only way they are useful for the customers if they interact and redeem it with the brand. It reduces the possibility of redemption, as many people simply hold them and never redeem them. Having vouchers as NFTs on a public chain, would allow tradability and offer more routes for redemption for customers. Moreover, it increases the potential redemptions with the rise in reach of vouchers, and would have higher probability of ending up with someone who is interested in redeeming it with the brand. Therefore, enabling trading of vouchers among brand customers enables first C2C interaction which is missing in current ecommerce models, and lets the brand and customer extract optimal value.

### Project Details

Brand ([UI Mockups](https://www.figma.com/file/t8FkOqHVKF2stb46hNOBpL/LALALA?node-id=0%3A1)):

- NFT store admin dashboard, login with web3 credentials, account setup
- Manage store (royalty, any other settings)
- Mint NFT vouchers - Submit promo codes with corresponding images (e.g. showing info for the promo code like 20% discount) and mint NFTs for the same. These promo codes are stored with the PickleLabs service, available to be redeemed against NFTs.
- NFT distribution - Generate claim links for NFTs, which could be shared on emails, Twitter (e.g. [TeaParty](https://app.teaparty.life/) or [SocialPay](https://github.com/Zilliqa/zilliqa-social-pay)).

Brand Customer ([UI Mockups](https://www.figma.com/file/t8FkOqHVKF2stb46hNOBpL/LALALA?node-id=19%3A688)):

- Uses the claimlink to open brand's NFT store to claim
- Connect existing web3 account
- Buy / sell NFT in the NFT store
- Burn NFT to redeem promo code, which could be used at the brand's e-commerce store

### **Technology Stack**

- Frontend
  - React - NextJs Framework
  - Mintbase JS SDK for mintbase interaction
- Backend
  - NodeJs for api development
  - MySQL database for storing information related to webapp. More details in database schema.
  - Rust, NEAR SDK, Mintbase API for smart contracts
- Cloud Infra
  - Amazon Web Service (AWS)
- CI/CD for deployment of webapp and backend application
- Scaling is not a challenge initially, once we require cloud infrastructure to scale we intend to use horizontal scaling for our traffic needs.

### Core Components

- Voucher Minting - As each NFT here could contain unique encrypted information in it, it could become a hassle for a user to create multiple NFTs. Therefore, this service allows a brand to simply upload the promo codes, and add a provided address as a minter; the service would take care of creating the corresponding NFTs for the store. The minter address would also be used to enable automatic redemption.

- Distribution Service - It allows a brand to share the minted NFTs as claim links with their customers/users. This provides a simplified
  way to onboard users to web3 and their NFT marketplace, and to share NFTs with their users on social media platforms or through emails.
  This would require the development of a smart contract similar to [Near drop](https://near.org/blog/send-near-to-anyone-with-near-drops/) but for NFTs.

- Redemption Service - This enables the redemption of the promo codes against the relevant NFTs. On redeeming the NFTs, the service decrypts
  the promo code, and shared with the verified redeemer through token gating based on on-chain credentials to confirm the burning and ownership of the address.
  Brands could easily use this service by adding a provided public key / wallet as a minter, which helps
  them automated the minting and redemption process. We would be engaging with the wallet teams to enable decrypt functionality to provide a better UX so
  users can safely decrypt the info with their wallets similar to [how MetaMask handles this](https://ethereum.stackexchange.com/questions/49321/how-to-decrypt-an-encrypted-message-using-metamask). Going forward, we would also explore decentralizing the redemption service with the help of protocols like [Lit protocol](https://litprotocol.com/). There could be many other usecases for info encryption/decryption in NFTs other than promo codes, however, we would be mostly focusing on business development with small to medium e-commerce brands with the tradable voucher codes, as this enables them to go web-3 without making drastic changes to their existing pipeline.

- Marketplace - A whitelabeled marketplace would be provided which is customized for the promo code NFTs, e.g. filters based on expiry date. Further customization options would be possible as we explore the usecase with different brands. Brands would be able to easily integrate the NFT marketplaces under their sub-domains.

### Database Design

- Schema for storing NFT creation via our minting service
- Store Schema for marketplace setting and NFT creation form

[DB Schema (tentative)](https://dbdiagram.io/d/633939717b3d2034ff04d900)

### API

- Create new business
- Update information / deployed stores for business
- Save NFT creation tokens for minting
- Fetch details for non minted NFTs
- Redeem Voucher

[API Collection](https://www.getpostman.com/collections/fab09c84c5192879f7dc)

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem? LaLaLa provides a novel redemption utility for encrypting/decrypting information in NFTs, e.g. promo codes info in the NFTs, allowing brands to make the utility enabled NFTs.

- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)? Target users include small to medium e-commerce brands and web3 stores looking to create NFTs for optimally incentivizing their customers, and boosting their business.

- What need(s) does your project meet? It allows brands to easily mint utility based NFTs, which easily fits in their existing business pipeline; thus providing them frictionless entry towards web3 to expand their business.

- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems?
    To the best of our knowledge, no project is focusing on these types of utility based NFTs e.g. promo codes.
    A related complementary project is (Lit Protocol)(https://litprotocol.com/), which we intend to use once it is mature enough and supports Near chain.

## Team :busts_in_silhouette:

### Team members

- Manish Kutaula
- Kartik Asooja
- Ruchika Dhamija

### Contact

- **Contact Name:** Manish Kutaula
- **Contact Email:** mk@picklelabs.xyz
- **Website:** www.picklelabs.xyz

### Legal Structure

- **Registered Address:** 91 Springboard, 90B, Udyog Vihar, Sec 18, Gurugram, Haryana, India
- **Registered Legal Entity:** Pickle Labs Technology Private Limited

### Team's experience

We love our day-to-day alignment struggles, but at the same time ring-fence, the ideas through research, belief & go to market plan when it comes to any concept. One such example is MustPool. It is a prize-linked saving platform where your deposits earn interest & you can win prizes in regular draws. It is an attempt to make sure every investor makes maximum return on their savings.

[Kartik](https://www.linkedin.com/in/kartik-asooja/) was the originator of the idea, and lead the product development. MustPool also received a grant from ZilHive accelerator, with a beta product already live. Kartik has been following the crypto industry since 2017, driven by the interest in the evolution of money, the open source nature of the industry, and the ability to deploy large finance applications on blockchains with not requiring trust and the ability to verify.

Later, he was joined by Manish, who comes with 10+ years of experience as a technolgy leader building solutions for ecommerce, hospitality and retail industries. Some of the past interesting projects involve building a restaurant reservation platform and scaling it to millions of users across multiple countries, building up supply chain and omnichannel ecommerce suite for retail brands, and getting his hands dirty with Web3 space. This space excites him so much that he has moved on from his last job as a CTO with a retail chain catering to rural India with a scale 10000 stores & is investing 100% time in his Web3 experimental projects.

They both are regularly pushed out of their ideal market scenario by Ruchika, who comes with 11 years of experience in marketing & design.
She started her career as a trend forecaster with General Motors but has moved to handle digital for a hyper-local edge CDN network catering to India's masses in semi connected or disconnected state.

The trio is supported by friends from marketing, design and technology backgrounds who are contributing to the project to learn more about this space & are willing to invest extra hours for the love of it!

### Team Code Repos

- https://github.com/graphicmist
- https://github.com/kasooja/

Some of the projects (e.g. MustPool) might not be shared publicly right now, however, happy to add any git accounts who is willing to review.

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/kutaula/
- https://www.linkedin.com/in/kartik-asooja/
- https://www.linkedin.com/in/ruchikadhamija/

## Development Status :open_book:

The work is currently in the design phase both in terms of architecture and UX. Some details have been shared above in the project details, api and db design sections above.

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 3 months
- **Full-Time Equivalent (FTE):** 2.5 FTE
- **Total Costs:** 48,000 USD

### Milestone 1 - Hidden info NFT minter / Creator UX

- **Estimated duration:** 1 month
- **FTE:** 2.5
- **Costs:** 16,000 USD

| Number | Deliverable                         | Specification                                                                                                                                                                                       |
| -----: | ----------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|    0a. | License                             | MIT                                                                                                                                                                                                 |
|    0b. | Documentation                       | We will provide both **inline documentation** of the code and a basic **tutorial** that explains the usage of the platform.                                                                         |
|    0c. | Testing Guide                       | Core business logic will be covered by unit tests to ensure functionality and robustnes.                                                                                                            |
|    0d. | Docker                              | We will provide a Dockerfile(s), wherever possible, that can be used to test all the functionality delivered with this milestone.                                                                   |
|    0e. | Article                             | We will publish a **article** explaining how anyone can now easily mint NFTs with unlockable content using LALALA platform, and the feature implementation with the help of Mintbase and its grant. |
|     1. | Business Onboarding & Account Setup | Business login and onboarding via near wallet, Adding minter for using minter service.                                                                                                              |
|     2. | NFT Studio Interface                | Interface to mint/create NFTs, view all NFTs, NFT detail page with actions like LIST, TRANSFER, BURN & VIEW Vouchers.                                                                               |
|     3. | Backend                             | Integration of APIs to store business info, save NFT form in database; Job to mint NFT via minter service.                                                                                          |

### Milestone 2 - Info redemption / Redeemer UX / Custom Marketplace

- **Estimated duration:** 1 month
- **FTE:** 2.5
- **Costs:** 16,000 USD

| Number | Deliverable            | Specification                                                                                                                     |
| -----: | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
|    0a. | License                | MIT                                                                                                                               |
|    0b. | Documentation          | We will provide both **inline documentation** of the code and a basic **tutorial** that explains the usage of the platform.       |
|    0c. | Testing Guide          | Core functions will be fully covered by unit tests to ensure functionality and robustness.                                        |
|    0d. | Docker                 | We will provide a Dockerfile(s), wherever possible, that can be used to test all the functionality delivered with this milestone. |
|    0e. | Article                | We will publish a **article** explaining how brands can build there own NFT markeplace and enable NFT trading for its customers   |
|     1. | Marketplace Storefront | A market where any users can list, view and purchase NFT.                                                                         |
|     2. | User NFTs & Redemption | User account section with functionalities to list, edit price and redeem NFT contents.                                            |

### Milestone 3 - Claim link generator

- **Estimated duration:** 1 month
- **FTE:** 2.5
- **Costs:** 16,000 USD

| Number | Deliverable    | Specification                                                                                                                                                                                       |
| -----: | -------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|    0a. | License        | MIT                                                                                                                                                                                                 |
|    0b. | Documentation  | We will provide both **inline documentation** of the code and a basic **tutorial** that explains the usage of the platform.                                                                         |
|    0c. | Testing Guide  | Core functions will be fully covered by unit tests to ensure functionality and robustness.                                                                                                          |
|    0d. | Docker         | Not applicable                                                                                                                                                                                      |
|    0e. | Article        | We will publish a **article** explaining how anyone can now easily mint NFTs with unlockable content using LALALA platform, and the feature implementation with the help of Mintbase and its grant. |
|     1. | Smart Contract | Smart contract which allows distribution of NFT in a non-custodial way.                                                                                                                             |
|     2. | Claim Link UX  | User Interface to generate claim link, toggle status of the the link and basic information whether the link has been used or not                                                                    |

## Future Plans

Short-term:

- Leveraging our network to explore business integration opportunities.

Long-term:

- Integration with projects like web3auth and lit protocol to easily onboard web2 users and provide decentralized governance structure.
- Redeemer ability to verify that the revealed information (e.g. promo code) was indeed decrypted from the burnt NFT.
- Integration with protocols like Lit and Ceramic to allow a non-custodial solution for the redemption of the decrypted information.
- Build gamification tools and loyalty programs for brands as a top layer to engage their customer and build communities.
- Marketing and expansion in business development

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?**
Mintbase Website and Twitter
