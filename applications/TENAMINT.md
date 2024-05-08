# Mintbase Grant Proposal

## Project Name: TENAMINT NFT Challenge Template
**Company Name:** TENAMINT  
**Payment Address:** bstt.near  
**Grant Level:** 2

## Project Overview

TENAMINT is dedicated to crafting innovative products and resources tailored to enhance the engagement of collectors and sports enthusiasts. Our mission is to elevate fan experiences, fortify connections between leagues and fans, and pioneer the integration of web3 technologies into fan communities. 

### Project Description

This grant proposal outlines our commitment to developing Mintbase templates, specifically designed to empower creatives and developers within the NEAR and Mintbase ecosystem. These templates will facilitate the creation of NFT Challenges, adding gamification elements to various projects. Drawing inspiration from sports-related challenges, these templates will incentivize collectors with rewards for completing collections or themed sets. Moreover, they will introduce opportunities to merge collectibles, unlocking exclusive unified or super collectibles reserved for those who fulfill specific requirements.

#### Key Features and Benefits

Key Features and Benefits
- Customizable Mintbase Template: Tailored NFT Challenges to suit project needs and objectives
- Gamification Elements: Infuse projects with gamified experiences, enhancing user engagement and interaction.
- Reward System: Incentivize collectors with rewards for completing collections or themed sets, fostering a sense of accomplishment and loyalty.
- Merge Mechanics: Introduce opportunities for collectors to combine collectibles, unlocking exclusive unified or super collectibles as a testament to their dedication.
- Exclusive Benefits: Offer exclusive rewards and privileges to participants who meet specific challenge requirements, cultivating a vibrant and dedicated fan community.

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
- Marketplace: NFT: Mintbase Marketplace Integration
- Blockchain Indexer: GraphQL API for blockchain indexer
- Wallet: Mintbase Wallet, NEAR Wallet Selector, HERE, and others

## Ecosystem Fit

Where and How TENAMINT fits into the Ecosystem:

**Mintbase:**
- Mintbase Wallet: User-friendly and streamlined user onboarding, Passkey/Biometric Sign-in, Social Auth Options
- Templates: Creation of Mintbase NFT Challenge Template
- Indexer: Utilization of GraphQL Indexer for efficient user data tracking
- Sports collectibles
- TCG type collectibles (and Experiences) amplified by challenges
- Future implementations of RWA collectibles (Asset-Backed)
- Future use of NFT Minter: "Mint Your Moment" implementation designed for IRL events
- Mintbase minted game submissions (ex. Bracket Challenge)

**NEAR:**
- TENAMINT aims to be an industry leader for  Sports, Gaming, and Collectibles on NEAR and beyond
- The products intertwine to introduce an innovative approach to sports gaming with free-to-play predictive gaming, fan engagement, league partnerships, and integrating collectibles with utility.
- TENAMINT aims to position itself as a pioneering force in the Web3 ecosystem, specifically targeting the sports entertainment and collectibles sector with an emphasis on creative, interconnected, and fan-first products.
- Developed templates will allow for creatives, builders, and more to create NFT challenges and incentivize collectors by providing more depth and interactive elements relating to their collections on NEAR.


## Target Audience and Needs Met

**Target Audience:**
- Sports Leagues and Teams
- Sports Enthusiasts
- Collectibles Enthusiasts
- Blockchain Enthusiasts
- Gamers
- Builders
- Creatives
- Artists
- IP Partners


**Audience Needs Met by the Project:**
- Engaging Collectibles Experience
- Engaging Gaming Experiences
- Enhanced Fan Engagement
- Immersive Fan Experience

## Team

### Primary Contact

