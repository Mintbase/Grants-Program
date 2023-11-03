# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** Infinfty
- **Team Name:** Enterprise Blockchain Labs LLC
- **Payment Address:** A NEAR account to where funds can be transfered (e.g. myproject.near) f7282dd09fb96525e6633f70ae00bc371a715d148896e6208318dedcc4aaa910
- **[Level](../README.md#level_slider-levels):** 2

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- If the name of your project is not descriptive, a tag line (one sentence summary). Enterprise SaaS platform enabling brands to pair their physical goods with NFTs.
- A brief description of your project. Include the NEAR protocol in our available blockchain list for our customers. Brands will be able to select the NEAR protocol as their preferred chain to deploy smart contracts, manage NFTs, and pair them with our NFC chips. To expand NEAR's presence we'll create a Shopify plugin where users will be able to directly link NEAR NFTs with NFC chips via the Shopify app.
- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem. Our platform will use the @mintbase-js/sdk library to deploy smart contacts and manage NFTs on the NEAR protocol.
- An indication of why your team is interested in creating this project. We believe the future of web3 is in Real World Assets. Our platform is one piece of the broader RWA ecosystem allowing brands to pair their inventory with NFTs.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
- Data models / API specifications of the core functionality
- An overview of the technology stack to be used
- Documentation of core components, protocols, architecture, etc. to be deployed
- PoC/MVP or other relevant prior work or research on the topic
- What your project is _not_ or will _not_ provide or implement
  - This is a place for you to manage expectations and to clarify any limitations that might not be obvious

**Overview:** The goal of this project is to expand the presence of the NEAR protocol into the digital-physical space. We will add the NEAR protocol to our list of existing EVM blockchains within our platform to enable users with a fast and secure blockchain to mint their NFTs. At the completion of this project, users will be able to mint NFTs on the NEAR protocol using the Mintbase SDK and pair their NFTs with either ARX or NTAG 424 tags. With the additional project scope of developing a Shopify plugin, we can expand the presence and legitimacy of the NEAR protocol and our application to the million-plus users on the Shopify platform to include the same NFT to NFC chip pairing.

**Technology Stack:** Bubble.io front-end and back-end. Python and Node.js functions interacting with the blockchain. React.js app to pair NFTs with Arx NFC chips.

**Launched App:** We've already launched our application and can be access [here](https://app.toinfinfty.com/). Feel free to reach out for credentials. 


### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem? Our project fits within the Mintbase utility ecosystem because we’re leveraging the codebase to create NFTs for digital-physical experiences. When brands pair NFTs with physical goods they unlock new use cases like digital certificates of authenticity, product-level CRM, loyal programs, and customer insights to strengthen the connection to their customers. 
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)? The target audience for our platform is any brand that’s creating a physical product. However, we see immediate applicability in the apparel, art, automotive, collectibles, footwear, furniture, games, luxury goods, and toy industries.
- What need(s) does your project meet? Sixty-four percent of customers want closer connections with their brands and fifty-seven percent of them are willing to choose a brand over a competitor if they have a close connection to them. With Digi-physical products, brands can strengthen their engagement with their customers in a new way. Brands can leverage the data provided by their customers’ wallets to create products or experiences that align better with their interests. NFTs will become the preferred method for market research as brands will make decisions based on their customers’ profiles and not sample populations.
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem? No.
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems? Yes. There are several competitors in the space like IYK, Legitimate, and Arianee. We differ from our competitors in the industries we’re able to target, the NFC chips we’re able to offer, our focus on full product life traceability, and with this grant be multi-chain. We originally built our platform on the Polygon network, but we’re interested in adding NEAR as this will set us apart from our competitors and leverage NEAR’s speed and security for enterprise customers.

## Team :busts_in_silhouette:

### Team members

- Name of team leader: Brett Blalock
- Names of team members: Sam White, Andrew Hart

### Contact

- **Contact Name:** Brett Blalock
- **Contact Email:** brett@toinfinfty.com
- **Website:** https://www.toinfinfty.com

