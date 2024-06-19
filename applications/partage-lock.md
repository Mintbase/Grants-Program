# Mintbase Grant Proposal

- **Project Name:** Partage Wallet
- **Team Name:** Partage
- **Payment Address:** partage-lock.near
- **Level:** 2

  :page_facing_up:

### Overview

- **Tag line:**
Keyless Solutions For Seamless Sharing

- **Brief Description:**
Partage is a peer-to-peer sharing network built on the NEAR Protocol, where users can temporarily access underused assets like cars, rooms, bikes, and more by minting unique access token NFTs.

- **How does Partage integrate into the Mintbase / NEAR ecosystem?**
Partage leverages Mintbase's wallet and NFT minting capabilities to create access tokens while utilizing NEAR as a decentralized backend database and integrating NEAR BOS components for a seamless user experience. It employs NEAR's permissionless social database to manage item listings and leverages NEAR BOS widgets to create a dynamic and engaging UI. This integration ensures a secure, scalable, and blockchain-native platform that aligns closely with Mintbase and NEAR’s decentralized ethos. 

- **Why is our team interested in creating this project?**
Our team is passionate about promoting sustainable consumption and empowering communities through the innovative use of blockchain technology. We aim to address the inefficiency of underused assets in urban environments by creating a secure and decentralized platform for temporary access. By building on Mintbase and NEAR, we ensure a cutting-edge, trustless solution that can grow and adapt to the evolving needs of the sharing economy, all while providing robust security and transparency through blockchain technology.

### Project Details

- **Description of UI components**
The app will feature at least three pages: the marketplace listing available items, individual item pages with booking capabilities, and a user dashboard for managing shared items and access tokens. A demo version is deployed at https://hellopartage.xyz. 
In terms of components, it will feature at least three components: a booking calendar element to select dates for using the asset, a minting interface to allow users to mint unique access token NFTs upon booking, and a camera component for taking and uploading photos of items. 
- **Data models / API specifications of the core functionality**
Owners will list their items on a permissionless social database, where data is stored under their account IDs. Access tokens will reference this data and be minted using Mintbase's NFT function.

- **An overview of the technology stack to be used**
NEAR blockchain as the backend database, Mintbase wallet for account creation and NFT minting, NEAR BOS web components for the frontend dashboard, and wasm-git proxy for offline data storage. Additionally, we will use Tauri to make a template of a native app integrating Mintbase wallet if the grant provider confirms an interest in it.

