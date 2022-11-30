# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** CP loyalty system
- **Team Name:** CoinPipe
- **Payment Address:** roke.near
- **[Level] 2

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- If the name of your project is not descriptive, a tag line (one sentence summary).
CoinPipe's NFT based loyalty system 
- A brief description of your project.
CP Loyalty system is a service for managing NFT based loalty programms for small and middle size businesses. 
Direct use case: the owner of the business wants to run loyalty system for its customers and at this point the owner of the business goes to CoinPipe's loyalty system, generates NFT's through MintBase and sends to the customers. The customers during the purchase are burning or showing their NFT's and getting royalties.
- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.
1. All the NFT's woud be run on MintBase
2. The amount of art works would be increased by the potential possibility to be used as a loyalty  NFT's and possibility to be bought by the owners of the businesses. 
3. We would increase recognition of MintBase by promoting MintBase as a minting platform and a marketplace
4. We would increase the number of NEAR wallet users and the NEAR token turnover value, cause the users would need to top up the wallet to burn NFT's 
5. The loyalty system would show to bigger corporation how the NFT's could be used in a real life and this might make them to make a step to cooperate with NFT as a technology. And the contact going in this case would be MintBase

- An indication of why your team is interested in creating this project.
We are working in the NEAR ecosystem more than 1.5 years and have created several projects as https://roke.biz, https://roke.to and https://coinpipe.finance.
We have a stable team of developers who really believe in NEAR blockchain and ecosystem. We've been on NEARCON with our beer truck and everybody on the event was able to buy beer and pay with NEAR. Probably they might be able to burn NFT's for the beer 😍.
so here are dry facts:
1. We think that loyalty systems now are pain in the ass (sorry)
2. They are always incomfortanble and you always need to store the paper or plastic cards (bad for the ecology)
3. People dont have any possibilities to use their loyalties somewhere else. With NFT you always have a various use cases
4. We know directly one of the owners of the business (14 bakeries in Austria), who would like to take this as a pilot project and we want to make it real.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
https://drive.google.com/file/d/1N8_Yd7KwrWYIVyKbZbUVL19ZbDI0HAFm/view?usp=share_link 
Here are two screens explaining high level user flow and NFT management through loyalty campaigns. 
- Data models / API specifications of the core functionality
The main business models are User and Organization
The project also contains 3 endpoints:
 - POST /api/v1/users (create a user)
- POST /api/v1/tokens (to get a JWT token)
- POST /api/v1/organization (create an organization)
- An overview of the technology stack to be used
- Golang as a simple, fast and reliable language which used a lot for blockchain projects and is designed to be used as a bullet proof server language. 
- Libraries: chi router, go-validator, sqlx, jwt
- Database: postgres
- Docker to containerize the app and deploy to the cloud.
- Documentation of core components, protocols, architecture, etc. to be deployed
The architecture of the project as simple as possible. The choice has been made to the direction of simplicity. 
The project consists of main.go which is the entry point, handlers which handles incoming http requests and db service.
- PoC/MVP or other relevant prior work or research on the topic
As a proof of concept would be taken an app where our user can mint NFTs through the MintBase and then organize promo loyalty companies for its customers, sending NFTs to the loyal customers.




### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
The project would be as a bridge between an offline business and NFT usage in daily life
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
Designers
Companies, who run their business in crypto
Small and middle size businesses
- What need(s) does your project meet?
Tehcnical advises 
Funding
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
There no related projects yet.
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems?

## Team :busts_in_silhouette:

### Team members

- David Virabian. Product manager. 5 years in web3, experience working with a high load fintech apps with 10M MAU users.
- Emil Khamdamov. Backend developer. 7 years in development, strong experience in development of stable infrastructures
– Andrey Zhevlakov. Frontend developer. 7 years in IT, development of HERE wallet, development of CoinPipe
– Alex Gorshkov. Backend developer. 8 years in IT, 5 years in blockchain. Development of CoinPipe and multiple fintech products.

### Contact

- **Contact Name:** David Virabian
- **Contact Email:** vdavid3476@gmail.com
- **Website:** https://coinpipe.finance

### Legal Structure

- **Registered Address:** Mosergasse, 5/15, Vienna, Austria
- **Registered Legal Entity:** CoinPipe

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

The team has experience in all of the technical and business fields which might be needed during development of NFT based loyalty system. 
Each of the members has proven experience working both with crypto currencies & loyalty systems.

Alex Gorshakov has developed https://coinpipe.finance, crypto acceptance solution which is now used in several places in Lisboa to accept crypto currencies 
David Virabian has developed a telegram bot https://nearbot.im for sending NEAR tokens to the people even if they don’t have NEAR wallet. The project was pilot tested during NearCon
Emil Khamdamov has developed a backend for loyalty system in https://yallamarket.ae
Andrey Zhevlakov has developed a frontend for https://herewallet.app


