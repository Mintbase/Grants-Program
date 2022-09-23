# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.
- **Project Name:** Exxaverse
- **Team Name:** Exxaverse LLC
- **Payment Address:** exxaverse.near
- **[Level](../README.md#level_slider-levels):** 1, 2 or 3

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*
## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- If the name of your project is not descriptive, a tag line (one sentence summary).
A cyber gladiator fighting game built with Unreal Engine and a blockchain backend.

- A brief description of your project.
Exxaverse rewards players for their time via sponsored tournaments and in-game wagering. We enable them to be in control of their game assets with a liquid marketplace to sell their assets directly. Exxaverse lowers the cost of gaming entertainment.

- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.
We are utilizing Near blockchain for our in-game assets and currency.

- An indication of why your team is interested in creating this project.
To increase the functionality of Nearlinker: https://github.com/ExxaVerse/NearLinker

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
- Data models / API specifications of the core functionality
- An overview of the technology stack to be used
- Documentation of core components, protocols, architecture, etc. to be deployed
- PoC/MVP or other relevant prior work or research on the topic
- What your project is _not_ or will _not_ provide or implement
  - This is a place for you to manage expectations and to clarify any limitations that might not be obvious

Integrate Mintbase API into Nearlinker to improve functionality related to reading and writing Near blockchain data. Making it easy for Unreal Engine developers to integrate blockchain services into their games.

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
- What need(s) does your project meet?
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems?

Our project is already being used by game developers in the Near ecosystem. Your team can checkout who forked our repo: https://github.com/ExxaVerse/NearLinker/network/members

The funding will allow us to integrate your API, improve the infrastructure, and bring new game devs to Nears ecosystem.

## Team :busts_in_silhouette:

### Team members

- Name of team leader
Josh Margulies

- Names of team members
Liviu Berciu
Will Schneider

### Contact

- **Contact Name:** Full name of the contact person in your team
Josh Margulies 

- **Contact Email:** Contact email (e.g. john@duo.com)
josh@exxaverse.com

- **Website:**
exxaverse.com

### Legal Structure

- **Registered Address:** Address of your registered legal entity, if available. Please keep it in a single line. (e.g. High Street 1, London LK1 234, UK)
1991 Crocker Road, Suite 600a
Westlake, OH 44145

- **Registered Legal Entity:** Name of your registered legal entity, if available. (e.g. Duo Ltd.)
LLC

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Mintbase previously, please list the name of the project and legal entity here.

Liviu is a polyglot software engineer with experince ranging from low-level security and hypervisors to higher-level web-pages and applications. He was worked with big companies such as Amazon, Garmin, and Bitdefender.
Will is a project manager and web developer. He also worked with Liviu to complete the first iteration of NearLinker.

### Team Code Repos

- https://github.com/ExxaVerse
- https://github.com/ExxaVerse/NearLinker

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/berciuliviu
- https://github.com/willschneider15

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/joshuastonemargulies/

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),
- academic publications relevant to the problem,
- links to your research diary, blog posts, articles, forum discussions or open GitHub issues,
- references to conversations you might have had related to this project with anyone from the Mintbase Foundation,
- previous interface iterations, such as mock-ups and wireframes.

This is all contained within the Readme: https://github.com/ExxaVerse/NearLinker/blob/main/README.md

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

- **Total Estimated Duration:** ~6 months
- **Full-Time Equivalent (FTE):** 2 part-time
- **Total Costs:** $150,000

### Milestone 1  — Implement Mintbase Modules

- **Estimated duration:** 3 month
- **FTE:**  1
- **Costs:** 50,000 USD

We will finish integrating API endpoints to read and write data from blockchain using Mintbase.

### Milestone 2 - Security Testing

- **Estimated Duration:** 4 month
- **FTE:**  1
- **Costs:** 100,000 USD

We will pay for penetration testing on Nearlinker to make sure it is secure.

...
## Future Plans

Please include here

- We plan to implement Nearlinker into our own game.
- We plan to onboard new games into the Near ecosystem.


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.

We have already recieved a grant from Near foundation for Nearlinker. We talked to your team at Nearcon.
