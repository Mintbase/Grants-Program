# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** Streaming Rental Skin NFTs Aurora POC
- **Team Name:** Block star Logic
- **Payment Address:** blockstarlogic.near
- **[Level](../README.md#level_slider-levels):** 1

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- **If the name of your project is not descriptive, a tag line (one sentence summary).**<br/>
Skin your NFTs, earn streaming rental from your NFT assets

- **A brief description of your project.**<br/>
The SRS NFT Aurora Proof of Concept project is to deploy the Stream Skin Rental NFT project on the Aurora blockchain as a proof of concept in preparation for deployment of a RUST based version on the NEAR blockchain. 
The SRS NFT project enables NFT holders to skin functional NFTs with artistic NFTs in exchange for rent paid to artistic NFT owners. The way the model works will be to  use a skinnable version of a standard NFT contract such that with the presence of an Agreement contract the Skinnable will refer to the rented NFT for authentic metadata from the skinning NFT contract. 

- **An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.**<br/>
This project creates an essential xFi primitive as it enables NFTs minted in the Mintbase/NEAR ecosystem to gain phenomenal utility as the rights they hold can now be delegated under agreement to yet to be created functional NFTs in exchange for rent or other compensation. It is analgous to the home leasing model whereby once the agreement is in place both the landlord (i.e. NFT owner) and tenant(i.e. NFT Renter) are conferred specific rights that remain in play as long as the agreement is in place. 

- **An indication of why your team is interested in creating this project.**<br/>
The interest in creating this NFT infrastructure stems from the need to implement functional NFTs in other projects with a desire to include an artistic element, in short we want to have aesthetically pleasing functional NFTs where the aesthetics can be changed freely and without the need to assume ownership. 


### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- **Mockups/designs of any UI components**<br/>
  - The current in build UI for the hack version of the project can be found here:
[wandering-forest-2153.on.fleek.co](https://wandering-forest-2153.on.fleek.co/)

- **Data models / API specifications of the core functionality**<br/>
  - to be documented as part of the project

- **An overview of the technology stack to be used**<br/>
  - The technology stack for this implementation will be as follows:
    - **Web3**- front end 
    - **Solidity** - Execution layer
    - **Aurora** - Deployment EVM
    - **IPFS** - Storage layer

- **Documentation of core components, protocols, architecture, etc. to be deployed**<br/>
  - to be provided as part of this implementation 

- **PoC/MVP or other relevant prior work or research on the topic**<br/>
    - current repository https://github.com/cryptotwilight/Streaming-Rent-Skin-NFT

- **What your project is _not_ or will _not_ provide or implement**<br/>
- **This is a place for you to manage expectations and to clarify any limitations that might not be obvious**<br/>
  - This project **will** deploy onto the Aurora Testnet EVM this will be a reference model deployment 
  - This project **will not** deploy onto the Aurora Mainnet EVM 
  - This project **will** be wholy opensource under Apache 2.0 

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- **Where and how does your project fit into the ecosystem?**
  - SRS NFT creates an NFT financial and utility primitive, i.e. more sophisticated forms of Skinning agreement can be built ontop of the basic smart contract architecture
- **Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?**
  - The target audience for this are developers, designers, artists and NFT holders
- What need(s) does your project meet?
  - On the buy side SRS NFT enables NFT holders to change the asthetic of their NFT without sacrificing functionality, and on the sell side it enables NFT holders to easily lease their NFTs as opposed to using them as securities
- **Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?**
  - The closest we've found is OP Games DAO
  - If so, how is your project different?
    - sRS NFT differs in that we enable NFT -> NFT skinning as opposed to in game skinning where the user actually holds the NFT that they are using as a skin
    - Also we enable rental income from the NFT relationship  
  - If not, are there similar projects in related ecosystems?

## Team :busts_in_silhouette:

### Team members

- Tony Ushe

### Contact

- **Contact Name:** Tony Ushe
- **Contact Email:** tonyusheuk@gmail.com
- **Website:** https://www.blockstarlogic.com

### Legal Structure

- **Registered Address:** Kemp House, 160 City Road, EC
- **Registered Legal Entity:** Tongai Ushewokunze Holdings UK Ltd

### Team's experience

**Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.**

Tony is a serial hackathon winner and former management consultant who served with the likes of Royal Bank of Scotland and Lloyds Bank. He is also the founder of the Web 3 Edi project which is an immersive education platform powered by NFTs

**If anyone on your team has applied for a grant at the Mintbase previously, please list the name of the project and legal entity here.**
N/A

### Team Code Repos

- [https://github.com/Block-Star-Logic](https://github.com/Block-Star-Logic)
- [https://github.com/cryptotwilight](https://github.com/cryptotwilight)

**Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.**

- [https://github.com/cryptotwilight](https://github.com/cryptotwilight)

### Team LinkedIn Profiles (if available)

- [https://www.linkedin.com/in/tony-kunz-358a78166/](https://www.linkedin.com/in/tony-kunz-358a78166/)

## Development Status :open_book:

**If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:**

- **links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),**
- **academic publications relevant to the problem,**
- **links to your research diary, blog posts, articles, forum discussions or open GitHub issues,**
- **references to conversations you might have had related to this project with anyone from the Mintbase Foundation,**
- **previous interface iterations, such as mock-ups and wireframes.**
  - current repository https://github.com/cryptotwilight/Streaming-Rent-Skin-NFT
  - current UI https://wandering-forest-2153.on.fleek.co/

## Development Roadmap :nut_and_bolt:

**This section should break the development roadmap down into milestones and deliverables. To assist you in defining it, we have created a document with examples for some grant categories [here](../docs/grant_guidelines_per_category.md). Since these will be part of the agreement, it helps to describe _the functionality we should expect in as much detail as possible_, plus how we can verify and test that functionality. Whenever milestones are delivered, we refer to this document to ensure that everything has been delivered as expected.**

**Below we provide an** **example roadmap**. **In the descriptions, it should be clear how your project is related to Mintbase. We _recommend_ that teams structure their roadmap as 1 milestone ≈ 1 month.**

**For each milestone,**

- **make sure to include a specification of your software. _Treat it as a contract_; the level of detail must be enough to later verify that the software meets the specification.**
- **include the amount of funding requested _per milestone_.**
- **include documentation (tutorials, API specifications, architecture diagrams, whatever is appropriate) in each milestone. This ensures that the code can be widely used by the community.**
- **provide a test suite, comprising unit and integration tests, along with a guide on how to set up and run them.**
- **commit to providing Dockerfiles for the delivery of your project.**
- **indicate milestone duration as well as number of full-time employees working on each milestone.**
- **Deliverables 0a-0d are mandatory for all milestones**, **and deliverable 0e at least for the last one. If you do not intend to deliver one of these, please state a reason in its specification (e.g. Milestone X is research oriented and as such there is no code to test).**

> :zap: If any of your deliverables is based on somebody else's work, make sure you work and publish _under the terms of the license_ of the respective project and that you **highlight this fact in your milestone documentation** and in the source code if applicable! **Teams that submit others' work without attributing it will be immediately terminated.**



### Overview

- **Total Estimated Duration:** Duration of the whole project (e.g. 2 months)
- **Full-Time Equivalent (FTE):**  Average number of full-time employees working on the project throughout its duration (see [Wikipedia](https://en.wikipedia.org/wiki/Full-time_equivalent), e.g. 2 FTE)
- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in Bitcoin, the amount will be calculated according to the exchange rate at the time of payment.

### Milestone 1 Implement Aurora EVM Smart Contracts & Reference Front end

- **Estimated duration:** 1 month
- **FTE:**  2
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 |
| 0b. | Documentation | We will provide: <br/> - **inline documentation** of the code <br/> - **basic user tutorial** that explains how a user can skin their NFT Skinnable using the SRS NFT UI. <br/> - **basic developer tutorial 1** that explains how a developer can implement a Skinnable NFT <br/> - **basic developer tutorial 2** that explains how a developer can implement a custom Skinnable NFT agreement |
| 0c. | Testing Guide | Core functions of solidity contracts will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will not provide docker files for testing as we will be using Remix for solidity development |
| 0e. | Article | We will publish an **article**/workshop that explains SRS NFT  (what was done/achieved as part of the grant). (Content, language and medium will reflect our target audience described above.)|
| 1. | Aurora Smart Contract Suite | We will implment EVM Smart Contracts in Solidity and Provide a Referencce Front End that captures the basic UI, this will also include smart contract documentation and user instructions as above |
 


...
## Future Plans

Please include here

- **how you intend to use, enhance, promote and support your project in the short term, and**
In the short term we will look to promote our infrastructure with NFT buidl communities in the NEAR ecosystem through twitter, telegram and discord 

- **the team's long-term plans and intentions in relation to it.**
In the longer team we will be incorporating this outputs of this project into other projects such as [Web 3 Edi](https://web3edi.com/)


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website & personal recommendation 

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- **Work you have already done.**
- **If there are any other teams who have already contributed (financially) to the project.**
- **Previous grants you may have applied for.**

Our rational for working with Aurora is that we would like to prove the concept on EVM before launching a bigger project to deploy these features natively on the NEAR blockchain. 
