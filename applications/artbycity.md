# Mintbase Grant Proposal

- **Project Name:** Art By City Protocol Integration
- **Team Name:** Art By City
- **Payment Address:** artbycity.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### **Overview**

Art By City is a Web3 art and creative content curation protocol built primarily on Arweave.  By publishing art and creative content to the Arweave blockchain through the Art By City Protocol, it is ensured that this content is stored permanently, immutably, and decentrally distributed.  Art and creative content published through the Art By City protocol are inseparable from the artist’s on-chain identity, regardless of their blockchain or platform of choice.  Licensing and royalty intentions are published alongside art and creative content, ensuring that artists, creators, and their work are protected legally and that they receive proper royalties as their content is monetized.  As art and creative content data are inseparable from the metadata, supporters and patrons will be able to trace back to the artist or creator given an example of their work.  Through the use of tags and crawling engagement graphs, artists and creators benefit from increased discoverability.

Our full vision for integration with Mintbase is for creators to be able to publish their creative assets to Arweave through the Art By City Protocol.  We believe it is important for creators to directly own their data on the Arweave network with their NEAR wallet and to be able to atomically license these creative assets for use across Web3.  This would allow any creators publishing through Mintbase to have their creative work to be discovered across Web3 and to eventually have automatic licensing of these works for use in metaverses, galleries, or radio stations for a few examples.  This also enables creators to publish once and use their creation in any number of manner across Web3 rather than having to constantly re-upload, re-publish, or worry about renting/hosting fees.

At Art By City we're interested in empowering artists across all of Web3 and to build the tools necessary for this vision so that they can focus on their art.  One of our core guidelines is enabling artists to own their data directly on Arweave so that they are in full control of their intellectual property which we aim to achieve through Atomic Licensing.

### **Project Details**

#### **Art By City Protocol Tags**

In order to participate in the Art By City Protocol, Arweave transactions are tagged depending on the purpose of the transaction.  These lists of tags are not exhaustive.  Additional tags are outlined in the Art By City whitepaper.

At the top-level, there is only one tag necessary to participate and have the transaction be indexed.  The tag name and value are **required** to be `Protocol` and `ArtByCity` respectively.

| Tag | Required Value | Description |
| - | - | - |
| `Protocol` | `ArtByCity` | Top-level Art By City Protocol tag to indicate this tx is of interest to the protocol |

Participating transactions should also indicate the origin platform or app.  Additionally, it is encouraged to tag transactions with an `App-Version` for further filtering, indexing, and support of triaging potential future issues.

| Tag | Example Value | Description |
| - | - | - |
| `App-Name` | `Mintbase` | The unique identifier name of the originating platform or app.  If using the vanilla Mintbase platform, this would be `Mintbase`.  If running another platform on top of Mintbase, this would be that app's name (e.g. `MyAwesomeMintbaseApp`) |
| `App-Version` | `1.0.0` | The version of the originating platform or app |

Art By City Protocol transactions do not all serve the same purpose, so it is important to categorize them properly.  This list is not exhaustive of all possibilities as more are added to support protocol features as they are developed.  Additional categories are outlined in the Art By City whitepaper as only the ones relevant to this grant are included.  In most cases, these tags apply primarily to the transaction responsible for the manifest or metadata files included with publications with the exception of `artwork:bundle` for `Category` and `Manifest-ID` for bundle transactions.

| Tag | Value | Description |
| - | - | - |
| `Category` | `artwork` | Tag for use on the manifest file of creative work publications |
| `Category` | `artwork:bundle` | Tag used on the top-level bundle of creative work publications |

Additionally, not all creative work publications are the same.  To further aid filtering and indexing, a `Sub-Category` tag can be added to indicate the type of media assets included and give hints about the presentation of these assets.  Typically, these align with the top-level MIME type of the primary creative asset.  This list is not exhaustive and future valid values do not strictly need to adhere to top-level MIME types.

| Tag | Value | Description |
| - | - | - |
| `Sub-Category` | `image` | Used to classify image-based creative works |
| `Sub-Category` | `audio` | Used to classify audio-based creative works |
| `Sub-Category` | `video` | Used to classify video-based creative works |
| `Sub-Category` | `model` | Used to classify 3D model-based creative works |
| `Sub-Category` | `text` | Used to classify text-based creative works |

To support Collections of creative works (e.g. NFT Collections), an additional tag can be added to these publications.

| Tag | Example Value | Description |
| - | - | - |
| `Collection` | `bekopi` | Indicates this creative work is part of the `bekopi` collection |

