# Mintbase Grant Proposal

- **Project Name:** Partage Lock
- **Team Name:** Partage
- **Payment Address:** partage-lock.near
- **Level:** 1 (open to 2)

## Project Overview :page_facing_up:

### Overview

- **Tag line:**
Keyless Solutions For Seamless Sharing


- **Brief Description:**
Partage allows property owners to make money by sharing their properties. It uses advanced smart locks that co-owners or temporary users can open easily with their smartphones, using NFC or Bluetooth signals. This system simplifies access control and monetization of shared properties.

- **How does Partage integrate into the Mintbase / NEAR ecosystem?**
Powered by the Mintbase templates, the Partage app utilizes Mintbase smart contracts to mint the access token and make market-available smart locks compatible with Near tokens. Our mission aligns with Mintbase's vision, answering the call for an application that facilitates the unlocking of gated content and exclusive unlockables on the NEAR ecosystem.

- **Why is our team interested in creating this project?**
Partage aims to contribute towards a better future by championing the sharing economy and optimizing resource allocation and functionality. Partage Lock achieves this by facilitating the monetization of idle assets through shared uses and encourages a shift towards responsible ownership to limit overproduction and overconsumption. 

Our choice to build Partage on the NEAR Protocol reflects our commitment to sustainability and reliability. Partage’s vision for the web3 sharing economy actively supports social and sustainable goals by fostering a connected community centered around collaborative ownership economies. In alignment with Mintbase's vision, the platform is spearheading an innovative application on the Near blockchain, contributing to pushing utility NFTs to their next level by making unlockables and gated content easy for Near users. 

The use of NFC and Bluetooth technologies ensures heightened digital security for all since all standard smartphones on the market already use them. 

### Project Details

- **Mockups/designs of any UI components**
  - A full design of our marketplace is available here: 
https://www.figma.com/file/D4u5WSOO1Tq2D7BNy7i6zv/Partage-NFT-Marketplace?type=design&node-id=1647%3A17907&mode=design&t=Zl0EjoPvRDlmdFIw-1 
  - Implemented in a demo version here:
https://hellopartage.xyz 

The above marketplace MVP was built 9 months ago and we are now working on integrating the latest Partage Lock MVP to deliver temporary access keys to a network of smart locks. 

  - A demo version of the first Partage Lock app is available here: https://lock.hellopartage.xyz 
  - We will integrate Mintbase template smart contracts to mint access tokens: https://shorturl.at/qANW3 
  - An example of a temporary access token (an NFT including start date and end date in its metadata) has been minted using Mintbase and is available on partage-lock.testnet wallet: https://www.mintbase.xyz/meta/moments.mintbase1.near:5e2027fd30e5f0dfb26dd3b3e005eeed 

- **Data models / API specifications of the core functionality**
We bring the Partage Lock to its next level of functionality and potential for market deployment through a signed partnership with Master Lock. Master is one of the world's most renowned lock brands, and they released a series of popular devices equipped with Bluetooth technology https://www.masterlock.com/products/bluetooth-electronic-locks?personal. All models from this series (padlocks of various sizes, pin pads, and door controller) are working with the same background technology, which we are integrating as a partner using their SDK and APIs. 

In short, Partage is using Mintbase tokens to control access to market-available Bluetooth devices from Master Lock. In a later step of development, we plan to integrate the Partage Lock app with other smart lock brands to cover a wider market share and various industries. We contacted 59 brands in October and a dozen already confirmed their interest in a partnership by sharing access to their API (including Akiles, Tedee, 2N, Brivo, Igloohome, Assa Bloy, Nuki, Yale, Schlage…)

- **An overview of the technology stack to be used**
We are using a usual NextJs/React stack to build our app with a Near backend smart contract written in Rust, and a Javascript/css frontend. 

