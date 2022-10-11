# Mintbase Grant Proposal

- **Project Name:** Gorilla Shops
- **Team Name:** Gorilla Funds UG
- **Payment Address:** gorillashops.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### Overview

Gorilla Shops is building the Batch-Minting-Tool "Automat". This Batch-Minting-Tool will provide an easy solution to mint and list collections just by uploading a well formed zip-File. This will make it easy to mint thousands of NFTs on Mintbase just with a few clicks.

### Project Details

The Tool will consists of two parts. An open API and a User Interface. The API will be open for everyone to use and the User Interface will be easy to add into your own project. The User Interface will be easy to customize so that it can be used by everyone, who wants to add this additional functionality to their DApps.
The Tool is in the first round designed to work best with Gorilla Shops Figma Plug-In Unikat. In the second iteration it will be generalized to make minting of batches and performing batch transactions like transfering of NFTs or Airdrops of NFTs way more easy like it is now.

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?

Automat is a usefull tool to help all projects that want to work with larger amounts of NFTs. Especially it is helpfull for creators of NFT collections.

- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?

We target Mintbase users that want to leverage Mintbase for minting collections or bigger amounts of NFTs.

- What need(s) does your project meet?

The API and the User Interface targets Mintbase Store owners that want to produce NFTs in larger amounts as they want to mint collections of unikats or thousands of NFTs.

- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?

As the project is designed to use with the Figma-Plugin their is right now no similar project out there.

- If so, how is your project different?

## Team :busts_in_silhouette:

### Team members

- CEO: Jakob Sievers

- Developers:

  - Sascha Metz (Full Stack)
  - Tobias Metz (Front End)
  - Jonas Hartweg (Front End, UI/UX-Expert)
  - Valentin Hehl (Security, Backend)

- Designer:

  - Reimar Servas

- Consultancy (Design Systems, Bridging Design and Code)

  - Reimar Servas

- Product Management:
  - Aymie Laipe

### Contact

- **Contact Name:** Jakob Sievers
- **Contact Email:** Jakob.Sievers@gorillashops.io
- **Website:** https://gorillashops.io

### Legal Structure

- **Registered Address:** Hermann-Wellensiek-Strasse 33, 67346 Speyer
- **Registered Legal Entity:** Gorilla Funds UG, Hermann-Wellensiek-Strasse 33, 67346 Speyer

### Team's experience

All Team members are experienced in their field as can be seen by our product.

- Jakob Sievers: Jakob Sievers is a former teacher for physics, chemistry and computer science. Founded the Gorilla Funds UG in 2020. I discovered the blockchain space in 2012 and got caught since then in the rabbit hole. Coding experience 15+ years.
- Reimar Servar: Reimar Servas is a Freelance consultant. Designer/Developer, early browser wars veteran. Working on systems and processes for humans, connecting programmers and designers.
- Tobias Metz: Tobias Metz is an full stack software engineer from south-west germany. He had worked in several  web-agencies  and hardware/network B2B company’s as well. He has an information technology background and ~10 years of customer/developer experience.
- Sascha Metz: Sascha Metz started building websites in 2012, is developing for a living since 2015. Started with "classic" WordPress Websites but nowadays i am all into the JAM-Stack. Also pretty enthusiastic about cloud computing, especially with AWS.
- Jonas Hartweg: Jonas Hartweg is a frontend engineer living in Berlin. He has ten years of development experience and worked for big B2B companies as well as smaller media agencies. Next to his work as a developer, he is currently finishing his M.A. in product design.
- Valentin Hehl: Valentin Hehl is a Computer Science student at the KIT University Karlsruhe and 21 years old. He was in the winning team of the European cyber security challenge in 2019 in the category _Junior_.
- Aymie Laipe: Aymie is our product manager and organizes our overall collaboration.

### Team Code Repos

- https://github.com/gorillafunds

Right now the repository is private. We will provide all necessary access to the repository to the Mintbase team.

