# Mintbase Grant Proposal

- **Project Name: DoNFT** 
- **Team Name: DoNFT** 
- **Payment Address:donft.near** 
- **[Level]:2**

## Project Overview :page_facing_up:

### Overview

DoNFT is a decentralized protocol that allows
- modification of NFT with the help of different algorithms, for example neural networks. 
These algorithms can be created by anyone and incorporated in our service. 
- bundling a few NFTs to a single NFT.

Our project extends functionality of existing NFT standart. We provide mechanics of creating hierarchical 
structures from NFTs and decentralized backend which can generate content based on content inside these structures.
With help of DoNFT Mintbase can help creators with new, more effective and less expensive ways to manage NFTs. 
Especially our bundling/unbundling solution which might attract new categories of users to use NFTs

We are a team of professionals (smart-contract developers, ML engineers and others) who 
focus on solving practical problems. We see some use cases which we can solve with our protocol and can help other projects grow faster.


### Project Details

Technical description of our protocol
https://docs.google.com/document/d/13RSSjjvsALcE6zmQkGFvekFzQJk5ILa4tfJCa4TD3Rg/edit

MVP on Near
https://near.donft.io/
Character Generator
https://newyork.donft.io/

### Ecosystem Fit

Our project provide new use cases of using NFTs such as
- advanced NFT management (creating hierarchical structures from NFTs inside NFT)
- content generating (providing mechanism of decentralized content generator which work directly with NFT content)
Our target auditorius are
- GameFi projects (we provide technology for storing character inventory and outfit character generating)
- DeFi project (dynamic avatars with achievements system)
- NFT Collections (modificators for existing collections)

## Team :busts_in_silhouette:

### Team members

Arseniy Pechenkin CEO, Founder (SmartAn, Waves and others)
Maximilian Kobernik CBDO, Co-Founder (CB Capital VC, Coinstreet Partners, Blockchain.ru)
Marina Danilyuk CLO, Co-Founder (Clover.finance
Angie Har, CMO
Ivan Malevany, Frontend Developer
Daniil Eliseev, ML engineer
Matvey Bochko, Fullstack developer


### Contact

- **Contact Name:** Arsenii Pechenkin or Maximilian Kobernik 
- **Contact Email:** ap@donft.io or mk@donft.io
- **Website:** donft.io

### Legal Structure

### Team's experience

Arsenii Pechenkin
7+ years of professional expertise in the Information Technologies (IT) industry as Software developer, Team Lead, Solution Architect in FinTech and DeFi industries.
Winner of many hackathons all around the world.
Also have experience as evangelist, lecturer and tech-couch.

Maximilian Kobernik
Experienced professional in venture capital and the blockchain industry, as well as an ardent enthusiast of Fintech and Crypto.
Maximilian has worked as a fund manager for CB Capital where he helped dozens of companies to raise funds, taking them through all ICO processes and also i was VP at Coinstreet Partners where i was leading Token Finance Consulting Business and helped CS become one of pioneers in Security token offerings globally and helping dozens of projects around the globe with exploring new ways of attracting capital and scaling their businesses.

Marina Danilyuk
Over 14 years of professional experience in commercial, corporate, and criminal law with international practices, across 4 jurisdictions (Singapore, Estonia, Russia, Ukraine), as both a corporate legal advisor, and a court lawyer.
Over 5 years working for fintech and blockchain companies as a legal advisor.
Co-founder in MiBit. Chief Legal Adviser at SKCHAIN Advisors and Clover.Finance. Legal adviser at the Swiss Blockchain Legal.
International Liaison of the Hong Kong Blockchain Society/Representative of the Europe-Middle-East-Africa Region for International Blockchain Olympiad (IBCOL).
 
Angie Har
Masters degree in International Management. 
Head of Customer Service and Key Account Manager for a company group, worked in Business Development and Marketing for Cointelegraph. 
Has worked in the blockchain industry since 2017.


### Team Code Repos

- https://github.com/DoNFT
- https://github.com/DoNFT/DoNFT-MVP-NEAR

- https://github.com/Pe4enable
- https://github.com/IvanMalevany
- https://github.com/matt5346
- https://github.com/DanElis


### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/arsenii-pechenkin/
- https://www.linkedin.com/in/maximilian-kobernik/
- https://www.linkedin.com/in/marinadanylyuk/

## Development Status :open_book:

We already implemented 
- basic login of bundle token 
- bundling/unbundling
- centralized content generator (style transfer ML model)
- fabric of contract deploying.

You can find code here
https://github.com/DoNFT/DoNFT-MVP-NEAR

And try demo here
https://near.donft.io

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 3 months
- **Full-Time Equivalent (FTE):**  5 FTE
- **Total Costs:** 50,000$
- 
### Milestone 1 Example — Partial Changing Bundle Logic

- **Estimated duration:** 1 month
- **FTE:**  5
- **Costs:** 18,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can work with bundle functionality. |
| 0c. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone.|
| 0d. | Article | We will publish an article that explains bundle functionality. |
| 1. | Smart Contract | We will create a NEAR smart contract that will provide bundle and unbundle logic, editing bundle token |
| 2. | Frontend | We will create a frontend for working with bundle functionality. |
| 3. | Backend | We will create a centralized backend which generate outfit of character |

### Milestone 2 — Approval Mechanics

- **Estimated Duration:** 1 month
- **FTE:**  5
- **Costs:** 18,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can work with approval mechanics of bundle functionality.|
| 0c. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone.|
| 0d. | Article |We will publish an article that explains approval mechanics of bundle functionality. |
| 1. | Smart Contract | We will create a NEAR smart contract that will provide approval mechanics of bundle functionality
- give rights for changing bundle to other smart contract
- revoke rights  for changing bundle to other smart contract
- get list of smart contract which have rights got changing bundles |
| 2. | Frontend | We will create a frontend for working with approval mechanics of bundle functionality.  |


### Milestone 3 — Royalty management

- **Estimated Duration:** 1 month
- **FTE:**  5
- **Costs:** 17,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can work with royalty management with bundle functionality.|
| 0c. | Docker |We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0d. | Article | We will publish an article that explains royalty management with bundle functionality.|
| 1. | Smart Contract | We will create a NEAR smart contract that will provide royalty management with bundle functionality. Setuping royalty for bundle. Recalculating royalties for bundled tokens. Distribute royalties to creators of bundled tokens |
| 2. | Frontend | We will create a frontend for working with royalty management with bundle functionality.  |

## Future Plans

DoNFT wants to serve every NFT vertical — from art, music, gaming, to digital IDs and “green” NFTs. 
Rather than being the marketplace that sells all of these NFTs across categories, DoNFT wants to enable other 
platforms and newcomers to build experiences, platforms, and NFT marketplaces and DAOs in these categories 
and become an ultimate toolkit for building metaverse(s). We already work on different pilots in different 
niches and keep getting more ideas from community and crypto projects on new ways of using our protocol and collaborations.

The  DoNFT protocol is applicable to a wide variety of use cases, from helping existing NFT communities and 
marketplaces to even bigger goals of bringing millions of new creators and users to the blockchain ecosystems and NFTs. 
We believe that more simple solutions and toolkits for creating NFTs and lowering the barrier in entering NFT space 
together with adding more functionality and use cases will help with adoption of Web 3.0 tech. The DoNFT protocol provides 
flexible and innovative solutions by which to do so.


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** From MintBase team

Work you have already done.
- MVP on Near (near.donft.io)
- MVP on EVM chains (donft.io)


Previous grants you may have applied for.
Human Guild, FileCoin, Polygon Studio