- **Documentation of core components, protocols, architecture, etc. to be deployed**
More details about our mission, history, and product are available in our white paper: https://medium.com/partage_xyz/partage-white-paper-v2-c0cbea46e2f8 

  - A slide deck for Partage’s general presentation is available here: https://docs.google.com/presentation/d/1-TpH2HP_HUyWBBmEq4RhhOUuiOw2xbeve7yN7WEi1S8/edit#slide=id.p 

  - It features the following architecture sketch:![Partage architecture](https://pbs.twimg.com/media/GBomJUFXkAA0qx4?format=jpg&name=4096x4096)

  - A technical slide deck is used by our dev team to reach our deadlines (Jan 24th, pitch to Encode Club x Near Horizon accelerator). It is available here: [dev pres Jan. 24](https://docs.google.com/presentation/d/1bfBZw7jXl_bvPlXOd2CdYmqmeSxj4JcRtHBPe2AC1OA/edit?usp=sharing) 
 
- **PoC and research on the topic:**
The relevance of developing a network of blockchain-controlled smart locks for adding a layer of shared utilities to urban infrastructure has been described in a medium article, rooted in academic research on Blockchain for Smart Cities (our CEO’s background studies). In short, the research states that the essence of cities has always been to share utilities between a gathering of inhabitants, and urban areas could be the context for mass adoption if citizens had a purpose to use it. Blockchain would help democratize access to a common resource or a shared utility within the daily functioning of an average city, that would be available through the most commonly used device, a smartphone.
The whole article is available here: https://medium.com/partage_xyz/democratizing-access-to-utilities-blockchain-for-smart-cities-25eefb0348e7. 

Another medium article presents a strategy for Partage Lock deployment rooted in another academic field called “citizen science”. It covers examples of successful citizen-led deployments of connected devices in our cities and provides ideas for building partnerships with smart cities and universities to successfully deploy our app at scale. It is available here: https://medium.com/partage_xyz/democratizing-access-to-utilities-citizen-engagement-strategy-in-blockchain-for-smart-cities-352a8096dd22. 

Our best proof-of-concept so far is coming from the clear interest from established industry leaders proposing that we engage in a mutual NDA that gives us access to their open API and reserved developer tools so we could integrate our solution into their devices. 

- **What Partage Lock will and will not provide:**
Partage Lock will provide access control of physical gates. We believe that there’s a great potential for giving access to real-world items through the blockchain while the tech ecosystem is looking for practical use cases and a business model built on tangible products to onboard new adopters. 

So far, Partage will not handle digital gated content. We do have big ideas for future developments in that space, which aligns with the Partage mission. Suppose we could use Partage to share revenues from a successful YouTube video generating millions of views with all the early supporters. Or to offer passive income to all team members having participated in a successful piece of art or video game. These ideas were reviewed last year and tested through lean experiments with niche customer targets, and we couldn’t find an easy go-to-market validation. A 5-min video presentation of our market research is available here: https://youtu.be/cWC9rXksXiI?si=m4413wCddohpt1KT 


### Ecosystem Fit
- **Where and how does your project fit into the ecosystem?**
Partage Lock seamlessly integrates into the NEAR ecosystem by aligning with Mintbase's "Unlockables / Gated content" build idea proposition, as detailed in the Mintbase documentation https://docs.mintbase.xyz/dev/build-ideas. 

Elected as a finalist in the IRL hackathon at Nearcon 23 in Lisbon, Partage has garnered exceptional support from mentors, executives, and the NEAR community. Our pitch is available here: https://www.youtube.com/live/Iw8_04Kk1aw?si=U2PoHlaDgrp5Nfaq&t=16350 

Since we started to work with the Near ecosystem we are feeling very well supported by the Near ecosystem of companies and people. As another proof of appreciation of Partage, we have been selected to join both accelerator programs: the Encode Club x Near https://www.encode.club/near-accelerator from Dec.23 to Jan.24, and the Near Horizon HZN2 starting on Jan.24 https://www.hzn.xyz/hzn/. These selection results have encouraged our confidence in the platform's strength and potential for long-term impact within the NEAR ecosystem.

- **Who is your target audience?**
We are targeting both crypto holders looking for IRL experiences and a means to spend their crypto in their daily lives, and non-crypto owners or utility providers willing to share access to their belongings while starting to collect revenues in cryptocurrencies. 

Our locks could go anywhere in the physical world, they don’t even need an internet connection to check a token’s validity and have an autonomy of about 2 years (with a high daily opening rate of 10 times/day). With both padlocks and door controllers we cover pretty much all scenarios of gated access control (house, office, lockers, cars, bikes, commercial fridges…). 

- **What need(s) does your project meet?**
Partage Lock meets the needs of our market by optimizing resource usage through:
Shared access
Monetizing assets
Secure and traceable transactions
Transparency and decentralization 
Ecosystem partnerships
Alignment with Mintbase's vision for blockchain applications
Sustainability practices

- **Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?**
We have reported similar projects on other blockchains, but not in the Near ecosystem as we know. This is somehow encouraging because we can benefit from the experience of other builders, without being directly in competition with them, while we’ll bring Near users a new use case of proven validity.

  - **If so, how is your project different?**
One interesting difference is that the Partage team is globally distributed. Our team members are spread across Europe, America, and Africa. This enhances the project's adaptability and innovation in the competitive landscape. Having different background experiences in tech adoption brings diverse perspectives and expertise from around the world to shape the platform's initiative, as well as a great potential for deployment. Since our team is spread worldwide, we can distribute our Partage Locks to businesses in each team member's area easily and quickly. 

  - **If not, are there similar projects in related ecosystems?**
- Myloby integrates master lock devices to the Thesos blockchain.
- Blockchain-controlled smart locks are a topic of interest for academic research in computer science (see various thesis and papers on the topic), 
- DIY/FabLab/Makerspaces like the use case to build a smart lock on an Arduino board and connect it to a blockchain (see multiple tutorials available).
- No one of our knowledge except Slock It tried to deploy this solution in 2016 for society at scale. They were granted 12M ETH of investments but have been hacked and stolen their funds (ethereum early days).

## Team :busts_in_silhouette:

### Team members
- Team leader: 
Julien Carbonnell - CEO

- Team members: 
John Toluwase - Frontend Developer
Daniel Makarov - Full Stack Developer
Sophie Kirsch - Marketing
James Sanderson - Business Development and Operations
Lilibel Obiadika - Community Management and Social Media

### Contact
- **Contact Name:** Julien Carbonnell
- **Contact Email:** julien.carbonnell@gmail.com
- **Website:** https://hellopartage.xyz

### Legal Structure
- **Registered Address:** CivicTech OÜ, Ahtri 12, 10151 Tallinn, Estonia
- **Registered Legal Entity:** CivicTech OÜ

### Team's experience
Julien Carbonnell (CEO) is a French urban developer, living as a digital nomad. He spent a decade working on Smart City and CivicTech projects before finding a purpose for adopting blockchain in his projects. He is an experienced entrepreneur with a main background in project management (12 years) and has been a programmer (machine learning/blockchain) for 3 years, and a sales/bizdev (6 years).

John Toluwase (Frontend Developer) is based in Nigeria. He is a full-stack blockchain developer with a strong background in front-end development. He collaborated with diverse teams to create innovative projects for hackathons and contributed his skills to tech startups, bringing ideas to life through code. 

Daniel Makarov (Lead Dev) is a computer scientist specializing in Artificial Intelligence and Blockchain Technology at the University of Toronto. With three years of experience in Web3, he built decentralized exchanges, provided consultation for blockchain-based telecommunication systems in Canadian Indigenous Communities, and was a software developer at Toronto's leading crypto brokerage. Daniel has a strong knowledge of Full Stack Development and a strong understanding of theoretical concepts in computer science as a whole.

Sophie Kirsch (Marketing) has worked alongside Julien since February 2023, supporting his vision in Brand Design, Content Creation, and Creative Strategy. Sophie has participated in some experimental marketing tactics and strategies, and aided in potential partner liaisons and relationship building. She has also worked alongside Julien during application submissions, overseeing the copy. 

James Sanderson (BizDevOps) works in fintech on business development, and procurement (mainly SaaS deals). He graduated from London Business School’s Masters in Finance, specializing in entrepreneurship and venture capital. He co-founded the school's inaugural Blockchain Society amongst working on other tech-for-good related initiatives and continues to build a coffee-centric startup backed by blockchain infrastructure.

Lilibel Obiadika (Community Management and Social Media) joined the Partage team as a contractor to assist with Digital Marketing, Social Media, and Community Management, notably contributing to Technical Writing. 


### Team Code Repos
- https://github.com/PartageProtocol 
- https://github.com/PartageProtocol/hellopartage-firebase (marketplace)
- https://github.com/PartageProtocol/partage-lock (lock app)

The GitHub accounts of all dev team members. 
https://github.com/juliencarbonnell
https://github.com/Tolujoh-n 
https://github.com/daniel-makarov 
https://github.com/lilibelkay 


### Team LinkedIn Profiles
- https://www.linkedin.com/in/juliencarbonnell/ 
- https://www.linkedin.com/in/sophie-kirsch-52351898/
- https://www.linkedin.com/in/daniel-makarov-99583020b/
- https://www.linkedin.com/in/tolu-john-028060241/ 
- https://www.linkedin.com/in/jamesoliversanderson/ 
- https://www.linkedin.com/in/lilibelkay/ 

## Development Status :open_book:
Before Partage, Julien Carbonnell was building Democracy Studio, an empowering platform for citizen engagement in urban development based on machine learning and computational simulation models built during his Ph.D. research in Urban Science. The thesis has been adapted from a book https://www.amazon.com/Democracy-Studio-artificial-intelligence-engagement/dp/B098GT2P96  and all models are opened and documented on GitHub https://github.com/DemocracyStudio. 

The post-doctorate period has been to travel the African continent for 8 months to meet with local startup ecosystems and look for ideas to help solve the informal housing challenges https://www.youtube.com/watch?v=VfNgoFFZRfc. As a result of that field research, Partage started with the idea to mint real estate property titles to tackle the lack of cadastre systems https://youtu.be/n8ZNEpoGSXw?si=G4iq6LOWNZVKLZZu in half of the countries in the world (mostly LATAM, Africa, South Asia).

More details about our roadmap, history, and development status are available in our white paper: https://medium.com/partage_xyz/partage-white-paper-v2-c0cbea46e2f8.

- academic publications relevant to the problem:
Blockchain for informal housing in LATAM https://drive.google.com/file/d/1dgk0HocmBU7NGdOF3MZZ9G72as4bD-Vx/view?usp=sharing 
Blockchain for the governance of common goods https://www.theses.fr/2022ASSA0046 
Blockchain-controlled Smart Locks https://drive.google.com/file/d/1yr9Jh1n0Z5nmB2Py2rfOLXVS3sWv91f_/view?usp=sharing 

- **links to your research diary, blog posts, articles, forum discussions, or open GitHub issues:**
All our research articles from tokenizing real-world assets to blockchain for smart cities and blockchain-controlled smart locks are available on our medium publication: https://medium.com/partage-btc 

- **references to conversations you might have had related to this project with anyone from the Mintbase Foundation:** 
After having attended the Mintbase workshop of the Encode Club X Near Accelerator, I met with Luis, on Thursday 14th November 2023 to discuss how Partage could use Mintbase templates for minting access tokens. The call was pretty straightforward as Luis already knew how to do this with Mintbase and seeing that we already did quite a lot of work on our app he incentivized us to submit Partage to Mintbase’s grant program. 

- **Previous interface iterations, such as mock-ups and wireframes.**
See above.

## Development Roadmap :nut_and_bolt:

![Partage roadmap (General presentation)](https://pbs.twimg.com/media/GBompXtWYAATgg-?format=jpg&name=medium)

MILESTONE description: 
- describe _the functionality in as much detail as possible_, 
- how we can verify and test that functionality. 
- how your project is related to Mintbase. 
- make sure to include a specification of your software. _Treat it as a contract_; the level of detail must be enough to later verify that the software meets the specification.
- include the amount of funding requested _per milestone_.
- include documentation (tutorials, API specifications, architecture diagrams, whatever is appropriate) in each milestone. This ensures that the code can be widely used by the community. 
- provide a test suite, comprising unit and integration tests, along with a guide on how to set up and run them.
- commit to providing Dockerfiles for the delivery of your project.
- indicate milestone duration as well as several full-time employees working on each milestone.
- **Deliverables 0a-0d are mandatory for all milestones**, and deliverable 0e at least for the last one. If you do not intend to deliver one of these, please state a reason in its specification (e.g. Milestone X is research-oriented and as such there is no code to test).

### Grant Overview:
- **Total Estimated Duration:** 
  - Level 1: 1 month, Near integration (Mintbase modules) into Master Lock devices, delivered at the end of January 2024.
  - Level 2: 3 months, A sharing network for locked items, delivered at the end of March 2024.

- **Full-Time Equivalent (FTE):**  3 FTE

- **Total Costs:** 
  - Level 1: 10k
  - Level 2: 50k

- **Milestones plan:**
  - Milestone 1: January 2024
  - Milestone 2: February 2024
  - Milestone 3: March 2024

### Milestone 1 — Near integration (Mintbase modules) into Master Lock devices
- **Estimated duration:** 1 month
- **FTE:**  3
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Unlicensed |
| 0b. | Documentation | Marketplace User’s Guide:
Interactive Video Guide (Duration: 5-10 minutes): Demonstrates unlocking the lock using the mobile app, highlighting NFC/Bluetooth unlocking, adding temporary users, and access management.
Extensive Step-by-Step Written Guide (Approx. 15-25 pages): Detailed instructions on app usage for unlocking and managing the smart lock, with app interface screenshots, security tips, crypto transaction info, and temporary access setup guidelines.
Using Mintbase Wallet for Unlocking (Duration: 3-5 minutes in video, 5-8 pages in written guide): A specific section or segment showing how to open a lock using the Mintbase wallet. It should include a tutorial on linking the Mintbase wallet to the user's account, initiating transactions, and the process of unlocking the lock through the wallet app, with emphasis on security and ease of use.|
| 0c. | Testing Guide | 
Master Lock Testing: Lock Installation & App Connection: Upon purchasing a master lock, verify the installation process and ensure a seamless connection to the Partage app.
Bluetooth and Connectivity Test: Confirm the lock's Bluetooth functionality and overall connectivity performance.
Live Demo Option
Full Code Demonstration: Request a live demo call with us to observe a comprehensive test of the app, including installation, app integration, and Bluetooth connectivity.
Mintbase Wallet Integration
Wallet Linking & Unlock Test: Connect your Mintbase wallet to the Partage app and perform a test to unlock the lock, ensuring smooth integration and functionality.
Additional Testing Option
You can also clone our Github repository to test the app locally on your system, and alternatively, you can deploy the smart contract on the NEAR blockchain and run security tests.|
| 0d. | Docker | A Dockerfile is a script of instructions used to create a Docker image. It automates the process of image creation by compiling commands typically executed manually on the command line. However, this technology does not apply to our current app development.|
| 0e. | Article | As described above, we will provide a written article for our users so they can easily use our product.|
| 0f. | Budget breakdown | 60% dev, 10% marketing, 30% bizdev. 
**Smart Contract Pre-Loading ($500):** Allocating funds to pre-load the smart contract onto the system for customers.
**Smart Contract Testing and Deployment ($300):** Covers the cost of rigorous testing of the smart contract to ensure security and functionality, and the deployment on the blockchain.
**Smart Locks Purchase ($1,200):** Acquiring smart locks to integrate with the blockchain technology and for testing the system.
**Salary for 3 Developers ($4,000):** Covering the cost of three developers who will work on the smart contract integration, application development, and system testing.
**Marketing Strategy Preparation ($1000):** Developing a comprehensive marketing plan to promote the new crypto-integrated smart lock system. This includes identifying target markets, creating promotional materials, and planning digital marketing campaigns.
**Travel Expenses for Partnerships ($3000):** Allocate $3,000 for travel expenses to facilitate in-person meetings for signing partnerships and negotiating device purchases.".|
| 1. | Mintbase module: Minter | We will use the Mintbase Minter template to create the Partage Lock smart contrat that will mint temporary access tokens and send them to users’ wallet after payment. |  
| 2. | Mintbase module: Access Token NFTs | The access token NFTs minted from our smart contract will include a start date and an end date in their metadata, to allow holders to open the lock during this time frame but not giving them access outside of it. |  
| 3. | NEAR chain integration | Master Lock devices will use the Bluetooth in the user’s smart phone to verify that they are holding a valid token in their Mintbase wallet, and opening at this condition. |  
 


### Milestone 2 — Embed Mintbase Modules into the Partage app marketplace
- **Estimated duration:** 1 month
- **FTE:**  4
- **Costs:** 20,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicensed |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 0f. | Budget breakdown | 50k: 60% dev, 30% marketing, 10% bizdev. Marketing strategy implementation, start customer support.
| 1. | Mintbase module: X | We will create a Mintbase / NEAR module that will... (Please list the functionality that will be implemented for the first milestone) |  
| 2. | Mintbase module: Y | We will create a Mintbase / NEAR module that will... |  
| 3. | Mintbase module: Z | We will create a Mintbase / NEAR module that will... |  
| 4. | NEAR chain integration | Modules X, Y & Z of our custom chain will interact in such a way... (Please describe the deliverable here as detailed as possible) |  


### Milestone 3 — Fully integrate the Partage app marketplace with Near elements
- **Estimated duration:** 1 month
- **FTE:**  4
- **Costs:** 20,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicensed |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains [...] (what was done/achieved as part of the grant). (Content, language and medium should reflect your target audience described above.)
| 0f. | Budget breakdown | 50k: 60% dev, 30% marketing, 10% bizdev. Marketing strategy implementation, start customer support.
| 1. | Mintbase module: X | We will create a Mintbase / NEAR module that will... (Please list the functionality that will be implemented for the first milestone) |  
| 2. | Mintbase module: Y | We will create a Mintbase / NEAR module that will... |  
| 3. | Mintbase module: Z | We will create a Mintbase / NEAR module that will... |  
| 4. | NEAR chain integration | Modules X, Y & Z of our custom chain will interact in such a way... (Please describe the deliverable here as detailed as possible) | 

**We’d be happy to give you more details about our budget allocation in a call.**

## Future Plans
- **How you intend to use, enhance, promote, and support your project in the short term**
We have many marketing-based options available for short-term support. Some examples are: 
    - Paid Digital Advertising on platforms like Meta, Instagram, or Google Ads to reach a broader audience. 
    - Social Media Campaigns to create awareness, engage potential users, and build a community around Partage Lock. Leverage visuals, user testimonials, and educational content to showcase the platform's features and benefits. 
    - Collaborations with Influencer and/or Brand Partnerships in the sharing economy, technology, or blockchain space. Their endorsement can lend credibility to Partage Lock and help reach a wider audience. 
    - We would continue to create high-quality content, including LinkedIn blog posts, articles, and infographics, to highlight the value proposition of Partage Lock. 
    - Community Engagement Events can be advantageous to actively engage with the community, address questions, and showcase the platform's functionality. This can strengthen user relationships and generate positive word-of-mouth - and can be considered for PR Campaigns. 

- **The team's long-term plans and intentions about it.**
    - Grow adopters within the Near ecosystem and the Master Lock customers. 
    - Deploy networks of lockers on public infrastructures through smart city partnerships and universities.
    - Diversify our solutions to other kinds of shared items and gated content (digital, like described above in the “what partage lock will not provide” section)
    - Integrate our locks-in starters for shared cars, scooters, or other electronic items available after payment.
    - Attend conferences (web3FC in Barcelona in April, Bitcoin 2024 in Nashville in July, …) to present Partage and onboard customers.

## Additional Information :heavy_plus_sign:
**How did you hear about the Grants Program?**
At the Encode Club workshop, followed by a 1-1 call with Luis.

- **Work you have already done.**
Aside from the coding of MVPs, research, and all listed above, we just started a partnership with Master Lock. They gave us 8 devices free of charge (worth 1200 USD) to start building with.
![Master Lock devices we start working with](https://pbs.twimg.com/media/GBn9TCgWsAA4BIq?format=jpg&name=medium)

- **If any other teams have already contributed (financially) to the project.**
Julien has been self-supporting the project from the beginning. We have never been financially supported yet. Julien and Daniel each received a 1000 USD grant from Near Dev Hub to cover their travels to Nearcon 23 last month.

- **Previous grants you may have applied for.** 
We started to build with the Web3 Startup Lab last year, supported by the Stacks blockchain, but they canceled part 2 of their program (fundraising) because they were lacking investment themselves. We finished 2nd of 16 in their pitch competition to VCs on Feb. 23.