- Sal Chaudhry  
- Email: [office@tenamint.com](mailto:office@tenamint.com)  
- LinkedIn: [https://www.linkedin.com/in/salchaudhry](https://www.linkedin.com/in/salchaudhry)

### Legal Structure

Registered Legal Entity: Ball Street Technologies Inc.  
Registered Address: 408-146 Fort York Blvd., Toronto, ON, Canada M5V0E1

### Team Experience

TENAMINT: [Website](www.tenamint.com/)  
TENAMINT leads the way in shaping the future of sports fan experiences, bringing immersive fantasy sports with play-to-earn rewards in addition to utility-driven collectibles, and other fan-first offerings to a global audience. From fantasy sports gaming and digital collectibles to real-world activations, our products seamlessly converge, redefining fan engagement through a harmonious blend of blockchain technology, enticing rewards, and unparalleled fan experiences. This innovative approach extends beyond the digital realm, fostering real-world activations that not only fortify fan loyalty but also spearhead the evolution of interactive fan experiences.

### Accelerators and similar programs

NEAR HZN1, Innovation Boost Zone, Accelerator Centre, Founder Institute, etc.

### Team Links

**TENAMINT:** 
- [GitHub](https://github.com/TENAMINT)
- [LinkedIn](https://www.linkedin.com/company/73837227/admin/feed/posts/)

**Individual Team Members:** 
- Nadir: [GitHub](https://github.com/0xNadir), [LinkedIn](https://www.linkedin.com/in/nadirchaudhry/)
- Sal: [GitHub](https://github.com/Salikc9), [LinkedIn](https://www.linkedin.com/in/salchaudhry/)
- Alex: [GitHub](https://github.com/alexastrum), [LinkedIn](https://www.linkedin.com/in/alexastrum/)
- Shawn: [GitHub](https://github.com/Benz222)
- Walt: [GitHub](https://github.com/yaozakai), [LinkedIn](https://www.linkedin.com/in/waltyao/)
- Wasim: [GitHub](https://github.com/wasim162010), [LinkedIn](https://www.linkedin.com/in/wasim-bari)


## Development Status

**Wireframes, Mock-ups, Other:**
- [Folder](https://drive.google.com/drive/folders/1-jmuOf-ubdDAY0ADfsPTALMGNEDnXYgc?usp=share_link)
- NFT Challenges: [Notes](https://drive.google.com/file/d/1gESUU2Zu9C9ZaO7wJNrQQ_VNsPa5JwsY/view?usp=sharing)
- Figma Mockup: [TENAMINT Pick'em](https://www.figma.com/proto/5ivxRYoqK2KHjKHhCht8up/TENAMINT-Pick'em?type=design&node-id=1300-2012&t=N4dCS0wkNofN63Fx-0&scaling=min-zoom&page-id=0%3A1&starting-point-node-id=1300%3A2012&show-proto-sidebar=1)
- Early Staging Site: [Basketball Pick'em App](https://basketball-pickem-app.vercel.app/)
- Data & Architecture Models: [Google Drive Link](https://drive.google.com/drive/folders/1-jmuOf-ubdDAY0ADfsPTALMGNEDnXYgc?usp=sharing)
- Other Products: [TENAMINT Phygital](https://tenamint.com/phygital)
- Bracket Challenge: [Bracket Challenge](https://bracket.tenamint.com/)
- Telegram App (alpha): [Telegram App](https://t.me/TENAMINT_bot)

**Research Done:**
- Engagement with League Executives: In-depth conversations and consultations have taken place with league executives to gain valuable insights into their specific needs and identify emerging opportunities within the industry.
- Comprehensive Review and Audit: Rigorous examination and audit of fantasy sports gaming products, pick'em games, and sports collectibles have been undertaken. This comprehensive review ensures a thorough understanding of the current landscape and identifies areas for innovation and improvement.
- Market Research with Collectors: Valuable perspectives on the future of sport collectibles have been gathered through extensive in-person research with physical sport card collectors. This direct interaction has provided nuanced insights into the preferences and expectations of collectors in the evolving market.
- Industry Specialization: The co-founder brings a wealth of expertise to the project, grounded in an educational background specializing in sport business. Emphasizing fan experience and sport technology, the co-founder adopts a strategic and thoroughly researched approach to advancements in sports innovation and fan engagement.
- Other Relevant Experiences: A lifetime dedicated to being a team of passionate enthusiasts, including roles as die-hard fans, fantasy sports players, league commissioners, and collectors of both digital and physical sports collectibles, has provided a unique and holistic perspective. This firsthand experience enriches the project with a deep understanding of user expectations and industry dynamics.
- Mintbase Products, Documentation, Templates, and opportunities for open-source templates.
- Discussions with Mintbase team (Paul & Luis)

## Milestones and Roadmap

**Overview — Milestone Summary**

Estimated Duration: 1.5 months  
FTE: 3  
Costs: $20,000 USD  


### Milestone 1 — TENAMINT NFT Challenge and Mintbase NFT Challenge Template

  **Estimated Duration:** 1.5 Months  
  **FTE:** 3  
  **Costs:** $20,000 USD  
  **Mintbase NFT Challenge Template:**  
  **Deliverables:** 

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can create a “NFT Challenge” through the creation of challenge requirements, circumstances, and rewards |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article + Video | We will publish an article and video explaining the full nft challenge concept and walkthrough. |
| 1. | Mintbase Template: NFT Challenges | We will create a versatile template empowering users to craft their unique NFT Challenges. Template and deliverables will include customization elements empowering creators with the ability to choose all variable elements relating to the challenges. Template will be designed for ease of use.|  
| 2. | TENAMINT Implementation | TENAMINT Challenge Examples (illustrating features, options, and functionality through primary/secondary marketplace. Documentation for Beta Testing, Feedback, and Improvements. |


## Mintbase NFT Challenge Template - Technical Approach and Preliminary Notes

The goal is to develop a versatile NFT Challenge template on the NEAR blockchain through Mintbase, facilitating the creation of unique NFT challenges with customizable requirements and rewarding outcomes.

For collectors, NFT challenges present collection tasks with rewarding outcomes. Earn exclusive NFTs specifically tailored for those who successfully complete the challenge. We envision this a variety of customization options that draw inspiration from sports collectibles and TCG collectibles. In sports collectibles, it's ideal for rewards to be provided in addition to keeping the requirements for the reward. In some TCG examples, the reward should 'replace' the required collectibles as they build towards the 'super' collectible (the Exodia Example). We recognize that additional types of challenges/rewards (with different technical approaches) may be uncovered and considered through the development process.

Objective: The goal is to develop a versatile NFT Challenge template on the NEAR blockchain through Mintbase, facilitating the creation of unique challenges with customizable requirements and rewarding outcomes.

**Potential Technical Approach for NFT Challenges and Template**

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
    - Ensures uniqueness of provided NFTs, preventing reuse (or as configured)

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

Note: Technical approach may be adjusted to best suit deliverables and customization options. Additionally, any marketplace and NFT display considerations will be taken into account throughout the development and template design process. 

**Testing:**
Documentation and instruction provided for a comprehensive testing environment. Documentation and Tutorials will also be created to guide users through the use of the template.

**Versatility:**
- The template design prioritizes configurability and ease of use, allowing users to easily tailor challenges with diverse collectibles and set specific completion circumstances and conditions.

## TENAMINT Future Plans

**Organizational Roadmap and Future Plans**

- Short-Term: In the short-term, our primary objectives include the launch of a telegram app allowing users to take part in daily sport related activities that provide them with the potential to earn reward points that can soon be utilized on a marketplace.


- Mid-Term: Moving into the mid-term, we plan to develop and expand our product line to include more strategic and in-depth game formats such as a Daily Fantasy Sports (DFS) style game and eventually a season-long format. Our emphasis during this phase will also involve establishing dedicated fan hubs for leagues, teams, and athletes. We also plan to integrate our product on the BOS and continue with the execution of our GTM plan in sync with our partners.


- Long-Term: Looking towards the long-term, our strategic vision includes the launch of fan tokens, with token gated channels, marketplaces, experiences and hubs. We plan to execute the pre-release and launch of the upcoming TENAMINT token, aiming to position it at the core of a fan-centric, sports-oriented, and gaming ecosystem formed by these products. If required, we will also consider multi-chain integrations and support in the long-term. Additional long-term goals involve increased collectible product lines through added partnerships.


- Note: In the future we aim to eventually continue development towards advancing and improving Phygital/RWA releases and technologies by integrating and leveraging our newer product lines. Pre-developed and qualified user bases may eventually provide lead-generation for the Phygital/RWA products. Partnerships that align with the TENAMINT Phygital vision will also be considered as we recognize the trends and long-term potential of RWA-backed collectibles, fractional ownership, and other core concepts we had previously prioritized. It's essential to highlight that as mentioned in previous discussions with the Mintbase team, the Multi-Token Standard (MTS) is intricately connected to the TENAMINT Phygital product line and we'd like to explore Mintbase integration for this initiative in the future once our new product line priorities are successfully released and established.

- Other Templates: We have previously identified other potential templates that may be valuable to the ecosystem in the future. These templates may be proposed following the completion of this submission.

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
- **Games**
  - Bracket Challenge and Final Four Game
  - Telegram Pick'em App (In Development)
