# Mintbase Grant Proposal

## Project Name: TENAMINT Fan Engagement Templates: Pick'em, Rewards, and Challenges
**Company Name:** TENAMINT  
**Payment Address:** bstt.near  
**Grant Level:** 2

## Project Overview

### Project Description

TENAMINT Fan Engagement Templates: Pick'em, Rewards, and Challenges. 
The game, rewards, collectibles, and upcoming product additions are intricately crafted to offer value in league partnerships, delivering exclusive content, digital collectibles, fan tokens, and IRL activations. This approach fosters a vibrant ecosystem of sports enthusiasts, resulting in heightened fan experiences, strengthened league-to-fan relationships, and a compelling avenue for web3 fan-base adoption. In addition to the release of these elements, the deliverables also include 3 related Mintbase templates to ensure that builders in the near and mintbase ecosystem can build NFT Challenges, Reward Programs, and use Pick’em templates to gamify aspects of their project.

#### Key Features and Rewards

**Pick'em Game:**
- Introduces dynamic live match prediction within Sports Pick'em.
- Users predict scores, player performances, and key play outcomes.
- Offers immediate rewards for correct predictions (points or tokens).
- Rewards can be redeemed for exclusive NFTs, fan experiences, or merchandise.
- Mintbase Pick’em Template.

**Collectables:**
- NFTs are a core aspect of the Pick'em game and can be used to enhance rewards in the Pick'em game and synergize with other games.
- Collectibles feature challenges that can be completed for further rewards.
- Collectibles are intended to be league partner/team partner specific.
- Collectibles can be redeemed in exchange for rewards creating a loop.
- Mintbase NFT Challenges Template.

**Rewards:**
- Pick'em contest winners receive reward tokens.
- Tokens can be used within the platform or exchanged.
- Redeemable for digital collectibles, IRL experiences, and merchandise.
- Mintbase Rewards Template

**In-Game Engagement with Mintbase:**
- "Mint Your Moment" feature captures fan experiences at games with the NFT Minter.

## Project Details

