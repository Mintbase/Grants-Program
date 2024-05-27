# Marketplace in Unity - Mintbase Grant Proposal



- **Project Name:** Mintbase Marketplace in Unity
- **Team Name:** Rogue Games Limited
- **Payment Address:** rogues.near
- **[Level](../README.md#level_slider-levels):** 2


## Project Overview :page_facing_up:


### Overview

Unity is one of the most popular game engines for creating web3 games (in the UK, for example, [72%  game developers use Unity](https://www.statista.com/statistics/321059/game-engines-used-by-video-game-developers-uk/), according to Statista). Many web3 developers want a simple solution for integrating a marketplace directly into their games so that players can trade items with other players in a straight forward manner.

By integrating the Mintbase marketplace directly into a Unity game, players can stay immersed in the game and trade with other players without ever leaving the game.

This ease of trading will encourage more interaction with other players, lowering the barrier and enabling more sales.

### Project Details

The Unity implementation will based on the example:

[Mintbase Simple Marketplace Example](https://marketplace-template.mintbase.xyz/)

Since this example relies heavily on the @mintbase-js/sdk, this will be integrated into Unity as part of the project, essentially enabling the whole of the mintbase js sdk in Unity.

The technology stack will consist of Unity, an enhanced WebGL template that imports the relevant js modules and a Unity JavaScript plugin that provides the interface between the JavaScript code and the C# Unity code.

The Unity game builds for this phase will be limited to WebGL builds. Firstly, this tends to be the most popular build for web3 games since both Google and iOS stores prohibit trading crypto assets. Secondly, supporting WebGL builds is simpler.

### Ecosystem Fit

NFTs have a strong future in gaming. Digital ownership of in-game items is a great use-case for NFTs and SFTs, giving these assets a tangible utility. This means that marketplaces like Mintbase will need to cater for gaming, gamers, support SFTs and support direct integration into game engines. This project is the first step in that integration process. The target audience of this project is any game developer who wants to integrate a web3 marketplace in their game but wants to reduce that integration development work from weeks to minutes.
There isn't a similar existing project in the NEAR ecosystem but a good example of another Unity SDK that provides an in-game NFT marketplace is:
https://docs.venly.io/docs/gaming-sdk-overview.

By creating a NEAR-native, NEAR-first Unity-integrated marketplace, this project will empower game developers within and outside the NEAR ecosystem to create their successful in-game economies, and contribute to the NEAR ecosystem's growth.

## Team

### Team members

- Lead: Morgan Page
- CTO / Technical Development: Morgan Page
- Project & Design Management: Anna Stoilova
- UX/UI Design: Simone Silva


### Contact

- **Contact Name:** Anna Stoilova
- **Contact Email:** anna@rogues.studio
- **Website:**  (https://www.rogues.studio)

### Legal Structure

- **Registered Address:** 63/66 Hatton Garden, Fifth Floor Suite 23, London, EC1N 8LE, UK
- **Registered Legal Entity:** Rogue Games Limited, Company registered in the UK, No.14368511

### Team's experience

The technical lead of the project, Morgan Page, is the person who developed the Unity SDK for NEAR. This API supports Android, iOS and WebGL, and has been used by game developers in the NEAR ecosystem to develop their own games on NEAR.

Rogues Studio is here to accelerate the adoption of web 3 gaming. We aim to reach billions of gamers by creating an easy onboarding to new and unique gaming experiences that redefine the genre.

The core founders of Rogues Studio, Morgan Page and Anna Stoilova, have over 40 years of combined game and VR development experience. From US to East Asia, they have delivered gamified training to World Health Organisation, Qatar Airways, hospitals and school districts in the US, and got on the list of Top VR UK companies.

The co-founder and CTO Morgan Page has previously single handedly created a mobile game with 1.4 million downloads, and has over 104,000 game development students on Udemy.

### Team Code Repos

- https://github.com/morganpage
- https://github.com/morganpage/near-api-unity
- https://github.com/morganpage/unity-multiplayer-blockchain-near-course

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/morganjpage/
- https://www.linkedin.com/in/annastoilova/
- https://www.linkedin.com/in/simone-silva-70178524/

## Development Roadmap



### Overview

- **Total Estimated Duration:** 3.5 months
- **Full-Time Equivalent (FTE):**  3
- **Total Costs:** 30,000 USD

### Milestone 1 -  Project Design

- **Estimated duration:** 3 weeks
- **FTE:**  2
- **Costs:** 5,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 1. | Project UI/UX Designs | Figma document outlining the in-game marketplace UI/UX |
| 2. | UI assets/art commissioned | Breakdown of assets commissioned and costed |


### Milestone 2 - Unity / mintbase-js interface code

- **Estimated Duration:**  1 month
- **FTE:**  2
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 1. | WebGL Template | WebGL Template enabling @mintbase-js access |
| 2. | Unity JavaScript Plugin | C# / JS interface code |

### Milestone 3 - Unity UI/UX and core development

- **Estimated Duration:**  1 month
- **FTE:**  3
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 1. | Unity UI/UX development | Unity UI/UX panels etc |
| 2. | Integration code | Hooking up UI with interface code etc |

### Milestone 4 - Product delivery

- **Estimated Duration:**  3 weeks
- **FTE:**  3
- **Costs:** 5,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide **inline documentation**  |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests |
| 0d. | Docker | N/A but live, deployed example will be made available |
| 0e. | Article | We will publish an **article** on the project on our socials
| 1. | Product tested | Bug fixing and beta testing |
| 2. | Product delivered | Unity Asset store submission |


## Future Plans

The launch of our game, World of Rogues, on the NEAR Mainnet, and the integration of our NFT avatars (and plans to integrate the in-game NFTs for a full game economy on NEAR), made us think about the need for such an in-game marketplace for every game planning to have a long-term, sustainable game economy in their game. While World of Rogues may be one of the first games to use such a marketplace, we can imagine many other games following suit, as many gaming products will need it.

Immediately after release, we’re planning to:
- Announce the launch/completion of the project to our community of more than 30,000 on socials, Discord and newsletter
- Integrate the final product into the Web version of our game, WoR
- Continue improving and enhancing the product based on user research and feedback from our users
Build any additional features that may be needed, based on user feedback and feature requests

In the long-term, we’re planning to:
- Continue using the product in our game(s), World of Rogues, and any additions/ new titles we make;
- Keep improving / enhancing / building feature based on user feedback

Most importantly, World of Rogues is a MMORPG, relying on in-game economy and trading for revenue generation, as well as compelling gameplay.

Our plans are to use the marketplace for the long-term, for in-game asset trading and revenue generating. It’s a fundamental part of our game’s business model.

## Additional Information

**How did you hear about the Grants Program?** We are heavily involved in the NEAR ecosystem, and we heard about the grants from everywhere:
- NEAR Horizon Accelerator Programme, where we were part of Cohort 1
- NEARCON 2023
- Social media
- Word of mouth

We have previously received grants from the NF to launch World of Rogues on Mainnet (completed), and Morgan Page has received a grant to create the Unity SDK API on Unity (completed).
