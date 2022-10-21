# Mintbase Grant Proposal


>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** Artivist dApp
- **Team Name:**  Artivist DAO
- **Payment Address:** artivist.sputnik-dao.near
- **[Level](../README.md#level_slider-levels):** 1

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

If this application is in response to an RFP, please indicate this on the first line of this section.

If this is an application for a follow-up grant (the continuation of an earlier, successful Mintbase grant), please provide name and/or pull request of said grant on the first line of this section.

### Overview

Please provide the following:

- If the name of your project is not descriptive, a tag line (one sentence summary).  
Artivist dApp will connect artists, activists, and NGOs in fundraising campaigns through the sale of NFTs offered on Mintbase.
- A brief description of your project.  
The dApp will allow NGOs to connect to artists based on the causes they support. They will be able to "match", similar to Tinder. When the NGO finds an artist, they get in touch to start a fundraising campaign. The artist will provide an NFT minted on Mintbase. Part of the sale will go to the NGO, part will go to the artist, and part will go to Artivist. Think of it as a mission-driven combination of Tinder, Kickstarter, and Mintbase.  
- An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.  
We will use Mintbase as our marketplace. 
- An indication of why your team is interested in creating this project.  
The idea behind Artivist comes from Chamada Artivista, an initiative started in 2020 by Carol Bampa. Chamada Artivista was a way to provide alternative funding options to mission-driven organizations. Carol invited artists to create artworks; the chosen pieces were sold online. The initiative was more successful than she initially believed. With Covid, however, the project came to a halt. Carol learned about the possibilities of NFTs and saw their potential to amplify fundraising campaigns. She envisioned an app that could facilitate the connection between NGOs and artists – a mix of Tinder and Kickstarter. She met Mariana Westphalen, who came from the Arts Sector and became Artivist co-founder, on NEAR Discord channel. They have been building Artivist since March 2022. In September they met Global DV Consulting, a team of developers that has been working on NEAR projects for fifteen years.


### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- Mockups/designs of any UI components  
Please take a look at our [Wireframes](https://www.figma.com/file/08AC3RjZSFVie47ddZ8afN/Artivist?node-id=411%3A887)  
- Data models / API specifications of the core functionality  
Mintbase. JS and function calls from the deployed store of mintbase.  
**Smart contract functions**  
nft_approve  
nft_transfer  
grant_minter  
nft_transfer_payout  
check_is_minter  
list_minters  

**Mintbase.js functions**  
burn  
connect  
list  
mint  
mintMore  
listMore  
transfer  
burn  

- An overview of the technology stack to be used  
**Vue JS:**  
A JavaScript framework for building user interfaces. It builds on top of standard HTML, CSS and JavaScript, and provides a declarative and component-based programming model that helps to efficiently develop user interfaces, be it simple or complex.  
**Nuxt JS:**  
Nuxt. js is a framework for creating Vue.js applications. Its goal is to help Vue developers take advantage of top-notch technologies, fast, easy and in an organized way.  
**Node JS:**  
Node.js is an open-source, cross-platform, back-end JavaScript runtime environment that runs on a JavaScript Engine and executes JavaScript code outside a web browser, which was designed to build scalable network applications.  
**Mintbase JS:**  
General purpose Mintbase API for interacting with NEAR, Arweave and other supported blockchains and decentralized filestorage systems.  
**Graphql:**  
GraphQL is a query language for APIs and a runtime for fulfilling those queries with your existing data. GraphQL provides a complete and understandable description of the data in your API, gives clients the power to ask for exactly what they need and nothing more, makes it easier to evolve APIs over time, and enables powerful developer tools.  
**Figma:**  
Figma creates a new level of collaboration that never really existed before. The team is able to work together and ship products faster." Nina Samarguliani. Design Manager at Coinbase. Build. Ship better outcomes. Deliver better products and make an impact with a platform that connects the dots across design, product, and development.  

- Documentation of core components, protocols, architecture, etc. to be deployed

**Layered Pattern**  
Presentation layer (UI) developed in Figma
Application layer will be the service, mintbase.js library, will be used to, mint, list, approve, sell nft and transfer nft.
Business logic layer: Domain service located in Digital Ocean as service  
![layered pattern](https://user-images.githubusercontent.com/97693233/196932305-ee63dea7-1cf7-40fc-ab32-2434e8fde7c0.png)  

**Client-server pattern**  
This pattern consists of two parties; a blockchain and multiple clients. The server component will provide services to multiple client components. Clients request services from the blockchain and the server provides relevant services to those clients. Furthermore, the server continues to listen to client requests.

**Interpreter pattern**  
This pattern is used for designing a component that interprets programs written in a dedicated language. It mainly specifies how to evaluate lines of programs, known as sentences or expressions written in a particular language. The basic idea is to have a class for each symbol of the language.  
![interpreter pattern](https://user-images.githubusercontent.com/97693233/196932716-04075412-3a57-4284-bdae-d4486731d6ba.png)  


- PoC/MVP or other relevant prior work or research on the topic  
Please refer to the report of the research with artists [Research with artists](https://drive.google.com/file/d/1au6lpNh7lob14ehDxszaHViX3m651JSc/view). We have also developed a hotsite wireframe (https://www.figma.com/file/k1UV0seZVpclPoE9V7vFJJ/Artivist-web-dapp?node-id=0%3A1) and had three workshops with **Brands with Empathy** a consulting company that helped us trace our business principles. This work was funded by the Creatives DAO, and the report sent to them tells more details about the process (https://gov.near.org/t/report-september-monthly-report-artivist-dao/30047). See our business principles here (https://drive.google.com/file/d/1JTaQYCRriqIIX3DCKvDwqcZFDviQ9m0x/view) 
Also, please take a look at the work we've been doing with the Creatives DAO since March [NEAR forum](https://gov.near.org/tag/artivist-dao).  
- What your project is _not_ or will _not_ provide or implement  
Artivist dApp will not be a marketplace. Our goal is to connect NGOs and artists.

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?  
Although there are places where artists and collectors connect to trade art (Mintbase, Paras, etc) in the Near ecosystem, the presence of NGOs is still marginal. 
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?  
The main target audience for Artivist dApp will be the NGOs. The dApp will also function as a portfolio of artists in favor of social causes, and a place where consciencious NFT collectors will find interesting artworks and be able to contribute with NGOs. Moreover, companies concerned with ESG goals can become involved and support Artivist campaigns. 
- What need(s) does your project meet?  
According to NGOFacts data, there are about 10 million NGOs in the world. And people increasingly believe in these projects: the number of individuals who donated money to these institutions increased from 1.2 billion in 2011 to 1.4 billion in 2014, and 80% of citizens agree that NGOs contribute to positive social change around the world. NGOs often don't have access to fair funding because of government restrictions and bureaucracy. Artivist dApp will be a tool for NGOs to obtain resources and carry out their governance in the Near ecosystem. Fostering this transition to web3 through the Near protocol implies a potential turnover of Near tokens in favor of social-environmental causes, strengthening the purpose of the Protocol to be a sustainable ecosystem, with neutral environmental impacts and positive social impacts. Artivist will spread the word about the Near ecosystem through campaigns, therefore attracting a conscious public that can discover all the possibilities of the Near blockchain through the community. Connecting with Mintbase will be an opportunity for growth for Artivist and Mintbase. 
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?  
Yes, BeeTogether is platform that promotes fundraising campaigns. 
- If so, how is your project different?  
It will be different because the dApp will automate the process of connecting NGOs to artists (https://bee-together.org/about/). 
- If not, are there similar projects in related ecosystems?  
DoinGud functions in a similar way to BeeTogether, and also doesn't connect automatically NGOs to artists (https://doingud.com/).

## Team :busts_in_silhouette:

### Team members

- Name of team leader  
Carol Bampa and Mariana Westphalen 

- Names of team members  
Andres Dominguez, Technical Leader

Linda Rosario, Frontend Engineer

Juan Ochando, Backend Engineer

Maria Arevalo, Project Manager

### Contact

- **Contact Name:** Carol Bampa, Mariana Westphalen
- **Contact Email:** artivist.dapp@gmail.com
- **Website:** [ArtivistDAO] (https://artivistdao.org/)   [DV Consultores](https://github.com/dvconsultores/)

### Legal Structure

- **Registered Address:** Address of your registered legal entity, if available. Please keep it in a single line. (e.g. High Street 1, London LK1 234, UK)
- **Registered Legal Entity:** Name of your registered legal entity, if available. (e.g. Duo Ltd.)

### Team's experience

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful.

If anyone on your team has applied for a grant at the Mintbase previously, please list the name of the project and legal entity here.

**Carol Bampa - Project Manager and Council of Artivist DAO**
Cultural Manager and Cultural Projects Consultant. Worked at the Secretary of Culture of the State of São Paulo for four years. Produced three editions of the São Paulo Cultural Map Festival, two editions of the Caraguatatuba National Theater Festival, and the National Folklore Congress. Collaborated with the Gender and Ethnicity Chapters of the Pontos de Cultura Program in São Paulo State. Finalist of the first edition of the Festival of Ideas held by the Ruth Cardoso Cultural Center with a project for recyclable waste, CATA-LOGO. Participated in the foundation of the Fazenda Cachoeira Park Movement, which promotes social, educational, and cultural activities related to environmental preservation and the historical heritage of the city of Vinhedo. Idealized, produced, and managed Paisagem Festival, an event for the graphic arts and independent publications started in 2017. Creator and manager of the Respeita Project - Space for the Empowerment of Immigrant Women and Chamada Artivista, a fundraising initiative for social causes.

**Mariana Westphalen - Art Curator, Product Manager and Council of Artivist DAO**
A museum specialist and curator with a PhD in Art History and a Master’s in Technology with more than twenty years of experience in the arts and culture sector in Brazil, the US, England, and now Portugal. Mariana loves to meet artists to learn about their practices and understand what makes them tick. Connecting art institutions and galleries to diverse communities is one of her passions, and she is fascinated with the intersections of art and technology, be it photography, architecture, or NFTs. Artivist’s mission to bring together artists, activists, and independent socio-environmental organizations through the creative economy aligns with her interests and her professional background.

**DV Consultores**
DV Consultores is an Application Development team founded in 2004. With 15 certified resources in the Near Protocol, they have participated in programs such as Developer In Residence and generated products such as NEAR P2P DEX, Artemis, and NearBook. Besides, DV Consultores has worked on initiatives such as NFT Aggregator Evie and Econear, and is currently developing projects such as Mintick and Music Feast.


### Team Code Repos

- [Artivist](https://github.com/Artivist-Dao)
- [DV Consultores] (https://github.com/dvconsultores/) 
- [DV Consultores - NEAR-Dex] (https://github.com/NEAR-P2P/NearP2PBackend)
- [DV Consultores - NearP2P-NearPricesFiat](https://github.com/NEAR-P2P/NearP2P-NearPricesFiat)

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

### Team LinkedIn Profiles (if available)

- [Carol Bampa] https://www.linkedin.com/in/carolbampa/
- [Mariana Westphalen] https://www.linkedin.com/in/mariana-westphalen/
- [Andres Dominguez] https://linkedin.com/in/andrés-dominguez-91418819 
- [Linda Rosario] https://linkedin.com/in/linda-rosario-duran-5b9559127
- [Juan Ochando] https://linkedin.com/in/juan-josé-ochando-327927220
- [Maria Arevalo] https://linkedin.com/in/mearevalol


## Development Status :open_book:

If you've already started implementing your project or it is part of a larger repository, please provide a link and a description of the code here. In any case, please provide some documentation on the research and other work you have conducted before applying. This could be:

- links to improvement proposals or [RFPs](https://github.com/mintbase/Grants-Program/tree/master/rfp-proposal) (requests for proposal),
- academic publications relevant to the problem,  
Hartenthal, Mariana W. von, Eitelwein, Cíntia. "The usability of visual resources on cultural conservation: the Graphics Atlas as a case study." Revista Tecnologia e Sociedade, v. 18, n. 52, jul./set. 2022, pp. 247-268. (the name of the author is Mariana Westphalen von Hartenthal) https://periodicos.utfpr.edu.br/rts/article/view/13594
- links to your research diary, blog posts, articles, forum discussions or open GitHub issues,  
https://medium.com/@marianahartenthal/nft-artists-views-on-blockchain-based-marketplaces-5c6a98f3202e  
https://medium.com/@marianahartenthal/why-perpetual-royalties-for-nfts-b88bcc5dbd6d  
https://medium.com/@marianahartenthal/nfts-scarcity-and-the-art-market-d934ea6a377f  

- references to conversations you might have had related to this project with anyone from the Mintbase Foundation,
- previous interface iterations, such as mock-ups and wireframes.  
[Wireframes](https://www.figma.com/file/08AC3RjZSFVie47ddZ8afN/Artivist?node-id=411%3A887)

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

> :zap: If any of your deliverables is based on somebody else's work, make sure you work and publish _under the terms of the license_ of the respective project and that you **highlight this fact in your milestone documentation** and in the source code if applicable! **Teams that submit others' work without attributing it will be immediately terminated.**

### Overview

- **Total Estimated Duration:** 4 months
- **Full-Time Equivalent (FTE):**  1.5
- **Total Costs:** 10,000 USD
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

### Milestone 1 - Layout  
**Estimated Duration:** 1 month  
**FTE:** 1  
**Costs:** $ 2.500,00    
	Define all features  
	Build UI/UX  
	Web design with all functionalities  
	Validate with different browsers  
	Responsive  
**Tech stack:** Figma, JavaScript, Vue.js, Nuxt.js  
	
### Milestone 2 - Backend  
**Estimated Duration:** 1 month  
**FTE:** 1  
**Costs:** $ 2.500,00 
	Function definition  
		Connect NEAR wallet  
		Artist registration  
		Create artist portfolio  
		Create NGO registration  
		Create communication by message  
	Smart contract functions  
    nft_approve  
    nft_transfer  
    grant_minter  
    nft_transfer_payout  
    check_is_minter  
    list_minters   
	Mintbase.js functions  
    burn  
    connect  
    list  
    mint  
    mintMore  
    listMore   
    transfer  
    burn  
**Tech stack:** Node.js, GraphQL  

### Milestone 3 – Frontend   
**Estimated Duration:** 1 month  
**FTE:** 1  
**Costs:** $ 2.500,00  
	Add behavior  
	Form validation  
	Integrate with backend  
	Integrate with smart contract  
	Behavior tests in different browsers  
**Tech stack:** JavaScript, Vue.js, Nuxt.js, Node.js, GraphQL  

### Milestone 4 – QA Testnet  
**Estimated duration:** 1 month  
**FTE:** 1  
**Costs:** $ 2.500,00  
	Functional and comprehensive tests  
	Correct functionality approval  
	Migrate to Mainnet  
	
...
## Future Plans

Please include here

- how you intend to use, enhance, promote and support your project in the short term, and  
If our application to Mintbase is accepted, we will continue to seek for support from the Creatives DAO and other resources to enable our non-technical team to spend more time building and engaging our community. 
- the team's long-term plans and intentions in relation to it.  
Our medium-term plan:  
This project focuses on the **creation of an MVP**. With the completion of the MVP, we will undertake tests with potential users to address problems and propose solutions. With the MVP in hand, we will work towards the development of a **Beta version** of the dApp to be launched by mid-2023. We believe that the MVP will help us to get funding for the development of the Beta, which we calculate as having a total cost of around US$30.000,00. Besides the development of the dApp, we are committed to other aspects that will make Artivist possible as a product and as a service, namely the community engagement, which encompasses the creation of a solid social media campaing in different channels. We will continue to seek for other funding sources to develop the non-technical aspects of our business and have estimated a budget of US$ 20.000,00 for social media plans, management, design and other jobs.  
Our long-term plan:  
Our long-term plan is to become a flagship web3 platform for social and environmental organizations, artists, and activist consumers looking for autonomy and transparency. 

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website / Medium / Twitter / Element / Announcement by another team / personal recommendation / etc.  
Personal recommendation, Mintbase website. 

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as:

- Work you have already done.
- If there are any other teams who have already contributed (financially) to the project.
- Previous grants you may have applied for.  
We have applied, and received, funding from the Creatives DAO. We have also applied for the NEAR Foundation grant and will have a meeting with them next week. 
