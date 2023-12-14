# Mintbase Grant Proposal
- **Project Name:** Cryptoracle Smart Analytics & Marketplace.
- **Team Name:**: Cryptoracle

- **Payment Address:** trust.cryptoracleio.near

- **[Level](../README.md#level_slider-levels):**  2


### Overview
Cryptoracle Smart Analytics & Marketplace.
Website: [Cryptoracle](https://app.cryptoracle.io/)

Cryptoracle stands at the forefront of leveraging web3 technologies within the NEAR protocol ecosystem. Our project encompasses a sophisticated analytics DApp that pioneers real-time market insights and analytics for NFTs, empowering investors with unparalleled data to make informed decisions. With a comprehensive view of top NFT collections, individual statistics, and Whales activity, Cryptoracle bridges the gap between creators and investors in the NFT space.

![Cryptoracle](https://imagedelivery.net/uHX7bl677huVTff9_Q4FJw/831ce441-17c6-4599-d17a-2973f4dbc700/public)

Which technologies of the mintbase stack are we implementing?
Cryptoracle seamlessly integrates with **Mintbase indexer and @mintbase-js/data**, enabling a vast array of blockchain data at the disposal of users, having as a core mission ease of navigation and visualizations while being a great example of open source in the data analytics field. 

This integration empowers users to explore and analyze data across NEAR Protocol, with special focus on NFTs, and primary chain statistics, enabling data driven decisions guided by source data from the blockchain.
Integration of wallet by using **wallet** selector and **@mintbase-js/wallet**.

The second phase goes through the implementation of **Cryptoracle marketplace** using the **@mintbase-js/sdk** to interact with the smart contracts.

Our **NFTs marketplace** development is grounded in **data science principles, engaging ux laws**, and focusing on providing users with valuable insights. From smart analytics to personalized recommendations, we aim to enhance the user experience by incorporating innovative features that respond to market trends, drawing inspiration from successful platforms in the space.

### Ecosystem fit
**Consumer DApp, NFTs Smart Analytics, NFTs Marketplace.**

We stand as a reference example of **open source development**, particularly within the dynamic landscape of NFT Analytics & Marketplace projects within Mintbase and NEAR Protocol. Showcasing excellence in implementation through the integration of the mintbase graphql indexer and @mintbase-js libraries. As a pioneering force, our open source example serves as a **beacon for others to learn from and follow**.

From a user's point of view: Cryptoracle is the previous stop before buying, they can look in one view what’s hot, trendy and what’s rising before it happens, this allows them to make data driven decisions, enhance their portfolio and connect with creators across NEAR Protocol.

What Cryptoracle brings to the NEAR Protocol and Mintbase from different points of views?

Resume:

NEAR Protocol:
* Smart analytics for the NFTs ecosystem.
* The place to find and trade outstanding NFTs collections in our Marketplace.

Mintbase: 
* Integrate the mintbase graphql indexer and @mintbase-js/data
* Data management to perform operations over indexer data.
* Integrate Mintbase Wallet. @mintbase-js/wallet
* Open source development.
* Documentation.
* Smart Analytics.
* Intuitive Dashboards.
* Marketplace Integration. @mintbase-js/sdk

Creators:
* Calendar to list their incoming stores, launch and events.
* Identify and track whales.
* Discover similar collections.

Users:
* Make data driven decisions.
* Enhance their portfolio.
* Know what is hot in the market.
* Know what is trendy.
* Know what is rising in the market.

### Project Details

Technology Stack:

* Cloudflare. Cloudflare workers. Cloudflare Images. Cloudflare Databases
* Reactjs
* mintbase-js/data to integrate the indexer
* mintbase-js/wallet integration of wallet selector and mintbase wallet
* mintbase-js/sdk for marketplace integration
* Markdown for documentation.
* Continuous Maintenance.



## Team

### Team members
[Software Engineer - Irvin Moreno](https://github.com/RevoltPW).
[Software Engineer - Oliver Moreno](https://github.com/pulsarforge).

### Contact
Email: contact@cryptoracle.io
Website: [Cryptoracle](https://app.cryptoracle.io/)


### Team's experience
Apart from the usual gathering years of experience in enterprises, We focus on high impact projects with scalability, having as a core the data analytics field, achieving outstanding results with small teams, preferring extreme quality over quantity.

### Team Code Repos
[Main Front-End Repository](https://github.com/Cryptoracle-io/analyticts-dapp)
[Back-End Repository](https://github.com/Cryptoracle-io/Serverless-backend)


## Development Roadmap
### Overview
- **Total Estimated Duration:** 5 Month
- **Full-Time Equivalent (FTE):** 2
- **Total Costs:** 50,000 USD

### Milestone 1 — Integrate the mintbase graphql indexer as the main source for data at Cryptoracle.

- **Estimated duration:** 3 month
- **FTE:**  2
- **Costs:** 35.000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License |  GPLv3  |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | @mintbase-js/data: Integration | Implementation of @mintbase-js/data built-in methods to fetch data from mintbase indexer.| 
| 2. | DataOps: Backend | Implementation of pipeline to extract data and perform operations over the raw GraphQL schema data results to transform them into key analytics. | 
| 3. | Mintbase Wallet | Mintbase Wallet Integration, @mintbase-js/wallet. | 
| 4. | Frontend integration | Indexer post processed data integration at the front-end.| 
| 5. | Frontend new interfaces  | NFT details, Portfolio Analysis.|
| 6. | Maintenance.| Part of the budget is reserved for maintenance and future upgrades |


### Milestone 2 — NFTs Marketplace deployment with Mintbase Tech Stack

- **Estimated duration:** 2 month
- **FTE:**  2
- **Costs:** 15.000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License |  GPLv3  |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | @mintbase-js/sdk: Integration | Integrate and interact with smart contracts using @mintbase-js/sdk.| 
| 2. | @mintbase-js/storage: Integration | Implementation of minter using @mintbase-js/storage. |  
| 3. | Frontend | Develop the user interfaces: Create collection, create NFT, Buy/sell/list NFT| 
| 4. | Users points reward system | Integration of user experience points system to reward users with NFTs badges after tasks completion.|



### Future Plans


Our roadmap includes expanding to multiple blockchains, reaching a substantial user base, deploying advanced analytics tools, fostering community interaction, forming strategic partnerships, offering educational resources, and ensuring a secure and user-friendly platform. These initiatives align with our commitment to becoming the leading NFT analytics marketplace DApp, providing unparalleled insights and support to the vibrant NFT community.
