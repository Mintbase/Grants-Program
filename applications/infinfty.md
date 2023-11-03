# Mintbase Grant Proposal

- **Project Name:** Infinfty
- **Team Name:** Enterprise Blockchain Labs LLC
- **Payment Address:** f7282dd09fb96525e6633f70ae00bc371a715d148896e6208318dedcc4aaa910
- **Level:** 2

## Project Overview :page_facing_up:

### Overview

**Tag Line:**
Infinfty is an enterprise platform strengthen connections between brands and customers through digital-physical ownership.

**Brief Description:**
Expand the NEAR protocol ecosystem by adding it to our suite of blockchains for our customers. Brands will be able to select the NEAR protocol as their preferred chain to deploy smart contracts, manage NFTs, and pair them with our NFC chips. To expand NEAR's presence, we'll create a Shopify plugin where users will be able to directly link NEAR NFTs with NFC chips via the Shopify app.

**NEAR and Mintbase Relation:**
Our platform will use the @mintbase-js/sdk library to deploy smart contacts and manage NFTs on the NEAR protocol.

**Team Interest:**
We believe the future of web3 is in Real World Assets. Our platform is one piece of the broader RWA ecosystem allowing brands to pair their physical goods with NFTs. We believe that digital-physical experiences will generate new value for customers and present them with tangible experiences in a growing digital world.

### Project Details

**Overview:** 
The goal of this project is to expand the presence of the NEAR protocol into the digital-physical space. We will add the NEAR protocol to our list of EVM blockchains to enable users with a fast and secure blockchain to mint their NFTs. At the completion of this project, users will be able to mint NFTs on the NEAR protocol using the Mintbase SDK and pair their NFTs with either ARX or NTAG 424 tags. By developing a Shopify plugin, we can expand the presence and legitimacy of the NEAR protocol, and our application, to the million-plus users on the Shopify platform by providing them with the same NFT to NFC tag pairing capabilities.

**Technology Stack:** 
Bubble.io front-end and back-end. Python and Node.js functions interacting with the Polygon blockchain. React.js app to pair NFTs with Arx NFC chips.

**Launched App:** 
We've already launched Infinfty, and it can be accessed [here](https://app.toinfinfty.com/). Feel free to reach out for credentials. 


### Ecosystem Fit

**Ecosystem Fit:**
Our project fits within the Mintbase utility ecosystem because we’re leveraging the codebase to create NFTs for digital-physical experiences. When brands pair NFTs with physical goods they unlock new use cases like digital certificates of authenticity, product-level CRM, loyal programs, and customer insights to strengthen the connection to their customers.

**Target Audience:**
Our target audience is any brand that’s creating a physical product. However, we see immediate applicability in the apparel, art, automotive, collectibles, footwear, furniture, games, luxury goods, and toy industries.

**Needs Met:**
Sixty-four percent of customers want closer connections with their brands and fifty-seven percent of them are willing to choose a brand over a competitor if they have a close connection to them. With digital-physical products, brands can strengthen their engagement with their customers in a new way. Brands can leverage the data provided by their customers’ wallets to create products or experiences that align better with their interests. NFTs will become the preferred method for market research as brands will make decisions based on their customers’ profiles and not sample populations.

**Similar Projects:**
While there are no projects like ours on the NEAR protocol, there are several competitors in the space like IYK, Legitimate, and Arianee. We differ from our competitors in the industries we’re able to target, the NFC chips we’re able to offer, our focus on full product lifecycle traceability, and multi-chain deployment (with support from this grant!). We originally built our platform on the Polygon network, but we’re interested in adding NEAR as this will set us apart from our competitors and leverage NEAR’s speed and security for enterprise customers.

## Team :busts_in_silhouette:

### Team members

- Name of team leader: Brett Blalock
- Names of team members: Sam White, Andrew Hart

### Contact

- **Contact Name:** Brett Blalock
- **Contact Email:** brett@toinfinfty.com
- **Website:** https://www.toinfinfty.com

### Legal Structure

- **Registered Address:** 6735 Crestwood Peninsula, Flowery Branch, GA 30542
- **Registered Legal Entity:** Enterprise Blockchain Labs LLC

### Team's experience

Brett Blalock - Creator of the Infinfty platform. Developed the Python and Node.js functions creating the smart contracts and NFTs using the Thirdweb SDK on the Polygon Network. The Thirdweb SDK has a comparable Typescript library enabling quick smart contract deployment and NFT management.

