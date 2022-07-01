# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- MEXICAN ART MACHINE (M.A.M)
- M.A.M Labs
- mexicanartmachine.near
- Level 2

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- MEXICAN ART MACHINE
- We're building a bridge between traditional Art and NFTs. With a platform, the artists we'll be able to mint their art, and display them in a virtual gallery. Through our DAO (AstroDAO), artists will be able to certificate their art.
- Built on the NEAR Ecosystem. We are working with Mintbase for the minting, 3XR fro the Virtual Gallery and various tools from Gorilla Shops
- We're Blockchain and Art aficionados. Trying to merge the 2 worlds

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
It’s already live at: www.mexicanartmachine.com , we’re in the process of adding the minting and gallery tools which will have an impact in the UI, the platform is still in progress.

- Data models / API specifications of the core functionality
Check Guerrilla Shops, they developed the tools we’re using

    Mintbase Datamodel for Minting NFT’s, fetching data and for Gorilla Shop.

    Mintbase REST + GraphQL API using:
-Fetch Store by ID.
-Fetch All Things / Fetch Thing.
-Fetch All Tokens / Fetch Token. 
-Fetch Marketplace.
-Fetch Token Approval on Contract.
-Fetch Account.

Mintbase API for 3XR generating a gallery from the store.

Gorilla Shops for displaying NFT’s and prices.

- An overview of the technology stack to be used
    API REST Calls from mintbase to retrieve all the metadata and in-chain requirements for transactions. 3XR Gallery for the display of the collections using the same API to mint tokens and further create the rooms where these will be displayed. 

    3XR Gallery using mintbase decentralized API to gather the information and data from the NFT’s being displayed.

    Gorilla Shops uses mintbase shop to display the sale products and provide new aesthetics and easier payment methods for the NFT’s and tokens.

- Documentation of core components, protocols, architecture, etc. to be deployed
Gorilla Shops:
    Works as a bridge between mintbase shops and gorilla shops adding new features such as PFP and also to refresh the aesthetics and view of the shop. https://medium.com/@gorillashops/how-to-set-up-your-own-nft-shop-810685bf491d

    Mintbase REST + GraphQL Documentation: 
    https://docs.mintbase.io/dev/getting-data/rest-+-graphql

    3XR Gallery:
    https://create.3xr.space/

- PoC/MVP or other relevant prior work or research on the topic
- What your project is _not_ or will _not_ provide or implement

  - This is a place for you to manage expectations and to clarify any limitations that might not be obvious

We’re an Art Foundation dedicated to on-board artists into NFT’s. Based on that we’ll use our strengths in marketing, community development and social reach for building a platform that can reach a lot of users/artists and introduce them into the Web 3.0 space. Using a 3 way formula:
a.	Collaborative tools already working inside the NEAR Ecosystem
•	Mintbase; Marketplace, Minting Tools
•	3XR; Virtual Gallery
•	Gorilla Shops; PFP Creator, Commercial Individual Web-Pages


b.	Out-Sourcing
•	Web 3.0 Programmers


c.	M.A.M Labs
•	Marketing
•	Community
•	Artists On Boarding
•	Events
•	Content Creation & Curation

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Art Platform
- All Artists. Own user base
- Art promotion and certification
- Similar Art marketplaces. We have a different approach. Easy tools for any artist. Trendy promotion. Certification.

## Team :busts_in_silhouette:

### Team members

- Juan Diego Lebrija
- Enrique Guasp

### Contact

- Juan Diego Lebrija Céspedes / Enrique Guasp
- hey@mexicanartmachine.com
- www.mexicanartmachine.com

### Legal Structure

- Sombrerete 502, Condesa. Cuauhtemoc CDMX

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Web3 Foundation previously, please list the name of the project and legal entity here.

### Team Code Repos

- https://github.com/<your_organisation>
- https://github.com/<your_organisation>/<project_1>
- https://github.com/<your_organisation>/<project_2>

N/A

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/<team_member_1>
- https://github.com/<team_member_2>

### Team LinkedIn Profiles (if available)

- linkedin.com/in/enriqueguasp
- linkedin.com/in/jdlebrija

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),
- academic publications relevant to the problem,
N/A
- links to your research diary, blog posts, articles, forum discussions or open GitHub issues,
N/A
- references to conversations you might have had related to this project with anyone from the Mintbase Foundation,
Regina Nogueira


- previous interface iterations, such as mock-ups and wireframes.
www.mexicanartmachine.com has the latest version of the platform

## Development Roadmap :nut_and_bolt:

This section should break the development roadmap down into milestones and deliverables. To assist you in defining it, we have created a document with examples for some grant categories [here](../docs/grant_guidelines_per_category.md). Since these will be part of the agreement, it helps to describe _the functionality we should expect in as much detail as possible_, plus how we can verify and test that functionality. Whenever milestones are delivered, we refer to this document to ensure that everything has been delivered as expected.

Below we provide an **example roadmap**. In the descriptions, it should be clear how your project is related to Mintbase. We _recommend_ that teams structure their roadmap as 1 milestone ≈ 1 month.

