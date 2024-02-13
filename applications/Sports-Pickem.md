#Mintbase Grant Proposal

- Project Name: Sports Pick'em
- Company Name: TENAMINT
- Payment Address: bstt.near
- Grant Level: 2
- [Deck Link](https://drive.google.com/file/d/1VYaIq1BaJJ2A7r9JUvZFcIovBGXemnLb/view?usp=share_link): [https://drive.google.com/file/d/1VYaIq1BaJJ2A7r9JUvZFcIovBGXemnLb/view?usp=share\_link](https://drive.google.com/file/d/1VYaIq1BaJJ2A7r9JUvZFcIovBGXemnLb/view?usp=share_link)

###Project Overview

Project Description: **TENAMINT Sports Pick'em Game and Digital Collectibles**

Introduce a groundbreaking era of sports gaming, inviting fans to actively participate in the exhilaration of sports fandom through the dynamic Sports Pick'em game. Engage by selecting winners in this free-to-play game, earning rewards, and collecting integrated sports collectibles.

The envisioned product is crafted to transform into token-gated fan hubs, facilitating fan engagement throughout the pre-game, in-game, and post-game phases. The fan-centric approach and product features are tailored to resonate with the modern fan that seeks alternative and dynamic fan experiences that extend beyond simply watching the game.

The game, rewards, collectibles, and upcoming product additions are intricately crafted to offer value in league partnerships, delivering exclusive content, digital collectibles, fan tokens and IRL activations. This approach fosters a vibrant ecosystem of sports enthusiasts, resulting in heightened fan experiences, strengthened league-to-fan relationships, and a compelling avenue for web3 fan-base adoption.

####Key Features and Rewards:

- **Pick'em Game:**
  - Introduces dynamic live match prediction within Sports Pick'em.
  - Users predict scores, player performances, and key play outcomes.
  - Offers immediate rewards for correct predictions (points or tokens).
  - Rewards can be redeemed for exclusive NFTs, fan experiences, or merchandise.
- **Collectables:**
  - NFTs are a core aspect of the Pick'em game and can be used to enhance rewards in the Pick'em game and synergize with other games.
  - Collectibles feature challenges that can be completed for further rewards.
  - Collectibles are intended to be league partner/team partner specific
  - Collectibles can be redeemed in exchange for rewards creating a loop
- **Marketplace:**
  - Mintbase as the secondary marketplace for TENAMINT and partner digital collectibles.
  - Enables wider audience reach and focus on program aspects.
  - Partnership with Mintbase for featured listings enhances exposure.
- **Rewards:**
  - Pick'em contest winners receive reward tokens.
  - Tokens can be used within the platform or exchanged.
  - Redeemable for digital collectibles, IRL experiences, and merchandise.
- **In-Game Engagement with Mintbase:**
  - "Mint Your Moment" feature captures fan experiences at games with the NFT Minter


####Project Details

- **Figma Exports:**
  - [https://drive.google.com/drive/folders/1-rek75CWt7GZhEPEWXRKxfW2pQ7xgPIo?usp=share\_link](https://drive.google.com/drive/folders/1-rek75CWt7GZhEPEWXRKxfW2pQ7xgPIo?usp=share_link)
- **Exploratory/Early Staging Site:**
  - [https://basketball-pickem-app.vercel.app/](https://basketball-pickem-app.vercel.app/)
- **Data Models**
  - Data Model 1 (Research: User Flow Preliminary Conceptualization)
    - [https://drive.google.com/file/d/1iCfI1y4dS4LQlhaMdx5paqJbsukCDHXm/view?usp=share\_link](https://drive.google.com/file/d/1iCfI1y4dS4LQlhaMdx5paqJbsukCDHXm/view?usp=share_link)
  - Data Model 2 (Research: Token Interconnectivity Preliminary Conceptulization )
    - [https://drive.google.com/file/d/1slCt0Iz0pLFRpbtcstNFMM13rdc6Axvz/view?usp=share\_link](https://drive.google.com/file/d/1slCt0Iz0pLFRpbtcstNFMM13rdc6Axvz/view?usp=share_link)
  - Architecture Model
    - [https://drive.google.com/file/d/1pg3gqbdVOpssSn9sB0beQ\_FGDmX\_8v1I/view?usp=sharing](https://drive.google.com/file/d/1pg3gqbdVOpssSn9sB0beQ_FGDmX_8v1I/view?usp=sharing)

####Tech-Stack, Architecture, and Components

- TENAMINT Digital & Pick'em Game:
  - Front-End:
    - Design Tools:
      - Figma (UI)
      - Adobe Suite (Graph/Vector Design)
    - Styling:
      - Tailwind CSS for utility-first styling
      - Tailwind UI, NextUI, Chakra UI for component libraries
      - Storybook for UI component development
    - Login Authentication
      - Firebase Auth for authentication
      - Mintbase Social Authentication
    - Email Notification
      - Firebase Auth for email notification
      - js for server-side functionality
    - Framework
      - js for React-based web applications with server-side rendering and static site generation and flexibility.
    - Hosting:
      - Vercel for optimized Next.js hosting with serverless functions

  - Backend
    - Database Requirements:
      - Primary: Firebase
      - Alternatively: MySQL or MongoDB
    - Backend Hosting:
      - Primary: Google Cloud Platform (GCP) for cloud services
      - Decentralized Storage: Arweave and NFTStorage
      - Additional Cloud Infrastructure: Digital Ocean
    - Scaling Considerations:
      - Utilize serverless architectures and content delivery networks (CDNs)
      - Implement dynamic scaling of backend services
      - Integrate NEAR blockchain technologies for efficient scaling and ecosystem integration
      - Ensure high availability and fast content delivery under varying loads
    - CI/CD Implementation:
      - GitHub Actions used for Continuous Integration/Continuous Deployment and to automate software build, test, and deployment processes
    - Data Models / API Specifications:
      - RESTful APIs or GraphQL endpoints for core functionality
      - Sports Data Provider: Genius Sports or Other
    - Documentation:
      - Comprehensive documentation for core components, protocols, and architecture
      - Include integration details with sports data provider or similar for real-time data retrieval
    - Blockchain
      - NEAR network for blockchain functionality
      - NEPs
        - NEP141 (FT)
          - Rewards
        - NEP171 (NFT)
          - Collectibles
        - NEP245 (MTS)
    - Marketplace
      - NFT: Mintbase Marketplace Integration
      - Merchandise E-Commerce: Droplinked
    - Blockchain Indexer
      - GraphQL API for blockchain indexer
    - Wallet:
      - NEAR Wallet Selector
        - Mintbase Wallet, HERE, and others
        - Tenamint Wallet App
          - Powered by Mintbase Wallet
          - Potential for direct to marketplace/rewards integration

###Ecosystem Fit

####Where and How TENAMINT Digital and Gaming fits into the Ecosystem:

  - Mintbase:
    - Mintbase Wallet
      - User friendly and streamlined user onboarding
      - Passkey/Biometric Sign-in
      - Social Auth Options
    - Marketplace
      - Primary Release Page (Collectible Packs)
      - Secondary Marketplace
        - Categorized by Sport, League, Team, Athlete byLeague\>Team\>Athlete
    - Stripe Integration
      - Payments to be processed in a variety of formats including NEAR and FIAT via Mintbase Stripe integration.
    - NFT Minter:
      - "Mint Your Moment" is a powerful way for TENAMINT to enable user-generated content providing data points for partner fan engagement and providing proof-of-watch metrics. This can be used whether at the game or tuning in from home. Fans can now seamlessly capture and immortalize their favorite game moments and easily share their fan perspective with other fanatics, earn rewards, and be showcased on screen during the game.
    - AffiliateDirect
      - Utilizing the Mintbase AffiliateDirect referral system allows TENAMINT to engage fans and influencers to generate a personalized affiliate link to earn rewards and foster active promotion and community collaboration.
    - Indexer
      - Utilization of GraphQL Indexer for efficient user data tracking, swiftly determining token ownership, royalties, and other critical information. This enhances platform dynamics, providing valuable insights into user behavior.

  - NEAR
    - Leading Sports Gaming and Collectibles on NEAR
      - TENAMINT aims to position itself as a pioneering force in the Web3 ecosystem, specifically targeting the sports entertainment and collectibles sector with an emphasis on creative, interconnected, and fan-first products.
      - The products intertwine to introduce an innovative approach to sports gaming with free-to-play predictive gaming, fan engagement, league partnerships, and integrated collectibles with utility.
      - The integration of NFTs as a core element in the Pick'em game adds a unique layer to the user experience, providing enhanced rewards, integrating challenges, and creating a fan-engagement ecosystem that encourages consistent activity pre-game, during the game, and post-game.

    - The Power of Sport Driven Web3 Adoption
      - Recognizing the vast market potential of sports, TENAMINT strategically taps into the historical significance of sports as a driving force for Web3 adoption.
      - In the realm of Web3 adoption, sports enthusiasts present a particularly engaged and passionate user base. TENAMINT aims not only to capture this existing audience but also to showcase how the intersection of sports and blockchain technology can drive widespread adoption.
      - As NEAR becomes the foundation for this innovative sports-centric platform, there is a symbiotic relationship as TENAMINT benefits from the scalability of NEAR but also contributes to the growth of the NEAR ecosystem
      - The proposed products aim to demonstrate the impact of fan-first products and seamless integration with user-experience focused blockchain technology.
    - Building for League Partnerships
      - The proposed products are conceptualized with sports league research and feedback with the goal of utilizing its technological developments as a catalyst for strategic league and team partnerships.
      - By forging alliances with sports entities, TENAMINT aims to bring entire fan-bases into its ecosystem, creating a collaborative space where sports enthusiasts can engage with their favorite teams and leagues through innovative digital experiences and collectibles.
    - IRL activations and use-cases
      - The proposed products and developments are designed with in-arena and other IRL use-cases in mind.
      - For example, the "Mint Your Moment" feature, capturing fan experiences at games and immortalizing them as NFTs on the NEAR blockchain, not only advances Web3 technology but also provides a tangible connection between the digital and physical worlds.
      - TENAMINT's commitment to bridging the digital-physical gap sets it apart as a holistic and immersive experience within the sports and gaming ecosystem.

####Target Audience and Needs Met:

- Target Audience:
  - Sports Leagues and Teams:
    - Providing fan experience solutions through partnerships that enable TENAMINT to revolutionize the fan experience for leagues through immersive games, collectibles, and rewards that cater to the evolving needs of the next generation of fans.
  - Sports Enthusiasts:
    - Designed to appeal to fans of all sports, leagues, teams, players regardless of their experience with web3
  - Collectibles Enthusiasts:
    - Sports Collectibles: Engaging collectors who appreciate sports collectibles whether digital or physical
    - Digital Collectibles: Catering to the growing community of NFT collectors, providing exclusive sports-related digital assets on the blockchain.
  - Blockchain Enthusiasts:
    - Attracting individuals interested in blockchain technology and its applications, introducing them to on-chain fantasy sports and collectibles.
  - Gamers:
    - Fantasy Sports Players: Captivating the interest of fantasy sports enthusiasts who relish strategic team-building, decision-making, challenges, and spirited competition. The engagement of these dedicated players is set to escalate with the rollout of additional game formats.
    - Web3 Gamers: Addressing the needs of gamers within the Web3 space who are seeking blockchain enabled games
    - Play-to-Earn Gamers: Built with an emphasis on rewards to attract those actively seeking games with play-to-earn features.

- Audience Needs Met by the Project
  - Sports Leagues and Teams:
    - Need: Enhanced Fan Engagement
      - Solution: Through strategic partnerships, TENAMINT revolutionizes the fan experience for sports leagues with immersive games, collectibles, and rewards, meeting the evolving expectations of the next generation of fans.
      - The goal (with reference to league needs) is to build technology that transforms sports engagement through data-driven insights and advanced sport technology. It monitors fan interactions with NFTs and fantasy leagues, providing crucial analyses of preferences, demographics, and digital asset performance. The platform can provide insights relating to fan acquisition, retention, behaviors, and data, empowering leagues with data for informed decision-making. This solution can help leagues navigate the dynamic landscape of evolving fan engagement.
  - Sports Enthusiasts:
    - Need: Immersive Fan Experience
      - Solution: Creating an all-encompassing fan experience through creation of gaming options, official collectibles, rewards, more as a part of a fan-first ecosystem. These offerings resonate with fans across diverse sports, leagues, teams, and players, ensuring inclusivity and enjoyment, irrespective of their familiarity with Web3 technologies.
  - Collectibles Enthusiasts:
    - Need: Engaging Collectibles Experience
      - Solution: Crafting captivating collectibles experiences by introducing exclusive collectibles that directly connect fans to their favorite teams and players.. These digital collectibles provide utility, enhancing rewards in games and addressing the evolving needs of fans.
  - Gamers:
    - Need: Engaging Gaming Experiences
      - Solution: For fantasy sports players, TENAMINT provides a platform for pick'em and fantasy sports based competition with a play-to-earn rewards system that connects fans to their favorite leagues, teams, and players.
      - Solution: Fantasy sports on the blockchain offer benefits such as enhanced transparency, immutable records, global accessibility, reduced fees, ownership of digital assets, and smart contract automation, ensuring fairness, security, and a more engaging user experience.
      - Solution: For play-to-earn gamers, the emphasis is on rewarding gameplay experiences, creating a dynamic and incentivized gaming environment.


####Similar Projects in NEAR Ecosystems:

- Playible also offers a fantasy sports focused product but with a different approach to the space.
- SailGP has a officially licensed league NFT on built on NEAR from previous Seasons
- Dropt has a loyalty platform related to sports advertising and sponsorship
- Details pending TBA: The NEAR Foundation will become the ICC's official blockchain partner.


###Team

####
 Team members

- Nadir Chaudhry
- Sal Chaudhry
- Alex Astrum
- Shawn Bender
- Shunsuke Kano
- Walt Yao

####Contact

- Sal Chaudhry
- [office@tenamint.com](mailto:office@tenamint.com)
- [https://www.linkedin.com/in/salchaudhry](https://www.linkedin.com/in/salchaudhry/?originalSubdomain=ca)

####Legal Structure

  - Registered Legal Entity
    - Ball Street Technologies Inc.
  - Registered Address:
    - 408-146 Fort York Blvd.,
      - Toronto, ON, Canada
      - M5V0E1

####Team Experience

- TENAMINT:
  - [www.tenamint.com/](http://www.tenamint.com/phygital)
  - [https://app.tenamint.com/](https://app.tenamint.com/)
  - TENAMINT leads the way in shaping the future of sports fan experiences, bringing immersive fantasy sports with play-to-earn rewards in addition to utility driven collectibles, and other fan-first offerings to a global audience. From fantasy sports gaming and digital collectibles to real-world activations, our products seamlessly converge, redefining fan engagement through a harmonious blend of blockchain technology, enticing rewards, and unparalleled fan experiences. This innovative approach extends beyond the digital realm, fostering real-world activations that not only fortify fan loyalty but also spearhead the evolution of interactive fan experiences.
  - Accelerators and similar programs:
    - NEAR HZN1, Innovation Boost Zone, Accelerator Centre, Founder Institute, etc.

####Team Links:

- TENAMINT:
    - [https://github.com/TENAMINT](https://github.com/TENAMINT)
    - [https://www.linkedin.com/company/73837227/admin/feed/posts/](https://www.linkedin.com/company/73837227/admin/feed/posts/)
  - Nadir
    - [https://www.linkedin.com/in/nadirchaudhry/](https://www.linkedin.com/in/nadirchaudhry/)
  - Sal
    - [https://github.com/Salikc9](https://github.com/Salikc9)
    - [https://www.linkedin.com/in/salchaudhry/](https://www.linkedin.com/in/salchaudhry/?originalSubdomain=ca)
  - Alex:
    - [https://github.com/alexastrum](https://github.com/alexastrum)
    - [https://www.linkedin.com/in/alexastrum/](https://www.linkedin.com/in/alexastrum/?originalSubdomain=ca)
  - Shunsuke:
    - [https://github.com/ShunsukeKano](https://github.com/ShunsukeKano)
    - [https://www.linkedin.com/in/shunsuke-kano/](https://www.linkedin.com/in/shunsuke-kano/)
  - Walt:
    - [https://github.com/yaozakai](https://github.com/yaozakai?tab=repositories)
    - [https://www.linkedin.com/in/waltyao/](https://www.linkedin.com/in/waltyao/)
  - Shawn:
    - [https://github.com/Benz222](https://github.com/Benz222)

###Development Status

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

####Wireframes, Mock-ups, Other:

- Figma Mockup:
  - [https://www.figma.com/proto/5ivxRYoqK2KHjKHhCht8up/TENAMINT-Pick'em?type=design&node-id=1300-2012&t=N4dCS0wkNofN63Fx-0&scaling=min-zoom&page-id=0%3A1&starting-point-node-id=1300%3A2012&show-proto-sidebar=1](https://www.figma.com/proto/5ivxRYoqK2KHjKHhCht8up/TENAMINT-Pick'em?type=design&node-id=1300-2012&t=N4dCS0wkNofN63Fx-0&scaling=min-zoom&page-id=0%3A1&starting-point-node-id=1300%3A2012&show-proto-sidebar=1)
- Early Staging Site:
  - [https://basketball-pickem-app.vercel.app/](https://basketball-pickem-app.vercel.app/)
- Data & Architecture Models
  - [https://drive.google.com/drive/folders/1-jmuOf-ubdDAY0ADfsPTALMGNEDnXYgc?usp=sharing](https://drive.google.com/drive/folders/1-jmuOf-ubdDAY0ADfsPTALMGNEDnXYgc?usp=sharing)
- Other Products: [https://tenamint.com/phygital](https://tenamint.com/phygital)
- _Note: A team development submission was submitted to abstraction hacks which featured some high-level concepts discussed in this proposal._

####Research Done:

- Engagement with League Executives: In-depth conversations and consultations have taken place with league executives to gain valuable insights into their specific needs and identify emerging opportunities within the industry.
- Comprehensive Review and Audit: Rigorous examination and audit of fantasy sports gaming products, pick'em games, and sports collectibles have been undertaken. This comprehensive review ensures a thorough understanding of the current landscape and identifies areas for innovation and improvement.
- Market Research with Collectors: Valuable perspectives on the future of sport collectibles have been gathered through extensive in-person research with physical sport card collectors. This direct interaction has provided nuanced insights into the preferences and expectations of collectors in the evolving market.
- Industry Specialization: The co-founder brings a wealth of expertise to the project, grounded in an educational background specializing in sport business. Emphasizing fan experience and sport technology, the co-founder adopts a strategic and thoroughly researched approach to advancements in sports innovation and fan engagement.
- Other Relevant Experiences: A lifetime dedicated to being a team of passionate enthusiasts, including roles as die-hard fans, fantasy sports players, league commissioners, and collectors of both digital and physical sports collectibles, has provided a unique and holistic perspective. This firsthand experience enriches the project with a deep understanding of user expectations and industry dynamics.

####Relevant Conversations: Mintbase

- Paul via Zoom Call - September 2023
  - Discussion Regarding: TENAMINT and Mintbase Grant
- Nate at NEARCON 2023 - November 2023
  - Discussion Regarding: MTS, TENAMINT, and Future Opportunities


###Milestones and Product Roadmap

####Overview — Milestone Summary

- Estimated Duration: 4.5 months
- FTE: 3
- Costs: $50,000 USD

####Milestone 1 — Sports Pick'em Game

- Estimated Duration: 1.5 months
- FTE: 3
- Costs: $15,000 USD
- Deliverables:

    - Game:
      - Mechanics Research and Refinement
      - UX/UI Design and Prototyping
      - Pick'em Game Development
      - Integrations
        - Onboarding
        - Wallet Integration
        - Data Provider Integration
        - Smart Contracts
      - Release: Game Beta
    - Rewards and Rewards Marketplace:
      - Planning
      - Research
      - UX/UI Design and Prototyping
      - Mechanics
    - Testing, Feedback, and Improvements:
      - Testing
      - Design Feedback Improvements
      - Experience Feedback Improvements
      - Mechanics Feedback Improvements
      - Documentation Update

####Milestone 2 — Rewards Development, Release, and Refinement

Duration: 1.5 months

- FTE: 3
- Costs: $15,000 USD
- Deliverables:

    - Rewards and Rewards Marketplace:
      - Design
      - Utility
      - Mechanics
      - Smart Contracts
      - Integrations
      - Release: Rewards Marketplace
      - Rewards & Game Integration
    - Affiliate Program Integration
    - NFT Planning:
      - Planning
      - Research
      - Design and Prototyping
      - Mechanics
    - Testing, Feedback, and Improvements:
      - Testing
      - Design Feedback Improvements
      - Experience Feedback Improvements
      - Mechanics Feedback Improvements
      - Documentation Update

####Milestone 3 — NFT Release, Integration, & Development

- Estimated Duration: 1.5 Months
- FTE: 3
- Costs: $20,000 USD
- Deliverables:

    - NFT
      - Pre-Release
      - Official Release (V0 or V1 - Dependent on BD Variables)
      - Primary Marketplace Open
      - Secondary Marketplace Open
      - Challenges
      - Stripe Connection
      - Game Integration
      - Rewards Integration
    - NFT Minter ("Mint Your Moment")
    - IRL Activation Beta
      - Game
      - NFT Minter
    - Testing, Feedback, and Improvements:
      - Testing
      - Design Feedback Improvements
      - Experience Feedback Improvements
      - Mechanics Feedback Improvements
      - Documentation Update

###Future Plans

- **Short-Term:** In the short-term, our primary objectives include the integration of merchandise E-Commerce for the Rewards Marketplace, along with the implementation of social share features and the introduction of a bracket game to enhance user engagement. Additionally, we aim to execute our Go-To-Market (GTM) plan, focusing on strategic partnerships and the execution of seamless IRL activations in arena venues.

- **Mid-Term:** Moving into the mid-term, we plan to develop and expand our product line to include more strategic and in-depth game formats such as a Daily Fantasy Sports (DFS) style game and eventually a season-long format. Our emphasis during this phase will also involve establishing dedicated fan hubs for leagues, teams, and athletes. We also plan to integrate our product on the BOS and continue with the execution of our GTM plan in sync with our partners.

- **Long-Term:** Looking towards the long-term, our strategic vision includes the launch of fan tokens, with token gated channels, marketplaces, experiences and hubs. We plan to execute the pre-release and launch of the upcoming TENAMINT token, aiming to position it at the core of a fan-centric, sports-oriented, and gaming ecosystem formed by these products. If required, we will also consider multi-chain integrations and support in the long-term. Additional long-term goals involve increased collectible product lines through added partnerships.

- **Note:** In the future we aim to eventually continue development towards advancing and improving Phygital/RWA releases and technologies by integrating and leveraging our newer product lines. Pre-developed and qualified userbases may eventually provide lead-generation for the Phygital/RWA products. Partnerships that align with the TENAMINT Phygital vision will also be considered as we recognize the trends and long-term potential of RWA-backed collectibles, fractional ownership, and other core concepts we had previously prioritized. It's essential to highlight that as mentioned in previous discussions with the Mintbase team, the Multi-Token Standard (MTS) is intricately connected to the TENAMINT Phygital product line and we'd like to explore Mintbase integration for this initiative in the future once our new product line priorities are successfully released and established.

###Additional Information

How did you hear about the Grants Program?

- Mintbase team members:
  - Nate G.
  - Paul K.


Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.

    - Tenamint Phygital
      - Fractional & RWA-backed collectible cards
      - ~$40,000 RWA Assets Listed
      - Established
    - Partnerships
    - Accelerator Programs
      - HZN, Accelerator Centre, Founder Institute, etc.
    - $40,000+ Listed Assets
    - MTS NEP Contributions
    - Pre-Seed Fundraise in Early 2022
    - Grants
      - NF Grant (2022 Builder & Quick-Starter Grants)

