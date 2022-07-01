# Mintbase Grant Proposal
- **Project Name:** NLend
- **Team Name:** NLend
- **Payment Address:** nlend.near
- **[Level](../README.md#level_slider-levels):** 2 or 3

> ?? *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

### Overview

####  ?Our mission is to democratize finance- by bringing non-fungible assets closer to the DeFi ecosystem?

The FTY, allows users to access instant liquidity by leveraging their non-fungible assets on NEAR blockchain. FTY users can access funds without impacting their credit scores, with flexible repayment options, as well as instant approval on the collateralized loans. The NFTs used as collateral must be approved by the FTYs Decentralized Autonomous Organizations (DAO). Despite highly volatile market conditions, FTY is an stable, unbiased NFT-backed borrowing protocol that offers fair and accessible finance to the masses. 

#### FTY PROTOCOL

Founded in 2022, FTY is a fully decentralized public lending & borrowing protocol built on near blockchain. It is a revenue sharing, community governed protocol which provides incentive and rewards to all participants. FTY accepts a wide range of NFT collections approved by FTY DAO as collateral in exchange for any whitelisted tokens. The components of the protocol include the lending smart contract, AI-driven NFT valuation engine and oracles. 


#### FTY DAO

DAO encompasses the FTY global community who manage and control various aspects of the protocol. We follow an effective governance operating model to provide unbiased,  transparent liquidity solutions to everyone. The DAO may influence the outcomes by leveraging their votes. DAO voting weight is directly proportional to the capital invested in the protocol. It means the greater the cap invested, a greater say in protocol?s decision-making. The primary responsibility of the DAO includes deciding on the whitelisted collections, tokens and the associated  risk parameters for each collection.

#### Workflow of the protocol
Lenders and borrowers interact by exchanging whitelisted NFTs for whitelisted tokens. Borrowers are obligated to repay their loans along with the interest in order to retrieve the collateralized NFT. If the borrower fails to do so, NFT enters the liquidation zone.

#### WHY OUR TEAM IS INTERESTED IN CREATING THIS PROJECT?

Our core team always had a passion for problem solving products. In 2022, we started iterating over ideas that could address the erosion of trust and inefficiencies in centralized financial systems using AI and blockchain. Our developers around the world started designing the user experience and implemented the first iterations of the code. 

### Project Details

####  CORE COMPONENTS OF FTY 

The FTY protocol is built on top of the NEAR chain and provides a highly transparent and scalable liquidity solution.

#### NFT collaterals 
Borrowers deposit their collateral assets into the FTY vaults on the protocol. A collateral asset is any NFT collection that FTY holders have approved into the protocol. NFT owners can collateralize their NFTs in exchange for whitelisted tokens. Collateralized assets must satisfy specific guideline requirements such as but not limited to having large trade volume, verified by leading marketplaces. Management of these risk parameters will also  be conducted by the DAO. 

Whitelisted NFTs valuation comes from Oracle integrated with AI algorithms. The cornerstone of a lending protocol is accurate valuation of the NFTs. We leverage AI to dynamically decide the value of the NFT based on the floor price, time-weighted sales and other parameters. Our team of seasoned data scientists working in multidisciplinary environments harness the power of data and technology to provide  accurate AI driven NFT valuation. All the decisions of the protocol are made through an highly effective on-chain governance operating model. Our protocol involves DAO in the entire life cycle  of the asset selection process. The initial makeup consists of a temporary DAO formed by a community chosen by the FTY team. 

#### FTY smart contracts
All whitelisted NFTs can be leveraged to generate liquidity through our smart contracts. Users can access the protocol and create vaults through interfaces built by our developers and community. Vaults are non-custodial, giving users a high level of autonomy and self-sovereignty. Users need not rely on third parties to manage their assets or perform transactions.  Users have complete control over their locked collaterals until the health score of collateral doesn?t fall below the required threshold. 

#### FTY participants
FTY  incentivizes all the participants in the system.

##### Borrowers
FTY allows users to access core lending services and borrow money in a simple and optimal way. Users can borrow money from public pools by depositing whitelisted NFTs as collateral. They are required to pay interest on their loans. Interest rates depend on the utilization value of the NFT collection. This means, More a NFT collection is collateralized, higher the interest rate. Once liquidity is provided, borrowers are obligated to pay their loans along with the incurred interest, in order to withdraw their collateral. Each collateral deposited is allocated a separate vault. When users deposit multiple NFTs, they have multiple vaults with different NFT collections as collateral.

##### Lenders
Users can lend money in the form of whitelisted tokens. Lenders are incentivized APY for providing liquidity. 

##### Liquidators
Market participants who secure the protocol, by buying liquidated NFTs at discounted prices.

#### Health score
The health score is the numeric representation of the safety of your deposited assets against the borrowed assets and its underlying value. FTY Protocol ensures that there is enough collateral to cover all outstanding debts, by liquidating risky vault based on their health score.

Health score is a cumulative of total viability score, number of collateralized assets held by the account, LTV ratio, outstanding loans held by the account and liquidation factors. To avoid liquidation scenarios, one must maintain a health score above 1 after the loan has been originated. If the user's health score reaches below 1, the positions are at the risk of liquidation. Each debt position has its own health score and  LTV which is determined by FTY DAO.

##### Categorization of health scores
Our health score is categorized into 3 zones: Good, Medium and Bad. Good health factor represents a safer place, far from liquidation. Medium health is when you start approaching liquidation, Bad is when your assets are in the position of being liquidated. Users can decrease or increase their health score by depositing more whitelisted NFTs or by repaying part of their loan. 

#### AI driven NFT valuation
AI acts as a collection tracking product. In a volatile market, offering NFT-collateralized loans is highly risky. This is because there is no reliable way to dynamically evaluate the worth of NFTs making it  highly impossible to operate an efficient lending protocol. 

Due to this reason, our AI developers focused on utilizing machine learning, etc. to predict the price of the assets at a particular time. The AI engine predicts the value using various parameters like time weighted sales information, listing price, trends of the market, previous transfers, NFTs growth and popularity, etc. This score helps in estimating the borrow limit of the assets. 

This approach enables scalability while handling exhaustive collections under one umbrella, becoming the cornerstone of our lending protocol.

#### Liquidation
If the health score of the borrowed loan becomes less than 1, the borrower gets liquidated. The liquidator can liquidate the borrower to retrieve part of the loan provided to the collateralized NFT. The liquidator collects the collateralized NFT and the penalty fees from the borrower. The FTY DAO sets the penalty fees for each NFT collection.

To incentivize liquidation of risky vault, liquidators receive a discount on the NFTs they withdraw from the borrower?s. Anyone can borrow or lend using the protocol, but only FTY holders can act as liquidators. The discount rate is determined by the tier of FTY holders (Diamond, Platinum, Gold, Silver). Diamond members get the highest discount, followed by Platinum, Gold and Silver members.

#### Volatility protection system 
Volatility protection systems enable the protocol to take the borrowers outstanding obligation, in exceptional cases. This ensures profitability for liquidators. The protocol contains the proceeds from lending and borrowing (including liquidation penalties) and stability fees accrued from the vaults. 

When the collateral?s value drops significantly in a short period, it is difficult for liquidators to earn profit from it. In these cases, the protocol assumes a portion of the outstanding debt to reduce the amount the liquidator has to repay. This makes the system resilient to market conditions. In other words, liquidations are independent of market behaviors.

#### Oracle 
Oracle integrated with our AI engine provides real-time information about market behavior of the collateral NFTs. They act as an on-chain APIs to safely query asset related information into our protocol. Decentralized oracles operate in the same way the blockchain networks operate, making it a reliable, trustless system to transfer off-chain data into our protocol. The FTY DAO chooses the trusted oracles that can interact with the protocol. 

##### What if the oracle gets compromised?
When an oracle gets compromised, the hacker is in control of the outcome, rather than the FTY. We handle this situation by delaying the decentralized oracle information by an hour, so our security team and FTY DAO can vote to freeze the oracle, if it is compromised. The security team actively resolves security issues and prevents damage to the protocol They have the authority to freeze compromised oracles.

### Ecosystem Fit

Our team is interested in exploring mintbase because the platform removes several layers of complexities by abstracting technical implementations. This is especially very important to our use case. The major challenges of DeFi based applications are the significant hacks. By using a platform to create NFTs, write smart contracts, our team can focus more on the security of the application. We will use the Mintbase for primarily three reasons

-  To mint our NFTs
-  To use GraphQL to query the health score, to get previous liquidation threshold information, risk parameters associated with each NFT collection, to get results from it to understand the behavior of risky NFT collections and vaults. 

## Team :busts_in_silhouette:

### Team members

- Founders : Rein, Acie, Ross
- Names of team members

### Contact

- **Contact Name:** Full name of the contact person in your team
- **Contact Email:** Contact email (e.g. john@duo.com)
- **Website:**

### Legal Structure

- **Registered Address:** Address of your registered legal entity, if available. Please keep it in a single line. (e.g. High Street 1, London LK1 234, UK)
- **Registered Legal Entity:** Name of your registered legal entity, if available. (e.g. Duo Ltd.)

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Web3 Foundation previously, please list the name of the project and legal entity here.

### Team Code Repos

- https://github.com/<your_organisation>
- https://github.com/<your_organisation>/<project_1>
- https://github.com/<your_organisation>/<project_2>

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/<team_member_1>
- https://github.com/<team_member_2>

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/<person_1>
- https://www.linkedin.com/<person_2>

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),
- academic publications relevant to the problem,
- links to your research diary, blog posts, articles, forum discussions or open GitHub issues,
- references to conversations you might have had related to this project with anyone from the Mintbase Foundation,
- previous interface iterations, such as mock-ups and wireframes.