- **Documentation of core components, protocols, architecture, etc. to be deployed**
Detailed documentation will be provided on the project's GitHub repository, including code samples, tutorials, and videos to help users understand the project's goals and use cases.

  - A slide deck for Partage’s general presentation is available here: https://docs.google.com/presentation/d/16IQYfZBbmjWOXWKl-e9Y7RuLObTEaUsdLnU5AWPVh8Y/edit?usp=sharing

  - It features the following architecture:![Partage architecture](https://github.com/jcarbonnell/jcarbonnell/blob/main/partage-architecture.jpg)
 
- **PoC and research on the topic:**
Our team has conducted extensive research on blockchain-based sharing economy models and has a working prototype of the Partage marketplace. 

- **What Partage Wallet will and will not provide:**
Partage Wallet is designed to offer robust access control for physical assets, leveraging blockchain technology to create a decentralized, peer-to-peer sharing network. It aims to be a leader in onboarding new adopters to the blockchain space by showcasing practical use cases and business models rooted in real-world applications.

Partage Wallet does not support the management of digital gated content at this time. However, the team has ambitious plans for future expansions in this area, which align with Partage's broader mission of promoting sustainable consumption and empowering communities through blockchain technology.

Partage has explored and conducted lean experiments on innovative concepts, such as distributing revenues from a YouTube video among influencers or offering passive income to contributors of a successful artwork or video game. Despite the challenges in finding a straightforward market entry strategy, Partage remains committed to these ideas and will continue exploring in that direction.


### Ecosystem Fit
- **Where and how does your project fit into the ecosystem?**
Partage fills a gap in the NEAR ecosystem by providing a decentralized, blockchain-powered solution for peer-to-peer sharing of real-world assets, aligning with Mintbase's "Unlockables / Gated content" build idea proposition, as detailed in the Mintbase documentation https://docs.mintbase.xyz/dev/build-ideas. 

- **Who is your target audience?**
Our target audience includes digital nomads who often travel to a new city where to settle a bit. And broadly all individuals interested in sustainable consumption, blockchain enthusiasts, and those looking for a convenient and secure way to get paid for sharing their underused assets.

- **What need(s) does your project meet?**
Partage addresses the need for a trustless, decentralized platform that enables the sharing of real-world assets while promoting sustainable consumption and empowering communities.

- **Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?**


Nothing directly similar in the NEAR ecosystem, but can be found in others:
- [Myloby](https://myloby.com/): Integrates Master Lock devices with the Thesos blockchain, offering a similar concept to Partage.
- Academic Research: There are various theses and papers on the topic of blockchain-controlled smart locks, indicating an interest in the scientific field.
- DIY/FabLab/Makerspaces: Several tutorials are available on building a smart lock on an Arduino board and connecting it to a blockchain, catering to a DIY audience.
- [Slock It](https://www.crunchbase.com/organization/slock-it): Attempted to deploy a similar solution in 2016 but was hacked and lost their funds. They received a significant investment of 12M ETH at the time.

Differences and Advantages:
- Global Distribution: The Partage team is globally distributed, with members across Europe, America, and Africa. This diversity enhances adaptability and innovation in the competitive landscape.
- Diverse Perspectives: Having team members from different backgrounds and experiences in tech adoption brings diverse perspectives and expertise to shape the platform's initiative.
- Easy and Quick Deployment: The global distribution of the team allows for easy and quick deployment of Partage Locks to businesses in each team member's area.


## Team :busts_in_silhouette:

### Team members
Julien Carbonnell - Team Lead

### Contact
- **Contact Name:** Julien Carbonnell
- **Contact Email:** julien.carbonnell@gmail.com
- **Website:** https://hellopartage.xyz

### Legal Structure
- **Registered Address:** CivicTech OÜ, Ahtri 12, 10151 Tallinn, Estonia
- **Registered Legal Entity:** CivicTech OÜ

### Team's experience
Julien Carbonnell (CEO) is a French urban developer, living as a digital nomad. He spent a decade working on Smart City and CivicTech projects before finding a purpose for adopting blockchain in his projects. He is an experienced entrepreneur with a main background in project management (12 years), and a programmer (machine learning/blockchain).

### Code Repos
- https://github.com/partagexyz

The GitHub accounts of all dev team members. 
https://github.com/juliencarbonnell

### Team LinkedIn Profiles
- https://www.linkedin.com/in/juliencarbonnell/ 

## Development Status :open_book:
 ![Partage roadmap](https://github.com/jcarbonnell/jcarbonnell/blob/main/partage-roadmap.jpg) 

- **links to your research diary, blog posts, articles, forum discussions, or open GitHub issues:**
All our research articles are available on our medium publication: https://medium.com/partage-btc 

- **references to conversations you might have had related to this project with anyone from the Mintbase Foundation:** 
I first met with Luis, on November, 14th 2023 to discuss how Partage could use Mintbase templates for minting access tokens. Luis liked the idea and incentivized us to submit Partage to Mintbase’s grant program. A first grant application was proposed on December, 18th 2023, and led to an interview with Paul and Luis on February 1st, 2024. A validation of interest by Mintbase was told by Paul in a phone call on March, 13th 2024, and a written message by Luis on April 24th.

- **Previous interface iterations, such as mock-ups and wireframes.**
See above.

## Development Roadmap :nut_and_bolt:

![Partage roadmap (General presentation)](https://pbs.twimg.com/media/GBompXtWYAATgg-?format=jpg&name=medium)

### Grant Overview:
- **Total Estimated Duration:** 
3 months

- **Full-Time Equivalent (FTE):**  6 FTE

- **Total Costs:** 
 50k

- **Milestones plan:**

The following budget breakdown ensures that each phase of the project has sufficient resources to achieve its objectives while providing a clear roadmap for development, testing, and deployment.

Milestone 1: $10,000 (Development: $6,000; Marketing: $1,000; Business Development: $3,000)
Milestone 2: $20,000 (Development: $12,000; Marketing: $4,000; Business Development: $4,000)
Milestone 3: $20,000 (Development: $12,000; Marketing: $6,000; Business Development: $2,000)

### Milestone 1 — Integration of Mintbase Modules and NEAR Components


- **Estimated duration:** 1 month
- **FTE:**  2
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | Marketplace User’s Guide: Interactive Video Guide (Duration: 5-10 minutes): Overview of the app, focusing on item listing, booking, and NFT minting. |
|  |  | Step-by-Step Written Guide (10-15 pages): Detailed instructions on using the marketplace, including screenshots, tips on asset management, and booking process. |
|  |  | Mintbase Wallet Guide (3-5 min video, 5-8 pages): Instructions on linking and using the Mintbase wallet for payments and access token minting.|
| 0c. | Testing Guide | Item Listing & Booking Test: Ensure smooth item listing, booking, and token minting processes.|
|  |  | Mintbase Wallet Integration: Test wallet linkage, transaction initiation, and NFT minting.|
|  |  | Live Demo Option: Request a live demo call to observe the full process from item listing to booking. |
| 0d. | Docker | Not applicable to current development. |
| 0e. | Article | Written article explaining the integration process and user benefits. |
| 0f. | Budget breakdown | 60% development, 10% marketing, 30% business development. |
|  |  | - Smart Contract Development ($2,000): Implement smart contracts for NFT minting. |
|  |  | - Application Integration ($4,000): Integrate Mintbase wallet and NEAR blockchain functionalities. |
|  |  | - Initial Marketing Campaign ($1,000): Promote the app through digital channels. |
|  |  | - Partnership Development ($3,000): Engage in activities to form partnerships and agreements.|
| 1. | Mintbase module: Access Token NFTs | Develop and test the NFT minting process for access tokens. |  
| 2. | Mintbase module: Mintbase Wallet | Integrate the Mintbase wallet for transaction and token management. |  
| 3. | NEAR blockchain integration | Implement NEAR blockchain components for transaction handling and data storage. |

**What we complete**: In Milestone 1, we integrate Mintbase modules and NEAR blockchain components into the Partage platform, including the NFT minting process and Mintbase wallet integration. This ensures secure transactions and efficient management of access tokens.


### Milestone 2 — Development of the Owner Dashboard and Enhanced Functionality

- **Estimated duration:** 1 month
- **FTE:**  4
- **Costs:** 20,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | Owner’s Comprehensive Guide: |
|  |  | In-Depth Video Tutorial (10-15 minutes): - In-Depth Video Tutorial (10-15 min): Detailed setup and management of item listings and bookings.|
|  |  | Written manual (10-15 pages): Guide with annotated images, troubleshooting tips, and contact info. |
| 0c. | Testing Guide | - Dashboard Functionality Test: Ensure real-time status updates and payment handling. |
|  |  | - Integration Test: Confirm integration with NEAR and Mintbase modules. |
| 0d. | Docker | Not applicable to current development. |
| 0e. | Article | Written article covering new dashboard features and enhanced functionalities. |
| 0f. | Budget breakdown | 60% development, 20% marketing, 20% business development. |
|  |  | - Dashboard Development ($8,000): Create and implement the owner dashboard. |
|  |  | - Advanced Features ($4,000): Integrate payment reception and monitoring functionalities. |
|  |  | - Marketing Campaigns ($4,000): Execute marketing strategies to attract new users. |
|  |  | - Customer Support Setup ($4,000): Establish support services for users. |
| 1. | Mintbase module: Payment Processing | Develop a smart contract for automatic fee distribution and payments. |  
| 2. | NEAR chain integration | Implement and deploy smart contracts on NEAR for payment management. |  

**What we complete:** In Milestone 2, we develop a comprehensive owner dashboard, integrate payment reception features, and implement advanced functionalities. The dashboard enables real-time asset management and streamlined financial transactions.

### Milestone 3 — Finalizing the Application and Scaling Up Customer Onboarding

- **Estimated duration:** 1 month
- **FTE:**  4
- **Costs:** 20,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | Finalize documentation from Milestones 1 & 2. |
| 0c. | Testing Guide | Comprehensive testing protocol from previous milestones. |
| 0d. | Docker | Not applicable to current development. |
| 0e. | Article | Complete and improve documentation and articles from previous milestones. |
| 0f. | Budget breakdown | 60% development, 30% marketing, 10% business development. | 
|  |  | - Final App Development ($7,000): Complete the app’s full development, including final testing and optimizations. |
|  |  | - Pilot Testing ($5,000): Conduct pilot testing with initial users to ensure functionality and reliability. |
|  |  | - Onboarding Campaigns ($6,000): Plan and execute onboarding events and marketing campaigns. |
|  |  | - Event Participation ($2,000): Organize and participate in events to promote the app and onboard new users. |
| 1. | Mintbase modules | Improve and finalize module implementations from Milestones 1 & 2. |

**What we complete:** In Milestone 3, we complete the full development of the application, conduct extensive internal and pilot testing, and prepare for large-scale user onboarding through targeted marketing and event participation.



## Future Plans
- **How you intend to use, enhance, promote, and support your project in the short term**
We plan to utilize various marketing strategies to enhance, promote, and support Partage Lock. This includes paid digital advertising on platforms like Meta, Instagram, and Google Ads to reach a broader audience.
We will launch social media campaigns to create awareness, engage potential users, and build a community around Partage Lock. This will involve leveraging visuals, user testimonials, and educational content to showcase the platform's features and benefits.
Collaborations with influencers and/or brand partnerships in the sharing economy, technology, or blockchain space will be pursued to lend credibility to Partage Lock and help reach a wider audience.
We will continue to create high-quality content, including LinkedIn blog posts, articles, and infographics, to highlight the value proposition of Partage Lock.
Community engagement events will be organized to actively engage with the community, address questions, and showcase the platform's functionality. This will strengthen user relationships and generate positive word-of-mouth, which can be leveraged for PR campaigns.

- **The team's long-term plans and intentions about it.**
Our long-term goal is to grow adopters within the NEAR ecosystem and Master Lock customers.
We plan to deploy networks of lockers on public infrastructures through smart city partnerships and universities.
We aim to diversify our solutions to other kinds of shared items and gated content (digital, as described above).
Integration of our locks-in starters for shared cars, scooters, or other electronic items available after payment is on the roadmap.
We intend to attend conferences (web3FC in Barcelona in April, Bitcoin 2024 in Nashville in July, etc.) to present Partage and onboard customers.

## Additional Information :heavy_plus_sign:
- **How We Heard About the Grants Program:**
We learned about the Grants Program at the Encode Club workshop and had a 1-1 call with Luis.

- **Work Already Done:**
This is the third iteration of this platform and we have a functional demo on mainnet. We keep conducting research on how to improve our product and implement new versions every 6 month or so.


- **Previous Grants Applied For**:
None for this project.

- **If any other teams have already contributed (financially) to the project.**
Julien has been self-supporting the project from the beginning through his registered company in Estonia “CivicTech OÜ”.

