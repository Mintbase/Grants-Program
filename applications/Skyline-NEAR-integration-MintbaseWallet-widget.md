# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** Skyline Digital
- **Team Name:** Skyline Digital AG
- **Payment Address:** skylinedigital.mintbase.near
- **[Level](../README.md#level_slider-levels):** 3

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- Skyline Digital offers non-custodial banking transactional services (on/offramp + 3rd Party payments)
- We want to integrate the NEAR network into Skyline to allow transactions using your network
- We also want to integrate our API directly into the Mintbase wallet allowing users to on/offramp and pay anyone directly from their wallet using fiat currencies
- We want to easily help open access to traditional banking services to anyone on and off-chain via a non-custodial way

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
- API documentation - https://skyline-digitial.gitbook.io/skyline-digital-api 
- The technology stack is divided into two categories:
	- NEAR network Integration: JavaScript, NearJS.
	- Mintbase Wallet Integration: JavaScript, React, MintbaseJS, NearJS, Skyline API.
- Integrating NEAR in the Skyline platform will not require a specific deployment as it will live in Skyline's current frontend stack.
- For Mintbase Wallet Integration the Widget will be deployed as a static website (JS, CSS, HTML) and probably loaded.
- Relevant to point out in terms of research will be that we have already in place an ERC20 “withdrawal logic” to process payment, this will need to be reflected while using the NEAR network with the corresponding if exists ERC20.
- What your project is _not_ or will _not_ provide or implement
- This is a place for you to manage expectations and clarify any limitations that might not be obvious
  
### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
Our product will allow mintbase wallet holders to easily on/offramp and/or pay anyone anywhere in fiat currencies using their tokens.

- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
Our audiences tend to be Businesses and HNWI but we also have a retail side for individual users.
Our main focuses are around DAOs, Foundations and Crypto companies but we also cater to regular retail crypto investors or users.
Mainly we would like to open our products to Mintbase wallet holders.

- What need(s) does your project meet?
Easy, quick and highly compliant on/offramping and 3rd party payments in any fiat currency worldwide (pay invoices, suppliers, contractors, taxes or anything else)

- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems?
Alchemy Pay and Trasak - but these are primarily onramping tools
We would offer both on/offramping and more importantly 3rd Party Payments allowing users to pay anyone for anything without having to first offramp into their bank account and then make a payment.


## Team :busts_in_silhouette:

### Team members

- Sebastiao Queiroz e Mello - CEO
- Matthieu Gueissaz - COO
- Filippo Minder Sotto Mayor Matoso - CPO
- Andre Fatia - CTO
- Joao Martins - Engineer
- Gonçalo Patrocínio - Engineer
- Bruno Martins - Designer

### Contact

- **Contact Name:** Filippo Sotto Mayor
- **Contact Email:** f@skylinedigital.xyz
- **Website:** https://www.skylinedigital.xyz/

### Legal Structure

- **Registered Address:** Baarerstrasse 82, 6300 Zug, Switzerland
- **Registered Legal Entity:** Skyline Digital AG

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Mintbase previously, please list the name of the project and legal entity here.

### Team Code Repos

- https://github.com/Skyline-Digital-AG

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/Fatxx
- Pending other links

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/squeirozmello
- https://www.linkedin.com/in/matthieu-gueissaz-4a50298b 
- https://www.linkedin.com/in/filipposottomayor 
- https://www.linkedin.com/in/andr%C3%A9-fatia-60b11058 
- Joao Martins - Engineer (missing)
- https://www.linkedin.com/in/gon%C3%A7alo-patroc%C3%ADnio-4a40661b4 
- https://www.linkedin.com/in/bruno-martins-29728b60 
- Telmo Ferreira - Engineer (missing)
- https://www.linkedin.com/in/juan-cruz-mazzochi-18301a1a4 
- https://www.linkedin.com/in/cintiatcosta 

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

- **Total Estimated Duration:** 6 months
- **Full-Time Equivalent (FTE):**  2 developers + 1 designer + 1 PM/QA
- **Total Costs:** 50,000 USD

### Milestone 1 — Integrate with NEAR network

- **Estimated duration:** 3 FTE - dev + designer + (PM/QA)
- **FTE:**  2 Months
- **Costs:** 20,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can use the Skyline Platform to on/offramp AND to do 3rd party payments |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | Yes |
| 0e. | Article | We will create a blog + social media posts announcing this feature |
| 1. | NEAR chain integration | We will integrate the NEAR chain so that users can connect with NEAR wallets and complete transactions within the Skyline ecosystem. We will also adapt existing UX/UI to offer this |  
| 2. | Increase token acceptance | We will increase the tokens we accept on Skyline ecosystem to include main NEAR tokens such as nUSDC, nUSDT + USN |  

![aaa](https://github.com/Skyline-Digital-AG/Grants-Program/assets/575479/54538e33-ea4b-40a0-a8c9-79489ad42daa)

### Milestone 2 — Open Source Widget

- **Estimated Duration:** 3 month
- **FTE:**  3 FTE - frontend dev + designer + (PM/QA + Designer)
- **Costs:** 30,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the code and documentation on how someone can use or integrate the Widget |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. |
| 0d. | Docker | Yes |
| 0e. | Article | We will create a blog + social media posts announcing this feature |
| 1. | Widget UX/UI design + development | We will design and create a widget to be used anywhere as a window into the Skyline ecosystem allowing customers to access our services directly from the Mintbase wallet. Services include: Onboarding, Login, 3rd party payments in fiat currencies |  
| 2. | Integrate widget into Mintbase wallet | We will create a Mintbase module that will load our widget to access Skyline services directly |  
