# V-Art NFT Converter. Mintbase Grant Proposal

- **Project Name:** V-Art NFT Converter
- **Team Name:** V-Art
- **Payment Address:** v-art-protocol.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

V-Art NFT Converter provides the utility of converting 2D and 3D digital assets right in the process of NFT minting, enabling a full bridge to their integration to digital platforms (metaverses, gaming, AR/VR, social media etc.).

- Project description:

V-Art is a blockchain platform that helps creative, tech, and luxury brands to license and monetize digital content through V-Art Protocol and provides virtual and augmented reality experiences. Brands and projects start earning new revenue from licensing their digital assets as sub-NFTs with IP rights and integrate them into Web3 world after connecting our white-label V-Art Protocol (currently on NEAR and Ethereum). For more information, visit https://vartprotocol.com/ and https://v-art.digital/.
Benefits: Connect the V-Art Protocol for free or get a customized integration for any platform, website or marketplace; earn new revenue by licensing any asset across multiple platforms and industries (integrating into metaverses, games, AR/VR solutions and social networks); verify, secure, and track IP rights on supported blockchains; simplify and lower development and rights management costs for any creative digital asset.


- Relation to the Mintbase / NEAR ecosystem:

The conversion solution helps to set different formats for the source file and also offers a ready-made built-in WEBGL viewer for compressed 3D models for users and NFT buyers.
In addition to being open-source for any creators on Mintbase and NEAR, the V-Art NFT Converter will be a part of utilities, provided by the V-Art Protocol. Which was launched in a Beta version in September 2022 on NEAR and Ethereum, is already a working solution with the first 25 integrations to be delivered in autumn. Full version is to be announced on the WebSummit, Nov 2. 
We seek a grant support from Mintbase, as both the community and vision correspond to our next steps and mission to enable multi-platform usage of creative assets for a new era, the Internet of Value. 

- Why we are interested in creating this project:

Our mission is to help creators, brands and anyone in the ecosystem to license and integrate their creative digital assets, thus, accelerating a new wave of digital creativity across Web3, making our dream of using any creative digital asset across multiple platforms come true. Because everyone should have a choice to live their digital lives with what they love.
We are interested in using conversion solutions to provide fast, clear and open content integrations between different platforms. Based on the results of the completed project, it will be possible to write a copy of the converted file into NFT to simplify the subsequent verification of its use. V-Art NFT Converter will be a part of the V-Art Protocol solution which provides the possibility to license their IP content on NEAR, supporting multiple contracts.


### Project Details

One of the main issues in the right to use digital content and copyright in relation to digital content is the change in file formats during use and integration. Our conversion solution shows how to bind an already converted file to NFT with the additional possibility of subsequent validation of the converted file. Thus, the minified NFT will have links not only to the original media, but also to the converted copies, some of which may already have links to the viewer or other integrations. Also, the approach to storing information about validated file storage is additional evidence of the authenticity of the NFT that is used and resold.

The main use case that our team will consider in the demo project is the sale of NFT art and web galleries, but this solution is used for completely different content - from e-commerce to games.

- **Data models / API specifications of the core functionality**

Project architecture: 
 <img alt="image" src="https://user-images.githubusercontent.com/116814057/199207954-c961f1d1-52b8-4386-b105-a6622250dc11.png">
 
Conversion info storage flow:
 <img alt="image" src="https://user-images.githubusercontent.com/116814057/199238136-4b9acd1c-9d7f-4a32-b8e3-ead83b1e0abb.png">

- **Technology stack**:
  - Node.JS
  - Typescript
  - Vue.JS
  - Mintbase JS SDK, Mintbase API
  - IPFS (connected by user, we will use Pinata) as storage 
  - CI/CD for deployment
  -	WebGL for 3d asset viewer (three.js)

- **What will be published**:
  -	SDK that allows connecting multiple converters in order to transform NFTs.
  -	Unified protocol of how to use converted data.
  -	Project that prepares 3d models (.fbx, .obj, .stl) for light-weight WebGL extension with help of the SDK described above.
  -	Examples of integration of such solutions.

- **What your project is _not_ or will _not_ provide or implement**
  - We will not provide a market-ready marketplace for 3d assets but modules with examples of how to connect conversion libraries. 
  
### Ecosystem Fit