Andrew Hart - At Once Upon (Human-readable Ethereum block explorer), he built the back-end technology enabling the company to index the Ethereum blockchain. 

### Team Code Repos

- https://github.com/toinfinfty
- https://github.com/toinfinfty/functions

**Team GitHub Accounts**

- https://github.com/brettbl
- https://github.com/andrewfhart
- https://github.com/samwhiteiv

### Team LinkedIn Profiles

- https://www.linkedin.com/in/brettblalock/
- https://www.linkedin.com/in/andrewfhart/
- https://www.linkedin.com/in/samwhiteiv/

## Development Status :open_book:

**Existing project:** We already developed a working platform with paid customers. Here's a [product demo](https://youtu.be/avmiv0NXNcI) of how users can mint and pair NFTs to physical products. You can access the app [here](https://app.toinfinfty.com). Please reach out an we'll provide you with credentials.

**Project Repo:** https://github.com/toinfinfty

## Development Roadmap :nut_and_bolt:

### Milestone 1 — Implement Mintbase Modules

- **Estimated duration:** 26 Hours
- **FTE:**  1
- **Costs:** 5,200 USD

| Number | Deliverable | Specification |
| -----: | :---------- | :------------ |
| 0a. | License | GPLv3 |
| 0b. | Documentation | Inline documentation of the code that explains how users can leverage the Mintbase SDK to create smart contracts and manage NFTs. |
| 0c. | Testing Guide | Instructions on how to test smart contract deployment and NFT management using the Mintbase SDK. |
| 0d. | Docker | Dockerfile(s) used for smart contract deployment and NFT management. |
| 0e. | Article | Article detailing the smart contract deployment and NFT management within the Infinfty platform. |
| 1. | Mintbase module: Create Wallet | Create a NEAR wallet for users using the near-api-js library. Functions include nearConnection, createAccount |  
| 2. | Mintbase module: Connect Wallet | Connect a user's NEAR wallet for NEAR protocol interactions using the mintbase-js/auth and mintbase-js/react libraries. Functions include KeyPair, InMemoryKeyStore, KeyStore, WalletContextProvider |  
| 3. | Mintbase module: Deploy Smart Contract | Deploy a smart contract on the NEAR protocol using the mintbase-js/sdk library. Functions include DeployContract Args, deployContract, AddMinterArgs, and addMinter |  
| 4. | Mintbase module: NFT Management | Manage NFTs on the NEAR protocol using the mintbase-js/sdk library. Functions include execute;  mint, burn, list, delist, buy, transfer functions; and related "arg" functions |  


### Milestone 2 — Enable NEAR NFT & Arx Chip Pairing

- **Estimated duration:** 15 Hours
- **FTE:**  1
- **Costs:** 3,000 USD

| Number | Deliverable | Specification |
| -----: | :---------- | :------------ |
| 0a. | License | GPLv3 |
| 0b. | Documentation | Inline documentation of the code that explains how users can leverage the libhalo library to link the ARX NFC tags with NEAR NFTs. |
| 0c. | Testing Guide | Instructions on how to test the link between the ARX NFC tag and the NEAR NFTs. |
| 0d. | Docker | Dockerfile(s) used for ARX NFC tag and NEAR NFT pairing. |
| 0e. | Article | Article detailing how the Infinfty platform enables digital-physical experiences by pairing ARX tags with NEAR NFTs. |
| 0f. | Code Base | We will fork the Halo verify app to speed up development: https://github.com/kong-org/halo-verify-web/tree/master |
| 1. | Arx module: Connect Wallet | Connect the user's NEAR wallet. Functions include deviceStore. |
| 2. | Arx module: Scan Chip | Scan the Arx chip to retrieve the chip's public key. Functions include deviceStore. |  
| 3. | Arx module: Provide NFT Details | Pass the NFT details to the chip. Functions include registerStore. |  
| 4. | Arx module: Sign Block Hash | Sign a recent block hash from the NEAR protocol using the chip. |  
| 5. | Arx module: Generate Signature | From the user's wallet, generate a signature of several parameters: including the pre-calculated IPFS hash of the media along with the name, description, device_id, the signature generated by the device and block hash used to generate the signature and the address of the user's wallet. | 
| 6. | Arx module: Post Data | POST this data to the KONG bridge server which will add the media and tag it will the information from the previous step on Arweave. |


### Milestone 3 — Shopify App