As not all publications on the Art By City Protocol originate from Arweave wallet identities, it is important to indicate the true owner and their original blockchain.  For the purposes of this grant and the usage with Mintbase and within the NEAR ecosystem, these tags are required.

| Tag | Example Value | Description |
| - | - | - |
| `External-Network` | `NEAR` | Indicates the originating blockchain as NEAR |
| `External-Owner` | `artbycity.near` | The address of the owner of the creative works being published |

As not all publications follow the same metadata or manifest standards, it is important to indicate some top-level information about the format of these files so they are more easily consumed and without enforcing metadata-level standards.  It is also possible that metadata or manifest files included with publications support multiple formats as long as the combination of property names and their purpose to not conflict.

| Tag | Example Value | Description |
| - | - | - |
| `Manifest-Format` | `Mintbase` | Indicates the included metadata or manifest file is of the format that Mintbase expects. |
| `Manifest-Format` | `ArtByCity` | Indicates the included metadata or manifest file is of the format that the Art By City DAPP expects. |

Additionally, publication bundle transactions have a tag to aid in finding the primary metadata or manifest file of the publication.

| Tag | Example Value | Description |
| - | - | - |
| `Manifest-ID` | `<Arweave TX ID>` |  The Arweave transaction ID of the metadata or manifest file |

#### **Art By City Gateway / Plugin HTTP API**

The Art By City Gateway, being a plugin extension of the gateways that ar.io is developing, will support the full API of Arweave Gateways as outlined here: https://docs.arweave.org/developers/server/http-api.  It is not known at this time what additional endpoints or functionality the ar.io gateways will provide.  As we plan to participate in the private beta test for ar.io's gateways, we will update this grant proposal when this information becomes available to us.

Additionally, the Art By City Gateway API will support domain-specific endpoints related to the Art By City Protocol, Network, & DAO.  The first few endpoints for view tracking, view reports, trending reports, and dapp statistics are outlined in the Art By City Node project here: https://gitlab.com/art-by-city/node#rest-api.

As we support further use cases such as bundling or batch publishing, additional endpoints will need to be outlined.  These endpoints will not be dissimilar to those used by the ArSeeding project from EverFinance: https://github.com/everFinance/arseeding#api.

#### **Atomic Licensing**

As the concept of Atomic Licensing is new and currently being designed there is not much documentation for it.  The base of the Atomic License is the Atomic NFT, more information can be found here: https://atomicnft.com/en/.  The base Atomic License will implement the full Atomic NFT interface, including bridging (although possibly in a later iteration).

#### **Unknowns**

Some of the later milestones outlined in this grant proposal are dependent on the public release of ar.io's new Arweave Gateways.  These should be available to the public late Q4 2022 or early Q1 2023.  If ar.io Arweave Gateways are not yet available, we will plan to implement short-term solutions in order to meet milestone requirements until the ar.io Arweave Gateways are publicly released.

#### **Final Project State**
- Creative asset data being uploaded to Arweave through Mintbase are able to participate in the Art By City Protocol via Arweave transaction tags provided by the Art By City JS Client Library.
- Creative asset data on Arweave is owned directly by creators on Mintbase and NEAR signed with their NEAR identity.
- Creators are able to permanently pin their creative data on IPFS to Arweave and retroactively participate in the Art By City Protocol.
- Creators and their creative asset data are indexed for use cases beyond what Arweave.net and the Mintbase API offers.
- Mintbase projects will be able to use the Art By City Node & Arweave Gateway to pay for Arweave uploads using NEAR.
- Creative works are able to be published as purpose-built atomic bundles on Arweave which include original creative assets, preview assets, manifest, and other optional assets such as project files or stems.
- Creative works are able to be published while instantiating an Atomic License.  This Atomic License represents the intellectual rights to the creative work and provides the creator with a way to create subsidary licenses for general or single-use cases such as for minting NFTs or licensing out to other projects or entities.

#### **Whitepaper**
An early draft of our whitepaper for the full vision of the Art By City Protocol can be found at https://docs.google.com/document/d/1-sp6J4z8nGP7DnBjW_ucU5IO9bZWt1kTz_YrCtNHVhU/edit?usp=sharing

#### **Technology Stack**
- Art By City Client Library - TypeScript (NodeJS)
- Art By City Node Plugin - TypeScript (NodeJS), KoaJS
- Art By City Gateway - TypeScript (NodeJS), Postgres, Apollo
- Testing - Mocha & Chai