We will bring our unique expertise in IP law and immersive tech to the Mintbase community, providing the converter to basically any NFT or sub-NFT, issued. We support cross-contract licensing already together with moderation of brands, creators, and much more.

- **Mintbase ecosystem seems like a perfect match for us as:**

1.	Our target audience are brands and businesses together with single creators, and our team has an immense experience in helping them to onboard immersive technologies (3D/AR/VR).
2.	We are already on NEAR with our V-Art Protocol.
3.	Minbase gives more utilities in comparisons to other ecosystems.

- **Target audience:**

1)	Brands and single creators, NFT marketplaces. Spheres: art, fashion, luxury, gaming, music, video, AR/VR, collectibles, avatars, metaverses.
2)	Developers: multi and cross-chain, NFT marketplaces.
3)	Our user base: 44,5K in total, 137 companies for V-Art Protocol, which will add the NFT Converter as a bonus utility.

-	**What need(s) our project meets:**

Today there is a demand for the use various creative digital assets all across the web3 world from games, art, fashion and e-commerce industries. One of the problems, stopping the market from mass adoption of digital assets (and making something like Ready Player One not our reality yet), in addition to questionable liquidity and IP rights mismanagement, is the need to change the file format for its further use and cross-integration between platforms. This is especially true for 3D content, but currently this conversion option is not provided by any NFT platform or Protocol, as the file within the NFT is usually stored on IPFS in only one possible format. From the legal standpoint, further changing the file format of the asset bought as NFT, to integrate it to other platforms, means that the rights for this change should have also been provided at the moment of the sale. Thus, the brands and creators cannot really track how their content is being further modified, integrated and used. It’s a time for a change.

- **Similar projects:**

There are currently no full competitors to the V-Art NFT Converter as a solution: the ones, which cover the conversion directly inside the NFT or sub-NFT during the minting, enabling the cross-platform integrations, as we do. Partial:
1)	Wrapping solutions. Cover a so called “wrapping” of existing NFTs, do not provide full cycle (from transaction with conversion of the file to actual consumption, struggle to add different file options & cross-platform usage, provide ready-to-go solutions. 
2)	Embed of previews to social media, virtual spaces, etc. Do not cover high resolution, 3D models, videos – only low resolution static imagery.

## Team :busts_in_silhouette:

### Team members

- Project Coordinator: Anastasiia Gliebova
- Technical Lead: Anton Dembytskyi
- 3d Assets guidance: Ruslan Kapustin
-	Full Stack/SDK Dev: Valerii Beloenko
-	Front-end Dev: Bogdan Malina
- Back-end Dev: Kostia Lopykh
- UI/UX design: Victor Grytsenko
- Marketing and communication: Eric Doyle 

We are in partnership with development companies Sigma, Ideasoft and Softuup who are providing business development, Rust development and consultancy.

### Contact

- **Contact Name:** : Anastasiia Gliebova
- **Contact Email:** anastasiia.gliebova@v-art.digital
- **TG:** anastasiia.gliebova@v-art.digital
- **Website:**  [v-art.digital](https://v-art.digital/), [vartprotocol.com](https://vartprotocol.com/),

### Legal Structure

- **Registered Address:** 1221 College Park Dr Suite 116, Dover, Kent, 19904, Delaware, USA
- **Registered Legal Entity:** V-Art Corp.

### Team's experience

V-Art team combines expertise in IP law, Blockchain, and immersive tech (3D/AR/VR), and is highly strengthened by a board of advisors and mentors. Founders are all serial Ukrainian entrepreneurs, located in Portugal, UK and Ukraine, who have previously built legal and tech businesses together. Now we have 14 full-time colleagues all over the world, our own tech department and an extensive IP portfolio of our own virtual exhibition tech, AR and VR applications, as well as a unique tech model for blockchain-powered licensing.  

Our mission: to launch and help to monetize a new wave of digital creativity across Web3. Our vision: to enable multi-platform licensing of creative assets for a new era, the Internet of Value.

V-Art’s executives, directly working on the project:

1)	**Anastasiia Gliebova, CEO and Co-founder**

Ukrainian tech entrepreneur with a deep passion for blockchain and creative digital assets. Prior to V-Art, she used her background in international law and business at 5+ years of experience as a manager and legal advisor of over 30 creative projects. She is also Partner and Head of Research at the Institute of Law, Technology and Innovation, guest lecturer at multiple universities, including Sotheby’s Institute of Art.

