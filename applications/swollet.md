# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** Swollet NFT Certificates
- **Team Name:** Swollet
- **Payment Address:** laliotis.near
- **[Level](../README.md#level_slider-levels):** 1

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:
This application is in response to an RFP.

### Overview

Please provide the following:

- If the name of your project is not descriptive, a tag line (one sentence summary).
  The Financial Empowerment Platform focused on gamifying financial education and providing on-chain credentials.
- A brief description of your project.
  Play and learn about financial literacy, work readiness, entrepreneurship through bite-sized courses. Content is created by top notch academic institutions. Portfolio Battles: Test your knowledge against the online community of fellow traders by playing the fantasy trading game. NFT certificates: Prove your talent on the blockchain by receiving your NFT certificate upon course completion.
- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.
  Upon course completion, learners will receive their badges. Those badges will be NFTs which they will prove their talent on the chain.
- An indication of why your team is interested in creating this project.
  We are building a financial education platform and our vision is to democratise financial education and make it accessible for everyone in the world.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
  https://drive.google.com/drive/folders/1rsdik9kqwe8JIxpFet8_SAH0ha9oCbpI?usp=sharing
- Data models / API specifications of the core functionality
- An overview of the technology stack to be used
  https://docs.google.com/document/d/1D5Teha13qPw2Sc69-KSiyAwnGnIy0XBSSREpvCgSF7M/edit?usp=sharing
- Documentation of core components, protocols, architecture, etc. to be deployed
  https://docs.google.com/document/d/1D5Teha13qPw2Sc69-KSiyAwnGnIy0XBSSREpvCgSF7M/edit?usp=sharing
- PoC/MVP or other relevant prior work or research on the topic
  https://drive.google.com/file/d/1Ik6RscdkMIwA8T86oFcQHwhraXF-OUO8/view?usp=sharing
- What your project is _not_ or will _not_ provide or implement
  We are looking to utilize the Mintbase and NEAR ecosystem in order to enahnce the learning experience of our learners by issuing their certifications on the blockchain. We are focusing on the educational side of things and by utilizing web3, we will democratize and create a creators economy for educators and students.
  
### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
  Our project fits in the NFT side of things because certificates need to be on the blockchain.
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
  Our target audience are youngsters 16-25 years old who are interested in learning about financial literacy. They are mobile app users and they do not need to have prior web3 knowledge. 
- What need(s) does your project meet?
  Our need is to be able to deploy smart contracts for our NFT certificates and a base for minting those NFTs.
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
    There are project similar to our project such as RabbitHole but they are purely focusing on crypto education. We are focusing on financial literacy.
  - If not, are there similar projects in related ecosystems?

## Team :busts_in_silhouette:

### Team members

- Name of team leader
Eleftherios Laliotis
- Names of team members
Alexandros Adamakis, Ali Riza Dagli

### Contact

- **Contact Name:** Eleftherios Laliotis
- **Contact Email:** lefteris@swollet.com
- **Website:** https://swollet.com/

### Legal Structure

- **Registered Address:** 6 Thomastown House, Spencer Dock, Dublin 1, Dublin, Ireland
- **Registered Legal Entity:** Swollet Technologies Ltd.

### Team's experience
Eleftherios Laliotis, CEO - 7+ years of investment management experience @ BlackRock and State Street. Founded Ultimo Digital Marketing.

Alexandros Adamakis, COO - 3+ years of start-up business development experience @ Vecna Robotics & Salesforce.

Ali Riza Dagli, CTO - 2+ years of full stack development experience.

### Team Code Repos

- https://github.com/ourappprototype/swollet
- https://github.com/ourappprototype/swadmin


Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/alirizadagli
- https://github.com/llaliotis

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/eleftherios-laliotis/
- https://www.linkedin.com/alexandrosadamakis/
- https://www.linkedin.com/alirizadagli/

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- We have finalized the MVP and can be found here: https://drive.google.com/file/d/1Ik6RscdkMIwA8T86oFcQHwhraXF-OUO8/view?usp=sharing
- We have partnered up with the National College of Ireland and they will provide us with raw educational content.

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
- **Full-Time Equivalent (FTE):**  1 FTE
- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in Bitcoin, the amount will be calculated according to the exchange rate at the time of payment.

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

**How did you hear about the Grants Program?** Web3 Foundation Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.
