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



#

## Development Status :open_book:

We have a working beta available to access on the [testnet](https://niftiqet-test.netlify.app/)

## Development Roadmap :nut_and_bolt:



### Overview

- **Total Estimated Duration:** 3
- **Full-Time Equivalent (FTE):**  4
- **Total Costs:** 50,000 USD

### Milestone 1  — Product and Architecure Design

- **Estimated duration:** 1 month
- **FTE:**  4
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 1 | System Architecture |Full technical description of app, flows, and desired outcomes|
|2.| Product design| Design of product UI/UX plus functions based on Architecture


### Milestone 2 Release on Testnet



- **Estimated duration:** 2 month
- **FTE:** 4
- **Costs:** 30,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** that explains **Niftiqet** works, why it is something we are interested in, and what to expect from the projec (Content, language and medium should reflect your target audience described above.)
| 1. |Modules | We will create modules using the  Mintbase / NEAR SDKs (if needed) for dashboards, marketplaces, transfering ownerships, resale price ceilings ,etc.|  
| 2. | Smart Contracts | As needed, we would write smart contracts to add any functionality we need that doesn't currently exist on Mintbase/NEAR |    
|3. | UI/UX | Continously improve usability of dapp|                                   |

### Milestone 2: Make it production ready:

This will include the following features

- Perform User-Tests to make the tool as user friendly as possible.
- Publish a npm-package for easy integration of Automat.
- Add detailed documentation
- Perform security and performance tests

- **Estimated duration:** 1 month
- **FTE:** 4
- **Costs:** 10,000 USD

| Number | Deliverable      | Specification                                                                                                                                     |
| -----: | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
|    0a. | License          | MIT                                                                                                                                               |
|    0b. | Documentation    | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user has to interact with Automat          |
|    0c. | Testing Guide    | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
|    0d. | Testnet-Instance | We will provide a Testnet-Instance that can be used to test all the functionalites of our product.                                                |
|    0e. | Article          | We will publish an **article** that explains all features and how to use them as well as two-weekly development updates in our discord-channel.   |


## Future Plans

Please include here

- We intend to keep on adding more features to improve usability as well as improve the robustness/scalability of the project
- We would partner with Mintbase, NEAR Africa, NEAR Foundation, Awesome NEAR, etc to make use of all available tools to grow our project.
- In the longterm, we are looking to seek out investors (Seed, VCs, etc) to help with hiring, operations, etc.


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** personal recommendation (one or both of Luis and Mariam)

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Members of the team have worked on the [Aave Java SDK](https://github.com/alienrobotninja/aave_java_sdk)
- We have received previous funding from the NEAR foundation to help us build the first part of this project.
- We previously unsuccessfully applied for a [grant](https://github.com/alienrobotninja/Mintbase-Grants-Program/blob/tunenifty/applications/tunenifty-application.md) from Mintbase.
