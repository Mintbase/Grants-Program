# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** ChainLib
- **Team Name:** ChainLib
- **Payment Address:** chainlib.near
- **[Level](../README.md#level_slider-levels):** 2

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

ChainLib is an NFT Gating Library that allows authors to create personalized file collections (libraries) on Encrypted Decentralized Storage and provide access to family members through NFT-based memberships.  
We aim to expand the traditional functionality of NFT, by implementing NFT-based memberships with Expiration and Renewable options. Our project introduces a new concept - NFT-based memberships with updatable metadata. NFT-based memberships is a fundamental medium that challenges common concept of pricing plans, offering endless possibilities of adding scarcity and urgency to digital art, providing exclusive access to content, distributing royalties, managing event tickets, enhancing gaming experiences, facilitating limited-time promotions, and more across various industries.


### Overview

ChainLib is an NFT Gating Library that allows authors to create personalized file collections (libraries) on Encrypted Decentralized Storage and provide access to family members through NFT-based memberships.   

**How our project relates to / integrates into the Mintbase / NEAR ecosystem**
ChainLib is a dapp built and functioning on the NEAR blockchain, accepting offers and payments in Near. We aim to extend NFT marketplace functionality by implementing :
- **Personalized file collections (libraries) on Encrypted Decentralized Storage** - programmable story tales with predefined reader’s parameters. I.E. story tales for children. Parents can order books with their child right inside the story. All they need is to add their child’s name, sex, age, and the book's subject and our platform will create the story tale with their child inside it and store it on an Encrypted Decentralized Storage
- **NFT based Memberships with Expiration and Renewable options**, constantly manipulating the variable possibilities. NFT-based memberships can be used for access control to exclusive content, events, communities, or services. Owners of specific NFT memberships can unlock privileges or access restricted areas that are not available to non-members.  
We aim to develop on Mintbase the functionality described above. Thus we hope to extend existing functionality of our platform and of Mintbase, as well.
 
**Why are we doing this?** 
Initially, we were a team of authors and illustrators from a small country from Europe. At this moment we have more than 10 books with fairy tales published and 20 more books in various writing/publishing stages. In 2020, we published Christmas Tales for ages 0+ and 7-12 in a small print run of 1000 copies, and in 2021 we successfully sold the entire print run, donating money to charity. We have a signed agreement with one of the biggest publishing agencies in our country, which allows us to distribute our books through the bookstores network in 2 countries from Europe. When registering copyright on our books, we found out that it covers only the territory of our country, which is very strange and unfair to the authors. Basically, anyone outside our country can take any of our books and publish them under their name. This is how the idea with Chainlib was born. Aside from the authors’ copyright protection problem there is another one we want to solve. Most publishers find it difficult to accept new emerging writers. This is why we created an NFT marketplace for books and other works, where any author can upload his/her works and sell them as NFTs to its audience. Our team now includes developers, designers and other IT staff. If the legislation cannot solve the authors' problems, the technology should do it.
In 2022 we obtained a grant from Near Foundation and developed the NFT marketplace, using Mintbase API. In process of development we realized that we can extend the functionality of standard NFT. This is how the concept of personalized books was developed. Since then, we created (almost manually) and sold IRL appr. 1k of personalized books.
During the 2023 our team:
- created 3 new story tales for children. Check the images here: https://drive.google.com/drive/folders/1Hhtf5dP4OzIfymMuhnToXeslgs92JXcT?usp=sharing
- sold more than 1000 hard copies of the story tales for children, mentioned above
- developed a new concept Kidbrary - a personalized library created based on Your child's digital identity.

### Project Details

**Project's links:**

- Chainlib platform: https://chainlib.xyz/en
- Chainlib pitch deck: https://www.canva.com/design/DAFSlUy9eBU/rCDSpzIFKq0ctl_U3z38xg/view?utm_content=DAFSlUy9eBU&utm_campaign=designshare&utm_medium=link&utm_source=publishsharelink
- GitHub: https://github.com/blockwebdevs/chainlib
- Mintbase store (mainnet): https://www.mintbase.io/contract/chainlib.mintbase1.near?tab=nfts&page=0


**Mockups/designs of any UI components**
Basically, the UI/UIX of our digital library marketplace is developed. It can be accessed by the following link:
https://chainlib.xyz/en

Currently, we are working on adding new functionality to our NFT marketplace:
- Personalized story tales as NFTs
- NFT based Memberships with Expiration and Renewable options

Below You can see how the personalized story tales look IRL:

![IMG_7730](https://user-images.githubusercontent.com/106525564/209929600-356b60f2-a4c1-401b-b028-96d24341d412.JPG)
![camphoto_1903590565](https://user-images.githubusercontent.com/106525564/209930851-f9a0dd7b-3166-409e-b48f-e9882b7ed760.jpg)


**Data models / API specifications of the core functionality**

- Project’s architecture (updated concept): https://miro.com/app/board/uXjVKceTKr4=/?moveToWidget=3458764583606951168&cot=14

**An overview of the technology stack to be used**

- Backend: NodeJS, Laravel, PHP 8+, MySQL
- Frontend: HTML5, CSS3, BOOTSTRAP
- JS - Vue / Nuxt, React / Next, JQUERY
- Rust, Near API & SDK, Mintbase API, Arveawe API

**Documentation of core components, protocols, architecture, etc. to be deployed**

- Project’s architecture: https://miro.com/app/board/uXjVKceTKr4=/?moveToWidget=3458764583606951168&cot=14

**PoC/MVP or other relevant prior work or research on the topic**

Our team has built the digital library as NFT marketplace and web3 dapp integrated with Near blockchain. The Main links:
- Digital library as NFT marketplace and web3 dapp: https://chainlib.xyz/en
- Integration with Mintbase API: https://github.com/blockwebdevs/chainlib-minter
- Near Smart contracts: Logination/authorization by Near wallet, Buy with Near and View Auction (through Mintbase API)
- Github repository of the project: https://github.com/blockwebdevs/chainlib


### Ecosystem Fit

We aim to expand the traditional functionality of NFT, by implementing NFT-based memberships with Expiration and Renewable options. 
Key possibilities that NFT-based memberships can provide (in future):

- Access Control: NFT-based memberships can be used for access control to exclusive content, events, communities, or services. Owners of specific NFT memberships can unlock privileges or access restricted areas that are not available to non-members.
- Collectible and Scarcity: NFT memberships can be designed as collectibles, with each membership representing a unique and limited edition. This scarcity can increase the perceived value of the memberships and encourage people to collect them.
- Tokenized Benefits: Membership benefits can be tokenized as NFTs, allowing members to trade, sell, or transfer their benefits to others. For example, if a membership includes discounts, special offers, or unique experiences, these can be represented as NFTs that can be exchanged or sold in secondary markets.
- Gamification: NFT memberships can add gamification elements to the membership experience. For instance, members may unlock achievements, badges, or special rewards based on their level of engagement or participation within the community.
- Community Engagement: NFT-based memberships can foster a sense of belonging and community among members. Owners of specific NFT memberships may gain access to private forums, chat groups, or networking opportunities, creating a more intimate and exclusive community experience.
- Royalties and Revenue Sharing: NFT memberships can be designed with built-in royalty mechanisms, allowing creators or organizations to earn royalties whenever their memberships are resold or traded in secondary markets. This can create ongoing revenue streams for creators and incentivize the creation of valuable memberships.
- Proof of Ownership: NFTs provide a transparent and immutable record of ownership on the blockchain. This means that NFT-based memberships offer a verifiable proof of ownership, preventing counterfeit memberships and ensuring the authenticity of each membership.
- Integration with DeFi: NFT-based memberships can be integrated with decentralized finance (DeFi) protocols, enabling features such as staking, lending, borrowing, or yield farming based on membership ownership. This can add financial utility and value to NFT memberships.

**Target audience**
Our target audience are families with children. We are going to expand this audience together with expansion of our functionality. Imagine, for example a famous author such as J. K. Rowling, writing next book about Harry Potter and her fans that can get access to every new chapter upon release, based on NFT-based memberships. 
**What need(s) does your project meet?**

- Personalization - with new functionality we solve a great demand for personalized content (stories with You or Your child the main heroes)
- Access to programmable art - new stories created programmatically based on your parameters, dropped as NFT
- Ownership and decentralization - Users own their stories
- Access to exclusive content, through NFT-based memberships

**Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?**
- There are a few project connected with the book industry. However, most of these projects are in early development stage. Additionally, our project has oustanding idea (Personalized story tales as NFTs + NFT-based memberships)
- There are similar projects in web2 (personalized stories). However, our Personalized story tales as NFTs + NFT-based memberships concepts make our project innovative and different from our competiotion.

## Team :busts_in_silhouette:

### Team members

- Anna Griza
- Fion Golden
- Valeriu Chocklea
- Andrei Tintari
- Iovita Tudor
- Arhiv Tudor
- Efim Serghei

### Contact

- **Contact Name:** Anna Griza
- **Contact Email:** chainlibdigital@gmail.com
- **Website:** https://chainlib.xyz/en

### Team's experience

**Anna Griza** 
I am a writer with around 30 books for children and moms, including Ebooks, Fairy Tales, Audio Books & Postcard books. I work through written texts and illustrations with psychologists, because I want fairy tales to be not only interesting but also useful, and have therapeutic character. Additionally, I set a global goal of adapting my stories to other languages/countries, and children's psycho types are somewhat different geographically.
**Fion Golden**
This is my co-author. He is dyslexic, and cannot read or write, but very beautiful stories are born in his fantasy. And I describe these stories.
**Valeriu Chocklea**
This is our main illustrator, who gives birth to exceptional characters from our stories. He is the creator of the illustrations from our postcard books.
**Andrei Tintari**
Andrei is responsible for the technical part of our platform. He is the head of Block Web Devs - a group of web2 and web3 developers, writing code on Node JS, Vue JS, React, PHP, Laravel and Blockchain.
Andrei has multiple Years of experience as an entrepreneur, founder, and executive at multiple companies in the IT & Digital Marketing Industry, namely being the Founder of Terra Digital (Romania), IT Mall (Estonia), and Like-Media (Republic of Moldova). Prior to his experience as Founder of mentioned companies, Andrei has worked for over 10 years in traditional financial services, initially in banking before moving to corporate development. Andrei spent six years in financial markets working as Internal Auditor in the Banking Industry and as CFO in the Corporate sector. Andrei holds an economics and finance degree from Romanian University.
Block Web Devs Group has built more than 25 projects in web2 and web3 space. You can find more details in one of there GitHub accounts: https://github.com/blockwebdevs
**Iovita Tudor**
This is a full-stack software engineer. His expertise includes:
- Backend: Node JD, Postgres
- Frontend: HTML5, CSS3, BOOTSTRAP
- JS - Vue / Nuxt, React / Next, JQUERY
- Rust, Near API & SDK, Mintbase API
**Arhiv Tudor**
Efim is a backend software engineer. His expertise includes:
- Backend: Laravel, PHP 8+, MySQL
- JS - Vue / Nuxt, React / Next, JQUERY
- Rust, Near API & SDK, Mintbase API
**Efim Serghei**
Joan is a backend software engineer. His expertise includes:
- Frontend: HTML5, CSS3, BOOTSTRAP
- JS - Vue / Nuxt, React / Next, JQUERY

Our project obtained a $25k grant from Near Foundation, succesefully implemented. The link to the project: https://chainlib.xyz/en

### Team Code Repos

- https://github.com/blockwebdevs/chainlib/
- https://github.com/blockwebdevs/chainlib/tree/master/assembly
- https://github.com/blockwebdevs/chainlib-minter

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/blockwebdevs
- https://github.com/chainlibdigital
- https://github.com/iovitatudor
- https://github.com/archiveiovita
- https://github.com/likedevs

### Team LinkedIn Profiles (if available)

- https://www.facebook.com/annaamita17
- https://www.instagram.com/anna_amita/
- https://www.linkedin.com/in/andrei-tintari-839580218/


## Development Status :open_book:

Our team has built the digital library as NFT marketplace and web3 dapp integrated with Near blockchain. The Main links:
- Digital library as NFT marketplace and web3 dapp: https://chainlib.xyz/en
- Integration with Mintbase API: https://github.com/blockwebdevs/chainlib-minter
- Near Smart contracts: Logination/authorization by Near wallet, Buy with Near and View Auction (through Mintbase API)
- Github repository of the project: https://github.com/blockwebdevs/chainlib

## Development Roadmap :nut_and_bolt:

### Overview

- Total Estimated Duration: 3 months
- Full-Time Equivalent (FTE): 5 FTE
- Total Costs: 50,000 USD

### Milestone 1 - Personalized  books and library (as an Encrypted  Decentralized Storage) generation module 

- Estimated duration: 4 weeks
- FTE: 4
- Costs: 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can upload personal params and generate books and library (as an Encrypted  Decentralized Storage) |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an article, that explains how a user can upload personal params and generate books and library (as an Encrypted  Decentralized Storage) |
| 1. | Frontend module: Personalized story tales : personal params upload module | We will create a frontend for submitting personal params by users (including book's subject. Technology Stack: HTML5, CSS3, BOOTSTRAP, React / Next JS |  
| 2. | Backend module: Personalized book generation module | We will create a Personalized book as NFT generation module for processing users’ personal params, storage in Postgres and generation of personalized books by special methodology. Technology Stack: Node JS, React / Next JS, JQUERY |  
| 3. | Decentralized Storage module: Books generation and storage as files in an Encrypted  Decentralized Storage (Library). Technology Stack: Arweave API | React 


### Milestone 2  — NFT based Memberships with Expiration and Renewable options

- Estimated duration: 2 months
- FTE: 5
- Costs: 35,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can use NFT based Memberships |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | We will publish an article, that explains how to use NFT based Memberships |
| 1. | NFT-based memberships generation module module: | (i) Library owner inserts the wallet adresses of family members (ii) NFT-based memberships are minted and sent to family members wallets (iii) expiry_date metadata set to standard (i.e. 1 month). Technology Stack: HTML5, CSS3, BOOTSTRAP, React / Next JS, Rust, Near API, Mintbase API |
| 2. | User autentication and contract check module | (i) User authenticates with the wallet (ii) Contract checks if the wallet holds one of the NFTs generated as access to this library (iii) Contract checks if expiry_date (metadata) > today Technology Stack: HTML5, CSS3, BOOTSTRAP, React / Next JS, Rust, Near API, Mintbase API |
| 3. | NFT based Memberships renewal module | (i) if expiry_date (metadata) > today < today (ii) user can purchase membership renewal (i.e. 1, 3 or 12 months) (iii) After purchase expiry_date (metadata) is updated. Technology Stack: Vue / Nuxt JS, React / Next JS, JQUERY, Rust, Mintbase API |  


## Future Plans

Our near future plans include:
- Add to Chainlib more personalized books and libraries
- Extend NFT based Memberships - more access control to exclusive content, events, communities, or services
- NFT based communities - authors that provide to their readers (community) NFT based Memberships, that offers early access to their works (it's the same as watching a TV series episode by episode)


## Additional Information :heavy_plus_sign:

Our team is a Near Foundation grantee. We received a grant for development of: Authorization Module, Copyright Registration and Transfer Modules.
