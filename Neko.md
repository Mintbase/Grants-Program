![NEKO BANNER](https://user-images.githubusercontent.com/103082550/221048086-556f1f32-a824-4dac-8fa8-dd64e24623fb.png)
# Mintbase Grant Proposal

- **Project Name:** NekoAI auto-minter & Marketplace
- **Team Name:** Neko
- **Payment Address:** nekotoken.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

- **_Neko is bringing the power of AI technology to the NEAR ecosystem._**

- Neko was born as the first community meme token on NEAR Protocol, filling a key role in the ecosystem. Neko unites the NEAR ecosystem under a shared mission that the entire community has rallied behind! NEKO comes in as the #3 most held NEP-141 token in the NEAR ecosystem with over 22K unique holders, proving we have strong buy-in from the NEAR community as a whole!

- Going beyond the memes, Neko powers the creator economy on NEAR. Neko has onboarded crypto content creators, with audiences ranging from 9K to 92K+ followers, into the NEAR ecosystem to make educational content. So far we have created hundreds of TikToks and YouTube videos that have amassed hundreds of thousands of views. The Nekosystem is now scaling to three exciting verticals; NekoAI NFT Minter and Marketplace, Gaming and dApp development.

- The NekoAI Minter uses powerful existing AI-image generation technology and NEAR smart contracts to allow creators to instantly generate and mint NFTs via a single prompt! AI-image generation enables anybody to create with limitless possibilities. NFT technology makes ownership of artwork verifiable and easily transferable via the blockchain. NekoAI users capture the value of AI art and NFT technology by simply inputting a prompt of their choosing. Users pay a fee in NEKO/NEAR per image generated & minted on the platform, or per a monthly service subscription.

- Initially, NekoAI will focus on generating 1/1 NFTs, with an intuitive front-end and a variety of advanced functions that allow creators to have increased control over AI-image generation. Additionally, we will build the NekoAI Marketplace using the Mintbase API, allowing users to display and list their AI-art immediately after minting into a NEAR NFT. NekoAI is a perfect fit for the Mintbase ecosystem because it enables creators to utilize cutting-edge AI image generation to create unique pieces of art and mint them directly into a non-fungible token. The integration of the NekoAI Minter with the Neko Marketplace is a new and innovative addition to the NEAR ecosystem that will effectively showcase the Mintbase Marketplace API.

- Empowering creators is a core tenant of the Nekosystem. We believe combining AI and NFT technology is a perfect solution for creators to gain provable ownership over the art they generate. By including the keywords used to generate the image in the metadata of each NFT, users have access to a database of prompts, keywords, and hash codes that can help them output their desired image.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

**NekoAI Auto-Minter**
![NEKO AI-Minter Example (1)](https://user-images.githubusercontent.com/103082550/220839058-d1797cb2-a213-4851-8a2e-89230408e3c1.png)

- The NekoAI platform allows users to seamlessly create AI-generated art and mint as an NFT directly on-chain while retaining the data used in the image. NekoAI focuses on an intuitive UX that guides users through the prompt engineering process to deliver results that closely match the users creative goals.

- Users can connect their wallet to access the platform and mint an image into a NEAR NFT once they are satisfied with the final generated image.

**NekoAI NFT**
![AI NFT Example](https://user-images.githubusercontent.com/103082550/220839011-f9d8ed5c-4110-47d3-9c0e-8acc56a9e363.png)

- To further capitalize on the AI-generated characteristics of the auto-minter, The AI-NFTs embed the prompt inputs into the metadata, along with the artist’s unique signature and description. 

- Users can browse through the NekoAI Marketplace with keyword filters that correspond to the same words used in generating the respective NFTs. Creators can utilize the prompt-inputs and seeds from other AI-artwork to build upon an existing NFT. 

**NekoAI Marketplace**
![Neko Market Example (1)](https://user-images.githubusercontent.com/103082550/220839091-c28315f9-e835-48e3-bcd9-ec4746226893.png)

- The NekoAI Marketplace brings a new layer to AI-image generation by effectively enabling users to view and display their own AI-generated art, as well as easily browse the NEKO gallery for art/artists they like from a detailed library of information. Artists now have an easier way of sharing how they created specific art pieces to their audience, and can easily generate themed collections of NFTs with similar prompt keywords/code-hashes.

- The NekoAI Marketplace will be powered by the Mintbase API.

Data models / API specifications of the core functionality: 

- For the NekoAI Image Generator, we are using DALL-E 2 as the vision model.

- DALL-E 2 is a popular image generator powered by the Open-AI API and can be integrated into a variety of platforms.

- We have two endpoints in our API Specification for generating and describing an image respectively. The first endpoint, `POST /image/generate` generates an image from a textual description. The second endpoint, `POST /image/describe` describes the contents of an image.

## **Data Model**

### **ImageDescription**

- **`description`** (string): Textual description of the image.
- **`size`** (string, optional): Size of the generated image in pixels (e.g., "256x256"). Default is "512x512".
- **`response_format`** (string, optional): Format of the API response (e.g., "url", "base64", "bytes"). Default is "url".

### **GeneratedImage**

- **`image`** (string): Generated image in the specified format (e.g., URL, base64-encoded string, bytes).

### Inputs

The prompt input for DALL-E is a textual description of the image that the user wants to generate. The input can be a single sentence or a paragraph of relevant artistic information. The model can also take multiple inputs at once, separated by a delimiter.

### Outputs

The output of DALL-E is an image that corresponds to the input textual description. The image is generated using a combination of convolutional neural networks and generative adversarial networks. The output image can be of any size, depending on the user's specifications.

## **API Specification**

### **`POST /image/generate`**

Generate an image from a textual description.

**Request**

- **`description`** (string): Textual description of the image.
- **`size`** (string, optional): Size of the generated image in pixels (e.g., "256x256"). Default is "512x512".
- **`response_format`** (string, optional): Format of the API response (e.g., "url", "base64", "bytes"). Default is "url".

**Response**

- **`200 OK`**: Successfully generated an image.
    - **`Content-Type: application/json`**
    - **`GeneratedImage`** object containing the generated image in the specified format.
- **`400 Bad Request`**: Invalid request parameters.
- **`500 Internal Server Error`**: Server error.

### **`POST /image/describe`**

Describe the contents of an image.

**Request**

- **`image`** (string): Image file in base64-encoded format.
- **`response_format`** (string, optional): Format of the API response (e.g., "text", "json"). Default is "text".

**Response**

- **`200 OK`**: Successfully described the image.
    - **`Content-Type: application/json`** or **`text/plain`**
    - Textual description of the contents of the image.
- **`400 Bad Request`**: Invalid request parameters.
- **`500 Internal Server Error`**: Server error.

An overview of the technology stack to be used:
-AI image generator API (Open AI, Midjourney etc.), Modified NEAR NFT contracts, Back end that supports AI end-user functionality.

**NekoAI Core Components** 
![NEKO AI Core Components (1)](https://user-images.githubusercontent.com/103082550/220841029-f8700c82-0f62-4d12-b557-9310564130a0.png)

PoC/MVP or other relevant prior work or research on the topic:

- We have experience using AI technology/APIs and have developed a basic POC that can generate AI images and mint them on chain. We can provide a showcase upon request by the Mintbase grants team.

- The base NEKO AI platform will not be designed to mint entire collections initially. The base functionality is intended for 1/1 NFTs. The project will not develop in-house AI technology, but will leverage existing AI generator API (ex. Open AI, Midjourney) in conjunction with NEAR smart contracts to create the AI-minter. The next grant milestone will focus on creating multiple NFTs based on a prompt or reference image with traits to build larger related NFT collections.
 
### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- The NEKO AI-minter/Marketplace will bring a unique experience for creators on NEAR by leveraging popular AI-tech and bringing it on-chain. We believe that AI technology will quickly become a staple of every major industry and that the creative potential of AI is too great to be ignored.

- The target audience for NekoAI is creators from any skill group that are interested in creating NFTs or Ai-art. The NFT metadata based on the generation prompt is a useful tool for both web2 and web3 creators and will be extremely appealing to creators that enjoy experimenting with AI-generated art and showcasing their unique artistic visions.

- Our project is one of the first to integrate AI art generation with web3 on NEAR and many other blockchains. Our project showcases the Mintbase marketplace developer tools.

- Currently, there are not any significant projects using AI-image generators on NEAR.

## Team :busts_in_silhouette:

### Team members

Brandon Cantu: Co-Founder of Trove Labs. 

Former Laboratory Manager with experience leading teams and scaling operations at a biochem startup. 

Co-Host of NEAR at NIGHT


David Leer: Co-Founder of Trove Labs.

Former Senior PM with Engineering background and experience managing projects $350MM in scope.

“Most Influential” on NEAR and Co-Host of NEAR at NIGHT.  


Eric Cheung: Head of Development

Full stack developer with 6+ years of professional software engineering experience in ML/AI automation, gaming and blockchain development.


### Contact

- **Contact Name:** Brandon Cantu

- **Contact Email:** team@trovelabs.xyz
- **Website:** https://www.nekotoken.xyz/

### Legal Structure

- **Registered Address:** QUIJANO & ASSOCIATES (BVI) LIMITED, Quijano Chambers, P.O. Box 3159, Road Town, Tortola, British Virgin Islands.
- **Registered Legal Entity:** Good Fortune Felines Limited

### Team's experience

- Trove Labs is a Web3 development and consulting firm with a focus on the NEAR ecosystem. We are experienced with smart contract development, front-end development, tokenomics engineering and web3 business development. Trove Labs is building Neko and Jump DeFi; two highly regarded Web3 startups in the NEAR ecosystem. Jump DeFi and Neko serve as a portal between NFTs and DeFi on NEAR.

- Between Neko and Jump DeFi we have developed a wide variety of dApps including: multiple NFT staking and fungible token staking contracts, no-code smart contract deployer, token launchpad for public and private sales, multiple NFT collections, mass airdrop and snapshot tool, and more! 

- The team has a track record of consistently effective social media marketing campaigns which have kept Neko and Jump DeFi at the top of the NEAR Golden Board for highest engagement, week after week, for nearly an entire year. Strong marketing experience combined with our community centric ethos has enabled us to onboard 15K+ new users into NEAR while serving thousands of daily active users.  

### Team Code Repos

- [Trove Labs](https://github.com/Trove-team)
- [NEKO/Good Fortune Felines](https://github.com/Good-Fortune-Felines-Core-Team)
- [Jump DeFi](https://github.com/Jump-Dex)

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- [Eric Cheung](https://github.com/ymc182)

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/brandon-cantu-758011183/
- https://www.linkedin.com/david-leer

## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- https://platform.openai.com/docs/guides/images/introduction

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:**  2.5 months
- **Full-Time Equivalent (FTE):**  2
- **Total Costs:** $25,000

### Milestone 1 — Create NekoAI & Front-end with Autominter integration

- **Estimated duration:** 1.5 months
- **FTE:**  2
- **Costs:** 17,500 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can use our front end to call AI API functions to generate detailed images and how to mint an image with Metadata based on prompt generation. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an article/workshop that explains the functions of the NEKO AI-Autominter and describes how to integrate an AI generation API into an on chain platform on NEAR.
| 1.1 | AI Prompt Front-end Integration | We will develop a front end powered by a 3rd party AI image generator that will allow users to generate images from customizable prompts.
| 1.2. | Auto Minter  | We will develop a smart contract that will connect to the AI front-end to seamlessly mint an image generated by the AI with applicable metadata & descriptions inputted by the user. |  

### Milestone 2 — Create NekoAI Marketplace & launch platform MVP to mainnet

- **Estimated duration:** 1 months
- **FTE:**  2
- **Costs:** 7,500 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can interact with the marketplace front-end. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an article/workshop that explains the functions of the NEKO AI-marketplace and the unique use-cases for AI generated NFTs
| 2.1. | NEKO AI-Marketplace | We will create the NEKO AI-marketplace using Mintbase architecture that allows users to create and list 1/1 AI-generated NFTs. Front end will come with advanced search and filter functionality using AI generation information stored in metadata.
| 2.2. | Near Mainnet Integration | All three components (Auto-minter, Image generator, & Marketplace) will be integrated and launched onto NEAR mainnet.
 |  
...
## Future Plans

- Once development of the AI Minter and Marketplace is completed we will move onto Phase 2 development. Phase 2 will enable users to create full scale generative NFT collections with traits powered by AI generation. Users will be able to isolate specific traits from various images and incorporate them into a generative NFT collection. A trait library on the NekoAI marketplace will enable users to customize and upgrade traits from existing collections as well. It has never been easier to create a generative NFT collection than with NekoAI!

- When NekoAI and the other Nekosystem expansion items are completed, we will embark on the largest marketing campaign yet; targeting crypto-natives from other blockchain ecosystems as well as non crypto-natives including AI artists and gamers to onboard them into the NEAR ecosystem.

## Additional Information :heavy_plus_sign:

- Due to the the monetization models used by Open-AI & Midjourney, we will create sustainable monetization structures for the platform but will ensure that there is a portion of grant funding dedicated onboarding users keeping the platform operational initially.

**How did you hear about the Grants Program?** 

- We heard about the mintbase grants program from the NEAR foundation.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- We have created a proof of concept for the platform and can present it to the Mintbase team.

- Trove labs helped Jump DeFi receive a grant for proximity to launch phase 1 of the platform on mainnet.