### Legal Structure

- **Registered Address:** 6735 Crestwood Peninsula, Flowery Branch, GA 30542
- **Registered Legal Entity:** Enterprise Blockchain Labs LLC

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

Brett Blalock - Creator of the Infinfty platform. Developed the Python and Node.js functions creating the smart contracts and NFTs with the system.

If anyone on your team has applied for a grant at the Mintbase previously, please list the name of the project and legal entity here.

### Team Code Repos

- https://github.com/toinfinfty
- https://github.com/toinfinfty/functions

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/brettbl
- https://github.com/samwhiteiv

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/brettblalock/
- https://www.linkedin.com/in/samwhiteiv/

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),
- academic publications relevant to the problem,
- links to your research diary, blog posts, articles, forum discussions or open GitHub issues,
- references to conversations you might have had related to this project with anyone from the Mintbase Foundation,
- previous interface iterations, such as mock-ups and wireframes.

**Existing project:** We already developed a working platform with paid customers. Here's a [product demo](https://youtu.be/avmiv0NXNcI) of how users can mint and pair NFTs to physical products. You can access the app [here](https://app.toinfinfty.com). Please reach out an we'll provide you with credentials.

**Project Repo:** https://github.com/toinfinfty

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

- **Total Estimated Duration:** Duration of the whole project (e.g. 2 months)
- **Full-Time Equivalent (FTE):**  Average number of full-time employees working on the project throughout its duration (see [Wikipedia](https://en.wikipedia.org/wiki/Full-time_equivalent), e.g. 2 FTE)
- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in USN, and potentially NEAR and nUSDC, the amount will be calculated according to the exchange rate at the time of payment.

### Milestone 1 — Implement Mintbase Modules

- **Estimated duration:** 26 Hours
- **FTE:**  1
- **Costs:** 5,200 USD

| Number | Deliverable | Specification |
| -----: | :---------- | :------------ |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide **inline documentation** of the code that explains how a user can leverage the Mintbase SDK to create smart contracts and manage NFTs. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Mintbase module: Create Wallet | Create a NEAR wallet for users using the near-api-js library. Functions include nearConnection, createAccount |  
| 2. | Mintbase module: Connect Wallet | Connect a user's NEAR wallet for NEAR protocol interactions using the mintbase-js/auth and mintbase-js/react libraries. Functions include KeyPair, InMemoryKeyStore, KeyStore, WalletContextProvider |  
| 3. | Mintbase module: Deploy Smart Contract | Deploy a smart contract on the NEAR protocol using the mintbase-js/sdk library. Functions include DeployContract Args, deployContract, AddMinterArgs, and addMinter |  
| 4. | Mintbase module: NFT Management | Manage NFTs on the NEAR protocol using the mintbase-js/sdk library. Functions include execute;  mint, burn, list, delist, buy, transfer functions; and related "arg" functions |  


### Milestone 2 — Enable NEAR NFT & Arx Chip Pairing

- **Estimated duration:** 15 Hours
- **FTE:**  1
- **Costs:** 3,000 USD

| Number | Deliverable | Specification |
| -----: | :---------- | :------------ |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide **inline documentation** of the code that explains how a user can leverage the LibHalo library link NEAR NFTs with Arx chips. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 0f. | Code Base | We will fork the Halo verify app to speed up development: https://github.com/kong-org/halo-verify-web/tree/master |
| 1. | Arx module: Connect Wallet | Connect the user's NEAR wallet. Functions include deviceStore |
| 2. | Arx module: Scan Chip | Scan the Arx chip to retrieve the chip's public key. Functions include deviceStore |  
| 3. | Arx module: Provide NFT Details | Pass the NFT details to the chip. Functions include registerStore |  
| 4. | Arx module: Sign Block Hash | Sign a recent block hash from the NEAR protocol using the chip |  
| 5. | Arx module: Generate Signature | From the user's wallet, generate a signature of several parameters: including the pre-calculated IPFS hash of the media along with the name, description, device_id, the signature generated by the device and block hash used to generate the signature and the address of the user's wallet | 
| 6. | Arx module: Post Data | POST this data to the KONG bridge server which will add the media and tag it will the information from the previous step on Arweave. |


### Milestone 3 — Shopify App

- **Estimated duration:** 29 Hours
- **FTE:**  1
- **Costs:** 5,800 USD

| Number | Deliverable | Specification |
| -----: | :---------- | :------------ |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide **inline documentation** of the code that explains how a user can leverage the LibHalo library link NEAR NFTs with Arx chips. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.) |
| 1. | Shopify module: Create Webhook Listener | Create a Shopify event listener using an Amazon EventBridge. |
| 2. | Shopify module: Products | Create a new product in the Infinfty platform and a new NEAR smart contract when a product is created in Shopify. |  
| 3. | Shopify module: Inventory | Create or remove inventory in the Infinfty platform and create or burn NEAR NFTs when inventory is created or removed in Shopify. | 
| 4. | Shopify module: UI | Create a metaobject boolean field indicating the whether the Shopify event listener should create a smart contract and NFTs for the product. | 
| 5. | Shopify module: Customer Verification | Create a Shopify GraphQL request that checks the order ID and email address to verify the customer minting an NFT purchased the product. | 

### Milestone 4 — Enable NEAR NFT & NTAG 424 DNA Chip Pairing

- **Estimated duration:** 7
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | :---------- | :------------ |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide **inline documentation** of the code that explains how a user can leverage the LibHalo library link NEAR NFTs with Arx chips. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.) |
| 0f. | Product | We will leverage an exsiting software toolkit created by an NFC developer to speed time to market. https://nfcdeveloper.com/ |
| 1. | Server Set Up | Set up a dedicated server to manage chip initialization and validation. |
| 2. | Chip Initialization | Leverage the NFC software toolkit to link chip id to NFT in the Infinfty database. |  
| 3. | Chip Validation | Leverage the NFC software toolkit to verify legitimate interactions with the chip. |   

