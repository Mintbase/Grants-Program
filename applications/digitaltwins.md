# Mintbase Grant Proposal
**Project Name:** Digital Twins

**Team Name:** Deluxe Knives

**Payment Address**: deluxeshop.near

**Level:** 1

[![Twitter Announcement Video](https://cdn.discordapp.com/attachments/617838827980980225/1044802239870533703/Screenshot_2022-11-22_at_8.30.56_PM.png)](https://twitter.com/DeluxeKnives/status/1593726336144740352) 

## Project Overview :page_facing_up: 

### Overview
**Tag Line:**  
Easy 1 to 1 NFT to physical product sales with Mintbase & Shopify

**Project Description:**  
Deluxe Knives has already built a store frontend & backend that integrates with Shopify. Our model allows users to seamlessly redeem their NFT for a Shopify discount code that gives them a physical product for free: in just a few clicks!   

What we call digital twin NFTs are NFTs that are 1-to-1 with a physical product. We believe that these digital twins can become an easy way to add value to physical products, because NFTs as a base unit can have additional virtual functionalities added to them over time (virtual environments, gamification, easy resale).  

Digital twins are difficult to pull off seamlessly due to the disconnect between Web2 and Web3, which is where we come in. In 3 steps, the user can get their physical item fulfilled via Shopify & Mintbase:  

1. User purchases NFT on frontend via Mintbase SDK
2. User generates a custom Shopify discount code, using our custom Shopify app backend
3. User gets directed into a Shopify cart with the product & a 100% off discount applied 

In just 3 steps, in less than 1 minute, a user can get a digital twin and a physical product using Mintbase!  

With our open source and well documented backend repository, any other Shopify store can come to NEAR's ecosystem with a digital twin store, powered by Mintbase.  

**Goal:**    
We aim to improve on the infrastructure that we have already built to make it even easier for Shopify e-commerce stores to come to Mintbase to sell digital twin NFTs.  
  
Our software is currently already open source and can be used by anyone. However, the setup is difficult for non-technical users. The goal of this grant application is to improve the user experience for anyone who wants to use digital twins with their Shopify store.  

The current flow for a Shopify owner to set this up is as follows:  

1. Create a Mintbase store
2. Mint NFTs with Shopify related metadata injected into them
3. Set up a MongoDB account & database
4. Set up AWS Linux Server (or something similar)
5. Clone the custom app repository, and inject environment variables
6. Install NGINX & an SSL certificate so that the API can be accessed over HTTPS

While the user experience for the purchasing & redeeming digital twins is great, setting up is a hassle for every Shopify developer. We aim to fix that during this Mintbase grant round.  

The planned improvements are:
1. Make the location of metadata modular, so that previous NFTs can integrate into the plugin
2. Dockerize the backend to easily spin it up and go straight into production (medium technical skill requirements)
3. Firebase"ize" the backend so that users who want an easier serverless option can spin it up equally fast (low technical skill requirements)
4. Make a centralized version of the API so that no server needs to spun up (zero technical skill requirements)

**How your project relates to / integrates into the Mintbase / NEAR ecosystem:**

This project relies on NEAR & Mintbase infrastructure for the minting and selling of digital twin NFTs. It will benefit NEAR & Mintbase because it will improve the utility that the platforms can offer their users by introducing viable e-commerce.

**Why your team is interested in creating this project:**

Deluxe Knives has spent a lot of time and energy getting the project to where it is. We hope that other stores can use the same system that we've pioneered because we think that has greater potential in other markets like fashion.

### Project Details:

There are 2 goals with the backend. First is to make it easy for a wide variety of Shopify owners, from those with technical abilities to those who have none. Second is to give many options to users so that they can decide how to do it themselves.

1. Dockerize the backend to easily spin it up and go straight into production (medium technical skill requirements)
2. Firebase"ize" the backend so that users who want an easier serverless option can spin it up equally fast (low technical skill requirements)
3. Make a centralized version of the API so that no server needs to spun up (zero technical skill requirements)

![Flow Chart](https://cdn.discordapp.com/attachments/617838827980980225/1046623867982069780/Screenshot_2022-11-27_at_9.09.31_PM.png)

In this flow chart, we are trying to illustrate the modularity of the backend that we are trying to achieve with this application.  


**Technology stack:**
- Backend:
	- MongoDB
	- AWS S3
	- Express.js
	- Node.js
	- NEAR SDK
    - Shopify SDK
    - Docker
    - Firebase
  
- Frontend (developed, but not included in goals):
	- Next.js
	- Mintbase SDK

### Ecosystem Fit

**Where and how does your project fit into the ecosystem?**
We believe that Web3 will act as a layer on top of Web2 

**Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's user base, yourself)?**
For this grant, we are shifting our target audience to 2 customer segments: Shopify stores & NFT projects.  

Shopify stores: these are Shopify powered stores that sell novelty and fashion, which we think will translate into NFTs easily. We will provide tooling and services to these stores so that they can use Mintbase as an NFT platform. We are currently a Shopify store, and are using this system to sell knives.

NFT projects: NFT projects that want to add additional value to their project can opt to use Shopify stores to sell merchandise while still retaining the use of Web3 for their digital payments & transactions.

**What need(s) does your project meet?**
We provide the ability to easily sell physical products using Web3. There is no such thing as completely decentralized stores when it comes to physical products, because there will always have to be a trusted actor that sends an item from point A to point B. This is why Web3 must build upon Web2 infrastructure, like our project does.

**Are there similar projects in related ecosystems?**
There are projects that have digital twins, but they aren't as efficient as our strategy. See the following chart:  

![Competitors](https://res.cloudinary.com/devpost/image/fetch/s--PTlMGgaQ--/c_limit,f_auto,fl_lossy,q_auto:eco,w_900/https://cdn.discordapp.com/attachments/426940183112318976/1041939066599514182/DK_Comparison_Infographic.png)

All of the NEAR projects listed have very inconvenient fulfillment channels. Users definitely do not want to have to send an email or join a Discord channel and manually ask for a product that they already bought. Imagine having to send an email every time that you bought something on Amazon!

Of the other projects that we've listed, some of them have better user experiences, such as BlockBar, a liquor NFT marketplace. But the developer experience is still comparatively worse than what it could be. This company has to create their own order management, fulfillment, and marketing channel workflow because they aren't integrated with e-commerce solutions like Shopify that have already built it out. Large companies like the GAP & Adidas can afford to pay for specialized Web3 developers, but most e-commerce stores can't.

We believe that our solution provides the most simplicity for developers and customers alike.

## Team

**Contact name:** Jeremy B  
**Contact Email:** [jeremy@projk.net](mailto:jeremy@projk.net)

While we have a larger team, other developers and members who have been associated with Deluxe Knives are only intermittently committed to developing out the Web3 tooling or are only supporting on the Shopify side.  

 **Team Code Repos:**

- https://github.com/DeluxeKnives/mintbase-shopify-redemption-backend

- https://github.com/DeluxeKnives/dk-frontend

## Development Status:
Initial development is complete. We have a mainnet frontend that works: it sells NFTs, which can be redeemed for discount codes on Shopify for a free knife. We encourage you to [check it out](https://digitaltwin.deluxeknives.com/), and maybe buy a knife.  

We also have [complete documentation](https://github.com/DeluxeKnives/mintbase-shopify-redemption-backend/wiki) for the backend, which is the custom shopify app. In theory, any Shopify store could use the method that we developed, for free. It's all open source.   
  
That being said, there are some user experience issues that need to be resolved to truly take this to as many people as possible.

## Development Roadmap:

### Overview

-  **Total Estimated Duration:** 1.5 months

-  **Full-Time Equivalent (FTE):** 1.5 FTE

-  **Total Costs:** 10,000 USD

### Milestone 1 - QoL, Dockerize, Firebase-"ize" Backend

-  **Estimated duration:** 2-3 weeks

-  **FTE:** ~.5 FTE  

-  **Costs:** 4,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | BSD-3 |
| 0b. | Documentation | There will be external documentation both in-line and in a Github wiki on how to utilize what was developed. |
| 0c. | Docker | We will provide a Dockerfile that can be used to spin up all functionality. |
| 1. | Configurable Metadata | Projects will be able to configure where their Shopify metadata is stored (on-chain, in a smart contract, or in a database). |  
| 2. | Dockerized | We will Dockerized the current setup so that the database (MongoDB) and backend (custom Shopify app) can be set up easily. |  
| 3. | Firebase | We will refactor the code to make it so that the database and logic are loosely coupled, and then create a Firebase version of the backend. |  
  
### Milestone 2 — User-Friendly Backend
  
-  **Estimated duration:** 3-4 weeks

-  **FTE:** 1 FTE  

-  **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | BSD-3 |
| 1.  | Frontend | A simple, functional hosted frontend that allows you to log in with NEAR and make changes to your configuration. |
| 2.  | Central Backend | A hosted backend & database that all Shopify users can use easily to set up a digital twin store. Each user who connects to the site will have a shared, clustered MongoDB database created for them. |

### Future Plans:

-   Integration with GorillaShops (in progress, will not be part of future grants)

-   Our code allows for the NFT -> Physical Item flow, but we also want to power the alternate direction. Purchase physical items, and then get their virtual NFT. This will be more difficult due to the need to work with Shopify's frontend system, the management and creation of new accounts for users not acquainted with Web3, and the question of gas.

-   Multichain support via Wormhole

-   E-Commerce -> Web3 Incubation

### Additional Information:

**How did you hear about the Grants Program?**  
From the NEAR Website.
