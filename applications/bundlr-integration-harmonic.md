# Mintbase Grant Proposal

- **Project Name:** Harmonic Bundlr Integration
- **Team Name:** Harmonic Technology Guild
- **Payment Address:** harmonic-guild-v1.sputnik-dao.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### Overview
Pay for your Arweave Storage in NEAR. No upload size limits for your NFTs. Upload big videos, audio, anything. No upper 15MB limit.

#### Problem
Mintbase users, Mintbase development partners, and Harmonic and it's clients are unable to upload files larger than 15MBs for minting to NFTs under Mintbase's existing systems. These users are limited to a certain set of file types. These users are also wary of file upload as a point of centralization. Users also do not want to have to own Arweave token to conduct file uploads.
#### Solution
To resolve these challenges, Harmonic will build an Arweave Bundlr Network integration to Near employing Mintbase contracts.
#### Why we do

Harmonic is the perfect partner for this development. We have intimate familiarity with Mintbase SDK. We have extensive experience employing Arweave. Our medium term business model requires this infrastructure to serve our music business clients' needs. We have the manpower, vision, organization, and drive for this project. 

### Project Details

Bundlr is an Layer 2 built on top of Arweave. See: https://bundlr.network/.

We will build a system to enable the upload of users' NFT metadata to Arweave using Bundlr Network with pay in NEAR fungible token. This will allow upload of data without any centralization point either in payer or API. Users will be able to without file size or file type restrictions. This will expand the posibilities for types of NFTs that can be minted with Mintbase contracts.  

### Ecosystem Fit
#### Where and how does your project fit into the ecosystem?
Harmonic is deeply integrated into the Near and Mintbase ecosystems. Any sophisticated Mintbase user or Mintbase development partner will be able to employ this technology. This will further the existing partnership between Harmonic and Mintbase. We already build marketplaces on top of Mintbase infrastructure. Supporting Mintbase NFT projects is a core component Harmonic's goal of being a content distributor and content infrastructure development group.

#### Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
Mintbase users, Mintbase development partners, Harmonic's network, and our clients

#### What need(s) does your project meet?
Users do not want to be limited by file types and size. Users want the NFT content uploading infrastructure to be decentralized.

#### Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
We have heard other projects such as MarmaJ be interested in these capabilities. We are unaware of any other projects that are content focused in a way where they might already created this infrastructure.

#### If so, how is your project different?
While most NFT projects focus on NFTs as an asset, we focus on them as a mechanism for delivering audio and video content to users. We apply this tack to the music business submarket as a proving ground. In this submarket we intend to prove there is a need for financial transparency and platform decentralization and democracy as enabled by web3. 

#### If not, are there similar projects in related ecosystems?
YouTube, Instagram, TicToc, Spotify... we're going after the big boys. As far as technical infrastructure, there are people in the Filecoin ecosystem who might be doing similar things. There are other audio and video infrastructure delivery applications but they are usually not decentralized. An interesting analogue is Livepeer (Livepeer.org) but we see them as more of a collaborator than a competitor. 

## Team :busts_in_silhouette:

### Team members

- Quinton Armitage (Admin Leader)
- Jaswinder Singh (Tech Leader)
- Muhammad Zangina Maje (Dev)
- Calvin Torra (Dev)
- Babagana Abba (Dev)
- Abdullahi Mohammed (Dev)

### Contact

- **Contact Name:** Quinton Armitage

- **Contact Email:** quinn@harmonicguild.io

- **Contact Twitter:** @qarmitage

  

- **Contact Name:** Jaswinder Singh

- **Contact Email:** jaswinder@harmonicguild.io

- **Contact Twitter:** @jassification

  

- **Website:** https://harmonicguild.io

  

### Legal Structure

- **Registered Address:** Tennessee, United States
- **Registered Legal Entity:** Harmonic Guild LLC 
- **Registered Jurisdiction:** Delaware, United States

### Team's experience
**Quinton Armitage**:

