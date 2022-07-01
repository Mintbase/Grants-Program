# Mintbase Grant Proposal
- **Project Name:** NLend
- **Team Name:** NLend
- **Payment Address:** nlend.near
- **[Level](../README.md#level_slider-levels):** 2 or 3



## Project Overview :page_facing_up:

### Overview

#### Our mission is to democratize finance- by bringing non-fungible assets closer to the DeFi ecosystem.

The NLend, allows users to access instant liquidity by leveraging their non-fungible assets on NEAR blockchain. NLend users can access funds without impacting their credit scores, with flexible repayment options, as well as instant approval on the collateralized loans. The NFTs used as collateral must be approved by the NLends Decentralized Autonomous Organizations (DAO). Despite highly volatile market conditions, NLend is an stable, unbiased NFT-backed borrowing protocol that offers fair and accessible finance to the masses. 

#### NLend PROTOCOL

Founded in 2022, NLend is a fully decentralized public lending & borrowing protocol built on near blockchain. It is a revenue sharing, community governed protocol which provides incentive and rewards to all participants. NLend accepts a wide range of NFT collections approved by NLend DAO as collateral in exchange for any whitelisted tokens. The components of the protocol include the lending smart contract, AI-driven NFT valuation engine and oracles. 


#### NLend DAO

DAO encompasses the NLend global community who manage and control various aspects of the protocol. We follow an effective governance operating model to provide unbiased,  transparent liquidity solutions to everyone. The DAO may influence the outcomes by leveraging their votes. DAO voting weight is directly proportional to the capital invested in the protocol. It means the greater the cap invested, a greater say in protocol?s decision-making. The primary responsibility of the DAO includes deciding on the whitelisted collections, tokens and the associated  risk parameters for each collection.

#### Workflow of the protocol
Lenders and borrowers interact by exchanging whitelisted NFTs for whitelisted tokens. Borrowers are obligated to repay their loans along with the interest in order to retrieve the collateralized NFT. If the borrower fails to do so, NFT enters the liquidation zone.

#### WHY OUR TEAM IS INTERESTED IN CREATING THIS PROJECT?

Our core team always had a passion for problem solving products. In 2022, we started iterating over ideas that could address the erosion of trust and inefficiencies in centralized financial systems using AI and blockchain. Our developers around the world started designing the user experience and implemented the first iterations of the code. 

### Project Details

####  CORE COMPONENTS OF NLend 

The NLend protocol is built on top of the NEAR chain and provides a highly transparent and scalable liquidity solution.

#### NFT collaterals 
Borrowers deposit their collateral assets into the NLend vaults on the protocol. A collateral asset is any NFT collection that NLend holders have approved into the protocol. NFT owners can collateralize their NFTs in exchange for whitelisted tokens. Collateralized assets must satisfy specific guideline requirements such as but not limited to having large trade volume, verified by leading marketplaces. Management of these risk parameters will also  be conducted by the DAO. 

Whitelisted NFTs valuation comes from Oracle integrated with AI algorithms. The cornerstone of a lending protocol is accurate valuation of the NFTs. We leverage AI to dynamically decide the value of the NFT based on the floor price, time-weighted sales and other parameters. Our team of seasoned data scientists working in multidisciplinary environments harness the power of data and technology to provide  accurate AI driven NFT valuation. All the decisions of the protocol are made through an highly effective on-chain governance operating model. Our protocol involves DAO in the entire life cycle  of the asset selection process. The initial makeup consists of a temporary DAO formed by a community chosen by the NLend team. 

#### NLend smart contracts
All whitelisted NFTs can be leveraged to generate liquidity through our smart contracts. Users can access the protocol and create vaults through interfaces built by our developers and community. Vaults are non-custodial, giving users a high level of autonomy and self-sovereignty. Users need not rely on third parties to manage their assets or perform transactions.  Users have complete control over their locked collaterals until the health score of collateral doesn?t fall below the required threshold. 

#### NLend participants
NLend  incentivizes all the participants in the system.

##### Borrowers
NLend allows users to access core lending services and borrow money in a simple and optimal way. Users can borrow money from public pools by depositing whitelisted NFTs as collateral. They are required to pay interest on their loans. Interest rates depend on the utilization value of the NFT collection. This means, More a NFT collection is collateralized, higher the interest rate. Once liquidity is provided, borrowers are obligated to pay their loans along with the incurred interest, in order to withdraw their collateral. Each collateral deposited is allocated a separate vault. When users deposit multiple NFTs, they have multiple vaults with different NFT collections as collateral.

##### Lenders
Users can lend money in the form of whitelisted tokens. Lenders are incentivized APY for providing liquidity. 

##### Liquidators
Market participants who secure the protocol, by buying liquidated NFTs at discounted prices.

#### Health score
The health score is the numeric representation of the safety of your deposited assets against the borrowed assets and its underlying value. NLend Protocol ensures that there is enough collateral to cover all outstanding debts, by liquidating risky vault based on their health score.

Health score is a cumulative of total viability score, number of collateralized assets held by the account, LTV ratio, outstanding loans held by the account and liquidation factors. To avoid liquidation scenarios, one must maintain a health score above 1 after the loan has been originated. If the user's health score reaches below 1, the positions are at the risk of liquidation. Each debt position has its own health score and  LTV which is determined by NLend DAO.

##### Categorization of health scores
Our health score is categorized into 3 zones: Good, Medium and Bad. Good health factor represents a safer place, far from liquidation. Medium health is when you start approaching liquidation, Bad is when your assets are in the position of being liquidated. Users can decrease or increase their health score by depositing more whitelisted NFTs or by repaying part of their loan. 

#### AI driven NFT valuation
AI acts as a collection tracking product. In a volatile market, offering NFT-collateralized loans is highly risky. This is because there is no reliable way to dynamically evaluate the worth of NFTs making it  highly impossible to operate an efficient lending protocol. 

Due to this reason, our AI developers focused on utilizing machine learning, etc. to predict the price of the assets at a particular time. The AI engine predicts the value using various parameters like time weighted sales information, listing price, trends of the market, previous transfers, NFTs growth and popularity, etc. This score helps in estimating the borrow limit of the assets. 

This approach enables scalability while handling exhaustive collections under one umbrella, becoming the cornerstone of our lending protocol.

#### Liquidation
If the health score of the borrowed loan becomes less than 1, the borrower gets liquidated. The liquidator can liquidate the borrower to retrieve part of the loan provided to the collateralized NFT. The liquidator collects the collateralized NFT and the penalty fees from the borrower. The NLend DAO sets the penalty fees for each NFT collection.

To incentivize liquidation of risky vault, liquidators receive a discount on the NFTs they withdraw from the borrower?s. Anyone can borrow or lend using the protocol, but only NLend holders can act as liquidators. The discount rate is determined by the tier of NLend holders (Diamond, Platinum, Gold, Silver). Diamond members get the highest discount, followed by Platinum, Gold and Silver members.

#### Volatility protection system 
Volatility protection systems enable the protocol to take the borrowers outstanding obligation, in exceptional cases. This ensures profitability for liquidators. The protocol contains the proceeds from lending and borrowing (including liquidation penalties) and stability fees accrued from the vaults. 

When the collateral?s value drops significantly in a short period, it is difficult for liquidators to earn profit from it. In these cases, the protocol assumes a portion of the outstanding debt to reduce the amount the liquidator has to repay. This makes the system resilient to market conditions. In other words, liquidations are independent of market behaviors.

#### Oracle 
Oracle integrated with our AI engine provides real-time information about market behavior of the collateral NFTs. They act as an on-chain APIs to safely query asset related information into our protocol. Decentralized oracles operate in the same way the blockchain networks operate, making it a reliable, trustless system to transfer off-chain data into our protocol. The NLend DAO chooses the trusted oracles that can interact with the protocol. 

##### What if the oracle gets compromised?
When an oracle gets compromised, the hacker is in control of the outcome, rather than the NLend. We handle this situation by delaying the decentralized oracle information by an hour, so our security team and NLend DAO can vote to freeze the oracle, if it is compromised. The security team actively resolves security issues and prevents damage to the protocol They have the authority to freeze compromised oracles.

### Ecosystem Fit

Our team is interested in exploring mintbase because the platform removes several layers of complexities by abstracting technical implementations. This is especially very important to our use case. The major challenges of DeFi based applications are the significant hacks. By using a platform to create NFTs, write smart contracts, our team can focus more on the security of the application. We will use the Mintbase for primarily three reasons

-  To mint our NFTs
-  To use GraphQL to query the health score, to get previous liquidation threshold information, risk parameters associated with each NFT collection, to get results from it to understand the behavior of risky NFT collections and vaults. 

## Team :busts_in_silhouette:

### Team members

- Founders : Acie, Ross
Acie's Twitter:https://twitter.com/aciedoteth
Ross's Twitter:https://twitter.com/addictednfthead

### Contact

- **Contact Name:** acie
- **Contact Email:** crpyticacie@protonmail.com

### Team's experience
Allow us to give you a brief idea about our team's expertise, we're a group of Web3 & crypto enthusiasts. We started adopting crypto about 8 years ago because we believe in the future of decentralized finance. 

During the many years of being in the crypto space we have sought numerous opportunities to build within the space. Coming from our backgrounds in automation, artificial intelligence, marketing, front and backend development we have came up with numerous ideas and built numerous service-based software and protocols. 

Our team expertise goes beyond simply directing and casting ideas, for example we were one of the first people to offer custom Discord bots that feature NFT services, as mentioned since we also have a background in automation and machine learning we have made even further advanced algorithms and software to aid with security for NFT projects.

Having the experience with backend development we have also been able to incorporate those bots & algorithms into modern, responsive and efficient websites.

And of course, with our marketing background we quickly understood how web3 marketing works and how to efficiently maximize our reach throughout the various techniques of web3 marketing to make sure that our services reach the right clients!


### Milestone 1 Example ? LENDING PROTOCOL DEVNET BASE LAUNCH

- **Estimated duration:** 1 month
- **FTE:**  4
- **Costs:** 25,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains to a user how anyone could utilize their NFTs as collateral to gain liquidity, and provide liquidity to earn APY.  |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. We mention about how we handle the impact of volatile market situation, oracle hacks, secuity m,itigation systems  |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish a few medium article that explains how NLend is a transparent, scalable and permissionless system that connects lenders, borrowers and liquidators in a dynamic borrowing model.
| 1. | Mintbase module: Public lending protocol encompasses the smart contract for lending and borrowing analytics  |  
| 2. | Mintbase module: UI integration | We will create a module that will integrate with lending and borrowing protocol.
| 3. | AI module: AI system design and development  | We will create a AI module that will acts as a collection tracking product.  We focus on utilizing machine learning, etc. to predict the price of the assets at a particular time. The AI engine predicts the value using various parameters like time weighted sales information, listing price, trends of the market, previous transfers, NFTs growth and popularity, etc. This score helps in estimating the borrow limit of the assets.  |  
| 4. | Oracle module: Chainlink oracle integration | We will create a Oracle module that will be integrated with our AI engine and provides real-time information about market behavior of the collateral NFTs.|  
| 5. | NEAR chain integration | Modules 1,2,3,4 of our custom chain will interact in such a way Borrowers deposit their collateral NFTs into the NLend vaults on the protocol. Collateralized assets must satisfy specific guideline requirements set by NLend DAO. Whitelisted NFTs valuation comes from Oracle integrated with AI algorithms. Health scores of whitelisted NFT is montiored regularly by the protocol. If it decrease below a certain threshold, it is liquidated. Lenders are incentivized APY for providing liquidity
|  

### Milestone 2 NFT MINTING

- **Estimated Duration:** 1 month
- **FTE:**  2
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains the minting and utility of our propreitary NFT  |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. We mention about how we handle minting vulnerabilities |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish a few medium article that explains how NLend tokenomics, coin mint for a dynamic borrowing model.
| 1. | Tokenomics module: tokenmoics design | A balanced economic model of the NLend token that takes into account the interests of all participants(investors or lenders, borrowers, liquidators)  |  
| 2. | Staking module: NLend staking program | Staking module leverage incentives and rewards to investors. The NLend holders get royalities, incentives, etc|
| 3. | Discord module: Discord member verification bot for private channels | Our NFT holders get access to private discord channels where we post updates, weekly AMA calls, and access to real time sales and listing bots |  
| 4. | Security module: Contract and Mint audit | We test the code against vulnerabilties including denial of service attacks, gas limit issues, reentrancy attacks, fake liquidation attacks, oracle compromise etc. We also provide bug bounties to incentive community to identify threats |  
| 5. | Community building| Build a strong community that brings a long lasting value of our NLend NFTs|  
| 6. | Mintbase module: NEAR NFT minting| Modules 1,2,3,4,5 of our custom chain will interact in such a way that we have a robust system that supports NFTs minting. We will use Mintbase to mint NFTs for the protocol
|  

### Milestone 2 MAINNET DEPLOYMENT OF THE LENDING PROTOCOL

- **Estimated Duration:** 1 month
- **FTE:**  4
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the open source code of deployment of the lending protocol in the NEAR mainnet  |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests.|
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish a article that explaining the architecture and core technical components of the lending protocol.
| 1. | Mintbase module: Mainnet deployment lending protocol| Full public lending launch on NEAR Mainnet, access for top tier collection holders.  |  
| 2. | Governance module: DAO governance formation | temporary DAO is chosen by the team to implement an effective governance operating model to provide unbiased,  transparent liquidity solutions to everyone. |
| 3. | Mobile wallets support module | To support mobile based wallet transactions and interactions with the protocol|  


## Future Plans

- Magnified Leverage of protocol - By integrating with broad range of wallets and NEAR marketplaces
- Plans and systematic framework design for achieving XXm TVL milestone


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Web3 Foundation Website & Medium.
