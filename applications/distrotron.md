# Distrotron - Mintbase Grant Proposal

- **Project Name:** Distrotron 
- **Team Name:** Stateless (pending DAO legal incorporation)
- **Payment Address:** stateless.near  
- **Level:** 1 (up to 10k)

## Project Overview 

**Distrotron is the distribution contract from Coötoo - an easy way to automatically distribute funds to all the authorized minters in a Mintbase store.**

*NOTE: our team had originally submitted the whole Coötoo toolkit for the grant level 2. This is a separate submission of just one element (the distribution contract) as a level 1 grant.*


- Why Distrotron 

Mintbase stores often evolve to become their own little ecosystem, a group of people brought together around a project, a cause or an idea, and this is reflected on chain as the list of authorized minters.

One way to reward and support all the minters at once is to distribute an amount of money equally between them, and this is what our contract does. 

Some of the use cases we envision:


- Distribute funds received by an artist cooperative (the Coötoo use case)
- Tip at once all the current minters in a store 
- Share revenues coming from sources other than NFT splits
- Expand the DAO-like capabilities of Mintbase store contracts  


Example: 

In the cooperative use case, every NFT sale is split between the artist & the co-op DAO. In this way, artists can support each other, creating a space for more collaboration and less competition. This means artists get revenue regardless of sales. In addition to the revenue distribution, the cooperative can choose where to invest extra funds through DAO governance. What is the best balance for revenue splits between individual & group? Each co-op instance can choose the numbers that make sense for their use case.


**Recommended watch:** Lenara and Ilan present coötoo at NFT BERLIN https://www.youtube.com/watch?v=aYHwy014RCA

**One page website:** https://www.statelessart.org/


*NOTE: Along this application we are leaving references to the whole Coötoo toolkit, but the distribution contract is the only part that will be developed as this Level 1 grant application, as shown on the updated milestones*


- Relation to the Mintbase / NEAR ecosystem:


Our toolkit integrates three contract factories in the NEAR ecosystem. The first is the Mintbase minting contract, which manages the list of co-op members as minter wallets authorized in the contract. Unlike each NFT royalty recipients list, which is immutable for the life of the token, minters can be added and removed by the contract in order to reflect the evolving composition of a group of artists, while having all this recorded and retrievable on-chain.

The second contract in the toolkit is our custom distribution contract. Whenever funds are sent to this distribution contract, it queries the list of minters from the Mintbase contract and proceeds to distribute funds equally to each of them.

The third contract in the toolkit, which is optional but highly recommended, is the AstroDAO. By having revenues passing through their DAO before going to distribution, the cooperative can govern them and choose where to invest extra funds. Besides these equitable distribution events, it can also hire help needed, give new members a welcome gift, or support aligned projects. The cooperative can basically future-proof itself by discovering new uses as it evolves in its mission and trajectory.

Besides the contracts, the toolkit includes a customizable viewing interface to be used as a front-end for the co-op. It pulls up the dynamic information from the NFTs minted from the contract and displays them in a minimalistic design, intended to give full attention to the artworks themselves. It purposefully does not include elements connected to competition, like sales leaderboards or rankings. It has on the other hand elements connected to serendipity discovery, as all artists have the same random chance to be featured.

- Why Coötoo?

We want to create open source tools to foster cooperation. We see cooperation and mutual support already happening naturally in artist communities all around us. Our goal is to build the tech to automate some of those actions, and help them in creating a space for more collaboration and less competition inside their community.


### Project Details


- API specification: https://github.com/myklemykle/stateless/#readme

- What Coötoo Is Not:

We are not a platform that you apply to join as an individual artist. We want people to use the toolkit to create their own cooperatives, their own platforms, with the friends and groups they are already doing things with. Our initial set of tools (the platform factory so to speak) is basic, the bare bones that you will need, but they are designed to be powerful and flexible enough for many kinds of situations. 

We won’t have a customer support service per se, but we plan to keep up with maintenance of this open source toolkit. We welcome collaborators in this, in the form of individual volunteers, and/or grants for software maintenance avenues that can be explored. We hope it proves itself of value enough to attract enough resources to keep going and evolving.