2)	**Anton Dembytskyi, CTO** 

Results-oriented IT professional with over 8 years experience in development and technical leadership for gaming, AR/VR, AI and blockchain projects. Anton leads the R&D team at V-Art, including product development, technical analysis and market research from initial idea to the final product launch. He’s led over 30 development projects for various startups and corporations in the USA, Ukraine, UK, EU and Israel. In 2017 Mr. Dembytskyi founded the software development company, Soft U Up, which provides services for AR/VR and complex R&D solutions. He also invented an artificial intelligence app for a new board game similar to backgammon.

3)	**Ruslan Kapustin, CCO**

Seasoned professional with over 14 years of emerging technology and application development experience. With a high business acumen and natural aptitude for leading international teams, Ruslan oversees the entire creative project lifecycle from initial scoping to deployment. His well-honed communication, negotiation, resource management and systems analysis skills enable his teams to maximize efficiency and ROI. Ruslan is the author of the first VR courses in Ukraine.

4) **Eric Doyle, Chief Communications Officer**

Eric has over 20 years of experience developing and executing integrated communications campaigns for over 50 tech startup and Fortune 500 brands in the crypto, blockchain and fintech sectors. He led Netflix’s first, record-breaking U.S. Latino PR campaign and designed and executed high-profile campaigns for startups Caviar and Embark, resulting in their acquisitions by Square and Apple, respectively. He’s also worked with crypto and blockchain brands Apifiny, Roxe, CasperLabs, Abra, Civic, Kik, Cindicator, Altonomy and Appsolutely, and Silicon Valley legend and former Cisco CEO, Judy Estrin. He’s spoken at and designed speaker sessions for Consensus, Digital Asset Summit NY, BlockCon, and The North American Bitcoin Conference among others. Fluent in Spanish, Eric served as a Peace Corps Volunteer in Panama. He is also a standup comedian.


### Team Code Repos

