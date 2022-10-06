# Mintbase Grant Proposal

- **Project Name:** Integration with [Magnolia CMS](https://www.magnolia-cms.com/)
- **Team Name:** Formentor Studio
- **Payment Address:** d86142f619a1c254ca4c2fec5c7143afb47754a89451bf9246bc7136b7a1d10d
- **[Level](../README.md#level_slider-levels):** 1

## Project Overview :page_facing_up:

### Overview

Integration of Mintbase NFTs with [Magnolia CMS](https://www.magnolia-cms.com/)

[Magnolia CMS](https://www.magnolia-cms.com/) is an open-source Content Management System and the integration with Mintbase NFTs will provide an interface to mint contents - pages and images - in NEAR as well as consume NFTs deployed in Mintbase.

We are interested in the project because we are involved actively with [Magnolia CMS](https://www.magnolia-cms.com/) and we think that there are a lot of digital contents in CMS's that could be minted/distributed as NFT's and at the same time, content editors could use NFT's as digital content for their websites - for example an NFT auction site served from a CMS -

### Project Details

The purpose of the project is to integrate Mintbase NFT's with [Magnolia CMS](https://www.magnolia-cms.com/) allowing editors to mint and consume NFT's from Mintbase.


As any CMS, Magnolia stores digital contents, serves web sites and expose API with raw contents  - see [Headless CMS](https://www.magnolia-cms.com/platform/solutions/headless-cms.html) - The integration will add the following features to Magnolia:
- Minting of pages as NFTs. It will be deployed a Mintbase store by web site from Magnolia, the pages will be rendered as PDF and the minted in Mintbase.

- Minting of assets (images) as NFTs. It will be deployed a Mintbase store by folder of assets, the assets will be minted as any image in Mintbase.

- Usage of NFTs published in Mintbase as digital content in the web sites served from Magnolia.

For a detailed explanation, please read the following article https://medium.com/coinmonks/how-to-create-nfts-from-contents-of-a-website-2980c4338b65 where it is explained the same features but minting NFTs in Ethereum.


The result of the project will be a custom module of [Magnolia CMS](https://www.magnolia-cms.com/) that will provide the new features.
The module will be coded in **java** as custom features in [Magnolia CMS](https://www.magnolia-cms.com/) have to be made in this language.

The implementation of the full API of mintbase in **java** is out of the scope this project. Anyway we will make an isolated library that will make it easier to extracted from Magnolia to be used in any **java** product

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?

The integration with Magnolia will allow current NFT creators to use a CMS as user interface and will help current contributors of contents in a Magnolia CMS to distribute their contents - pages or images - as NFTs in Mintbase.

- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?

Target audience are customers of [Magnolia CMS](https://www.magnolia-cms.com/)

- What need(s) does your project meet?

Knowledge of [Magnolia CMS](https://www.magnolia-cms.com/)
Knowledge and experience in java and typescript/javascript
> Needs are provided by the team

- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?

We have not found any integration of Mintbase/NEAR with a CMS - Wordpress, Drupal, Liferay etc. -  
The [Build Ideas](https://docs.mintbase.io/dev/build-ideas) suggests the integration with Figma or Shopify and it is the more similar project we have found.

## Team :busts_in_silhouette:

### Team members

- Name of team leader

Joaquín Alfaro 

- Names of team members

Joaquín Alfaro
### Contact

- **Contact Name:** Joaquín Alfaro
- **Contact Email:** joaquin.alfaro@formentor-studio.com
- **Website:** [formentor-studio.com](https://formentor-studio.com)

### Legal Structure

- **Registered Address:** Calle Quito, 2A, esc. 4, 5B - Palma, Islas Baleares (Spain)
- **Registered Legal Entity:** Formentor Studio S.L.

### Team's experience

The more relevant experience of the team related with **Blockchain** is the development of [travel-inblock.io](https://travel-inblock.io), which is a Bed bank that lays on DLT technologies to distribute the inventory of hotels. At this moment it is deployed the B2C model that provides a website that allows hotels to load rooms, inventory and prices and a website to make reservations.

The business logic is implemented in solidity smart contracts and deployed in an Ethereum network. The responsibility of the blockchain network is to be the source of truth and we mantain a soft copy on MongoDb and Elastic Search to calculate availability requests and text/geospatial queries.

- Link to the hotels backoffice http://hotels.travel-inblock.io/bedbank/0x39B4466D8A42060A3bBcdbBCaE3e8b2579aC96CC
- Link to the booking engine http://booking.travel-inblock.io/

We have wide experience in [Magnolia CMS](https://www.magnolia-cms.com/) as we have participated as consultants in multiple installations of [Magnolia CMS](https://www.magnolia-cms.com/)

### Team Code Repos

- https://github.com/formentor-studio

Related with Blockchain
- https://github.com/formentor-studio/magnolia-tokenizer-bundle
- https://github.com/formentor-studio/dapp-auction-with-bdd-cucumber
- https://github.com/formentor-studio/ethereum-payment-vuejs

Related with Magnolia
- https://github.com/joaquin-alfaro/magnolia-event-driven-service
- https://github.com/joaquin-alfaro/magnolia-rest-graphql
- https://github.com/joaquin-alfaro/magnolia-content-translation-ml

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/joaquin-alfaro

### Team LinkedIn Profiles

- https://www.linkedin.com/in/joaquin-alfaro/

## Development Status :open_book:

We have made a POC to mint pages and assets in [Magnolia CMS](https://www.magnolia-cms.com/) as NFTs in Ethereum.
Details of the project can be found in the following links:
- [Medium article](https://medium.com/coinmonks/how-to-create-nfts-from-contents-of-a-website-2980c4338b65)
- [GitHub repository](https://github.com/joaquin-alfaro/magnolia-tokenizer-bundle)

We have evaluated the feasibility of integrating [Magnolia CMS](https://www.magnolia-cms.com/) with Mintbase/NEAR studying the java SDK of [SyntiFi](https://github.com/syntifi) available in https://github.com/syntifi/near-java-api and we were able to create a store in Mintbase calling the function `create_store` of the factory **mintspace2.testnet** 

The testing code can be found here https://github.com/formentor-studio/near-java-api/blob/mintbase-create-store/near-java-api-rpc/src/test/java/com/syntifi/near/api/rpc/MintbaseTest.java

> Example of a transaction related with a deployed store from server side using java
>
> https://explorer.testnet.near.org/transactions/ENJkCBRqCd58xoyUMZQmsQhsynnbUhptyWuw9k1ftPiH

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 5 weeks
- **Full-Time Equivalent (FTE):**  1 FTE
- **Total Costs:** 10,000.

Project planning in Github:
- https://github.com/orgs/formentor-studio/projects/1/views/2
### Milestone 1 - Minting of pages and assets of Magnolia in Mintbase NFTs

- **Estimated duration:** 3 weeks
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 0b. | Documentation | The repository will be published in Github and the README will provide the **instructions** to add and setup the module in a Magnolia CMS installation. It will include also a **tutorial** for editors explaining how to mint assets in Mintbase. The [Magnolia Marketplace](https://www.magnolia-cms.com/marketplace) will show a summary of the module |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. We will apply TDD for the development and will use [JUnit](https://junit.org/), [mockito](https://site.mockito.org/) and [Magnolia testing libraries](https://git.magnolia-cms.com/projects/PLATFORM/repos/main.pub/browse/magnolia-core). Unit tests will be based on mocks of Mintbase and NEAR |
| 0d. | Docker | We will provide a docker-compose to run an installation of Magnolia with the integration with Mintbase to test the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** in [Medium](https://medium.com/) explaining the integration of Mintbase with Magnolia from the point of view of a content editor - this article will not cover technical aspects - Mainly the article will map Mintbase UI with Magnolia UI, showing the result of minting an asset from Magnolia in Mintbase.
| 1. | Deployment of stores | We will create a module in Magnolia that will provide a new feature to deploy Mintbase stores for a given website and a folder of assets. |  
| 2. | Minting of assets | We will add a feature in the module delivered at point 1 to mint pages and assets of Magnolia in stores deploy by site or folder of assets. Pages will be minted as PDF's and assets will be minted as usual |  
| 3. | Publication of the java artifact | The Magnolia module will be packaged as a java artifact and published in Maven Central Repository |  
| 4. | Publication in Magnolia Marketplace | The module will be proposed to be published in [Magnolia Marketplace](https://www.magnolia-cms.com/marketplace.html) |  

The detail of the planning can be seen in this github project https://github.com/orgs/formentor-studio/projects/1

### Milestone 2 - Usage of NFTs in Mintbase as digital assets in Web sites served by Magnolia CMS

- **Estimated Duration:** 2 weeks
- **FTE:**  1
- **Costs:** 4,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 0b. | Documentation | The README of the repository and the summary of the module in [Magnolia Marketplace](https://www.magnolia-cms.com/marketplace) will be extended with an explanation of how to consume Mintbase NFTs from Magnolia|
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. We will apply TDD for the development and will use [JUnit](https://junit.org/), [mockito](https://site.mockito.org/) and [Magnolia testing libraries](https://git.magnolia-cms.com/projects/PLATFORM/repos/main.pub/browse/magnolia-core). Unit tests will be based on mocks of Mintbase and NEAR |
| 0d. | Docker | The docker-compose provided in *Milestone 1* will not suffer any change. |
| 0e. | Article | We will publish an **article** in [Medium](https://medium.com/) explaining how to consume Mintbase NFTs from Magnolia as digital assets. We will use as example an auction web site for NFTs of a store in Mintbase.
| 1. | Client with [GraphQL Indexer API](https://docs.mintbase.io/dev/read-data/mintbase-graph) | We will implement a client with the [Mintbase GraphQL Indexer API](https://docs.mintbase.io/dev/read-data/mintbase-graph) and the related queries to fetch NFTs minted or listed in Mintbase NFTs. |  
| 2. | Asset provider | We will implement a [Magnolia asset provider](https://docs.magnolia-cms.com/product-docs/6.2/Developing/API/Java-Server-APIs/DAM-API.html) to make the NFTs of Mintbase available in Magnolia as external digital asset to be published in pages of websites served from Magnolia|  
| 3. | Publication of the java artifact | The artifact will be published again in Maven Central Repository with the changes in the module|  
| 4. | Publication in Magnolia Marketplace | The module will proposed to be published in [Magnolia Marketplace](https://www.magnolia-cms.com/marketplace.html) or the summary of the module will be extended with the new feature|  

The detail of the planning can be seen in this github project https://github.com/orgs/formentor-studio/projects/1

## Future Plans

- We will propose the publication of the module in [Magnolia Marketplace](https://www.magnolia-cms.com/marketplace.html)
- The team will support Magnolia customers in the usage of the module and will introduce them in the usage of Mintbase NFT to setup the stores and listing the NFTs in the marketplace.


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?**  
In the webinar "Mintbase Grants AMA" announced in Discord channel

We have created a project in GitHub to estimate the required time and resources of the project. Link to the project https://github.com/orgs/formentor-studio/projects/1