We don’t have a business plan and monetary return on investment to offer to venture capital. We just want to build an open source tool and release it. After our presentation in NFT Berlin we got contacted by an investment group specialized in creating “defensible moats”. We don’t want to create any moats. We want to create a meadow and we want flowers to grow there, and people to come to have picnics and play music. Maybe we can have a pond. But definitely no medieval moats and no fortified castles on the other side, or their web3 equivalents. Put simply: any healthy ecosystem needs quality public goods, and we aim to be one of them.



### Ecosystem Fit

* Where and how does Coötoo fit into the ecosystem?

Since the start of Mintbase’s journey on the NEAR chain, many artists who valued the flexibility and power to have their own contract came on board to explore this new ecosystem. The capability for multiple royalty and revenue splits, native to both NEAR NFT standard and Mintbase minting contracts, is one of the features setting them ahead in the landscape. Governance is pretty much begging to be put into this mix, and the possibilities for NFTs and governance to be integrated are vast. 

On the back-end side, we are fortunate to be able to plug into two already existing powerful contract factories, Mintbase and AstroDAO, and contribute with another tool for this exploration, leveraging a feature in the minting contracts - the adding and removing of authorized minter wallets, to expand revenue and royalty distribution into not just immutable settings, but an evolving group composition with its own customizable, also evolving governance tools.

On the front-end, we want to provide an interface giving the foremost space to the art itself, inspired by success stories from other NFT art platforms, which in turn explore digital versions of what real world galleries have been learning for decades. What would a smart-white wall look like? We want to create a place where the art draws undistracted attention, but at the same time provides easy interaction with the tools web3 has to offer. Furthermore, how would a cooperative model be materialized in interface elements and counterpoint algorithm design trends currently favoring a winner-takes-it-all flow? UI design choices are an important complement to foster cooperation as well.



* Who is Coötoo for?

Our users will be groups of artists that are either already formed or are in the process of being created. They can be familiar with NFTs or new to the NFT scene. In the latter case, they would benefit from at least one member or an external advisor who is familiar with NFTs in general and ideally NEAR specifically. Even if they are completely new to the scene, they can of course get the toolkit installed with the documentation provided, and figure out their governance model and other details by experimenting with the tools. Being such a new scene, even groups counting with experience and advisors will still be ultimately conducting their own experiments in governance. There is no one size fits all for this, and our goal is to provide a flexible enough tool that can be adapted to many unique circumstances. 

We will be creating our own example experiment / instance in the form of the StatelessDAO co-op, and joining our toolkit users in this adventure of figuring out how a cooperative can look like in the web3 world.



* What need(s) does Coötoo meet?

Cooperation is a human need, and it’s one hardwired into our biology. The human species excels in in-group cooperation, even while we might perform out-group competition for limited resources. We are beautiful and complex social beings, and as modern research now acknowledges, “survival of the (individual) fittest” is a reductionist myth. Even people who are not evolution science scholars, by just looking around one’s life can easily see plenty of examples of cooperation. It just makes sense (and feels good).

Artists are human, and the hardness of earning a living doing art makes this a great environment for beneficial in-group cooperation. Having at their disposal extra help from automated tools can greatly potentialize these beneficial outcomes for groups of artists.



* Are there any other projects similar to Coötoo in the Mintbase / NEAR ecosystem?

While not being aware of a project offering similar tools, we bring here several initiatives that exist in the ecosystem featuring cooperation between artists, and can be used as inspiration as well as demonstration of needs and trends.

One of the first artists to mint on Mintbase, Barbara Tosti, added on her first mints several friends to the split royalty and revenue fields. She did this without them having directly collaborated with her in the artwork being minted, or even at all in any other project, but just for being part of her friends and support network, and her wish to connect and share with them.

Several Mintbase stores have set up in their default royalties a percentage going to a charity or to a fund set aside for managing the activities of an art collective, like CODAME, Gambiarra and BeeTogether. They are already demonstrating several cooperation practices, and if they feel inclined could also integrate and customize our distribution contract. These and many other stores which have an extensive roster of artists already authorized as minters can start to receive group revenue distribution via coötoo.