- Master of Public Policy, University of Chicago
- Former, Engineer at ERM 
- Former, Risk Analyst at Praedicat

**Jaswinder Singh**

- Product Manager at zblocks
- Former, Analyst at Barclays


### Team Code Repos
https://github.com/Harmonic-Guild

### Team LinkedIn Profiles (if available)

- [Quinton Armitage](https://www.linkedin.com/in/quinton-armitage-612a597b/)
- [Jaswinder Singh](https://www.linkedin.com/in/jaswinder-singh-95ab8412b/) 
- [Calvin Torra](https://www.linkedin.com/in/calvinturn/)
- [Babagana Abba](https://www.linkedin.com/in/babagana-abba-4957411b2/)
- [Abdullahi Mohammed](https://www.linkedin.com/in/abdullahi-sani-mohammed-61a4b8218/)

## Development Status :open_book:

Harmonic has created a set of useful tools and projects for our clients.

- Custom marketplaces on top of Mintbase infrustructure
- Airdrops of Mintbase compliant NFTs
- EnrollMint, a Membership NFT products DAO of DAOs to which we are the development steward

We aim to build on these successes to broaden our client base as well as expand our tools and projects. We want to ease onboarding, expand utility, and build a replicating network.

Status: Ideation phase completed. Technical details and architecture in Process. Checked out Bundlr documentation and examples with other L1s. Carved out below Architecture. Experimenting with Bundlr js library currently. 

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 4 months

- **Full-Time Equivalent (FTE):**  1.5 FTE

- **Total Costs:** 15,000 USD


  <img width="967" alt="Screenshot 2022-11-22 at 11 07 56 PM" src="https://user-images.githubusercontent.com/12672862/203488301-b982fa44-c7f2-4820-8e36-54f21b7bcbdb.png">



### Milestone 1 — Build the Bundlr Integration

- **Estimated duration:** 2 month

- **FTE:**  1.5

- **Costs:** 7,500 USD

  

- Building an NFT minting application using Mintbase contracts and integrating with Bundlr to upload assets. 

- Fund Bundlr transaction and NFT Minting in separate transaction.

  

| Number | Deliverable   | Specification                                                |
| -----: | ------------- | ------------------------------------------------------------ |
|    0a. | License       | Apache 2.0                                                   |
|    0b. | Documentation | We will provide **inline documentation** of the code.        |
|    0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. |
|    0d. | Demonstration | Harmonic will produce and post video documentation of how the Bundlr integration works. |

### Milestone 1 — Build and Deploy Bundlr User Interface

- **Estimated Duration:** 2 months
- **FTE:**  1.5
- **Costs:** 7,500 USD



- Convert internal application into an application usable by the public.
- Define a functioning UX for the system. Design and build a UI to suit. 
- Provide users with the option to prefund Bundlr. 
- Keep track of user states and histories.
 - Merging the two transaction(Funding bundlr + mint) into a single signed transaction (if possible, this is experimental), otherwise create user flows that require both to take place and handle edge cases. 

| Number | Deliverable   | Specification                                                |
| -----: | ------------- | ------------------------------------------------------------ |
|    0a. | License       | proprietary                                                  |
|    0b. | Documentation | We will provide both **inline documentation** of the code and a basic tutorial on how to deploy and run created module. |
|    0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
|    0d. | Demonstration | Harmonic will produce and post video documentation of how the Bundlr integration UI works. |
|      1 | Publish       | Harmonic will publish the code of this repository barring security concerns. |



## Future Plans

Harmonic has *tons* plans. We want to build our network of content creation and consumption... more on that later..

For the purposes of this deliverable our follow on goals will be to employ and propigate the use of this application:
- Onboard a feature length film from a studio
- Onboard a grammy winning artist
- 50 clients employing this tech for music videos
- Upload 1000 hours of video
- Get to 250 weekly active consumers
- Integrate the technology with one of our development patners 

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** 

From Twitter and being active in the NEAR Ecosystem as well as personal connections we have cultivated with the Mintbase Team since NearCon Alpha.
