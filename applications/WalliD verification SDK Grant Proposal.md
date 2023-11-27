# WalliD verification SDK Grant Proposal

- **Project Name:** WalliD verification SDK as Mintbase module
- **Team Name:** WalliD S.A.
- **Payment Address:** 7c5db8d0d80eec153e8ed78d2706224428ebdcab4c8103b8e676b697452c75d7
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### Overview

![dAPP use case mockup](https://drive.google.com/uc?id=1uIBR05V1uXzxzf-OTLpz0B9-CK98M_N-) 


WalliD is the aggregator toolkit of all ID protocols and networks, making it simple for WebApps to authenticate and verify users' digital IDs from multiple infrastructures through a single SDK connection.
With WalliD Verifier SDK, WebApps and dAPPs are able to customise different trust and access levels for their users, based on the amount or quality of the Digital IDs and other assets provided (eg. Citizen ID, Twitter account, NEAR wallet address or token held within) and ensure this proof-of-ownership  is based on wallet signatures.
The SDK v1 is currently under development, after the architecture and main features have been detailed in a POC implemented within DocuSign during 2022.

**The goal of the project desctibbed in this application is to:**
- Integrate WalliD Verifier SDK features with MintBase toolkit, making it available as an authentication and ID verifier module through their own toolkit

**With this project we intend to enrich the NEAR and MintBase ecosystems by providing a tool that nables NEAR dAPPS to:**
- Implement reputation and trust systems for their users
- Verify user's IDs  and NEAR assets through NEAR wallets
- Create different trust scores and access levels for their users based on ID and asset verifications.

### Project Details

This project will combine WalliD’s verification features with Mintbase’s toolkit for NFT marketplace builders.
With it, Mintbase will be able to provide a universal authentication and ID verification module that can be integrated in any NEAR dApp through their toolkit, adding up to potential scope of features to their community of developers.

**Below is the depiction of the architecture components and the agents’ interaction with them:**
<p align="center">
  <img src="https://drive.google.com/uc?id=1zbQm1zvPqwoeC9Gx08Eh7SvtpLghFjjD" style="height:800px";>
</p>


**During this project, we will be building:**

- Verification SDK config module features. See [mockups here](https://docs.google.com/presentation/d/1IPxpi3hZmU1jtTuG7j3T8mWPoA1FLxPh1CY-2SrWtgU/edit?usp=sharing)
- OAuth, / ID verification component (adapted from WalliD wallet verification component already built). See [mockups here](https://docs.google.com/presentation/d/1GxYymkknA61B-BsJ1qzAMb2TLMvQK9g5WBkQB5B6fME/edit?usp=sharing)
- Frontend libs for dAPP developers
- NPM libs for Mintbase admins to install the module 

See detailed milestones and roadmap in the milestones  in the appropriate chapter of this application

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- **Where and how does your project fit into the ecosystem?:** As an ID verification module available to all dAPP builders through Mintbase toolkit. 
- **Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?** dAAPs building reputation systems, trust scores or in need to comply with ID verification and data processing regulations.
**- What need(s) does your project meet?:** We try to solve the problem of trust and accountability while allowing users to keep their self-sovereignty
**- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?:** Not that we're aware of
 ** - If not, are there similar projects in related ecosystems?** [Project Galaxy](https://galxe.com/galxeid)

## Team :busts_in_silhouette:

### Team members

- Filipe Veiga (CEO)
- Vitor Viana (CTO)
- Beatriz Pereira (UX/UI)
- Guilherme Arsénio (Full stack)

### Contact

- **Contact Name:** Filipe da Cruz Esteves Casal da Veiga
- **Contact Email:** filipe.veiga@wallid.io
- **Website:** [WalliD](https://wallid.io/)

### Legal Structure

- **Registered Address:** Rua Fernanda Seno, nr 6, 7005-485, Évora, Portugal
- **Registered Legal Entity:** WalliD S.A

### Team's experience

Filipe and Vitor both met while working the development of Digital infrastructures in Portugal and have a background in product development of PKI based infrastrcutures, middleware and cryptography. Guilherme and Beatriz joined the team in 2019 when we first started building WalliD and have been the builders of all the teck stack already developed by WalliD.


### Team Code Repos

- [WalliD](https://github.com/walliDprotocol)
- [WalliD Wallet](https://github.com/walliDprotocol/WalliD-Wallet)
- [WalliD POC with DocuSign ](https://github.com/walliDprotocol/NFT-Proof-of-Signature)

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- [Filipe Veiga](https://github.com/FCVeiga)
- [Vitor Viana ](https://github.com/masterviana)
- [Guilherme Arsénio](https://github.com/GuilhermeArsenio)

### Team LinkedIn Profiles (if available)

- [Filipe Veiga ](https://www.linkedin.com/in/fiveiga/)
- [Vitor Viana ](https://www.linkedin.com/in/vviana/)
- [Beatriz Pereira](https://www.linkedin.com/in/beatrizpereira215/)
- [Guilherme Arsénio ](https://www.linkedin.com/in/guilherme-ars%C3%A9nio-4b6a1a148/)

## Development Status :open_book:

Apart from conceptualizing the architecture of the SDK and building the mockups, we have implemented some of the integrations with Oauths (Twitter, Discord, Reddit, Githuub) and digital IDs (Portuguese Digital ID, LUKSO network) in WalliD wallet. 

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 10 weeks
- **Full-Time Equivalent (FTE):**  4
- **Total Costs:** 20,000 USD


### Milestone 1 — Verification connector

- **Estimated duration:** 6 weeks
- **FTE:**  4
- **Costs:** 15,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 1. | Connector API service | Integrates with OAuth protocols from web2 providers such as twitter, discord, github google, facebook, reddit and github. Implements NEAR Wallet authentication, NEAR tokens verification, NEAR social and connects to digital ID providers connected to WalliD ecosystem such as Chave Móvel Digital |
| 2. | Documentation | Tecnical documentation, setup samples,  |
| 3. | Tests | Unit Tests |


### Milestone 2 — SDK NPM lib

- **Estimated Duration:** 4 weeks
- **FTE:**  4
- **Costs:** 5,000 USD


| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 1. | NPM Lib part 1 | Javascript section of the lib to implement operations supported by the verification connector. Allows users to authorize using NEAR wallets, OAuth protocols and digital ID protocols connected to WalliD's ecosystem. Supports admin operations for setup module. |
| 2. | NPM Lib part 2 | Vuejs, React section of the lib to implement the already configured Verification connector on client dApp frontends  |
| 3. | Testing guide | coverage tests, unit tests | 
| 4. | Documentation | Tecnical documentation, coverage test, pratical samples, build instructions |


...
## Future Plans

We intend to pilot test our Verifier aggregator value proposition amongst Web3 dAPPs with this grant and expand the reach of the SDK within the NEAR and other ecosystems by building a SaaS business model on top if. The integration and business development that will follow from the POC will the be the determing data points to espablish the way we will scale the business


## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase team