#### **Prior Work**
Our dapp at https://artby.city is live on Arweave mainnet and allows artists to self-publish using the Art By City Protocol.  Views are tracked and redirected to arweave.net through our Art By City Node Plugin as NuxtJS middleware.  Creators can publish images, audio, and soon 3D models as an atomic bundle along with preview assets.  These bundles are posted directly to arweave.net and owned directly by the creator's identity.  A white-listed feed of recent publications is on the [Discover](https://artby.city/discover) page.  This feed and others will soon be replaced by our SmartWeave Curation Contracts.  Other features our dapp currently supports are profiles, usernames, artwork URL slugs, tips, and likes.

### **Ecosystem Fit**

The goal of the Art By City Protocol, Network, & DAO is to empower artists with direct control over their creative works and assets across the entire Web3 ecosystem.  Many assets being stored for use on Web3 use ephemeral, rented space while being represented
on blockchains with immutable contract state chains and should instead be published to a permanent, immutable data-focused blockchain such as Arweave.  Furthermore, most creative works that *are* being published permanently to Arweave are not being directly linked to the creator, instead being owned by utility wallets for a given platform.

Artists continue to struggle with discoverability even though the Web3 ecosystem is meant to be decentralized and allow the composability of on-chain data.  By publishing with the Art By City Protocol, creative works are able to be decentrally curated and discovered through user network graphs.  Currently, many platforms that do use Arweave for creative asset storage rely on Arweave Gateways which are limited in queryability and indexing capabilities - GQL queries are limited to 100 results per request which causes scalability pains.  It is difficult to answer questions that are very simple in the Web2.0 ecosystem such as *"What are all the publications for this artist?"* or *"How many Views have my artworks accumulated?"*. Going further, Web3 allows for the infrastructure to ask questions beyond the capabilities of Web2 such as *"How many views have my artworks accumulated **across all of Web3**?"*. The Art By City Gateway will support these extended use cases for indexing, querying, caching, and publishing.

Many artists rely on NFT sales while licensing is still a manual, offline process.  Blockchain is a natural fit for such mechanisms and to allow artists other avenues to support themselves and their art.  With Art By City Atomic Licenses, creative works become inseperable from the artist's on-chain identity allowing for sub-contracts for single-use or general use cases.

We aren't aware of any similar projects in the Mintbase or NEAR ecosystem other than RSS3, which is a more general and social-network oriented protocol.  The Art By City Protocol, Network, & DAO will always prioritize the creator's Web3 experience.

In addition to artists, anyone consuming creative works through the Art By City Protocol provides curation value and is thus a curator which provides even further artistic and discoverability value.  Furthermore, Arweave Gateway operators interested in supporting artists are able to run their own Art By City Gateway which helps to decentralize the Art By City Network.

## Team :busts_in_silhouette:

### **Team members**

- Jim Toth, CEO / CTO
- Slava Mushyakov, COO
- Efraim Aizenman, CFO
- Michael Moon, Gallery Coordinator

### **Contact**

- **Contact Name:** Jim Toth or Slava Mushyakov
- **Contact Email:** jim@artby.city or slava@artby.city
- **Website:** https://artby.city

### **Legal Structure**

- **Registered Address:** 1309 Coffeen Avenue STE 1200 Sheridan 82801 Wyoming, United States
- **Registered Legal Entity:** Memetic Block, LLC

### **Team's experience**

- Jim Toth - Graduated from Rochester Institute of Technology in 2012 with BS in Software Engineering.  Worked professionally for a decade as a full-stack web developer contractor for companies such as MLB, NBA, EdApp, and Minds.  Became interested in Blockchain through mining Ethereum & attempting to write algorithmic trading bots.  Fell in love with Open Source Software through working with Minds where he first heard about Arweave.  Committed to Web3 Open Source development after discovering Arweave Q1 2021.

- Slava Mushyakov - Background in management and operations. Worked in medical field for over 15 years, live audio/video production for 5 years, involved with blockchain technology for over 5 years, robotics for 2 years, VR/metaverse for 2 years and event management for 3 years for musicians.  Founded and operated Audio/Video production company (one step up productions), VR company (Wonder World Reality).

### **Team Code Repos**

- Primary on GitLab: https://gitlab.com/art-by-city
- Mirrored Repos on GitHub: https://github.com/art-by-city
- dapp: https://gitlab.com/art-by-city/art-by-city
- contracts: https://gitlab.com/art-by-city/contracts (only contains our usernames registration contract used in dapp at the moment)
- art by city node / plugin: https://gitlab.com/art-by-city/node hackathon submission for ETH Shanghai

