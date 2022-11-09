# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** NanoStore by explorins 
- **Team Name:** eXplorins-nanoStore
- **Payment Address:** explorins.near
- **[Level](../README.md#level_slider-levels):** 2

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

EXPLORINS is an app & creative hub based in Barcelona. Departing from our app promoting neighbourhood dynamization, local cultural activitivies and experiential marketing we evolved into a "shopify for interactive experiences". With our modular components we offer our clients a wide range and increasing set of features to build customized applications to engage with their audience: from gps treasure hunts and maps for hybrid cultural storytelling to AR virtual galleries... Our clients are commercial districts and public administrations, artistic communities and innovative brands. All our projects aim to combine technology, culture and social aspects. Our projects may be standalone applications, feature integration into 3 party infrastructures or content hosted in our own app eXplorins. Therefore we are a certified B-Corp (awarded as Best for the World 2022 "governance and social impact") and build our own libraries to integrate state of the art technology.

Part of our evolution is the ongoing integration of a blockchain based token system to empower our content creators through a participatory renumeration logic. As a first step we integrated Mintbase listed NFTs as part of our first virtual gallery launched in Feb 2022, however we are aiming at building an entirely token based system where blockchain is a powerful technology behind utility nfts rather than focussing on purely collectable nfts as assets for speculation.

The present project "NanoStore by eXplorins" will allow to experience virtual items in the real world through AR and convert them into physical goods through 3D printing. The intended target audience are designers & artists who want to showcase their works onsite and 3D printing services likewise.

Through the NEAR based NanoStore app users may place items in their sorrounding whereever they are and before their actual purchase decision, and in case of a transaction into their own wallet send the item out for 3D printing to later receive the piece as physical item afterwards. In that sense this applies as well for 3D printing services which can let their customers check out any piece in AR before the actual printing job.

Kickoff: Dec 1st, 2022
Prod-ready MVP: March 1st, 2023

For eXplorins this project is highly aligned with our strategy to build a increasing set of features combining online & offline experiences.
https://about.explorins.com/en/


We see a very high potential targeting very broad audiences from creator's industry as artists & designers and 3D printings.

---

Please provide the following:

- If the name of your project is not descriptive, a tag line (one sentence summary).
- A brief description of your project.
- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.
- An indication of why your team is interested in creating this project.

### Project Details

At NanoStore can be set up indivually for different events, galleriers, stores etc and users will be able to:
- check out items of a creator's store (on mintbase)
- purchase items directly within app into their connected wallet (and via Mintbase behid the scenes)
- check out all items in user's own wallet via AR
- scan QR codes to check out a specific item
- choose connected printing service
- pay printing job with NEAR and send item for printing
- creator gets royalties per print && print service charges per print
- receive physical print at their shipping address

* Mockups/designs of any UI components:
https://miro.com/app/board/uXjVOiE0K5E=/?share_link_id=935893954284
https://www.figma.com/file/xC8RUmJQGzQiUpyKkkA5ja/NanoStore_eXplorins?node-id=5%3A16

* Data models / API specifications of the core functionality
- connect NEAR wallet
- read NEAR wallet (Mintbase Indexer)
- make transaction NFT
- find printer and order printing job https://polar3d.com/ or equivalent API
- transaction for 3D printing
- royalities per print

* An overview of the technology stack to be used
- Stack: Typescript, Angular, Node 16, Firebase

* Documentation of core components, protocols, architecture, etc. to be deployed
- 1. open Source (MIT) Typescript Library NEAR / Mintbase integrations 
- 2. open Source (MIT) Typescript Library Printer: API & integration remote printers
- 3. AR Feature ritch standalone webapp project (PWA, Angular) consuming UI components from eXplorins component library (private)
- 4. deployed and customizable application (Firebase)

* What your project is _not_ or will _not_ provide or implement
- project will NOT contain any upload into App Stores etc
- project will not include any specific AR items or galleries


We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components: 
- Data models / API specifications of the core functionality
- An overview of the technology stack to be used
- Documentation of core components, protocols, architecture, etc. to be deployed
- PoC/MVP or other relevant prior work or research on the topic
- What your project is _not_ or will _not_ provide or implement
  - This is a place for you to manage expectations and to clarify any limitations that might not be obvious

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

* Where and how does your project fit into the ecosystem?
- We believe this project is also very much in the scope of Mintbase promoting a growing utility nft ecosystem and real-world usecases. We think NanoStore will be a attractive way to connect an abstract virtual web3 world for physical tangeable assets. It's applied usecase will lower the barrier to implement Token based systems in real world transactions.
- 
* Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
- The intended target audience are artists who want to showcase their works onsite and 3D printing services likewise.

* What need(s) does your project meet?
- Connecting offline & online worlds and goods. Deliver a broad set of real life use cases
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems? As Far as we know not

## Team :busts_in_silhouette:

### Team members

- Daniel Ferrer, Marc Farres, Didac Pascual, Angela Bustillos, Maciá Dalmau

### Contact

- **Contact Name:** Daniel Ferrer
- **Contact Email:** daniel@explorins.com
- **Website:** https://about.explorins.com/en/

### Legal Structure

- **Registered Address:** C/ Aribau 47, 2o 1a, 08011 Barcelona
- **Registered Legal Entity:** EXPLORINS PMW SL

### Team's experience

- Daniel
  - Co-Founder and Tech lead at eXplorins.
- Angela
  - Co-Founder and Head of Communication at eXplorins
- Marc
  - Fullstack developer with 12 years of experience
- Didac
  - Fullstack developer with 7 years of experience
  - Author of eXplorins backend
- Maciá
  - UI/UX Designer at eXplorins

Angela Co-Founder at eXplorins will be in charge of design process and communication.

---

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Web3 Foundation previously, please list the name of the project and legal entity here.

### Team Code Repos

- https://git-codecommit.eu-west-1.amazonaws.com/v1/repos/frontend-app
- https://git-codecommit.eu-west-1.amazonaws.com/v1/repos/backend-api
- https://SupiDani@bitbucket.org/Daniel_eXplorins/explorins-ios-app.git

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/daniel-explorins
- https://github.com/didac-pf
- https://github.com/MarcFarres

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/daniel-ferrer-muc-bcn/
- https://www.linkedin.com/in/marc-farres-pijuan-94b53564/
- https://www.linkedin.com/in/didac-pf

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),
- academic publications relevant to the problem,
- links to your research diary, blog posts, articles, forum discussions or open GitHub issues,
- references to conversations you might have had related to this project with anyone from the Mintbase Foundation,
- previous interface iterations, such as mock-ups and wireframes.

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