An interesting practice happening in the NEAR NFT platform Paras, inspired by a similar initiative from Tezos NFT platform TEIA (former Hic et Nunc) is their art share events, called card4card (and on Tezos objkt4objkt). By minting large editions and setting an almost zero price, they create a kind of “distribution” between the group, not of revenues but of the art itself.

On the Palm blockchain, there is a new NFT publishing platform called “The Platform”. It is being organized in a cooperative model, and even being incorporated into a real-world cooperative in the UK. There is a possibility that they will release their code as open source (at the moment they are in very early stages). Being EVM compatible, we could choose for example to incorporate it in our toolkit and it could be deployed on Aurora or any other EVM compatible chain.


## Team 👥


### Team members

* Lenara Verle - executive project manager/team leader
* Mykle Hansen - senior developer
* Ilan Katin - creative director
* Sparrow Read - ethical technology consultant

### Contact

- **Contact Name:** Lenara Verle
- **Contact Email:** lenara@verle.com
- **Website:** statelessart.org


### Legal Structure

* Registered Address: N/A (DAO incorporation pending)
* Registered Legal Entity: N/A (DAO incorporation pending)


### Team's experience

Mykle has been working as a developer since the early 90s and got involved with web3 in 2020 when he was invited by Lenara to develop Plantary - an NFT platform on the NEAR blockchain. Plantary took part and won prizes in the Hack the Rainbow and ETHOnline hackathons in 2020. Mykle and Lenara were also selected to be fellows in the Kernel Block2 cohort in 2021. This early project generated quite a lot of interest, but they decided they didn't want to manage and curate a specific NFT platform, and instead develop tools to enable anyone to create many different platforms of their own. 

Lenara, Sparrow and Ilan developed in 2021 as part of the Open Web Hackathon a project that was exactly that: Stateless DAO. The project won the first prize in the governance category, and they contacted Mykle to continue the work Sparrow had started in the distribution contract. StatelessDAO presented the prototype version of its cooperative toolkit - coötoo - at the NFT BERLIN conference in 2022. 

Ilan brings his experience as a user of several NFT platforms since 2018, to distill in visual form cooperative values and ease of use. Sparrow is also an OG cryptoartist and a long time advocate in the NFT space, being one of the advisors for the NEAR NFT standard group. She is our ethical technology consultant. Lenara is the executive project manager and brings to the team her experience as a researcher, educator and communicator, having focused on art, technology & collaboration since 1995, and NFTs since early 2018.

While Mykle and Sparrow are both seasoned developers and have experience writing contracts with Rust, Lenara and Ilan also have some programming experience, mostly with Javascript, having developed together in 2017 "The Warhols" - an experimental currency Telegram bot.

Some selected team presentations:

* Lenara and Ilan present coötoo at NFT BERLIN https://www.youtube.com/watch?v=aYHwy014RCA
* Lenara and Mykle pitch Plantary at Kernel https://www.youtube.com/watch?v=qUDBh8S_WNs 
* Lenara and Mykle talk about their background in decentralized art at NEAR Without The Noise podcast #3 https://www.youtube.com/watch?v=VG8j_sGQqAQ  
* Bonus: picture of ilan presenting Stateless at the Wilde Möhre Festival https://www.statelessart.org/statelesstalk01.jpg

### Team Code Repos

* https://github.com/STATELESSART
* https://github.com/myklemykle/stateless

### Team github accounts

* https://github.com/myklemykle
* https://github.com/lenara
* https://github.com/silantnode
* https://github.com/SLRead

### Team LinkedIn Profiles

* N/A

## Development Status 📖