- **Figma Exports:** [Figma Exports](https://drive.google.com/drive/folders/1-rek75CWt7GZhEPEWXRKxfW2pQ7xgPIo?usp=share_link)  
- **Exploratory/Early Staging Site:** [Basketball Pick'em App](https://basketball-pickem-app.vercel.app/)  
- **Data Models:**  
  - [Data Model 1 (User Flow Preliminary Conceptualization)](https://drive.google.com/file/d/1iCfI1y4dS4LQlhaMdx5paqJbsukCDHXm/view?usp=share_link)  
  - [Data Model 2 (Token Interconnectivity Preliminary Conceptulization)](https://drive.google.com/file/d/1slCt0Iz0pLFRpbtcstNFMM13rdc6Axvz/view?usp=share_link)  
- **Architecture Model:** [Architecture Model](https://drive.google.com/file/d/1pg3gqbdVOpssSn9sB0beQ_FGDmX_8v1I/view?usp=sharing)

## Tech-Stack, Architecture, and Components

### TENAMINT Digital & Pick'em Game

**Front-End:**
- Design Tools: Figma (UI), Adobe Suite (Graph/Vector Design)
- Styling: Tailwind CSS for utility-first styling, Tailwind UI, NextUI, Chakra UI for component libraries, Storybook for UI component development
- Login Authentication: Firebase Auth for authentication, Mintbase Social Authentication
- Email Notification: Firebase Auth for email notification, js for server-side functionality
- Framework: js for React-based web applications with server-side rendering and static site generation and flexibility.
- Hosting: Vercel for optimized Next.js hosting with serverless functions

**Backend:**
- Database Requirements: Primary: Firebase; Alternatively: MySQL or MongoDB
- Backend Hosting: Primary: Google Cloud Platform (GCP) for cloud services
- Decentralized Storage: Arweave and NFTStorage
- Additional Cloud Infrastructure: Digital Ocean
- Scaling Considerations: Utilize serverless architectures and content delivery networks (CDNs), Implement dynamic scaling of backend services, Integrate NEAR blockchain technologies for efficient scaling and ecosystem integration
- CI/CD Implementation: GitHub Actions used for Continuous Integration/Continuous Deployment and to automate software build, test, and deployment processes
- Data Models / API Specifications: RESTful APIs or GraphQL endpoints for core functionality, Sports Data Provider: Genius Sports or Other
- Documentation: Comprehensive documentation for core components, protocols, and architecture, Include integration details with sports data provider or similar for real-time data retrieval

**Blockchain:**
- NEAR network for blockchain functionality
- NEPs: NEP141 (FT) Rewards, NEP171 (NFT) Collectibles, NEP245 (MTS)
- Marketplace: NFT: Mintbase Marketplace Integration, Merchandise E-Commerce: Droplinked
- Blockchain Indexer: GraphQL API for blockchain indexer
- Wallet: NEAR Wallet Selector, Mintbase Wallet, HERE, and others, Tenamint Wallet App powered by Mintbase Wallet, Potential for direct to marketplace/rewards integration

## Ecosystem Fit

Where and How TENAMINT fits into the Ecosystem:

**Mintbase:**
- Mintbase Wallet: User-friendly and streamlined user onboarding, Passkey/Biometric Sign-in, Social Auth Options
- Templates: Mintbase Rewards Template, Mintbase Pick’em Template, Mintbase NFT Challenges Template
- NFT Minter: "Mint Your Moment" implementation designed for IRL events
- Indexer: Utilization of GraphQL Indexer for efficient user data tracking

**NEAR:**
- Leading Sports Gaming and Collectibles on NEAR
- The products intertwine to introduce an innovative approach to sports gaming with free-to-play predictive gaming, fan engagement, league partnerships, and integrated collectibles with utility.
- TENAMINT aims to position itself as a pioneering force in the Web3 ecosystem, specifically targeting the sports entertainment and collectibles sector with an emphasis on creative, interconnected, and fan-first products.

## Target Audience and Needs Met

**Target Audience:**
- Sports Leagues and Teams
- Sports Enthusiasts
- Collectibles Enthusiasts
- Blockchain Enthusiasts
- Gamers

**Audience Needs Met by the Project:**
- Enhanced Fan Engagement
- Immersive Fan Experience
- Engaging Collectibles Experience
- Engaging Gaming Experiences

## Similar Projects in NEAR Ecosystems

- Playible
- SailGP
- Dropt
- NEAR Foundation (Details pending TBA)

## Team

### Primary Contact

Sal Chaudhry  
Email: [office@tenamint.com](mailto:office@tenamint.com)  
LinkedIn: [https://www.linkedin.com/in/salchaudhry](https://www.linkedin.com/in/salchaudhry)

### Legal Structure

Registered Legal Entity: Ball Street Technologies Inc.  
Registered Address: 408-146 Fort York Blvd., Toronto, ON, Canada M5V0E1

### Team Experience

TENAMINT: [Website](www.tenamint.com/), [App](https://app.tenamint.com/)  
TENAMINT leads the way in shaping the future of sports fan experiences, bringing immersive fantasy sports with play-to-earn rewards in addition to utility-driven collectibles, and other fan-first offerings to a global audience. From fantasy sports gaming and digital collectibles to real-world activations, our products seamlessly converge, redefining fan engagement through a harmonious blend of blockchain technology, enticing rewards, and unparalleled fan experiences. This innovative approach extends beyond the digital realm, fostering real-world activations that not only fortify fan loyalty but also spearhead the evolution of interactive fan experiences.

### Accelerators and similar programs

NEAR HZN1, Innovation Boost Zone, Accelerator Centre, Founder Institute, etc.

### Team Links

**TENAMINT:** 
- [GitHub](https://github.com/TENAMINT)
- [LinkedIn](https://www.linkedin.com/company/73837227/admin/feed/posts/)

**Individual Team Members:** 
- Nadir: [LinkedIn](https://www.linkedin.com/in/nadirchaudhry/)
- Sal: [GitHub](https://github.com/Salikc9), [LinkedIn](https://www.linkedin.com/in/salchaudhry/)
- Alex: [GitHub](https://github.com/alexastrum), [LinkedIn](https://www.linkedin.com/in/alexastrum/)
- Shunsuke: [GitHub](https://github.com/ShunsukeKano), [LinkedIn](https://www.linkedin.com/in/shunsuke-kano/)
- Walt: [GitHub](https://github.com/yaozakai), [LinkedIn](https://www.linkedin.com/in/waltyao/)
- Shawn: [GitHub](https://github.com/Benz222)

## Development Status

**Wireframes, Mock-ups, Other:**
- Figma Mockup: [TENAMINT Pick'em](https://www.figma.com/proto/5ivxRYoqK2KHjKHhCht8up/TENAMINT-Pick'em?type=design&node-id=1300-2012&t=N4dCS0wkNofN63Fx-0&scaling=min-zoom&page-id=0%3A1&starting-point-node-id=1300%3A2012&show-proto-sidebar=1)
- Early Staging Site: [Basketball Pick'em App](https://basketball-pickem-app.vercel.app/)
- Data & Architecture Models: [Google Drive Link](https://drive.google.com/drive/folders/1-jmuOf-ubdDAY0ADfsPTALMGNEDnXYgc?usp=sharing)
- Other Products: [TENAMINT Phygital](https://tenamint.com/phygital)

**Research Done:**
- Engagement with League Executives: In-depth conversations and consultations have taken place with league executives to gain valuable insights into their specific needs and identify emerging opportunities within the industry.
- Comprehensive Review and Audit: Rigorous examination and audit of fantasy sports gaming products, pick'em games, and sports collectibles have been undertaken. This comprehensive review ensures a thorough understanding of the current landscape and identifies areas for innovation and improvement.
- Market Research with Collectors: Valuable perspectives on the future of sport collectibles have been gathered through extensive in-person research with physical sport card collectors. This direct interaction has provided nuanced insights into the preferences and expectations of collectors in the evolving market.
- Industry Specialization: The co-founder brings a wealth of expertise to the project, grounded in an educational background specializing in sport business. Emphasizing fan experience and sport technology, the co-founder adopts a strategic and thoroughly researched approach to advancements in sports innovation and fan engagement.
- Other Relevant Experiences: A lifetime dedicated to being a team of passionate enthusiasts, including roles as die-hard fans, fantasy sports players, league commissioners, and collectors of both digital and physical sports collectibles, has provided a unique and holistic perspective. This firsthand experience enriches the project with a deep understanding of user expectations and industry dynamics.
- Mintbase Products, Documentation, Templates, and opportunities for open-source templates.

**Relevant Conversations with Mintbase:**
- Paul via Zoom Call - September 2023
- Nate at NEARCON 2023 - November 2023
- Paul & Micro February/March 2024

## Milestones and Product Roadmap

**Overview — Milestone Summary**

Estimated Duration: 4.5 months  
FTE: 3  
Costs: $50,000 USD  
Note: order of Milestone Delivery may change 

### Milestone 1 — Sports Pick'em Game

  **Estimated Duration:** 1.5 months  
  **FTE:** 3  
  **Costs:** $15,000 USD  
  **Mintbase Pick’em Template:**  
  **Deliverables:** 
  - License | MIT
  - Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can create a simple ‘Pick’Em’ that can be used for a game, polling, or other creative uses.
  - Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone.
  - Article & Video | We will publish an article and video explaining the full rewards concept and walkthrough.
  - Mintbase Template | We will create a template that will enable users to craft their unique Pick’Em Scenarios.
  - TENAMINT Implementation | Pick'em Game
    - Sport’s Pick’em Game Release
    - Documentation: Comprehensive documentation covering all aspects
    - Design: Engaging and intuitive design elements
    - Mechanics: Fundamental game mechanics for an immersive experience
    - Smart Contracts: Implementation and deployment details
    - Integrations: Integration with other platforms and services for enhanced functionality



### Milestone 2 — TENAMINT Rewards and Mintbase Rewards Template

  **Duration:** 1.5 months  
  **FTE:** 3  
  **Costs:** $15,000 USD  
  **Mintbase Pick’em Template:**  
  **Deliverables:** 
  - License | MIT
  - Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can create a personalized Reward Token.
  - Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone.
  - Article & Video | We will publish an article and video explaining the full rewards concept and walkthrough.
  - Mintbase Template | We will create a template that empowers users to create their personalized Reward Token.
  - TENAMINT Implementation | Rewards
    - Deliverables: Rewards Implementation with Rewards Marketplace Features
    - Documentation: Complete coverage of all aspects
    - Design: Aesthetic and functional design elements
    - Mechanics: Core operational mechanics
    - Smart Contracts: Deployment and functionality
    - Integrations: Seamless integration with existing systems and services


### Milestone 3 — TENAMINT NFT Challenge and Mintbase NFT Challenge Template

  **Estimated Duration:** 1.5 Months  
  **FTE:** 3  
  **Costs:** $20,000 USD  
  **Mintbase NFT Challenge Template:**  
  **Deliverables:** 
  - License | MIT
  - Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can create a “NFT Challenge” through the creation of a ‘check-list’ of NFTs that are to be collected.
  - Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone.
  - Article & Video | We will publish an article and video explaining the full rewards concept and walkthrough.
  - Mintbase Template | We will create a versatile template empowering users to craft their unique NFT Challenges.
  - TENAMINT Implementation | NFT & Challenges
    - NFTs & Challenges Release (V0 or V1 - Dependent on Business Development Variables)
    - Primary Marketplace: Open
    - Secondary Marketplace: Open
    - NFT Challenge Integration
    - NFT Minter ("Mint Your Moment") & In-Real-Life (IRL) Activation
    - Beta Testing, Feedback, and Improvements


## Mintabase Templates Technical Approach and Preliminary Notes

### Milestone 1 — Mintbase Pick’em Template:

The objective of this internal high-level overview is to outline the implementation approach for building a Pick'Em template on NEAR via Mintbase. The goal is to create an engaging and versatile smart contract that allows users to participate in Pick'Em scenarios, associating potential outcomes with corresponding airdrops. The contract should seamlessly interact with NEAR infrastructure, validate user picks, and integrate with a rewards payer for token distribution. 


**Preliminary Notes on Technical Approach: Pick’em Template Implementation**

**Contract Structure:**
1. Modularity and Extensibility:
    - Components are modular, allowing for easy addition or modification of functionalities.
    - Contracts can be extended to accommodate new features or integrate with other contracts seamlessly.

2. State Variables:
    - Well-defined state variables encapsulate essential information such as user balances, task completion status, and reward distribution details.
    - Variables are organized in a structured manner for clarity and efficient access.

**Contract Methods:**
1. Singular Validation:
    - A method is implemented to facilitate singular validation through Zk.proof or assigned transactions.
    - Validators or oracles utilize this method to confirm the completion of specific tasks by users before triggering reward distribution.

2. Security Measures:
    - Methods are designed with security in mind, incorporating measures to prevent unauthorized access, tampering, or malicious activities.
    - Permission settings and access controls are implemented to restrict certain actions to authorized entities.

3. Transparency:
    - The contract methods provide clear and transparent processes for task validation and reward distribution.
    - Event logging ensures a transparent record of completed tasks, helping in auditing and accountability.

4. Integration with Minter and Token Drop:
    - Seamless integration with Mintbase is achieved through well-defined methods for token creation and distribution.
    - The contract can efficiently mint new reward tokens based on predefined criteria and distribute them to users who successfully complete incentivized tasks.

**Interaction with Rewards Payer:**
The contract interacts seamlessly with the Rewards Payer, which could be an external entity responsible for validating tasks and distributing rewards. Validators or oracles can utilize the contract's methods to verify task completion, ensuring a secure and tamper-proof process for rewarding users.

**Testing:**
Documentation and instruction provided for a comprehensive testing environment. Documentation and Tutorial will be created to guide users through use of the template.

**Versatility:**
The template's versatility is a key highlight, enabling users to configure and enhance their rewards by assigning values to incentivized actions. Whether it's related to NFT collections, check-ins, game participation, or referrals, the Rewards Token template empowers users to tailor their incentivization strategies according to their specific use cases.


### Milestone 2 — Mintbase Rewards Template: 
    
The objective of this Rewards Token implementation is to provide an easy to use and  versatile NEP-141 rewards smart contract template via Mintbase. This template allows users to create personalized Reward Tokens that can be utilized for various purposes, such as measuring activity, loyalty, performance, or other values. The focus is on encouraging user involvement in Pick'em games, NFT challenges, or any incentivized actions, enhancing engagement and participation.

**Preliminary Notes on Technical Approach: Rewards Template Implementation**

**Contract Structure:**
1. NEP-141 Standard Compliance:
    - Design the smart contract to comply with the NEP-141 standard for fungible tokens on the NEAR protocol.
    - Implement core functionalities such as `total_supply()`, `balance_of(account)`, `transfer(sender, receiver, amount)`, and other standard methods.

2. Zk.proof Integration:
    - Leverage Zk.proof capabilities for zero-knowledge proof implementations.
    - Incorporate Zk.proof into the contract logic to ensure secure and private validations of completed tasks

3. Mintbase Integration:
    - Establish a seamless integration with Mintbase, utilizing the Mintbase API for creating and managing Reward Tokens.
    - Configure the contract to support Mintbase's marketplace functionalities, enabling users to mint, trade, and interact with their Reward Tokens directly through Mintbase.

**Contract Methods:**
1. NEP-141 Standard Methods:
    - Implement the standard NEP-141 methods for the Reward Token contract, allowing users to create, transfer, and manage their tokens with ease.
    - Ensure that the contract adheres to the fungible token specifications outlined in the NEP-141 standard.

2. Zk.proof-enabled Validation:
    - Develop methods for task completion validation, incorporating Zk.proof to secure the validation process.
    - Utilize zero-knowledge proofs to verify that a task has been completed without exposing the details of the task on the blockchain.

3. Singular Validation through Oracles/Validators:
    - Establish a mechanism for singular validation using oracles or validators on the NEAR platform.
    - Design the contract to trigger validation requests to external oracles or validators, ensuring an additional layer of reliability in confirming completed tasks.

**Testing:** 
Documentation and instruction provided for a comprehensive testing environment Documentation and Tutorial will be created to guide users through the use of the template.

**Versatility:**
The template's design intends to emphasize  versatility, allowing users to configure rewards for incentivizing user actions. 

### Milestone 3 — Mintbase NFT Challenge Template: 
  
The goal is to develop a versatile NFT Challenge template on the NEAR blockchain through Mintbase, facilitating the creation of unique NFT challenges with customizable requirements and rewarding outcomes. 

**Preliminary Notes on Technical Approach: NFT Challenge Template Implementation**

For collectors, NFT challenges present collection tasks with rewarding outcomes. Earn exclusive NFTs specifically tailored for those who successfully complete the challenge. 

**Objective:** 
The goal is to develop a versatile NFT Challenge template on the NEAR blockchain through Mintbase, facilitating the creation of unique challenges with customizable requirements and rewarding outcomes.

**Technical Approach for NFT Challenge Contract**

**Contract Structure:**
1. Initialization:
    - The contract is designed with three states: Configuration, Claiming, and Finished.
    - Initialization includes setting challenge parameters and defining required NFTs with associated contract addresses.

2. Configuration State:
    - Parameters like start time, duration, and claiming window status are tracked internally.
    - Utilizes NEAR Smart Contract functionality to initialize the challenge.

3. Claiming State:
    - Activation via the unlock mechanism, transitioning from Configuration.
    - Asynchronous claiming window for collectors to submit NFTs.
    - NEAR runtime environment used to handle asynchronous actions.

4. Cross-Check Mechanism:
    - Implemented in the claim_rewards method during Claiming State.
    - Ensures uniqueness of provided NFTs, preventing reuse.

5. Finished State:
    - Triggered post-claiming window closure.
    - Cleanup operations and challenge finalization are executed.
    - NEAR contract state is adjusted, and further interactions are prevented by locking down the contract.

**Contract Methods:**
1. configure_nfts:
    - NEAR Smart Contract method allowing the setting of required NFTs during Configuration State.
    - Parameters updated in the contract state.

2. unlock_challenge:
    - NEAR method transitioning the contract from Configuration to Claiming State.
    - Activates the asynchronous claiming window.

3. claim_rewards:
    - NEAR method for collectors to claim rewards during Claiming State.
    - Employs cross-checks to ensure NFT uniqueness before processing rewards.

4. finish_challenge:
    - NEAR method transitioning the contract from Claiming to Finished State.
    - Executed cleanup operations and locks down the contract to prevent further interactions.

**Testing:**
Documentation and instruction provided for a comprehensive testing environment Documentation and Tutorial will be created to guide users through the use of the template.

**Versatility:**
- The template design prioritizes configurability, allowing users to tailor challenges with diverse collectibles and set specific completion circumstances and conditions.


## TENAMINT Future Plans

**Organizational Roadmap and Future Plans**

- Short-Term: In the short-term, our primary objectives include the integration of merchandise E-Commerce for the Rewards Marketplace, along with the implementation of social share features and the introduction of a bracket game to enhance user engagement. Additionally, we aim to execute our Go-To-Market (GTM) plan, focusing on strategic partnerships and the execution of seamless IRL activations in arena venues.


- Mid-Term: Moving into the mid-term, we plan to develop and expand our product line to include more strategic and in-depth game formats such as a Daily Fantasy Sports (DFS) style game and eventually a season-long format. Our emphasis during this phase will also involve establishing dedicated fan hubs for leagues, teams, and athletes. We also plan to integrate our product on the BOS and continue with the execution of our GTM plan in sync with our partners.


- Long-Term: Looking towards the long-term, our strategic vision includes the launch of fan tokens, with token gated channels, marketplaces, experiences and hubs. We plan to execute the pre-release and launch of the upcoming TENAMINT token, aiming to position it at the core of a fan-centric, sports-oriented, and gaming ecosystem formed by these products. If required, we will also consider multi-chain integrations and support in the long-term. Additional long-term goals involve increased collectible product lines through added partnerships.


- Note: In the future we aim to eventually continue development towards advancing and improving Phygital/RWA releases and technologies by integrating and leveraging our newer product lines. Pre-developed and qualified user bases may eventually provide lead-generation for the Phygital/RWA products. Partnerships that align with the TENAMINT Phygital vision will also be considered as we recognize the trends and long-term potential of RWA-backed collectibles, fractional ownership, and other core concepts we had previously prioritized. It's essential to highlight that as mentioned in previous discussions with the Mintbase team, the Multi-Token Standard (MTS) is intricately connected to the TENAMINT Phygital product line and we'd like to explore Mintbase integration for this initiative in the future once our new product line priorities are successfully released and established.

## Additional Information

**How did you hear about the Grants Program?**

Mintbase team members:
  - Nate G.
  - Paul K.

**Work you have already done:**

Tenamint Phygital
- **Fractional & RWA-backed Collectible Cards**
  - ~$40,000 RWA Assets Listed
- **Established**
  - Partnerships
  - Accelerator Programs
    - HZN, Accelerator Centre, Founder Institute, etc.
  - $40,000+ Listed Assets
  - MTS NEP Contributions
- **Pre-Seed Fundraise in Early 2022**
- **Grants**
  - NF Grant (2022 Builder & Quick-Starter Grants)
