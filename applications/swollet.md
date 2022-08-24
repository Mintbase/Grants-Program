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
  - The Financial Empowerment Platform focused on gamifying financial education and providing on-chain credentials.
- A brief description of your project.
  - Play and learn about financial literacy, work readiness, entrepreneurship through bite-sized courses. Content is created by top notch academic institutions. Portfolio Battles: Test your knowledge against the online community of fellow traders by playing the fantasy trading game. NFT certificates: Prove your talent on the blockchain by receiving your NFT certificate upon course completion.
- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.
  - Upon course completion, learners will receive their badges. Those badges will be NFTs which they will prove their talent on the chain.
- An indication of why your team is interested in creating this project.
  - We are building a financial education platform and our vision is to democratise financial education and make it accessible for everyone in the world.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components
  - https://drive.google.com/drive/folders/1rsdik9kqwe8JIxpFet8_SAH0ha9oCbpI?usp=sharing
- Data models / API specifications of the core functionality
- An overview of the technology stack to be used
  - https://docs.google.com/document/d/1D5Teha13qPw2Sc69-KSiyAwnGnIy0XBSSREpvCgSF7M/edit?usp=sharing
- Documentation of core components, protocols, architecture, etc. to be deployed
  - https://docs.google.com/document/d/1D5Teha13qPw2Sc69-KSiyAwnGnIy0XBSSREpvCgSF7M/edit?usp=sharing
- PoC/MVP or other relevant prior work or research on the topic
  - https://drive.google.com/file/d/1Ik6RscdkMIwA8T86oFcQHwhraXF-OUO8/view?usp=sharing
- What your project is _not_ or will _not_ provide or implement
  - We are looking to utilize the Mintbase and NEAR ecosystem in order to enahnce the learning experience of our learners by issuing their certifications on the blockchain. We are focusing on the educational side of things and by utilizing web3, we will democratize and create a creators economy for educators and students.
  
### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
  - Our project fits in the NFT side of things because certificates need to be on the blockchain.
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
  - Our target audience are youngsters 16-25 years old who are interested in learning about financial literacy. They are mobile app users and they do not need to have prior web3 knowledge. 
- What need(s) does your project meet?
  - Our need is to be able to deploy smart contracts for our NFT certificates and a base for minting those NFTs.
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
  - There are project similar to our project such as RabbitHole but they are purely focusing on crypto education. We are focusing on financial literacy.
  

## Team :busts_in_silhouette:

### Team members

- Name of team leader
  - Eleftherios Laliotis
- Names of team members
  - Alexandros Adamakis, Ali Riza Dagli

### Contact

- **Contact Name:** Eleftherios Laliotis
- **Contact Email:** lefteris@swollet.com
- **Website:** https://swollet.com/

### Legal Structure

- **Registered Address:** 6 Thomastown House, Spencer Dock, Dublin 1, Dublin, Ireland
- **Registered Legal Entity:** Swollet Technologies Ltd.

### Team's experience
- Eleftherios Laliotis, CEO - 7+ years of investment management experience @ BlackRock and State Street. Founded Ultimo Digital Marketing.

- Alexandros Adamakis, COO - 3+ years of start-up business development experience @ Vecna Robotics & Salesforce.

- Ali Riza Dagli, CTO - 2+ years of full stack development experience.

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


### Overview

- **Total Estimated Duration:** 3 months
- **Full-Time Equivalent (FTE):**  1 FTE
- **Total Costs:** 10,000 USD

### Milestone 1 — Implement Mintbase / NEAR Modules on Testnet

- **Estimated duration:** 1 month
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can take a course and receive their NFT certificate upon completion. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Article | We will publish an **article** and a **video** that explains what was done and what was achieved as part of the grant). |
| 1. | Mintbase module: A | We will create a Mintbase / NEAR testnet module that will mint an NFT badge for each course created. |
| 2. | Mintbase module: B | We will create a Mintbase / NEAR testnet module that will mint an NFT and transfer it to the owner for each post each user makes. |
| 3. | Mintbase module: C | We will create a Mintbase / NEAR testnet module that will transfer the NFT certificate to each user's profile who completes the respective course or achievement. |
| 4. | Design UI/UX and implement | NEAR chain integration | Modules A, B & C of our custom chain will interact in such a way that every time a user posts or completes a course (and/or an achievement), they should be reveiving their NFT in their NEAR testnet wallet. |  


### Milestone 2 — Implement Mintbase / NEAR Modules on Mainnet

- **Estimated Duration:** 1 month
- **FTE:**  1
- **Costs:** 4,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | MIT |
| 1. | Mintbase module: A | We will create a Mintbase / NEAR mainnet module that will mint an NFT badge for each course created. |
| 2. | Mintbase module: B | We will create a Mintbase / NEAR mainnet module that will mint an NFT and transfer it to the owner for each post each user makes. |
| 3. | Mintbase module: C | We will create a Mintbase / NEAR mainnet module that will transfer the NFT certificate to each user's profile who completes the respective course or achievement. |
| 4. | Design UI/UX and implement | NEAR chain integration | Modules A, B & C of our custom chain will interact in such a way that every time a user posts or completes a course (and/or an achievement), they should be reveiving their NFT in their NEAR mainnet wallet. | 
| 5. | 100 NEAR wallets created | 50 daily active users (active user is defined as a wallet with 1 or more transaction
within a given period) | 



## Future Plans

- We have created a Discord channel and have invited some members (30) to start testing our MVP in order to get early feedback on our app.
- We have been invited to pitch on Dragon's Den in Greece which will give us some exposure as well as to the Mintbase and NEAR ecosystem.
- We have partnered up with National College of Ireland and they are providing us with educational content. We are going to gamify this content and start distributing via our mobile app (web app will come soon)
- We are active on LinkedIn, Twitter, TikTok in order to attract some early adopters.
- We have been pitching to investors in order to raise our pre-seed round and we have a soft commitment from Enterprise Ireland for a co-investment for up to 300,000 EUR.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** I was doing the NEAR tutorial for the NFT smart contracts and I saw that Mintbase is on the NEAR ecosystem. I have also started testing out the NFT minting process on the Mintbase testnet. Then, I saw the Grants application link and I clicked on it!

We have been awarded with a NEAR Foundation grant too.