### **Jim's Git Accounts**

- GitLab (primary) https://gitlab.com/jim-toth
- GitHub https://github.com/jim-toth

### **Team LinkedIn Profiles**

- https://www.linkedin.com/in/iamjimtoth/
- https://www.linkedin.com/in/slava-mushyakov-6566a73a/

## Development Status :open_book:

Art By City Client & Node projects are the main repositories related to this proposal.  There is an Art By City dapp at https://artby.city that currently uses the Art By City Node project as NuxtJS middleware.

### **Art By City JS Client Library**
Development of the ABC Client library has not yet begun but has been developed as part of our dapp code.  This business logic would be refactored out of the dapp and into the ABC Client library.  This library will be written in TypeScript and published as a JS package on npm.

### **Art By City Node (Arweave Gateway Plugin)**
Development of the ABC Node began for our submission to ETH Shanghai and contains hackathon-related technical debt.  ABC Node code currently supports our dapp by tracking views/access of ABC publications and dapp user statistics.  The ABC Node code is intended to be used as a plugin for Arweave Gateways that itself can be extended with ABC plugins.  Currently, all Arweave Gateways run in a NodeJS environment and their repositories are either unmaintained or heavily coupled to Amazon Web Services while AR.IO develops the next generation.  AR.IO Arweave Gateway development is expected to be available to the public late Q4 2022 to Q1 2023.  Without running a dedicated Arweave Gateway, it is difficult to both index and compile statistics for a given dapp, project, creative asset, or creator.  It is also difficult to have access to fresh information at the rate users expect from Web 2.0.

After our introduction call with Maria, Ben, Tony, and Till we got a good sense of the mutually beneficial overlaps between Mintbase and the Art By City Protocol.  Both the Art By City Client and Node projects are essential to offer additional indexing and querying to the Mintbase and NEAR creator ecosystem and empower creators with decentralized curation and discoverability across Web3.

## Development Roadmap :nut_and_bolt:

### **Overview**

- **Total Estimated Duration:** 12 Months
- **Full-Time Equivalent (FTE):**  1
- **Total Costs:** 48,000 USD

### Milestone 1 — Art By City Protocol Tag Integration on Arweave Uploads using Art By City Client Library

- **Estimated duration:** 1.5 months
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | AGPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains the Art By City Client & Art By City Protocol Tags |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Art By City JS Client | We will refactor the Art By City Protocol business logic out of our dapp and into a TypeScript/JavaScript project published on npm. |
| 2. | Mintbase Art By City Module | We will create PRs and/or an Art By City Module to be used by the Mintbase back-end API that is responsible for uploading data to Arweave. |
| 3. | UI | If necessary, will provide UI updates to allow users the choice of ABC protocol participation |

#### Unknowns
- What is the tech stack / language used by the Mintbase API server code responsible for uploading data to Arweave?
- Does this server code upload in batches?
- Is ABC participation an optional choice set by the creator through the Mintbase UI?


### Milestone 2 — Arweave Uploads Signed with Creator's NEAR Wallet

- **Estimated Duration:** 1.5 months
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | AGPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how to sign data to be uploaded to Arweave with NEAR wallets. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Art By City JS Client | We will add NEAR wallet support to the Art By City JS Client. |
| 2. | Mintbase Art By City Module | We will create PRs and/or extend the Art By City Mintbase Module with a workflow that allows the creator of the asset data being uploaded to Arweave to sign this data with their NEAR wallet. |

#### Unknowns
- How are the bundled `Data-Item` currently being signed before given to Bundlr?  Is an Arweave utility wallet used?  Is a NEAR utility wallet used?

### Milestone 3 — Publishing IPFS-based assets to Arweave / Retroactive Art By City Protocol Participation

- **Estimated Duration:** 1.5 months
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | AGPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how users can permanently publish their existing IPFS asset data to Arweave & participate in the Art By City Protocol. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Mintbase Art By City UI Module | We will provide either a module that extends the Mintbase UI or a standalone front-end that allows Mintbase users to back-up and permanently publish their IPFS assets to Arweave |
| 2. | Mintbase Art By City Module | We will create PRs and/or extend the Art By City Mintbase Module with an API-based workflow that allows creators to permanently publish their assets from IPFS to Arweave. |

### Milestone 4 — Art By City Indexing