### Team Code Repos

- https://github.com/11me/loyalty-system

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/11me/ Emil Khamdamov

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/david-virabian-9791491a7/ David Virabian
- https://www.linkedin.com/in/emil-khamdamov-a2656722a/ Emil Khamdamov 
- https://www.linkedin.com/in/andrey-zhevlakov-1609001a9/ Andrey Zhevlakov

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs]
- https://github.com/11me/loyalty-system

## Development Roadmap :nut_and_bolt:

This section should break the development roadmap down into milestones and deliverables. To assist you in defining it, we have created a document with examples for some grant categories [here](../docs/grant_guidelines_per_category.md). Since these will be part of the agreement, it helps to describe _the functionality we should expect in as much detail as possible_, plus how we can verify and test that functionality. Whenever milestones are delivered, we refer to this document to ensure that everything has been delivered as expected.

Below we provide an **example roadmap**. In the descriptions, it should be clear how your project is related to Mintbase. We _recommend_ that teams structure their roadmap as 1 milestone ≈ 1 month.


For each milestone,

- make sure to include a specification of your software. _Treat it as a contract_; the level of detail must be enough to later verify that the software meets the specification.
- include the amount of funding requested _per milestone_.
- include documentation (tutorials, API specifications, architecture diagrams, whatever is appropriate) in each milestone. This ensures that the code can be widely used by the community.
- provide a test suite, comprising unit and integration tests, along with a guide on how to set up and run them.
- commit to providing Dockerfiles for the delivery of your project.
- indicate milestone duration as well as number of full-time employees working on each milestone.
- **Deliverables 0a-0d are mandatory for all milestones**, and deliverable 0e at least for the last one. If you do not intend to deliver one of these, please state a reason in its specification (e.g. Milestone X is research oriented and as such there is no code to test).

> :zap: If any of your deliverables is based on somebody else's work, make sure you work and publish _under the terms of the license_ of the respective project and that you **highlight this fact in your milestone documentation** and in the source code if applicable! **Teams that submit others' work without attributing it will be immediately terminated.**

### Overview

- **Total Estimated Duration:** 2 months
- **Full-Time Equivalent (FTE):**  FTE
- **Total Costs:** 30,000 USD

### Milestone 1  — Technical preparation, project setup

- **Estimated duration:** 1 month
- **FTE:**  3
- **Costs:** 15,000 USD


| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish 2 articles which explain the purpose of the product and the release content. Also we would publish what was achieved during the grant program and mention MintBase sources 
| 1. | Mintbase module: X | We will create a MintBase module that will make it possible to mint files artworks and create NFT’s out of the MindBase, but using the MintBase API. So as an open-source solution, everyone in the MintBase ecosystem would be able to use our module to use inside their products   |  
| 2. | Mintbase module: Y | We will create a MintBase module that will make it possible to mint multiple artworks with 1 button click to make possible mass NFT generation and mass sending over the list of the receivers.
| 3. | Exploring all of the MintBase documentation
| 4. | Local development setup: setting up a MintBase environment
| 5. | Backend development and MintBase integration
| 6. | Backend testing  
| 7. | Frontend development 
| 8. | Design preparation
| 9. | Infrastructure setup: server setup for demo purposes & CI/CD
| 10. | UX development, UX testing, PoC testing, customer development

### Milestone 2 – Release & Testing. Getting first feedback

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 11. | Service stabilization, backend bug fixing
| 12. | Setting up a first pilot user (Austrian bakery chain) and testing out creating of first NFT royalties 
| 13. | Re-working the UX based on the user feedback
| 14. | Frontend bug fixing, improvements
| 15. | Improvements of MVP features: managing the customers list, NFT generation and account setting
| 16. | New tests and code refactoring
| 17. | Improvement and adding documentation 
| 18. | Adding F.A.Q & Terms of Use and legal basement

...
## Future Plans

Please include here

- how you intend to use, enhance, promote and support your project in the short term
We are in a really close relationships with http://www.baeckereibauer.at Bakery chain in Austria. There are 14 shops where we would test out our loyalty system. 
We already have a business commitments that we would put our loyalty system in use at the moment when it will be ready.
So the plan is to develop loyalty system and test in a real environment. Then we plan to figure out some changes and bugs and go to bigger market, attract companies and do promo companies.
- the team's long-term plans and intentions in relation to it.
We plan to develop a platform where NFT creators would be able to find a different ways to use their NFT’s with a help of small and medium size businesses. 
So the art creators would create and sell NFT’s, the business would buy them and use as a loyalty cards, tickets to the museums, concerts (etc) and the MintBase would be a basement for this bigger use case. CoinPipe would provide a working environment for the businesses to manage their assets to release their promo campaigns 


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
https://coinpipe.finance
https://nearbot.im
https://herewallet.app
https://yallamarket.ae
Work which was already done for this grant application is here https://github.com/11me/loyalty-system

