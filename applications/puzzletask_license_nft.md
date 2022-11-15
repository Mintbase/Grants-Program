# Mintbase Grant Proposal



- **Project Name:** Puzzletask NFT Client Certification
- **Team Name:** Puzzletask
- **Payment Address:** puzzletask.near
- **[Level](../README.md#level_slider-levels):** 2


## Project Overview


Please provide the following:

Puzzletask - Blockchain Client Certification
- Client NFT Certification
- A brief description of your project.
- Puzzletask is a no-code platform for Food Safety and Traceability Management serving Hospitality and Food Retail Sectors towards supply chain optimization. We cater to suppliers, hotels, retailers, and end consumers to encourage interaction in a data exchange micro-economy that incentivizes the sharing of valuable data to optimize the supply chain. Our solution has two revenue streams, one based on a SaaS Module and the other based on a Data Monetizing module.
In orther to slowly bring Blockchain technology for such a traditional sector as Food retail and Hospitality, we are launching our NFT Certificate to highlight the customers that are getting into the digital transformation path. This certification approach, from customer certification to license management, can be used in many ways in our business model. The solution will be based on the implementation of an ERP connected NFT Minter that will be available for the community in a form of a library.

- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.
  - We are bringing a real life use case for NFTs as a way to certify our customers and provide them with a simple first contact with blockchain and NEAR wallet.
- An indication of why your team is interested in creating this project.
  - We aim to highlight the customers that are buying our tool for food safety and traceability management.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- [Mockups/designs of any UI components](https://www.figma.com/proto/LQa40vQvo3WsaHbvMzAXK1/Puzzletask-NFT-certification?node-id=3%3A12&scaling=min-zoom&page-id=0%3A1&starting-point-node-id=3%3A12)
- Data models / API specifications of the core functionality:
  - We propose to develop a javascript library based on MintbaseJS and an extended version of NEP-171 that will be used to mint, transfer and burn an NFT with a set of attributes dependant on third party authorization. This authorizations will be granted by an external API via an Oracle. 
  
  - After this project implementation, the library will be used by the Puzzletask SaaS to mint a certificate with a set of relevant attributes according to the permissions set by Puzzletask. The transferability of the NFT will also be authorized by Puzzletask API according to the target wallet address defined by the user. NFT Burn function will be available to the owner and won't be dependant on any Puzzletask permission. 

  - After this first project, there will be a set of pages in Puzzletask SaaS certificate workflow that will use this library to mint, transfer and burn for the owner. There will be a public page for NFT existence verification and consequent data retrieval.

- An overview of the technology stack to be used:
  - Javascript;
  - React;
  - MintbaseJS;
  - near-sdk-rs

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
  - The project is kind of a new category among the NFT's and we defined has a Business Utility NFT - Under Puzzletask context, after this project implementation, we will develop and provide a first interaction with blockchain for businesses in the food industry. By implementing our project on this kind of market players we will allow them to start to accept NEAR because it will be mandatory to create a wallet to mint and manage the NFT;
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
  - Food Operators such as hotels, retailers and restaurants. We are planning to implement a feedback solution with consumers in a later version of these project.
- What need(s) does your project meet?
  - NFT Certification to promote the companies that are reducing their food waste and paper by using our digital food safety and traceability tool. 
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - There isn't a similar project on NEAR Protocol targeting the Supply Chain Sector and even considering other ecosystems, there is a lack of tradicional business adoption and this could a great oportunity for Mintbase and Near to leverage their market positioning. Puzzletask will be a Supply Chain Optimization company and already have a fully functional SaaS for hotels and food retailers.

## Team

### Team members

- Adalberto Rodrigues
- Pedro Rodrigues
- Sandro Leitão
- Vicente Almeida
- Esfandiar Lagevardi

### Contact

- **Contact Name:** Adalberto Rodrigues
- **Contact Email:** arodrigues@puzzletask.com
- **Website:** https://puzzletask.com

### Legal Structure

- **Registered Address:** Rua António Alçada Batista, nº2, Bloco A, 5ºa, Lisboa 
- **Registered Legal Entity:** 

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Mintbase previously, please list the name of the project and legal entity here.

In terms of technical knowledge the team doesn't have experience in RUST or Solidity but that demonstrates the capabilities to successfully create, develop, deploy and maintain applications with sustainable business models. 

The development team accumulates experience in Backend and Frontend Development, Systems and Data Architecture, DevOps and SRE.
It has been leading software development and maintenance in portuguese top companies and projects always using the best practices in the industry. We may highlight SAPO Adwords that served millions of ads requests per day, MEO Cloud - Altice Labs that served petabyte scale data to near half a million users, the foundation and launch of Advertio's technical team, processes, software and infrastructure and BPstat - Banco de Portugal's statistical portal that manages millions of time series that are explorable by the public via an extensive set of tools and related content.

On the project/ product management and tokenomics, the team presents a senior level regarding the capability to create a product from 0 to scale with 10 years of experience in tech projects as well as arround 8 years in tokenomics desing, treasury manament strategies and security audits.


### Team Code Repos


Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/car3ca
- https://github.com/vice2al

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/adalbertorodrigues/
- https://www.linkedin.com/in/pgr42/
- https://www.linkedin.com/in/vicente-moitinho-de-almeida/
- https://www.linkedin.com/in/sandro-leitao/
- https://www.linkedin.com/in/esfandiar-benjamin-lagevardi-9b631086/



## Development Status


### Overview

- **Total Estimated Duration:** 5 months
- **Full-Time Equivalent (FTE):**  1,5 (Team of 5 members)
- **Total Costs:** 20.000 USD

### Milestone 1 — User session connected NFT minter library development

- **Estimated duration:** 3 months
- **FTE:**  1,5
- **Costs:** 12.000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 1. | Mintbase module: User session connected NFT minter library | We will create a Mintbase / NEAR module that will able developers to use a NFT for certifications on Near Blockchain. The library will be developed in javascript based on MintbaseJS that will be used to mint, manage and subsequently retrieve an NFT with a set of attributes for a connected wallet. This will be a beta version. |  



### Milestone 2 — Library final version

- **Estimated Duration:** 2 month
- **FTE:**  1,5
- **Costs:** 8.000 USD



| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0b. | Documentation | We will provide general library usage documentation. |
| 1. | Documentation for library integration | We will write documentation with an example of this library integration. This example will try to demonstrate a real use case with the following features / workflow: client authentication and wallet connection to mint, transfer and burn a certificate dependant on third party authorization; |  
| 2. | NEAR chain integration | The Module will interact with the Near Wallet and chain.|
| 3. | Mintbase Module: User session connected NFT Minter library final version| We will create the final version of the library.|




## Future Plans

Please include here

- how you intend to use, enhance, promote and support your project in the short term, and the team's long-term plans and intentions in relation to it.
  
We will integrate the library into the Puzzletask SaaS. After the client authentication and after wallet connection to mint a certificate with a set of relevant attributes. There will be a set of pages in Puzzletask SaaS certificate workflow that will use this library to mint, transfer and burn for the owner. There will be a public page for NFT existence verification and consequent data retrieval. 

By implementing this project we open the possibility to allow hotels to start to receive payments using $NEAR. This first approach will be important to start to educate our customer base about blockchain. This fact is crucial to allow our migration for a business model based on a data monetization in a micro-economy that incentivizes the sharing of valuable data to optimize the supply chain.

## Additional Information

**How did you hear about the Grants Program?** 

Personal recommendation (Oscar Andrade)

**Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:**

- We developed a fully functional SaaS for food safety and traceability digitalization used by brands such as Four Seasons Fairways, Mother Burguer, Quiosques de Lisboa and Intermarché.