- **Estimated Duration:** 2 months
- **FTE:**  1
- **Costs:** 8,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | AGPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how Art By City extends Mintbase indexing & provides extended queryability & discoverability of creative assets. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Art By City Node / Arweave Gateway Plugin | We will provide the Art By City Node plugin for Arweave Gateways |
| 2. | Art By City Gateway | We will launch our own Art By City Gateway to begin accepting Arweave transactions, bundles, & provide extended querying & indexing of creative assets |

#### Unknowns
- This milestone is dependent on the public launch of ar.io Arweave Gateways which is expected sometime between Q4 2022 and Q1 2023.  Before this, we will have participated in their private beta testnet to gain experience with running our Gateway and how we'll be integrating our Art By City Node plugin.

### Milestone 5 — Art By City Gateway NEAR Currency Support
- **Estimated Duration:** 2 months
- **FTE:**  1
- **Costs:** 8,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | AGPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how Art By City's Gateway allows publishing of creative assets originating on Mintbase & Near can be published to Arweave using $NEAR. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Art By City Node / Arweave Gateway Plugin | We will provide updates to the Art By City Node plugin that allows accepting NEAR for Arweave uploads and bundles. |
| 2. | Art By City Gateway | We will update our mainnet Art By City Gateway to begin accepting NEAR for Arweave uploads and bundles. |

#### Unknowns
- This milestone is dependent on Milestone 4, and thus also dependent on the release timing of the ar.io Arweave Gateways.

### Milestone 6 — Publication Bundles
- **Estimated Duration:** 1.5 months
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | AGPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how Art By City publication bundles ensure atomic finality of a published work & ensure these assets are inseparable both from eachother and from the creator. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Art By City Node / Arweave Gateway Plugin | We will provide updates to the Art By City Node plugin that allows publishing of assets as purpose-built Arweave bundles to ensure publications are a single atomic action to the Arweave Network. |
| 2. | Art By City Client | We will update the Art By City JS Client to use purpose-built bundles for Mintbase publications & if support is ready for L2 recursive bundles, provide batch upload support of these bundles in a top-level batch-bundle. |

#### Unknowns
- If Mintbase code uses a batching mechanism, then this milestone is dependent on ar.io Arweave Gateways supporting recursive L2 Arweave Bundles.  I've confirmed the ar.io that this feature is on their roadmap.
- It is possible to accomplish a short-term batch-based solution lacking official Arweave Network support for recursive L2 bundles.

### Milestone 7 — Atomic Licensing
- **Estimated Duration:** 2 months
- **FTE:**  1
- **Costs:** 8,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | AGPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains what an Atomic License is & how it can be used to create child-licenses for single-use or other general types of licenses. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 1. | Art By City Atomic License | We will provide the Arweave SmartWeave Contract implementation, interface, code, and contract published to the Arweave Network. |
| 2. | Art By City Client | We will update the Art By City JS Client to ensure bundle publications to Arweave are in the form of an Atomic NFT License.  This Atomic License would represent creative rights over the asset and would exist separately from Mintbase NFTs.  These Atomic Licenses would be compatible with a potential NEAR <-> Arweave bridge. |

#### Unknowns
- This milestone requires Mintbase assets be published as purpose-built atomic bundles as outlined in Milestone 6.

## Future Plans

The Art By City Protocol, Network, & DAO will be continually developed to address creator needs in Web3.

We currently promote artist publications regularly on Web2 social media sites.  We host artist feedback sessions & host talks on social platforms such as Twitter & Clubhouse to keep our ears close to what creators are doing and what their needs are.

We plan to continue to promote the Art By City Protocol in the above manner and onboard more Web3 platforms.  Our co-founder Mike runs 108 Gallery in Brooklyn, NY who we are partnered with to begin to bridge Web3 and IRL.


## Additional Information :heavy_plus_sign:

During our feedback sessions we were introduced to NEAR Protocol and Mintbase by some of the artists publishing with the Art By City Protocol.  They were excited to use their creative assets in the NEAR ecosystem which led us to Mintbase.  After speaking with Maria, she recommended we look into the Mintbase Grant Program.  Meeting with the Mintbase team was a true pleasure as we found much common ground on which we can build a Web3 ecosystem that empowers creators in the NEAR ecosystem.

Art By City is an alumni project of Arweave’s Open Web Foundry (https://owf.dev).  We’ve received prize bounty rewards from ETH Shanghai and Tezos Web Arts & Culture hackathons.  We also have a Gitcoin Grant here: https://gitcoin.co/grants/6572/art-by-city
