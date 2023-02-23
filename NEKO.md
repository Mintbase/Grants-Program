![NEKO BANNER](https://user-images.githubusercontent.com/103082550/221048086-556f1f32-a824-4dac-8fa8-dd64e24623fb.png)
# Mintbase Grant Proposal

- **Project Name:** NEKO-AI Minter & Marketplace
- **Team Name:** NEKO
- **Payment Address:** nekotoken.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- NEKO AI, bringing the power of AI technology to web3.

- NEKO was born as the first meme coin on NEAR Protocol, filling a key role in the ecosystem. NEKO unites the NEAR ecosystem under a meme-powered force that the entire community can rally behind! Going beyond the memes, NEKO token powers the creator economy on NEAR Protocol. NEKO introduced the Learn to Earn movement that rewards our audience for engaging with NEKO content and is now introducing NEKO-AI to further empower creators on NEAR protocol.

The Next addition to the "NEKO-system" will be the NEKO-AI Minter & Marketplace! The potential for Modern AI technology to automate difficult tasks in a wide variety of industries has never been higher. The NEKO-AI minter will use existing AI-image generation technology to allow creators to instantly generate and mint NFTs with a single prompt. Users pay a fee in NEKO/NEAR per image generated & minted on the platform, or per monthly service subscription. Initially, the NEKO AI-minter will focus on creating 1/1 NFTs, and will provide an intuitive front-end, offering a variety of advanced functions allowing the creator increased control over AI-image generation. Additionally, we will create the NEKO AI-marketplace using the Mintbase API, which will allow users to display and list their AI-art immediately after minting.

- The NEKO-AI minter is a perfect fit for the Mintbase ecosystem because it will enable creators to utilize cutting-edge AI image generation technology to create unique pieces of art and mint them directly onto NEAR protocol. The integration of the NEKO AI-minter with the NEKO marketplace is a new and innovative addition to the NEAR ecosystem and will effectively showcase the Mintbase Marketplace API.

- Since the conception of NEKO, empowering creators has been one of our core tenants. We believe that when used together, AI and web3 technology can grant a significant advantage to creators and users within the NEAR ecosystem. By including the information used to generate the image on each NFT, users will have access to a database of prompts, keywords, and hash codes that can help them output their desired image. We are eager to explore the capabilities of AI in web3 applications and know that bringing this technology directly to builders/users  would provide an invaluable tool to both the NEKO, NEAR & Mintbase communities.

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

![NEKO AI-Minter Example (1)](https://user-images.githubusercontent.com/103082550/220839058-d1797cb2-a213-4851-8a2e-89230408e3c1.png)

- The NEKO-AI platform will feature a front-end that allows users to seamlessly create AI-generated art and mint NFTs directly on-chain while retaining the AI-generation data used in the image. The NEKO platform will focus on delivering a quality UX while delivering results that are as close to the user’s artistic preferences as is possible with current AI-tech.

- The user will be able to connect their wallet to access the platform and mint a an image if they are satisfied with the final image.

![AI NFT Example](https://user-images.githubusercontent.com/103082550/220839011-f9d8ed5c-4110-47d3-9c0e-8acc56a9e363.png)

- To further capitalize upon the AI-generated characteristics of the auto-minter, The 1/1 AI-NFTs will have the AI-generation prompt embedded into the NFT metadata, along with the  artist’s  unique signature and description. Users will be able to browse through the NEKO AI-marketplace by keyword filters that correspond to the same words used in generating the respective NFTs. 

![Neko Market Example (1)](https://user-images.githubusercontent.com/103082550/220839091-c28315f9-e835-48e3-bcd9-ec4746226893.png)

- The NEKO AI-marketplace brings a new layer to AI-image generation by effectively enabling users to not only view and display their own Ai-generated art, but to easily browse the NEKO gallery for art/artists they like from a detailed library of information. Artists will have an easier way of sharing how they created specific art pieces to their audience, and will be able to easily generate themed collections of NFTs with similar prompt keywords/code-hashes.

- The NEKO-AI Marketplace will be powered by the Mintbase API.

Data models / API specifications of the core functionality: For the NEKO AI Image Generator, we are using DALL-E 2 as the vision model.

- DALL-E 2 is a popular image generator powered by the Open-AI API and can be integrated into a variety of platforms.

-We have two endpoints in our API Specification for generating and describing an image respectively. The first endpoint, `POST /image/generate` generates an image from a textual description. The second endpoint, `POST /image/describe` describes the contents of an image.

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
 
![NEKO AI Core Components (1)](https://user-images.githubusercontent.com/103082550/220841029-f8700c82-0f62-4d12-b557-9310564130a0.png)

PoC/MVP or other relevant prior work or research on the topic:

- We have experience using AI technology/APIs and have developed a basic POC that can generate AI images and mint them on chain. We can provide a showcase upon request by the Mintbase grants team.

- The base NEKO AI platform will not be designed to mint entire collections initially. The base functionality is intended for 1/1 NFTs. The project will not develop in-house AI technology, but will leverage existing AI generator API (ex. Open AI, Midjourney) in conjunction with NEAR smart contracts to create the AI-minter. The next grant milestone will focus on creating multiple NFTs based on a prompt or reference image with traits to build larger related NFT collections.
 
### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- The NEKO AI-minter/Marketplace will bring a unique experience for creators on NEAR by leveraging popular AI-tech and bringing it on-chain. We believe that AI technology will quickly become a staple of every major industry and that the creative potential of AI is too great to be ignored.

- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?

The target audience is creators from any skill group that are interested in creating NFTs or Ai-art. The NFT metadata based on the generation prompt is a useful tool for both web2 and web3 creators and will be extremely appealing to creators that enjoy experimenting with AI-generated art and showcasing their unique artistic visions.

- What need(s) does your project meet?

Our project is one of the first to integrate AI art generation with web3 on NEAR and many other blockchains. Our project showcases the Mintbase marketplace developer tools.

- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?

Currently, there are not any significant projects using AI-image generators on NEAR.

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

- The Trove labs team is developing multiple projects on NEAR, including NEKO, and Jump DeFi. We have extensive experience with a variety of smart contracts, front-end design, token economics and web3 business development. We have deployed multiple NFT contracts on NEAR and have all of the skills necessary to create the above proposed project.

- We built 2 successful NFT collections for NEKO as well as a staking platform with both NFT and NEKO single-sided staking.

- For Jump DeFi, we have created multiple contracts comprising the Jump NFT staking platform, Jump Token Launcher, xJUMP single sided staking pool, Jump IDO platform and more. 

- Our team is also proficient with building strong communities and marketing within the NEAR ecosystem in an effort to bolster adoption and stickiness for web3 users interested in NEAR protocol.

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

### Overview

- **Total Estimated Duration:**  2.5 months
- **Full-Time Equivalent (FTE):**  2
- **Total Costs:** $25,000

### Milestone 1 — Create NEKO-AI & Front-end with Autominter integration

- **Estimated duration:** 1.5 months
- **FTE:**  2
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can use our front end to call AI API functions to generate detailed images and how to mint an image with Metadata based on prompt generation. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an article/workshop that explains the functions of the NEKO AI-Autominter and describes how to integrate an AI generation API into an on chain platform on NEAR.
| 1. | AI Prompt Front-end Integration | We will develop a front end powered by a 3rd party AI image generator that will allow users to generate images from customizable prompts.
| 2. | Auto Minter  | We will develop a smart contract that will connect to the AI front-end to seamlessly mint an image generated by the AI with applicable metadata & descriptions inputted by the user. |  

### Milestone 1 — Create NEKO-AI Marketplace & Integrate Auto-minter/AI generator

- **Estimated duration:** 1 months
- **FTE:**  2
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can interact with the marketplace front-end. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an article/workshop that explains the functions of the NEKO AI-marketplace and the unique use-cases for AI generated NFTs
| 2.1. | NEKO AI-Marketplace | We will create a NEKO AI-marketplace using Mintbase architecture that allows users to create and list 1/1 AI-generated NFTs.
| 2.2. | Auto Minter  | All three components will be integrated and launched onto NEAR mainnet.
 |  
...
## Future Plans

- We intend to market the new NEKO-system feature to our community by leveraging NEKO giveaways, marketing events (spaces, podcasts), and NEAR ecosystem collaborations. We will develop the above proposal into a high-quality functioning product. The "NEKO-system" will continue to be developed, improved and maintained well into the future thanks to the support from our community. Continued development of NEKO-AI after completion of this proposal will explore advanced functionalities including minting large NFT collections from a single prompt or reference image.

## Additional Information :heavy_plus_sign:

- Due to the the monetization models used by Open-AI & Midjourney, we will create sustainable monetization structures for the platform but will ensure that there is a portion of grant funding dedicated onboarding users keeping the platform operational initially.

**How did you hear about the Grants Program?** 

- We heard about the mintbase grants program from the NEAR foundation.

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- We have created a proof of concept for the platform and can present it to the Mintbase team.

- Trove labs helped Jump DeFi receive a grant for proximity to launch phase 1 of the platform on mainnet.