...
## Future Plans

Please include here

- how you intend to use, enhance, promote and support your project in the short term, and
- the team's long-term plans and intentions in relation to it.

In the short term, the addition of the NTAG 424 tag encoding and Shopify integration will open our platform up to more use cases and a broader audience. With the NTAG 424 tags, we’ll be able to create tailored and secure digital-physical experiences for industries like apparel, automotive, art, collectibles, furniture, gaming, music, and toys. These markets combined together are worth $3.75 trillion globally and $871 billion in the USA. We believe there will be substantial interest in the next several years in digital-physical experiences and capturing just 1% of these markets would represent a substantial opportunity to grow the size of the NEAR ecosystem. We plan to support the projects Chris Ghent is leading with Barrett Jackson, Brooklyn Coachworks, Ferrari, and more to create unique digital-physical experiences with NFT and NFC technology.

In the long term, we'd like to expand our offerings to support the full traceability of products throughout the supply chain. This will enable brands to follow their products from cradle to grave, unleashing new insights into the lifespan and useability of their products. We see the NEAR protocol as an integral part of this implementation as blockchains will act as the central source of truth for disparate systems along a product’s lifecycle. We see this use case playing out in multiple industries where provenance and prior history are important. For example, we could see this technology replace incumbents like Carfax in the automotive industry as the data captured will be richer than what’s currently available.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc. Through HUMANS, a growing collective of web3 founders, and NEAR Foundation’s former Head of Brand Partnerships, Chris Ghent.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.

Our platform already has a couple of users with several more starting early next year. With the support of the Near Foundation, we believe we can offer our customers a secure and quick blockchain to mint their NFTs. We really want to develop a platform that can support enterprise clients which is why we originally started on Polygon. We think the speed and security of NEAR’s blockchain will enable us to attract more enterprise clients to our platform which is why we’re excited about this opportunity.