Solutions were closed-sourced so far, for that reason we cannot provide source code of the V-Art Protocol project.

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- [https://github.com/KostiaLopukh](https://github.com/KostiaLopukh)
- [https://github.com/MalinaBogdan](https://github.com/MalinaBogdan)
- [https://github.com/valexandrovich](https://github.com/valexandrovich)
- [https://www.behance.net/victordetroy](https://www.behance.net/victordetroy)

### Team LinkedIn Profiles (if available)

https://www.linkedin.com/in/anastasiia-gliebova-96a4681a3

https://www.linkedin.com/in/therealericdoyle/

https://www.linkedin.com/in/ruslan-kapustin

https://www.linkedin.com/in/anton-dembitsky-ba71471b9/

## Development Status :open_book:

The team has already developed smart contracts that allow for the licensing of digital assets, as well as a large number of interactive exhibitions. A top-level architecture of the conversion service, an algorithm for the operation of converters, and a specification for the conversion object have been developed. The team has extensive experience in developing interactive web galleries, working with 2D and 3D content in media projects so we have already worked with all convertional solution which are described in this application. 

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 2.5 months
- **Full-Time Equivalent (FTE):**  4.5 FTE
- **Total Costs:** 48 000 USD

### Milestone 1 Converter API development

- **Estimated duration:** 4 weeks
- **FTE:**  4
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 License |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can interact with the 3d assets and convert them. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 1. | UI development | Creation on minting and viewing page UI/UX. Providing the Figma-based design of the solution. Important flows: minting, previewing, interaction with NFTs, integration to other platforms. |  
| 2. | Conversional metadata module | Development of the metadata formatter for minted NFTs (add more formats) |  
| 3. | IPFS upload module | IPFS upload of conveted assets and binding to the NFT |  
| 4. | Draco and conversion module implementation | Implementation of [Draco](https://github.com/google/draco) compressing library. Implementation of [Facebook FBX to gLTF](https://github.com/facebookincubator/FBX2glTF)  coverter. Development of back-end conversion API based on those projects|  
| 5. | Conversion API developmen| Development of the library that enables converting of the 3d assets via, stores compression data on IPFS and provides links to compressed results. |  


### Milestone 2 WebGL viewer on Mintbase

- **Estimated Duration:** 3 weeks
- **FTE:**  4
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 License |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a deveopers can run their own marketplace with customly converted assets and setup the WebGL viewer |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 1. | WebGL viewer module | Development of module that enables viewing of the compressed 3d models in iframe |  
| 2. | Demo marketplace prototype | Development of the simple demo marketplace with support of interactive 3d assets based on Mintbase JS and Mintbase GraphQL Indexer API.|  
| 3. | Demo models preparation |  Preparation and insert of at least 20 art-based 3d models with different parameters, including at least 4 models with VFX and animation. Included assets (origibal non-cinverte parameters): STL (more than 400k poligons), FBX (more than 400k poligons), OBJ models. Those will be available on demo marketplace to examine. |  

### Milestone 3 V-Art Converter SDK

- **Estimated Duration:** 3 weeks
- **FTE:**  5
- **Costs:** 18,000 USD


| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 License |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a deveopers can run their own marketplace with customly converted assets and setup the WebGL viewer |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** and tutorial that explains how to quickly setup a marketplace connected to Minbase contract and insert convertation directly into the NFT on the minting phase. Language of the tutorial is English. Also demo project will be launched live on testnet in order to provide an understanding of the project to audience.
| 1. | V-Art Converter SDK development | Development of module the JS SDK that contains all needed functionality such as: minting, conversion obtainig and verifying conversion results and additional mintbase features of assets tranfering, selling, etc. |  
| 2. | Minting user flow development| Implementation of V-Art conversion SDK that enables possibilities of multi compression/conversion.|  
| 3. | WebGL marketplace space| Implementation of V-Art conversion SDK into demo project of NFT web gallery with pre-converted assets|  
| 4. | Announcements | Announcement about project completion on our channels, presetnation of the soltuion to our partners and community of the mintbase, publishing of the step-by-step tutorial abouy advantages of the system.|  


## Future Plans

V-Art will provide this library to be implemented in several marketplaces including those which are developed or to be developed via Mintbase. After the grant is completed, we will also onboard projects and web2 businesses, connecting the  V-Art Protocol (admin panel, API, automated certification and many more features), to use the NFT Converter solution and join the Minbase community. Education and ongoing legal support to the community, integrating the NFT Converter and/or V-Art Protocol, are as always our priority.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** 

From Mintbase team, referred by NEAR UA.

**Additional information** 

V-Art is a blockchain platform that helps to license and monetize digital content and provides virtual and augmented reality experiences to creative, tech, and luxury brands. 

We developed the V-Art Protocol, the most simple, safe, and flexible way for any brand to earn new revenue from licensing creative digital assets. Companies and projects can connect the V-Art Protocol to their website / platform, which will allow their users to have access to primary and secondary non-fungible tokens (NFT) markets and licensing tokens (developed by V-Art).  Connection includes providing an access to the semi-open access to the SDK, admin panel, API and user guide. Full turnkey integration may include additional customization, agreed in the technical specifications. 
 
As additional utility, virtual and augmented reality experiences and custom metaverses, AR and VR apps are provided by our V-Art Immersion team, as we have a rich background and an IP portfolio, which includes our own technologies for virtual spaces, AR and VR.

Our founders are all serial Ukrainian tech entrepreneurs with deep expertise in IP law and immersive technologies, successfully delivering over 100 projects across art, fashion and gaming sectors. V-Art is an early-stage startup, backed by Geek Ventures, SID Venture Partners, TNX Ventures and Adrian Slywotzky. Starting from digital art and expanding to all creative industries, our user base reached 44,5 K in total, our IP portfolio includes own virtual exhibition tech, AR and VR applications, as well as a unique tech model for blockchain-powered licensing. 

V-Art has 22 partnerships in creative industries, including dslcollection, Creative Business network, Vitrine gallery, AiKa, ArtPool, Naked Inc., etc. 6500 connections to profile specialists, including top-tier digital artists, fashion, luxury, game companies. 5 strategic partnerships in the blockchain sphere, including NEAR, Sigma Software, Hacken, IdeaSoft, Datrics.

We have not received any blockchain grants, however got support for our art-related projects by British Council, Ukrainian Cultural Foundation and Creative Business Network. Any other teams contributed financially to the project.

