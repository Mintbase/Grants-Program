# Mintbase Grant Proposal

- **Project Name:** Partage Lock
- **Team Name:** Partage
- **Payment Address:** partage-lock.near
- **[Level](../README.md#level_slider-levels):** 1 (open to 2)

## Project Overview :page_facing_up:

### Overview

- Tag line:
“A Sharing Network of Locked Utilities.”

These are some tag line options that are more “Commercial Marketing” in their style/tone: 

Unlock Possibilities, Share Access with Partage Lock.
Keyless Connections For Seamless Sharing.
Keyless Solutions For Seamless Sharing?
Seamless Access For Shared Spaces.
Sharing Made Secure: Partage Lock, the Future of Access.
Connect, Share, Secure – Partage Lock Redefining Ownership.
Connect, Share, Secure – Redefined Ownership?
Partage Lock is where ownership meets access in the digital age.
Smartlock Solutions For Shared Networks 


- Brief description:
Partage enables owners to monetize shares in their properties by controling access with co-owners or temporary users. Our innovative system is based on a network of smart locks that users can effortlessly open using NFC signals or Bluetooth technology via a token stored on their smartphones.

- How Partage integrates into the Mintbase / NEAR ecosystem?
Powered by Mintbase template, the Partage app utilize Mintbase smart contracts to mint the access token and make market available smart locks compatible with Near tokens. Our mission aligns seamlessly with Mintbase's vision, answering the call for an application that facilitates the unlocking of exclusive content and gated access on the NEAR ecosystem.

- Why is our team interested in creating this project?
Partage aims to contribute towards a better future by championing the sharing economy and optimizing resource allocation and functionality. Partage Lock achieves this by facilitating the monetization of idle assets through shared uses, and encourages a shift towards responsible ownership. Partage aim to limit overproduction and overconsumption which tends to waste natural resources and have a negative impact on pollution and climate change. 

The use of NFC and Bluetooth technologies ensure heightened digital security for all, since all standard smartphones on the market already uses them. 

Our choice to build Partage on the NEAR Protocol reflects our commitment to sustainability and reliability. Partage’s vision for the web3 sharing economy actively supports social and sustainable goals by fostering a connected community centered around collaborative ownership economies. In alignment with Mintbase's vision, the platform is spearheading an innovative application on the Near blockchain, contributing to pushing NFTs to their next level by making unlockables and gated content easy for Near users. 

### Project Details
- **Mockups/designs of any UI components:**
A full design of our marketplace is available here: 
https://www.figma.com/file/D4u5WSOO1Tq2D7BNy7i6zv/Partage-NFT-Marketplace?type=design&node-id=1647%3A17907&mode=design&t=Zl0EjoPvRDlmdFIw-1 
Implemented in a demo version here:
https://hellopartage.xyz 

The above MVP was built on another blockchain 9 months ago (Stacks) and we are now working on a NEAR version of it that will integrate our NEAR app Partage Lock to deliver temporary access keys to a network of smart locks. 

A demo version of the first Partage Lock app is available here:
https://lock.hellopartage.xyz 
We will integrate Mintbase template smart contracts to mint access tokens:
https://shorturl.at/qANW3 
An example of temporary access token (an NFT including startdate and enddate in its metadata) has been minted using mintbase and is available on partage-lock.testnet wallet: https://www.mintbase.xyz/meta/moments.mintbase1.near:5e2027fd30e5f0dfb26dd3b3e005eeed 

- **Data models / API specifications of the core functionality:**
We bring the Partage Lock to its next level of functionality and potential for market deployment through a signed partnership with Master Lock. Master is one of the world most reknown lock brand, and they released a series of their popular devices equipped with bluetooth technology https://www.masterlock.com/products/bluetooth-electronic-locks?personal. All models from this series (padlocks of various sizes, pinpads, and door controller) are working with the same background technology, which we are integrating as a partner using their SDK and APIs. 

In short, Partage is using mintbase tokens to control access to market-available bluetooth devices from Master Lock. In a latter step of development we could potentially think of integrating the Partage app with other smart lock brands to cover a wider market share and various industries. We have contacted 59 brands in october and a dozen already confirmed their interest for a partnership by sharing access to their API (including Akiles, Tedee, 2N, Brivo, Igloohome, Assa Bloy, Nuki, Yale, Schlage…)

- **An overview of the technology stack to be used:**
We are using a usual NextJs/React stack to build our app with a Near backend smart contract written in Rust, and a Javascript/css frontend. 

- **Documentation of core components, protocols, architecture, etc. to be deployed:**
More details about our mission, history and product are available in our white paper: https://medium.com/partage_xyz/partage-white-paper-v2-c0cbea46e2f8 

A slides deck for Partage’s general presentation contains a roadmap and the UX architecture. It is available here: https://docs.google.com/presentation/d/1-TpH2HP_HUyWBBmEq4RhhOUuiOw2xbeve7yN7WEi1S8/edit#slide=id.p 

A technical slides deck is used by our dev team to reach our deadline (Jan 24th, the pitch day of the Encode x Near Horizon accelerator). It is available here: https://www.masterlock.com/products/bluetooth-electronic-locks?personal 
 
- **PoC and research on the topic:**
The relevance of developing a network of blockchain controlled smart locks for adding a layer of shared utilities to urban infrastructure has been described in a medium article, rooted in academic research of Blockchain for Smart Cities (our CEO’s background studies). It is available here: https://medium.com/partage_xyz/democratizing-access-to-utilities-blockchain-for-smart-cities-25eefb0348e7. 

Another medium article presents a strategy for Partage Lock deployment rooted in another academic field called “citizen science”. It covers examples of successful citizen-led deployments of connected devices in our cities, and provides ideas for building partnerships with cities and universities in order to successfully deploy our app at scale. It is available here: https://medium.com/partage_xyz/democratizing-access-to-utilities-citizen-engagement-strategy-in-blockchain-for-smart-cities-352a8096dd22. 

An inspiring PoC is coming from a former competitor, Slock It https://www.crunchbase.com/organization/slock-it, which raised 12M eth in 2016 (worth 150M at that time, 4.5B nowadays). They are quite well documented, and we trust that the success of their fundraising campaign is a great inspiration to give us confidence in what we’re building. Slock.it stopped have having been hacked and stolen their funds (ethereum early days) which never say their idea/model was wrong.

- **What Partage Lock will and will not provide:**
So far we are focusing our efforts on physical gates access control. Even though we are conscious that this brings legal and insurance concerns, we do believe that there’s a great potential for giving access to real-world items through the blockchain while the tech ecosystem is looking for its practical use cases and a business model built on tangible products to onboard new adopters. 

We are conscious that solving access to digital content gated is a massive market, that is somehow easier to handle in terms of legal and insurance, but will also need more financial resources available to scale up fast… Suppose we could use Partage in a couple of year to share the revenues from a youtube video generating millions of views with all the early supporters? Suppose we could use Partage in order to offer life-long passive income to all participating dev of a successful video game or movie? Wee have been reviewing those ideas last year, and collected feedbacks through lean experiments of customer targets. If the ideas are valid, it lacks concreteness and a proof that we can reach these giant markets from scratch. A 5-min video presentation of our market research is available here: https://youtu.be/cWC9rXksXiI?si=m4413wCddohpt1KT 


### Ecosystem Fit
- **Where and how does your project fit into the ecosystem?**
Partage Lock seamlessly integrates into the NEAR ecosystem by aligning with Mintbase's "Unlockables / Gated content" build idea proposition, as detailed in the Mintbase documentation https://docs.mintbase.xyz/dev/build-ideas. 

Selected as a finalist in the IRL hackathon at Nearcon 23 in Lisbon, Partage has garnered exceptional support from mentors, executives, and the NEAR community. Our pitch is available here: https://www.youtube.com/live/Iw8_04Kk1aw?si=U2PoHlaDgrp5Nfaq&t=16350 

Since we started to work with the Near ecosystem we are feeling very well supported by the Near ecosystem of companies and people. As another proof of appreciation of Partage, we have been selected to join both accelerators programs: the Encode Club x Near https://www.encode.club/near-accelerator from Dec.23 to Jan.24, and the Near Horizon HZN2 starting in Jan.24 https://www.hzn.xyz/hzn/ has encouraged our confidence in the platform's strength and potential for long-term impact within the NEAR ecosystem.

- **Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?**
We are targeting both crypto holders looking for IRL experiences and a means to spend their crypto in their daily lives, and non-crypto owners or utility providers willing to share access to their belongings while starting to collect revenues in crypto currencies. 

Our locks could go anywhere in a physical world, they don’t even need an internet connection to check a token’s validity, and have an autonomy of about 2 years (with a high daily opening rate of 10 times/day). With both padlocks and door controllers we cover pretty much all scenarios of gated access control (house, office, lockers, cars, bikes, commercial fridge…). 

- **What need(s) does your project meet?**
Partage Lock meets the needs of our market by optimizing resource usage through:
Shared access
Monetizing assets
Secure and traceable transactions
Transparency and decentralisation 
Ecosystem partnerships
Alignment with Mintbase's vision for blockchain applications
Sustainability practices

- **Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?**
We have reported similar projects on other chains, but not in the Near ecosystem as we know. Which is somehow encouraging, because we can benefit the experience of other builders, without being directly in competition with them, while we’ll bring Near users a new use case of proven validity.

  - **If so, how is your project different?**
One interesting difference is that the Partage Lock team is a global team. Our team members are spread across Europe, America, and Africa. This brings diverse perspectives and expertise from around the world to shape the platform's initiative, as well as a great potential for deployment. This enhances the project's adaptability and innovation in the competitive landscape.

  - **If not, are there similar projects in related ecosystems?**
- Mylobi integrates master lock devices to the Thesos blockchain.
- Blockchain-controlled smart locks are a topic of interest for academic research in computer science (see various thesis and papers on the topic), DIY/FabLab/Makerspaces (see various tutorials on how to build a smart lock on an Arduino board and connect it to a blockchain).
- No one of our knowledge except Slock It (described above) tried to deploy this solution for society at scale.

## Team :busts_in_silhouette:

### Team members

- **Team leader:** 
Julien Carbonnell - CEO

- **Team members:** 
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
Julien Carbonnell (CEO) is a French urban developer, living as a digital nomad. He spent a decade working on Smart City and CivicTech projects before finding a purpose to adopting blockchain in his projects. He is a programmer (machine learning/blockchain) for 3 year, with a main background in project management (12 years) and sales/bizdev for 6 years.

John Toluwase (Frontend Developer) is based in Nigeria. He is a full-stack blockchain developer with a strong background in frontend development. He collaborated with diverse teams to create innovative projects for hackathons and contributed his skills to tech startups, bringing ideas to life through code. 

Daniel Makarov (Lead Dev) is a computer scientist specializing in Artificial Intelligence and Blockchain Technology at the University of Toronto. With three years of experience in Web3, he built decentralized exchanges, provided consultation for blockchain-based telecommunication systems in Canadian Indigenous Communities, and was a software developer at Toronto's leading crypto brokerage. Daniel have a strong knowledge in Full Stack Development, and a strong understanding of theoretical concepts in computer science as a whole.

Sophie Kirsch (Marketing) has worked alongside Julien since February 2023, supporting his vision in Brand Design, Content Creation and Creative Strategy. Sophie has participated in some experimental marketing tactics and strategies, and aided in potential partner liaisons and relationship building. She has also worked alongside Julien during application submissions, overseeing the copy. 

James Sanderson (BizDevOps) works in fintech on business development, and procurement (mainly SaaS deals). He graduated from London Business School’s Masters in Finance, specialising in entrepreneurship and venture capital. He co-founded the school's inaugural Blockchain Society amongst working on other tech-for-good related initiatives, and continues to build a coffee-centric startup backed by blockchain infrastructure.

Lilibel Obiadika (Community Management and Social Media) joined the Partage team as a contractor to assist with Digital Marketing, Social Media and Community Management, and notably contributing to Technical Writing. 


### Team Code Repos
- https://github.com/PartageProtocol 
- https://github.com/PartageProtocol/hellopartage-firebase 
- https://github.com/PartageProtocol/partage-lock 

GitHub accounts of all dev team members.
https://github.com/juliencarbonnell
https://github.com/Tolujoh-n 
https://github.com/daniel-makarov 
https://github.com/lilibelkay 


### LinkedIn Profiles of all team members
- https://www.linkedin.com/in/juliencarbonnell/ 
- https://www.linkedin.com/in/sophie-kirsch-52351898/
- https://www.linkedin.com/in/daniel-makarov-99583020b/
- https://www.linkedin.com/in/tolu-john-028060241/ 
- https://www.linkedin.com/in/jamesoliversanderson/ 
- https://www.linkedin.com/in/lilibelkay/ 

## Development Status :open_book:
Before Partage, Julien Carbonnell was building Democracy Studio, an empowering platform for citizen-engagement in urban development projects based on machine learning and computational simulation models built during his Ph.D. thesis in Urban Science. The thesis has been adapted in a book https://www.amazon.com/Democracy-Studio-artificial-intelligence-engagement/dp/B098GT2P96  and all models are opened and documented on github https://github.com/DemocracyStudio. 

The post-doctorate period have been to travel the african continent 8 months in order to meet with local startup ecosystems and look for ideas to help solving the informal housing challenges https://www.youtube.com/watch?v=VfNgoFFZRfc. As a result of that field research, Partage started with the idea to mint real estate properties titles to tackle the lack of cadastre systems https://youtu.be/n8ZNEpoGSXw?si=G4iq6LOWNZVKLZZu in half of the countries in the world (mostly LATAM, Africa, South Asia).

More details about our roadmap, history and development status are available in our white paper: https://medium.com/partage_xyz/partage-white-paper-v2-c0cbea46e2f8.

- **academic publications relevant to the problem:**
Blockchain for informal housing in LATAM https://drive.google.com/file/d/1dgk0HocmBU7NGdOF3MZZ9G72as4bD-Vx/view?usp=sharing 
Blockchain for governance of common goods https://www.theses.fr/2022ASSA0046 
Blockchain controlled Smart Locks https://drive.google.com/file/d/1yr9Jh1n0Z5nmB2Py2rfOLXVS3sWv91f_/view?usp=sharing 

- **links to your research diary, blog posts, articles, forum discussions or open GitHub issues:**
All our research articles from tokenizing real-world assets to blockchain for smart cities and blockchain controlled smart locks are available on our medium publication: https://medium.com/partage-btc 

- **references to conversations you might have had related to this project with anyone from the Mintbase Foundation:**
I met with Luis, on Thursday 14th to discuss how Partage could use Mintbase templates for minting access tokens. The call was pretty straightforward as Luis already knew how to do this with Mintbase and seeing that we already did quite a lot of work on our app he incentivized us to submit Partage to Mintbase’s grant program. 

- **previous interface iterations, such as mock-ups and wireframes.**
See above in the "Mockups/designs of any UI components".
We also had a landing page https://www.figma.com/file/IDGpSIdseE9UYaSvw0XtW3/landing-page?type=design&node-id=0%3A1&mode=design&t=aaFpJaUob8fjw44D-1 and a low tech version of our fractional ownership protocol based on our telegram group that we used to collectively purchase an NFT with 9 early adopters. 

## Development Roadmap :nut_and_bolt:
Partage's general roadmap: https://docs.google.com/presentation/d/1-TpH2HP_HUyWBBmEq4RhhOUuiOw2xbeve7yN7WEi1S8/edit#slide=id.g29e6421ff84_0_23 

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

…

Milestones:
Jan
Feb
Mar
Apr

### Overview
- **Total Estimated Duration:**
- Dec-Jan: Near-Master Lock integration on testnet.
- Feb-March: Improved MVP on mainnet.
- April: Fundraising.

- **Full-Time Equivalent (FTE):**  Average number of full-time employees working on the project throughout its duration (see [Wikipedia](https://en.wikipedia.org/wiki/Full-time_equivalent), e.g. 2 FTE)
- **Total Costs:** Requested amount in USD for the whole project (e.g. 12,000 USD). Note that the acceptance criteria and additional benefits vary depending on the [level](../README.md#level_slider-levels) of funding requested. 
- with a 10k budget, we’ll integrate mintbase tokens in master lock to open them with a bluetooth signal. (delivered end of january).

10k: 60% dev, 10% marketing, 30% bizdev

20% of $10,000.00
Marketing Strategy Breakdown:

Digital Advertising (40% - $800)
Meta, TikTok, Instagram, or Google Ads to reach a broader audience.

Influencer Partnerships (20% - $400)
Influencers or industry expert collaborations to reach a wider audience through their followers. This can be on channels such as Youtube, Spaces, LinkedIn, Podcasts, Radio, Television and Expos. 

Social Media Campaigns (15% - $300)
Campaigns to create awareness, engage potential users, and build a community around Partage Lock. We'd leverage visuals, user testimonials, and educational content to showcasing features and benefits.

Content Creation (15% - $300)
Blog posts, articles, and infographics, to highlight the value proposition of Partage Lock. This content can be shared across social media, email newsletters, and other platforms like LinkedIn.

Community Engagement Events (5% - $100):
AMAs (Ask Me Anything sessions) to actively engage with the community, address questions, and showcase the platform's functionality. This can happen on Youtube, Twitter or Instagram.

Salary (5% - $100)
The Marketing Team/or Member could reserve 5% of the allocated fund for their salary.

- with a 50k budget, well build the owner’s marketplace to list their shared belongings and sell accesses to it. (delivered end of march).

50k: 50% dev, 35% marketing, 15% sale 

35% of $50,000.00 Marketing Strategy Breakdown:

Digital Advertising (35% - $6125)
Meta, TikTok, Instagram, or Google Ads to reach a broader audience.

Social Media Campaigns (20% - $3500)
Campaigns to create awareness, engage potential users, and build a community around Partage Lock. We'd leverage visuals, user testimonials, and educational content to showcasing features and benefits.

Influencer Partnerships (15% - $2625)
Influencers or industry expert collaborations to reach a wider audience through their followers. This can be on channels such as Youtube, Spaces, LinkedIn, Podcasts, Radio, Television and Expos. 

Content Creation (15% - $2625)
Blog posts, articles, and infographics, to highlight the value proposition of Partage Lock. This content can be shared across social media, email newsletters, and other platforms like LinkedIn.

Community Engagement Events (10% - $1750)
AMAs (Ask Me Anything sessions) to actively engage with the community, address questions, and showcase the platform's functionality. This can happen on Youtube, Twitter or Instagram.

Salary (5% - $875)
The Marketing Team/or Member could reserve 5% of the allocated fund for their salary.


## Future Plans
- **how you intend to use, enhance, promote and support your project in the short term**
Social media / marketing plan here instead of the whole break down in the budget section.

- **the team's long-term plans and intentions in relation to it.**
Grow adopters within the Near ecosystem, and within the Master Lock customers. 
Deploy networks of lockers on public infrastructures through smart city partnerships and universities.
Diversify our solutions to other kinds of shared items and gated content (digital, like described above in the “what partage lock will and will not provide” section)
Integrate our locks in starters for shared cars, scooters, or other electronic items available after payment. 

## Additional Information :heavy_plus_sign:
**How did you hear about the Grants Program?**
At the encode club workshop this week, followed by a 1-1 call with Luis.

- **Work you have already done.**
Aside of the coding of MVPs, research, and all listed above, we just started a partnership with Master Lock. They gave us 8 devices free of charge (worth 1200 USD) to start building with. 

- **If there are any other teams who have already contributed (financially) to the project.**
Julien has been self-supporting the project from the beginning. We have never been financially supported yet. Julien and Daniel each received a 1000 USD grant from Near Dev Hub to cover their travels to Nearcon 23 last month.

- **Previous grants you may have applied for.** 
- We started to build with the Web3 Startup Lab last year, supported by the Stacks blockchain, but they cancelled the part 2 of their program (fundraising) because they were lacking investment themselves. We finished 2nd of 16 in their pitch competition to VCs in Feb. 23.
- We applied to Y Combinator SF in May and were not selected for their summer batch. They selected an almost 100% AI startup cohort?
