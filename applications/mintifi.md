# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** MintiFi
- **Team Name:** Wize
- **Payment Address:** TBD
- **[Level](../README.md#level_slider-levels):** 1, 2 or 3

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:



### Overview

**MintiFi** is a NFT marketplace for artists, creators and those who are looking to utilize the power of NFT digital assets to curate and montetize their creative talent and IP. The roadmap is to provide support to local museums and historical places to leverage NFTs to increase memberships and renuve.

![image](https://user-images.githubusercontent.com/52714019/210244063-851cb3bb-0f74-48c1-ad51-63fc036f20f7.png)


There are several challenges when it comes to building a NFT marketplace for Indian users. These are related to lack of technology, knowledge, awareness and clarify on regulations around digital and crypto payments for the NFT sales. We connected with several artists, existing marketplace creators and domain experts to understand these challenges. This provided us with the insights into what and how the marketplace should be conceptualized. Overall, from a creators viewpoint, due to lack of mature marketplace focused on India themes, most of them end up not using NFTs. A few who can figure out the tech + twitter + discord are able to succeed in limited ways. 

In addition, we have a proven record of making NFT transactions with local payments and onchain transactions in parallel, which makes it easier and convenient for mainstream audience to get onbaorded faster. We have used this succesfully over 2022. We have worked with academics, event organizers to issue NFT creatives across various engagements and hence feel confident that we can now buidl and deliver this project. This is also our first grant proposal.

This project fits well with the Mintbase ecosystem, and we feel that we can utilize the combination of Mintbase NFT solutions and Near Protocol to deliver both the digital assets and transactional capability needed for the marketplace. For example, Mintbase docs already provide the Royalty splitting for sale items, which is a key requirement for building the marketplace. We already have built our model around this (example below), where c8a7adde6c244348a9229c9ee4fd69cd2 is the creator and recives a 10% commision on every sale.

![image](https://user-images.githubusercontent.com/52714019/210233607-4cb64020-0bbd-42c3-a913-67b6da340dd2.png)

reference: https://snbx.minti.fi/assets/a2b16b1343d54f5e8124d04d2352d7f06


The main motivation for this comes from our recent discussions with various stakeholders in the ecosystem, including existing marketplace owners. A few other reasons for us to consider this project are:
- provide localized version and support for a themed NFT marketplace 
- easy onboarding, content creation and monetization a plug and play system
- provide a sandbox setup in parallel, which can be used for training workshops and educational activities 
- provide a better alternative to USD x INR scenario and complicated routing challenges with existing marketplaces and replace with INR x INR || NEAR as settlement
- provide necessary support around local taxation guide


![image](https://user-images.githubusercontent.com/52714019/210235144-9b4c1dd9-3f9f-4ec4-907e-a9b585063bd9.png)



### Project Details

We expect the teams to provide as much details on our project's expected final state. We look forward to your questions and feedback, which can help us succeed in delivering a useful and viable project:

#### Mockups/designs of any UI components 
- Our initial mockup of the sandbox is already live at the url: https://snbx.mintif.fi
- An example of the asset listing is here: https://snbx.minti.fi/assets/a2b16b1343d54f5e8124d04d2352d7f06
- The sandbox is a web2 modelled system of how users can create collections, add artwork and media, set price and royalty and list the items on the marketplace. 
- The visitors can review and then choose to buy the artwork and once payment is completed, the application takes care of routing the payment and necessary splits for the current owner and fees to the creator. 
- Similarly, we will add marketplace commission and the tax deduction transfers, so the creators do not have to worry about complex finances andfocus on their core creative tasks.

**Dashboard**
![image](https://user-images.githubusercontent.com/52714019/210245975-919b81ee-1e66-4183-b52e-699efed85243.png)

**Market - Listing + Detailed Views**
![image](https://user-images.githubusercontent.com/52714019/210246050-ae8ed5ed-7822-4ba0-8924-90d331c43647.png)
![image](https://user-images.githubusercontent.com/52714019/210246201-0ca0d84d-a72f-4615-93c8-a5fdb1219542.png)

**Store**: Contains both user collections (for creators) and list of owned artworks
![image](https://user-images.githubusercontent.com/52714019/210246294-f0f0cf24-f28e-464a-a275-2c74a00461f5.png)

**Transactions** - all transactions including buy, sell, royalty payments and itemized for clarity
![image](https://user-images.githubusercontent.com/52714019/210246489-9d541dad-c1ce-4b6d-9977-1d4d70186ece.png)



***
#### Data models / API specifications of the core functionality
We have anticipated the following core modules:
- users - users will be individual accounts, which can be both creators (sellers) and buyers and facilitators. 
- teams - we anticipate the usage of teams, which brings in users of different skills together. these can be multiple creators working together, and/or creators + community builders / facilitators working on a common project
- collections - collections are the individual projects and can include from 1 to many artworks (media/content)
- artwork / media - these are the individual saleable items (not considering fractional at this step)
- tokens - users have the option of minting or lazy minting and hence a separate data model is assigned to handle minting and subsequent transfers. this handles the ingtegration with mintbase + near
- smart contracts / compute - this handles the smart contracts calls for the marketplace.
- storage - this includes both decentralized and cerntralized (backup) storage for media content, token-data and other application data
- transfers - this handles all transactions on hte application. the transactions are further categoried for users, artwork/item and for the entire marketplace
- alerts - this modules handles all notifications and user alerts for various activities on the application
- community forum - this includes both token-gated forums and open forums for people to get and provide help to other creators and application users

![image](https://user-images.githubusercontent.com/52714019/210238763-a2699790-ad2d-4ad3-8607-55847799f305.png)


***
#### An overview of the technology stack to be used
Our ideal environment setup is micro-services and micro-apps delivery model. This allows us to work on multiple components at the same time, and fastrack development, delivery and maintenance. 
- Tech is NodeJS + JS
- Compute - we use a combination of serverless for most of application calls and some dedicated VM resource for contract deployment and other specific tasks
- Storage - this is a combination of decentralized storage via IPFS /Filecoin and S3
- Database - we have relied on using MongoDB for application data
- Alerts - notifications systems, SES / SNS / Mobile and Whatsapp Alerts
- front end - this is a combination of reactjs and nextjs (for SEO and Social-Sharable content)
- mobile applcation - react native (planned for phase 2
- Transactions - this is built individually with existing finance service providers with our custom routing
- other web3 components as needed to achieve the desirable user experiene

Our project setup across various modules, with each its own storage + compute for on-demand scalability

![image](https://user-images.githubusercontent.com/52714019/210242024-68522775-0771-44a2-99cd-1a6bcdcf908a.png)



***
#### Documentation of core components, protocols, architecture, etc. to be deployed
- **Gitbook**: The documentation of all modules will be maintained with a dedicated site which has already been whitelisted here - https://docs.minti.fi/. The content will be added as modules are being developed. 

- in addition, this will also list the smart contract, standards on modelling NFT data (many people do not know about attributes), how to apply IP and Licensing rights to the NFT etc. 

- **Github** The source code and smart contracts will be maintained here for public review, audit and comments. 



***
#### PoC/MVP or other relevant prior work or research on the topic
- PoC Link: https://snbx.minti.fi


![image](https://user-images.githubusercontent.com/52714019/210243398-55add347-8625-451f-873e-3e41c2a8d99d.png)
![image](https://user-images.githubusercontent.com/52714019/210243409-f08d3e3b-bd45-4c22-b29c-85e39f807814.png)
![image](https://user-images.githubusercontent.com/52714019/210243424-98d1b9bf-0686-4364-9cb9-fd026f2fc923.png)
![image](https://user-images.githubusercontent.com/52714019/210243446-e443e6ff-7249-47f4-b45d-73a728143d83.png)
![image](https://user-images.githubusercontent.com/52714019/210243467-ffd1ca66-3c84-4c7c-b64b-eff0a95c6d94.png)

- Considerations

![image](https://user-images.githubusercontent.com/52714019/210243941-9970426c-1645-42fe-974a-c5e304d1ef26.png)
![image](https://user-images.githubusercontent.com/52714019/210243954-cd7fe670-a072-4e8a-a2f7-eb224c57ee3c.png)
![image](https://user-images.githubusercontent.com/52714019/210243964-4e3484b9-fc3e-4831-ab10-cf643fc5ba36.png)
![image](https://user-images.githubusercontent.com/52714019/210243974-a7face94-8397-4e26-a248-645da780dd1b.png)
![image](https://user-images.githubusercontent.com/52714019/210243982-8967388a-5897-42c6-b0d3-2dbd1960a0e5.png)
![image](https://user-images.githubusercontent.com/52714019/210243990-09358717-783d-408f-8370-5cdefac42803.png)


***
#### What your project is _not_ or will _not_ provide or implement
- while we will make provisions for support for legal and tax related topics, this is not a service or a feature that will be provided. these will be managed via the community forums where users will be able to ask and find answers
- the delivery model is product-based and not service based, hence we do not intend to provide niche customizations and technology development. however, if certain features add value to the entire community / users, we might consider adding those to the development roadmap. 
- we do not provide manpower or other resources for custom / software development
- 

***
This is a place for you to manage expectations and to clarify any limitations that might not be obvious



### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

#### Where and how does your project fit into the ecosystem?
- the project is a great fit with Mintbase which provides the right NFT Infrastructure to build NFT centric marketplace applications. Mintbase + 

### Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
- our first and initial target user base is artists and creators in india, which provides a rich and diverse cultural
- our second target is museums, NFT galleries and historical sites which can utilize NFT and our marketplace for revenue and promotions
- the buyers are made up of indians living in and outside of india, tourists and vistors
- this will be done in phases

#### What need(s) does your project meet?
- the first aim is to provide localization of technology for mainstream benefit. existing marketplaces do not cater to local language and demographics within India. 
- typical marketplaces are storefronts, whereas mintifi brings the community + forum + commerce together. the success of amazon is mainly attributed to the social component where buyers and sellers can interact on the same platform without having to goto discord, slack or twitter. why should a NFT markeplace exist in 3 other platforms instead of being one
- finally it addresses the operational challenges around payments, tax and legal support to provide confidence and build trust with the user base

### Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
- NFT marketplaces are not new concept, and there are several examples out there. what we are doing is to first address the user challenges from an adoption and usage point of view than driving sale metrics of jpegs. 
- secondly, our focus is on promotion local art and creative forms in addition to the digital artworks. this will need additional components and integrtions such as postage and logistics, however, we have those solutions avalable to deploy it at scale and good unit economics



## Team :busts_in_silhouette:

### Team members

- Saurav Raaj, Technology
- Smriti Chaudhry, Operations and Business
- Chandan K, Blockchain Lead
- Satya Verma, Full Stack Developer
- Rohit Sujan, Sales and Community
- 2x Industry Mentors and Advisors
- 1x Legal Advisor
- 

### Contact

- **Contact Name:** Saurav Raaj
- **Contact Email:** saurav.raaj@gmail.com
- **Website:** https://snbx.minti.fi

### Legal Structure

- **Registered Address:** 160 ROBINSON ROAD, #14-04, SINGAPORE BUSINESS FEDERATION CENTER
- **Registered Legal Entity:** NIMBL Solution Pte Ltd

### Team's experience

The team is well balanced in terms of technology, business and commercial and legal aspects of managing marketplace business. Project is led by Saurav Raaj, who has been building in web3 for several years including multiple wins at various hackathons both in web3, web3 and fintech projects. He has built Wize - tokenization solution with applications in pharma, education, supply chain and real estate domains. he has been invited by various organizations for seminars, panel discussions including blockchain copmanies suchas Algorand and Hyperledger.

![image](https://user-images.githubusercontent.com/52714019/210250394-df7ce95e-4ac4-4bbd-ae1e-b26bf00d4d8d.png)

Smriti is very active in the web3 space with several community meetups and panel discussions. 

![image](https://user-images.githubusercontent.com/52714019/210250646-10a15d8e-7984-4f57-b0e6-1ec10af9b77e.png)
![image](https://user-images.githubusercontent.com/52714019/210250677-2a60a5fb-38e2-469b-83d3-4fee1c29a0f9.png)


### Team Code Repos

- [MintiFi - Sandbox](https://github.com/saurav2799/mintifi-snbx)
- [Route Finance - Payments Infra](https://github.com/saurav2799/routebz-snbx)


Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/saurav2799
- https://github.com/ChandanKumar665
- https://github.com/satya-pverma


### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/sauravraaj
- https://www.linkedin.com/in/smritichaudhry

## Development Status :open_book:
We have started building the sandbox (testnet) version of the marketplace. The web2 setup is nearly complete and we are now looking to NFT and Smart Contract development as next steps.

**Project Link** https://snbx.minti.fi/

**Research, Information, Publications**.  
Following are few references: 
https://www.linkedin.com/feed/update/urn:li:activity:7011624949211688960
https://www.linkedin.com/feed/update/urn:li:activity:7009382146511437824
https://www.linkedin.com/feed/update/urn:li:activity:7008727860597194752
https://www.linkedin.com/feed/update/urn:li:activity:7006651757745385473
https://tclf.in/2022/01/16/nfts-present-legal-challenges-and-the-future-ahead/
https://scroll.in/magazine/1020893/in-india-the-world-of-nfts-suffers-from-the-same-inequalities-as-the-world-of-traditional-art
https://www.moneycontrol.com/news/business/nft-platforms-have-their-task-cut-out-with-tds-coming-into-effect-brace-for-challenges-8783771.html

## Development Roadmap :nut_and_bolt:

This section is breakup the development roadmap with milestones and deliverables.  

### Overview

- **Total Estimated Duration:** 4 months
- **Full-Time Equivalent (FTE):**  10-11
- **Total Costs:** 50,000


![image](https://user-images.githubusercontent.com/52714019/210263068-31630272-5a5b-4214-b235-4f446c619a6d.png)



### Milestone 1 Example — Implement Mintbase Modules

- **Estimated duration:** 1 month
- **FTE:**  2
- **Costs:** 8,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Mintbase module: X | We will create a Mintbase / NEAR module that will... (Please list the functionality that will be implemented for the first milestone) |  
| 2. | Mintbase module: Y | We will create a Mintbase / NEAR module that will... |  
| 3. | Mintbase module: Z | We will create a Mintbase / NEAR module that will... |  
| 4. | NEAR chain integration | Modules X, Y & Z of our custom chain will interact in such a way... (Please describe the deliverable here as detailed as possible) |  


### Milestone 2 Example — Additional features

- **Estimated Duration:** 1 month
- **FTE:**  1
- **Costs:** 4,000 USD

...
## Future Plans

Please include here

- how you intend to use, enhance, promote and support your project in the short term, and
- the team's long-term plans and intentions in relation to it.


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.