## Development Roadmap :nut_and_bolt:

For each milestone,

- provide a test suite, comprising unit and integration tests, along with a guide on how to set up and run them.

- **Deliverables 0a-0d are mandatory for all milestones**, and deliverable 0e at least for the last one. If you do not intend to deliver one of these, please state a reason in its specification (e.g. Milestone X is research oriented and as such there is no code to test).

### Overview

- **Total Estimated Duration:** Duration of the whole project (e.g. 2 months)
- **Full-Time Equivalent (FTE):**  Average number of full-time employees working on the project throughout its duration (see [Wikipedia](https://en.wikipedia.org/wiki/Full-time_equivalent), e.g. 2 FTE)
- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in Bitcoin, the amount will be calculated according to the exchange rate at the time of payment.

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
| 0e. | Article | We will publish a few medium article that explains how FTY is a transparent, scalable and permissionless system that connects lenders, borrowers and liquidators in a dynamic borrowing model.
| 1. | Mintbase module: Public lending protocol encompasses the smart contract for lending and borrowing analytics  |  
| 2. | Mintbase module: UI integration | We will create a module that will integrate with lending and borrowing protocol.
| 3. | AI module: AI system design and development  | We will create a AI module that will acts as a collection tracking product.  We focus on utilizing machine learning, etc. to predict the price of the assets at a particular time. The AI engine predicts the value using various parameters like time weighted sales information, listing price, trends of the market, previous transfers, NFTs growth and popularity, etc. This score helps in estimating the borrow limit of the assets.  |  
| 4. | Oracle module: Chainlink oracle integration | We will create a Oracle module that will be integrated with our AI engine and provides real-time information about market behavior of the collateral NFTs.|  
| 5. | NEAR chain integration | Modules 1,2,3,4 of our custom chain will interact in such a way Borrowers deposit their collateral NFTs into the FTY vaults on the protocol. Collateralized assets must satisfy specific guideline requirements set by FTY DAO. Whitelisted NFTs valuation comes from Oracle integrated with AI algorithms. Health scores of whitelisted NFT is montiored regularly by the protocol. If it decrease below a certain threshold, it is liquidated. Lenders are incentivized APY for providing liquidity
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
| 0e. | Article | We will publish a few medium article that explains how FTY tokenomics, coin mint for a dynamic borrowing model.
| 1. | Tokenomics module: tokenmoics design | A balanced economic model of the FTY token that takes into account the interests of all participants(investors or lenders, borrowers, liquidators)  |  
| 2. | Staking module: FTY staking program | Staking module leverage incentives and rewards to investors. The FTY holders get royalities, incentives, etc|
| 3. | Discord module: Discord member verification bot for private channels | Our NFT holders get access to private discord channels where we post updates, weekly AMA calls, and access to real time sales and listing bots |  
| 4. | Security module: Contract and Mint audit | We test the code against vulnerabilties including denial of service attacks, gas limit issues, reentrancy attacks, fake liquidation attacks, oracle compromise etc. We also provide bug bounties to incentive community to identify threats |  
| 5. | Community building| Build a strong community that brings a long lasting value of our FTY NFTs|  
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

**How did you hear about the Grants Program?** Web3 Foundation Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.