- **Total Estimated Duration:** 3 months
- **Full-Time Equivalent (FTE):**  5 FTE
- **Total Costs:** 20,000 USD
- 
- Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in Bitcoin, the amount will be calculated according to the exchange rate at the time of payment.

### Milestone 1 — Project Setup & UX + Design Mockups & Wallet Connect + Opensource Lib Setup + documentation & testing structure

- **Estimated duration:** 1 month
- **FTE:**  2
- **Costs:** 8,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License |  MIT  |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | - |
| 1. | Library NEAR / Mintbase | Integrate with NEAR. Handle wallet connections and fetch data from NEAR and Mintbase |  
| 2. | Library Printer | - |  
| 3. | webapp project | Angular project // detailed design Mockups & UX |  
| 4. | Firebase / DB | Store data needed by the webapp at the current status of development | 
| 5. | NEAR chain integration | testnet |

---

### Milestone 2 — Read Wallet, AR Experience & 3D printing API

- **Estimated duration:** 1 month
- **FTE:**  2
- **Costs:** 8,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License |  MIT  |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | - |
| 1. | Library NEAR / Mintbase | Read Wallet / Mintbase transaction / Smart Contract |  
| 2. | Library Printer | Connect to 3D printers API / Send printing job |  
| 3. | webapp project | AR experience / Virtual Gallery |  
| 4. | Firebase / DB | - |  
| 5. | NEAR chain integration | - |

### Milestone 3 — Revenue split + royalities & shipping & MVP

- **Estimated duration:** 1 month
- **FTE:**  1
- **Costs:** 4,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License |  MIT  |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | Introduction Concept (Medium + Sector Cultural BCN), promotion Barcelona,  |
| 1. | Library NEAR / Mintbase | Transaction printingjob + creator royalties |  
| 2. | Library Printer | Select printer and send shipping address |  
| 3. | webapp project | MVP |  
| 4. | Firebase / DB | Deploy a production database, deploy at nano-store.app |  
| 5. | NEAR chain integration | mainnet |

## Future Plans

Please include here
- Project will receive high attention on launch event due to its innovative approach within Barcelona cultural sector
- eXplorins will maintain the standalone project and will use it for future events and promote it within our increasing community of artists
- MOB / Fab Lab will use the application for their daily printing business and promote it within their community
- deployed application will be available for use within Mintbase ecosystem
- All library integration will be used and maintained within the eXplorins "shopify" ecosystem
- When this project got traction we would need to build a decent onboarding // dashboard for    creators & printing services to choose
- Further we would need to increase user experience by enhancing features such as scaling, colors, meterials etc, as well as choosing the closest // favorite printing service
- 
- how you intend to use, enhance, promote and support your project in the short term, and
- the team's long-term plans and intentions in relation to it.


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** You Mintbase folks told me about ;-)

Web3 Foundation Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.