Github-Repositories of the Team-Members:

- https://github.com/gorillafunds

Tobias Metz

- https://github.com/tmtz

Sascha Metz

- https://github.com/smetzdev

Jonas Hartweg

- https://github.com/johnny353535

Reimar Servas

- https://github.com/reimar

Valentin Hehl

- https://github.com/ZTube

Jakob Sievers

- https://github.com/gorillafund

Team Code Repos
Right now the Team Code Repo is private. We have to pull some things out as we are working with a mono-repo structure,
as this repository also contains our corporate site code

- https://github.com/gorillafund/gorilla-technologies

### Team LinkedIn Profiles

- Jakob Sievers: https://www.linkedin.com/in/jakob-sievers-97579b201/
- Reimar Servas: https://www.linkedin.com/in/reimarservas/
- Jonas Hartweg: https://www.linkedin.com/in/jonas-hartweg-66272449/
- Sascha Metz: No LinkedIn
- Tobias Metz: No LinkedIn
- Valentin Hehl: No LinkedIn
- Aymie Laipe: https://www.linkedin.com/in/aymie-laipe/

## Development Status :open_book:

The User inferface design is finished. This is how it will look like:

https://youtu.be/nsCMwNvmtgk

Right now we are building the components and the flow. The API is functional as well, but is not open accessible right now.

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 3 Months
- **Full-Time Equivalent (FTE):** 4 FTE
- **Total Costs:** 50000 US$

### Milestone 1: Release MVP:

- **Estimated duration:** 2 month
- **FTE:** 4
- **Costs:** 35,000 USD

| Number | Deliverable          | Specification                                                                                                                                                                       |
| -----: | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|    0a. | License              | MIT                                                                                                                                                                                 |
|    0b. | Documentation        | We will provide both **inline documentation** of the code and a basic **video tutorial** that explains how Automat works.                                                           |
|    0c. | Testing Guide        | Core functions will be fully covered by **unit tests** to ensure functionality and robustness. In the guide, we will describe how to run these tests.                               |
|    0d. | Testnet-API instance | We will provide a **Testnet-API-Instance** that can be used to test all the functionalites of our product.                                                                          |
|    0e. | Article              | We will publish an **article** that explains all features and how to use Automat.                                                                                                   |
|     1. | API                  | The API allow **minting, listing or burning of every amount of NFTs** on a store if our wallet has minted the NFTs. You can also transfer the ownership of any of these minted NFTs |
|     2. | User-Interface       | The User-Interface allows **easy minting of thousands of NFTs** where the necessary images and information are uploaded within a zipped folder.                                     |

### Milestone 2: Make it production ready:

This will include the following features

- Perform User-Tests to make the tool as user friendly as possible.
- Publish a npm-package for easy integration of Automat.
- Add detailed documentation
- Perform security and performance tests

- **Estimated duration:** 1 month
- **FTE:** 4
- **Costs:** 15,000 USD

| Number | Deliverable      | Specification                                                                                                                                     |
| -----: | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
|    0a. | License          | MIT                                                                                                                                               |
|    0b. | Documentation    | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user has to interact with Automat          |
|    0c. | Testing Guide    | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
|    0d. | Testnet-Instance | We will provide a Testnet-Instance that can be used to test all the functionalites of our product.                                                |
|    0e. | Article          | We will publish an **article** that explains all features and how to use them as well as two-weekly development updates in our discord-channel.   |

## Clarifications

The Code for the User Interface will be released under MIT-License. The backend code is not part of this proposal. The API-will be open for anyone to use who is the owner of a store on Mintbase.

## Future Plans

The next part of "Automat" will be a generalized version, that allows way more options for the minting operation of the NFTs, like individual options per NFT. Minting of thousands of just one NFT and way more.The following version will also add an easy white-label solution that can easily be integrated in your very own web app.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** From the Mintbase-Team