Current development by Mykle is at [https://github.com/myklemykle/stateless](https://github.com/myklemykle/stateless) 

The first work on the early contract was done by Sparrow during the Open Web Hackathon in 2021 and is published to [https://github.com/STATELESSART](https://github.com/STATELESSART) 

The sketches and early UI designs are in the Figma document [https://www.figma.com/file/K1CfvB26A8dbQMc6uCvZlG/FACELESS?node-id=0%3A1](https://www.figma.com/file/K1CfvB26A8dbQMc6uCvZlG/FACELESS?node-id=0%3A1)  

Conversations with Mintbase team members happened on Discord and Telegram mostly.

Our one page website  [https://www.statelessart.org/](https://www.statelessart.org/) and our Twitter account [https://twitter.com/statelessdao](https://twitter.com/statelessdao) also have documentation or early work. (We had originally another twitter account that was unfortunately restricted due to some weird Twitter error: [https://twitter.com/statelessart](https://twitter.com/statelessart) )

The submission for Open Web Hackathon including early slide deck and testnet DAO can be found here: [https://gov.near.org/t/proposal-stateless-art-cooperative-blockchain-art-protocol/1918](https://gov.near.org/t/proposal-stateless-art-cooperative-blockchain-art-protocol/1918) and the winners announcement was posted at: [https://metagov.github.io/open-web-challenge/](https://metagov.github.io/open-web-challenge/)

The recording of our presentation at NFT Berlin is available at: [https://www.youtube.com/watch?v=aYHwy014RCA](https://www.youtube.com/watch?v=aYHwy014RCA)

## Development Roadmap 🔩

### Overview

* Total Estimated Duration: 7 weeks
* Full-Time Equivalent (FTE): 1.6
* Total Costs: 1000 USD

### Milestone 1: Stateless DAO Distribution Contract.



* Est. duration: 4 weeks
* FTE: 1.4
* Costs: 7000 USD

|  |  |  |
|---|---|---|
| # | Deliverable | Specification |
| 0a.  | License | Unlicense |
| 0b.  | Documentation | All public contract methods will be documented inline with rustdoc.  The README.md will explain steps for publishing the contract on testnet, mainnet or a sandbox node. The repo directory structure, the purpose of every included external module, and the purpose of every other repo file will also be documented in the README.  |
| 0c.  | Testing Guide | All methods will be covered by a combination of unit tests, simulator tests and sandbox tests. TESTING.md will explain how to use Makefile targets to run all tests at the command line.  This will include options to deploy the contract on a local sandbox node, a remote sandbox node, or on the public NEAR testnet |
| 0d.  | Docker | EITHER a Docker image with a NEAR sandbox & the contract on it, ready for CLI testing … OR a GitLab instance for building & running the sandbox and CLI tests.  |
| 1.  | Distribution Contract | We will develop a Distribution Contract that distributes a single payment equally between all NEAR accounts defined as minters on a given Mintbase store contract.  It will make a cross-contract call to the list_minters() method of a Mintbase store contract to fetch that minters list.  |
| 2 | Mock Mintbase Contract | For testing the Distribution Contract, we will develop a Mock Contract that provides the same list_minters() method as a real Mintbase store contract, and that can be configured to return arbitrary test output or exhibit various failure modes.  |
| 3 | Fuel Economy | We will tune the Distribution Contract to minimize gas-per-minter and maximize the number of minters that can be paid in one invocation, given the hard limit on gas-per-method-call built into the NEAR blockchain.  Upon delivery of this Milestone, the contract will support splitting a payment between at least 40 users.  |






### Milestone 2: Stateless DAO Distribution Interface



* Est duration: 3 weeks
* FTE: 1.6
* Costs: 3000 USD

|  |  |  |
|---|---|---|
| # | Deliverable | Specification |
| 0a.  | License | Unlicense |
| 0b.  | Documentation | Steps for setup and activation of the website, and for configuring it to use a specific contract instance on a specific network, will be covered in README.md & supported by Makefile targets.  The UI itself will provide an overview link explaining the payment steps.  |
| 0c.  | Testing Guide | The web UI will be human-testable, by making test payments on the NEAR public testnet .  |
| 0d.  | Docker | We will provide a Dockerfile that serves the Distribution UI.  |
| 1.  | Distribution Interface | We will create a single-page web interface to inspect the minters list of a Mintbase contract, and to distribute a payment to those users via the Distribution Contract.  |
| 2 | NEAR Wallet integration | We will integrate with the web-based NEAR Wallet for authentication & payment authorization.  |
| 3 | NEAR Explorer integration | We will integrate with the web-based NEAR Explorer to inspect the entire transaction.  |









## **Future Plans**

* The short term

Our primary channel for communicating the development, launch and subsequent life of the project is through twitter. Should opportunities arise we will also attend online or in person events where we can present, less as a means to market the platform and its artists, and more about promoting the concepts that drive it.

We will be launching one example instance of coötoo called “Stateless Art” and use it to showcase what can be done using the toolkit. We plan to promote artworks minted, new artists who join the co-op, set up exhibitions in VR and celebrate 'distribution events' as a means of spreading this manifestation of our solidarity.

As coötoo becomes adopted by other cooperatives we will make efforts to share information about those projects with our community and audience. A few groups have already demonstrated interest in deploying their cooperatives using coötoo, for example the Bryggeriet Croatian skating crew, and the KAPiTAL art salon in Berlin.

* The long-term

We plan to keep developing the toolkit and have a list of features we identified as interesting to implement in subsequent versions. Those are:



* Overcome gas restrictions to raise the number of minters that can share a single payment. This might involve a multi-stage payment system, tuning improvements, and/or more sophisticated features of NEAR.
* Distribute to minters not just NEAR tokens but any fungible token compliant with NEP-141 .
* Tighter integration with AstroDAO API & multi-sig wallet on NEAR.

As the project continues, It would be beneficial to apply and secure new grants with the specific purpose of software maintenance and a bare bones user support, as well as covering ongoing costs needed for servers and licensing.



## Additional Information ➕

* Previous grants and work

We have received funding previously in May 2021, when the project participated in and won the governance track in the Open Web Hackathon, sponsored by NEAR and Metagov.

After receiving the $5,000 USD prize, our hackathon team agreed to donate their individual shares of the prize to a common DAO fund earmarked for artist onboarding for the day when the StatelessDAO cooperative platform goes live in the future. 

We continued to develop the project in our free time and presented the idea at a few venues, including the Wilde Möhre Festival in August 2021. At the time, Mintbase reached out to ask how they could help support our project, but we were still figuring out the best way to move forward with it.

As our lead developer Sparrow became involved in several other projects and didn’t have a lot of free time, we decided to invite Mykle to continue the developer work, as he was familiar with NEAR and Rust contracts, having worked with Lenara before. He was happy to get on board as Sparrow continued to contribute as tech consultant. 

Mykle started working on a proof-of-concept contract around March 2022, focusing on getting the basic funds distribution function to work and exploring ways to overcome some blocks we had encountered in the first version written by Sparrow.

We were able to overcome the largest block as Mykle contacted the Mintbase developer team and they kindly pushed an update to their minting contract to include a function for querying the list of authorized minters. After that, our proof-of-concept distribution contract development proceeded and it successfully passed initial tests on Testnet. It’s great to see the Mintbase list_minters function live on mainnet as well, and we are ready to proceed.

For this, we needed our team to have the resources to focus more intensively on the project, and we are happy that NEAR and Mintbase are providing support for builders like us in the form of grants, documentation, office hours, and of course, providing such a powerful and flexible set of open source tools, and allowing people to use them in order to build their custom solution aligned to their vision and passion.

We heard about this grant from several channels, twitter, blog posts and the like, and were just waiting for our team to clear up enough time in our calendars to be able to focus a more consistent amount of hours on the project. We are now ready to start and looking forward to this journey.

* Trivia: Why the umlaut in coötoo? 

It’s not an umlaut, it’s a dieresis. A dieresis marks when double vowels are both pronounced as separate sounds. You can also indicate it by writing “co-op”. A “coop” (pronounced like in the political “coup d’état”) means a place to keep chickens. When you use the full word - cooperative, or cooperation - the context is enough to deduct the right pronunciation, but if you want to look extra erudite you can use the dieresis as well and write coöperation. The name coötoo is a contraction of “cooperative toolkit”, and the two O in “too” are pronounced as a single sound.
