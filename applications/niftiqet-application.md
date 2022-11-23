# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** Niftiqet
- **Team Name:** Utinifty 
- **Payment Address:** niftiqet.near
- **[Level](../README.md#level_slider-levels):** 2

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:



### Overview

Niftiqet is an NFT ticketing management system built on Mintbase and NEAR.

#### About Niftiqet

Niftiqet is a ticketing management system built using NFT technology. It is built on [NEAR](https://near.org) and [Mintbase](https://mintbase.io). It is meant to provide solutions to some of the problems plaguing the ticketing industry. These include keeping costs down, preventing or severely reducing fakes/scams, and scalping.

- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.
- An indication of why your team is interested in creating this project.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):


- Data models / API specifications of the core functionality
 ```i. Data models
+ Activity: 
- price: Int,
- memo: String,
- kind: String,
- action_sender: String,
- action_receiver": String

+Event:
- name
- Ticket
- price
- description
- price_category
- event_category
- ticket_quantity
- time
- venue
- royalties
- split
- burn_timestamp

```

#### Technoglogy Overview
 - Frontend - VueJS (Nuxt2)
-  Backend - NodeJS (Express)
-  Smart Contract - Mintbase and supplimentary Smart contract on the NEAR blockchain.
- API- GraphQL
- Static hosting - Akash, Netlify, Heroku

- PoC/MVP: https://niftiqet-test.netlify.app/
- What your project is _not_ or will _not_ provide or implement:
Project would include:

``` 
i. Ticket management service
 ii. Events dashboard
 iii. Buyers dashboard
 iv. Tickets
 v. Marketplace
 vi. % ceiling for ticket price in event of resales
 vii. Royalties
 viii. Stand-alone stores for events.
```
 

Project would not include:
```
i. Crosschain purchases
ii. Fiat payments
iii. QR CODE acc
iv. Ticket reuse (as opposed to burning). Events simply recorded on blockchaon
v. Mobile app
```
  
### Ecosystem Fit

- We would be using the Mintbase API and Indexer, as well as building supplimentary smart contracts when needed on NEAR.
- We are targeting event managers, event creators, musicians, and the like.
- There is the need of a decentralized ticket management system to fight monopolies. There's also a need to costs down, preventing or severely reducing fakes/scams, price management, and scalping.

- As far as we know, [Mintickt](https://www.mintickt.com/). They are still in stealth mode.

## Team :busts_in_silhouette:

### Team members

- Name of team leader: oïclid
- Names of team members: Aef Gbadamosi, Mary Ehapa, Handrii

### Contact

- **Contact Name:** oïclid
- **Contact Email:** oiclid@utinifty.com
- **Website:** niftiqet.com

### Legal Structure

- **Registered Address:** 19 Luka Chung Street, Jos, Nigeria
- **Registered Legal Entity:** CommandLine Technologies Ltd

### Team's experience

- oïclid - Developer with close to 15 years experience.
- Aef Gbadamosi - Fullstack Developer with a passion for NodeJS, crypto, and learning.
- Mary Ehapa - Young Product Designer with an eye for string design, a desire to make great products, and an enthusiasm for web3.
- Handrii - Blockchain developer with experience building in Rust, Solidity, and JavaScript




### Team Code Repos

- https://github.com/utinifty
- https://github.com/utinifty/niftiqet


Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/oiclid
- https://github.com/daaef
- https://www.behance.net/ehapamary
- https://github.com/beeust



## Development Status :open_book:

We are already have a beta deployed on the mainnet, but want to not only add features, but 
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
- **Total Costs:** 50,000 USD

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