- **Estimated duration:** 29 Hours
- **FTE:**  1
- **Costs:** 5,800 USD

| Number | Deliverable | Specification |
| -----: | :---------- | :------------ |
| 0a. | License | GPLv3 |
| 0b. | Documentation | Inline documentation of the code that explains how the Shopify API creates smart contracts and manages NFTs using the Mintbase SDK. |
| 0c. | Testing Guide | Instructions on how to test the creation of smart contracts and management from Shopify. |
| 0d. | Docker | Dockerfile(s) used to create smart contracts and manage NFTs from Shopify. Used to verify a customer's account. |
| 0e. | Article | Article detailing how the Infinfty plugin enables Shopufy users to create digital-physical experiences. |
| 1. | Shopify module: Create Webhook Listener | Create a Shopify event listener using an Amazon EventBridge. |
| 2. | Shopify module: Products | Create a new product in the Infinfty platform and a new NEAR smart contract when a product is created in Shopify. |  
| 3. | Shopify module: Inventory | Create or remove inventory in the Infinfty platform and create or burn NEAR NFTs when inventory is created or removed in Shopify. | 
| 4. | Shopify module: UI | Create a metaobject boolean field indicating the whether the Shopify event listener should create a smart contract and NFTs for the product. | 
| 5. | Shopify module: Customer Verification | Create a Shopify GraphQL request that checks the order ID and email address to verify the customer minting an NFT purchased the product. | 

### Milestone 4 — Enable NEAR NFT & NTAG 424 DNA Chip Pairing

- **Estimated duration:** 7
- **FTE:**  1
- **Costs:** 6,000 USD

| Number | Deliverable | Specification |
| -----: | :---------- | :------------ |
| 0a. | License | GPLv3 |
| 0b. | Documentation | Inline documentation of the code that explains how the NTAG 424 connects to a NEAR NFT. |
| 0c. | Testing Guide | Instructions on how to test the link between the NTAG 424 and a NEAR NFT. |
| 0d. | Docker | Dockerfile(s) used to for NTAG 424 and NEAR NFT pairing. |
| 0e. | Article | Article detailing how the Infinfty platform enables digital-physical experiences by pairing NTAG 424 tags with NEAR NFTs. |
| 0f. | Product | We will leverage an exsiting software toolkit created by an NFC developer to speed time to market. https://nfcdeveloper.com/ |
| 1. | Server Set Up | Set up a dedicated server to manage chip initialization and validation. |
| 2. | Chip Initialization | Leverage the NFC software toolkit to link the chip id to the NEAR NFT in the Infinfty database. |  
| 3. | Chip Validation | Leverage the NFC software toolkit to verify legitimate interactions with the chip. | 

...
## Future Plans

In the short term, the addition of the NTAG 424 tag encoding and Shopify integration will open our platform up to more use cases and a broader audience. With the NTAG 424 tags, we’ll be able to create tailored and secure digital-physical experiences for industries like apparel, automotive, art, collectibles, furniture, gaming, music, and toys. These markets combined together are worth $3.75 trillion globally and $871 billion in the USA. We believe there will be substantial interest in the next several years in digital-physical experiences and capturing just 1% of these markets would represent a substantial opportunity to grow the size of the NEAR ecosystem. We plan to support the projects Chris Ghent is leading with Barrett Jackson, Brooklyn Coachworks, Ferrari, and more to create unique digital-physical experiences with NFT and NFC technology.

In the long term, we'd like to expand our offerings to support the full traceability of products throughout the supply chain. This will enable brands to follow their products from cradle to grave, unleashing new insights into the lifespan and useability of their products. We see the NEAR protocol as an integral part of this implementation as blockchains will act as the central source of truth for disparate systems along a product’s lifecycle. We see this use case playing out in multiple industries where provenance and prior history are important. For example, we could see this technology replace incumbents like Carfax in the automotive industry as the data captured will be richer than what’s currently available.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?**
Through HUMANS, a growing collective of web3 founders, and NEAR Foundation’s former Head of Brand Partnerships, Chris Ghent.

**Closing Thoughts:**
Our platform already has a couple of users with several more starting early next year. With the support of the Near Foundation, we believe we can offer our customers a secure and quick blockchain to mint their NFTs. We really want to develop a platform that can support enterprise clients which is why we originally started on Polygon. We think the speed and security of NEAR’s blockchain will enable us to attract more enterprise clients to our platform which is why we’re excited about this opportunity.