For each milestone,

- make sure to include a specification of your software. _Treat it as a contract_; the level of detail must be enough to later verify that the software meets the specification.
- include the amount of funding requested _per milestone_.
$5,000 per milestone
- include documentation (tutorials, API specifications, architecture diagrams, whatever is appropriate) in each milestone. This ensures that the code can be widely used by the community.
No one can use our code. Mexican Art Machine is a brandbuilding company, we focus on validating physical artwork with the NFT Pair.
- provide a test suite, comprising unit and integration tests, along with a guide on how to set up and run them.
N/A
- commit to providing Dockerfiles for the delivery of your project.
N/A
- indicate milestone duration as well as number of full-time employees working on each milestone.
Each milestone will depend on the growth of the company, as our main focus is to onboard traditional artists, our milestones will be achieved in relation to the growth of the company
- **Deliverables 0a-0d are mandatory for all milestones**, and deliverable 0e at least for the last one. If you do not intend to deliver one of these, please state a reason in its specification (e.g. Milestone X is research oriented and as such there is no code to test).
Only the platform has code to test, and it’s already tested on Guerilla Shops
> :zap: If any of your deliverables is based on somebody else's work, make sure you work and publish _under the terms of the license_ of the respective project and that you **highlight this fact in your milestone documentation** and in the source code if applicable! **Teams that submit others' work without attributing it will be immediately terminated.**

### Overview

- **Total Estimated Duration:** Duration of the whole project (e.g. 2 months)
5 months

- **Full-Time Equivalent (FTE):**  Average number of full-time employees working on the project throughout its duration (see [Wikipedia](https://en.wikipedia.org/wiki/Full-time_equivalent), e.g. 2 FTE)
2 FTE to 4 FTE

- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. This and the costs for each milestone need to be provided in USD; if the grant is paid out in Bitcoin, the amount will be calculated according to the exchange rate at the time of payment.
$45,000 USD

### Milestone 1 — Implement Mintbase Modules

- **Estimated duration:** 2 months
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
Finished platform
| 1. | Mintbase module: X | We will create a Mintbase / NEAR module that will... (Please list the functionality that will be implemented for the first milestone) |  
The first module will allow approved artists to mint their artwork through our platform (Guerilla Shops tool on Mintbase) with the M.A.M certification.
| 2. | Mintbase module: Y | We will create a Mintbase / NEAR module that will... |  
The second module will allow artists to generate a virtual shop of their collection (Guerrilla Shops-Mintbase tools)
| 3. | Mintbase module: Z | We will create a Mintbase / NEAR module that will... | 
The third module will allow users to create their PFP projects through the Guerrilla Shops-Mintbase tool 



### Milestone 2 — Content Creation

- **Estimated Duration:** 1 month
- **FTE:**  2
- **Costs:** 6,000 USD

Through brand awareness in social media, we will use digital content to position ourselves as the main art validator machine.

Concept art and content
Brand Strategy
Viral Ads


### Milestone 3 — Brand activation 

- **Estimated Duration:** 2 month
- **FTE:**  3
- **Costs:** 15,000 USD

We will focus on activating galleries in Mexico City creating a community of emerging artists interested in entering the NFT space. 
We plan to make a monthly event with workshops, conferences and meetups where artists and enthusiasts can learn about the crypto space and how to onboard themselves and mint and sell their art on our platform. All this closing partnership deals with mexican galleries for NFT projects.

### Milestone 4 Example — PFP membership
| Number | Deliverable | Specification |

- **Estimated Duration:** 1 month
- **FTE:**  4
- **Costs:** 12,000 USD

M.A.M PFP Collection (Mintbase and Gorilla Shops)
First Membership Airdrop
M.A.M PFP Merch Release
Event Launch

### Milestone 5 Example — DAO Release
| Number | Deliverable | Specification |

- **Estimated Duration:** 1 month
- **FTE:**  4
- **Costs:** 6,000 USD

Revolution DAO Release
Brand Establsihment


- how you intend to use, enhance, promote and support your project in the short term, and
the team's long-term plans and intentions in relation to it.
Through brand building and consolidating our reputation, we will position ourselves as the main NFT validators of Mexican Art, in the short term we want to create hype and raise expectations of the NFT technology and it’s advances, our long term plan is to collaborate with museums and art foundations and provide them with the necessary tools for the art tokenization

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Web3 Foundation Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.
Regina Nogueira from Mintbase

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
·First NFT Gallery in Mexico City, December 2021
·Art Gallery Exhibition during Art Week in Mexico City, Feb 2022
·Residency at the Museum of the Future in Mexico City, Abril-June  2022
- If there are any other teams who have already contributed (financially) to the project.
Juan Diego Lebrija
Enrique Guasp

- Previous grants you may have applied for.
·Near Foundation Creator Grant 

![image](https://user-images.githubusercontent.com/106277490/176938495-28ec1a5e-ab6f-410b-982b-57b22cbc3ca9.png)
