# Mintbase Grant Proposal

- **Project Name:** the **everything** project
- **Team Name:** everything labs
- **Payment Address:** evrything.near (notice the missing e)
- **Level:** 1


## Project Overview :page_facing_up:

### Overview

the **everything** project, centralizing data for the circular economy.
    
Inconveniences and inefficiencies in secondary product markets are creating wasteful consumer habits and stifling innovation in addressing the climate crisis. The solution is to centralize product data online and to create open source white-label applications, marketplaces, and services that can be adapted and serve as the foundation for an interconnected circular economy.
Check out this diagram [here](https://about.everything.dev/diagram) to see what this looks like. 

Through the **everything** app or any of its derivatives, users can upload "Things" and, when marked public or available for sale, these Things will be minted on chain in order to handle transactions and to maintain a history of ownership.
Then, minted things become available on the **everything** marketplace or any of its derivatives, where users can buy, sell, trade, or transfer their real, tangible assets.


### Project Details

- An overview of the project and it's technical architecture: [Link](https://about.everything.dev/diagram)
- An amatuer Figma of the **everything** app: [Link](https://www.figma.com/file/8cL2PwMBPPj2voOOMu54y9/everything-app-v1)
- The original POC submitted to NEAR Megabuild: [Link](https://devpost.com/software/near-frontier)
- A video showing v0 of the **everything** app: [Link](https://www.instagram.com/reel/ChA8Ic0NaRb/?igshid=ZjA0NjI3M2I=)


**NOTE**:

While fraud and item illegitimacy are real issues when pairing NFTs with real, tangible assets, we need to consider that this project is focusing on secondhand and common products, not luxury goods.
Of course there will be some level of protection, but this is a risk that the user needs to acknowledge and the product is not exactly trying to solve.
Eventually, some sort of distribution network (grass-roots supply chain) could serve as an oracle for verifying things.   

### Ecosystem Fit

A [recent study](https://newsroom.thredup.com/news/thredup-releases-10th-annual-resale-report-with-insights-on-a-decade-of-resale) suggests that the secondhand product market has become a global phenomenon and is estimated to more than double by 2026, reaching $82 billion — 3X faster than the global apparel market overall.
However, it is also a very unorganized and disjointed product market: current solutions are not prepared for the mass onboarding of every day objects onto the internet.
1) Thrift stores and small businesses want effective ways to move inventory online without having to maintain listings across multiple different marketplaces. As opposed to Shopify, with everything, they can upload their products and have them not only featured on their own branded site, but automatically listed on the **everything** marketplace.
2) Consumers want a central place where they can search for a product and find the closest and best match, without having to visit multiple marketplaces or visit multiple physical locations.
3) Recycling services want their inflows to be sorted prior to arrival in order to keep down costs.
4) Recycling services want to secure buyers for the recycled materials and consumers want confidence in the sustainability of the products they're buying (the closed loop forming the circular economy).

All of these opportunities need a foundation to build on so things only need to be uploaded once and then can be queried by the many applications, marketplaces, and services individually.

With that said, the **everything** app (and its white-label implementation) features a streamlined process for uploading real, tangible things online. 
The white label implementation can be adapted by anybody for any reason in order to create a personalized application and this application could also need a specific streamlined process for uploading things online.

As long as these applications use the **everything** SDK to upload their things online (which is a necessity for the Thing to be added to the inventory of everything), then Mintbase will be the solution for minting.
    
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?

Outside of the Mintbase/ NEAR ecosystem, there are other projects that are attempting to tokenize real, tangible assets such as [4K Marketplace](https://4k.com/press), but they all focus on real estate or luxury items.
There are also off-chain solutions like [Digimarc](https://www.digimarc.com), but they focus on new products straight from production.

Nobody in any ecosystem is tokenizing common everyday and secondhand items.

## Team :busts_in_silhouette:

### Team members

Project Lead and Developer: Elliot Braem

Marketing & PR: Danielle Dvorchak

Accounting & Junior Developer: Bobby Rein

Also associated with design studio [Akaer.Studio](https://www.akaer.studio) for business development, graphic design, and more. 


### Contact

- **Contact Name:** Elliot Braem
- **Contact Email:** elliot@everything.dev
- **Website:** [about.everything.dev](https://about.everything.dev)

### Legal Structure

- **Registered Address:** 16192 Coastal Highway, Lewes, Delaware 19958 USA
- **Registered Legal Entity:** everything labs LLC

### Team's experience

For three years, I designed, developed, and scaled critical applications for a large asset management bank's fund accounting.
I've been exploring technologies and building out several proof of concepts for **everything** for the better part of the year.
The team is based in Brooklyn and has connections to thrift stores, fashion activist organizations, and secondary market organizations in NYC.



### Team Code Repos

- https://github.com/near-everything
- https://github.com/near-everything/app (building new mobile app out on feature/native-app branch)
- https://github.com/near-everything/marketplace
- https://github.com/near-everything/dashboard

(currently have the api private)

(subgraph and smart contracts repos exist, but this would be replaced with Mintbase)

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/elliotBraem

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/elliot-braem/


## Development Status :open_book:

This project has been an effort since the beginning of the year, starting with [this submission](https://devpost.com/software/near-frontier) into NEAR Megabuild. It has gone through several prototypes and redesigns while the idea has been explored and fine-tuned.
An example of a redesign is [v0](https://www.instagram.com/reel/ChA8Ic0NaRb/?igshid=ZjA0NjI3M2I=) of the everything app which was integrated with NEAR and had a POC marketplace querying from the subgraph.

The team is now building v1-beta of the everything app following the adjustments that have been made since v0. 

Follow the progress on [Tik Tok](https://www.tiktok.com/@everything.project?lang=en) or [Instagram](https://www.instagram.com/evrythingprject/?hl=en).


## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 2 months
- **Full-Time Equivalent (FTE):**  1.5 (will be actively recruiting contribitors)
- **Total Costs:** 10,000 USD

### Milestone 1 — SDK Development

- **Estimated duration:** 1 month
- **FTE:**  1.5 (will be actively recruiting contribitors)
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a developer can use the SDK in their own application to mint "things", pull public data from the indexer api, and request additional details from the express api. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 1. | SDK Development: Creating Things  | We will build functionality that allows developers to use our SDK to create Things in the off-chain database and mint records of these using Mintbase (if marked public) |  
| 2. | SDK Development: Saving media | We will build functionality that uploads media to a distributed file system (IPFS)  |  
| 3. | SDK Development: Reading private data | We will build functionality that enables developers to read private data from the off chain database   |  

### Milestone 2 Example — SDK Integration with the everything app

- **Estimated duration:** 1 month
- **FTE:**  1.5 (will be actively recruiting contribitors)
- **Costs:** 4,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a developer can clone and adapt the **everything** app for their own specific purpose. |
| 0c. | Testing Guide | Core functions of the app will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 1. | UI Design  | We will hire a product designer to design the core functionalities of the everything app: User can onboard a Thing, User can upload Media, User can view their Things, User can view other user's public things. (ideally this would start during milestone 1)  |
| 2. | Figma Template | We will provide a Figma template that can be adapted to any purpose relevant to the **everything** app. |
| 1. | SDK Integration: Creating Things  | We will integrate the **everything** SDK into the Thing onboarding functionality. Users can mark a Thing to be public or private. Public things are minted on chain via Mintbase, private things are not. |  
| 2. | SDK Integration: Saving media | We will integrate the **everything** SDK into the media upload functionality during creation of a Thing. Media will be stored on a distributed file system.  |  
| 3. | SDK Integration: Reading private data | We will integrate the **everything** SDK into the users profile so they can read their private data using their access token from the identity provider. Other users will not be able to access this data.  |   
| 4. | Full UI Integration | All SDK developments above have been developed to the **everything** app. Users will be able to create things privately and publicly via the app. If private, then the thing will be saved off-chain only, and if public, then the thing will be saved off-chain as well as minted via Mintbase. Users will be able to upload media attached to these things and this Media will be stored in a distributed file system via the SDK. Users will be able to see their privately uploaded things on their profile. | 

...
## Future Plans

I do daily standups on [Tik Tok](https://www.tiktok.com/@everything.project?lang=en) and [Instagram](https://www.instagram.com/evrythingprject/?hl=en) and will be producing other videos as well. Through these channels I hope to gain support and recruit contributors.

For long term plans, see the [current roadmap](https://about.everything.dev/roadmap).



## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Attended "Creating business models with NFTs" presented by Mintbase COO Paul Kuveke, hosted by EmpireDAO.

I found out about this grant program yesterday and have rushed to put together this proposal all last night and today. Please mind any shortcomings although I tried to be as thorough as possible. Thank you for considering my application and please let me know if there is any more information needed.
