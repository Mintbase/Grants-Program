# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** Skyline Digital
- **Team Name:** Skyline Digital AG
- **Payment Address:** skylinedigital.mintbase.near
- **[Level](../README.md#level_slider-levels):** 3

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

- Skyline Digital offers non-custodial banking transactional services (on/offramp + 3rd Party payments)
- We want to integrate the NEAR network into Skyline to allow transactions using your network
- We also want to integrate our API directly into the Mintbase wallet allowing users to on/offramp and pay anyone directly from their wallet using fiat currencies
- We want to easily help open access to traditional banking services to anyone on and off-chain via a non-custodial way

### Project Details

- Mockups/designs of any UI components
- API documentation - https://skyline-digitial.gitbook.io/skyline-digital-api 
- The technology stack is divided into two categories:
	- NEAR network Integration: JavaScript, NearJS.
	- Mintbase Wallet Integration: JavaScript, React, MintbaseJS, NearJS, Skyline API.
- Integrating NEAR in the Skyline platform will not require a specific deployment as it will live in Skyline's current frontend stack.
- For Mintbase Wallet Integration the Widget will be deployed as a static website (JS, CSS, HTML) and probably loaded.
- Relevant to point out in terms of research will be that we have already in place an ERC20 “withdrawal logic” to process payment, this will need to be reflected while using the NEAR network with the corresponding if exists ERC20.
- What your project is _not_ or will _not_ provide or implement
- This is a place for you to manage expectations and clarify any limitations that might not be obvious
  
### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?

	- _Our product will allow mintbase wallet holders to easily on/offramp and/or pay anyone anywhere in fiat currencies using their tokens._

- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?

	- _Our audiences tend to be Businesses and HNWI but we also have a retail side for individual users.
Our main focuses are around DAOs, Foundations and Crypto companies but we also cater to regular retail crypto investors or users.
Mainly we would like to open our products to Mintbase wallet holders._

- What need(s) does your project meet?
	- _Easy, quick and highly compliant on/offramping and 3rd party payments in any fiat currency worldwide (pay invoices, suppliers, contractors, taxes or anything else)_

- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - If so, how is your project different?
  - If not, are there similar projects in related ecosystems?

	- _Alchemy Pay and Trasak - but these are primarily onramping tools
We would offer both on/offramping and more importantly 3rd Party Payments allowing users to pay anyone for anything without having to first offramp into their bank account and then make a payment._


## Team :busts_in_silhouette:

### Team members

- Sebastiao Queiroz e Mello - CEO
- Matthieu Gueissaz - COO
- Filippo Minder Sotto Mayor Matoso - CPO
- Andre Fatia - CTO
- Joao Martins - Engineer
- Gonçalo Patrocínio - Engineer
- Bruno Martins - Designer

### Contact

- **Contact Name:** Filippo Sotto Mayor
- **Contact Email:** f@skylinedigital.xyz
- **Website:** https://www.skylinedigital.xyz/

### Legal Structure

- **Registered Address:** Baarerstrasse 82, 6300 Zug, Switzerland
- **Registered Legal Entity:** Skyline Digital AG

### Team's experience

**Sebastião Queiroz e Mello, Co-Founder and CEO of Skyline Digital**, has graduated from Instituto Superior Técnico in Lisbon and has been connected to entrepreneurial projects ever since, with multiple startups founded, such as The Code Venture, a software house, and Lisbon Tech Guide, softlanding startup in Portugal. As a creator of new products and businesses, Sebastião has a mind for finance and management. His engineering brain brings all the components together to create killer products.

**Matthieu Gueissaz, Co-Founder and COO of Skyline Digital**, has been working as a legal professional since 2013. As Ambassador of Business Angels Switzerland, Co-Founder of a real estate investment advisory platform and a Tokenized Venture Fund, Matthieu has gained a thorough understanding of the startup ecosystem and venture investing, along with the challenges and opportunities of Blockchain. He has also worked with Lenz & Staehelin, University of Lucerne, HvB Investments, Schellenberg Wittmer Ltd, Space Metaverse, Blockchain Valley Ventures, and LynxDAO as a consultant and advisor for blockchain technologies. More recently, he founded Blockchain Lawyers Forum and, of course, Skyline Digital.

**Filippo Sotto Mayor, Chief Product Officer of Skyline Digital**, with over 8 years of experience in the FinTech industry. He has a proven track record of launching innovative and successful card and credit products that solve real customer problems and create value for the business. He has worked for Ebury, where he launched a new debit and credit card offering for the global trade finance provider, and also Tymit, where he led the development and launch of Europe's first ever installment-based credit card product, which won the PayTech 2022 Best Consumer Cards Initiative award.

**André Fatia, Chief Technology Officer of Skyline Digital**, has been building software for several different industries since 2012, with experience with many web paradigms from low code to serverless computing. Recently, he has been focusing on building in the Web3 space as a previous consultant for Consensys in the MetaMask mobile project he's now responsible for the technology in Skyline Digital.

### Team Code Repos

- https://github.com/Skyline-Digital-AG

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/Fatxx
- https://github.com/TelmoFerreira
- https://github.com/Gongas99
- Joao Martins - Engineer (missing)

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/squeirozmello
- https://www.linkedin.com/in/matthieu-gueissaz-4a50298b 
- https://www.linkedin.com/in/filipposottomayor 
- https://www.linkedin.com/in/andr%C3%A9-fatia-60b11058 
- Joao Martins - Engineer (missing)
- https://www.linkedin.com/in/gon%C3%A7alo-patroc%C3%ADnio-4a40661b4 
- https://www.linkedin.com/in/bruno-martins-29728b60 
- https://www.linkedin.com/in/telmoferreira-dev/
- https://www.linkedin.com/in/juan-cruz-mazzochi-18301a1a4 
- https://www.linkedin.com/in/cintiatcosta 

## Development Status :open_book:

At this point in time research was made by looking at [NEAR Docs](https://docs.near.org/), we intend to use:
- NEAR JS API or/and RCP API for wallet and account connections with the network
- Transactions and Non-Fungible Tokens interactions for handling payments

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 6 months
- **Full-Time Equivalent (FTE):**  2 developers + 1 designer + 1 PM/QA
- **Total Costs:** 50,000 USD

### Milestone 1 — Integrate with NEAR network

- **Estimated duration:** 3 FTE - dev + designer + (PM/QA)
- **FTE:**  2 Months
- **Costs:** 20,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the code and a basic tutorial that explains how a user can use the Skyline Platform to on/offramp AND to do 3rd party payments |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | Yes |
| 0e. | Article | We will create a blog + social media posts announcing this feature |
| 1. | NEAR chain integration | We will integrate the NEAR chain so that users can connect with NEAR wallets and complete transactions within the Skyline ecosystem. We will also adapt existing UX/UI to offer this |  
| 2. | Increase token acceptance | We will increase the tokens we accept on Skyline ecosystem to include main NEAR tokens such as nUSDC, nUSDT + USN |  

![aaa](https://github.com/Skyline-Digital-AG/Grants-Program/assets/575479/54538e33-ea4b-40a0-a8c9-79489ad42daa)

### Milestone 2 — Open Source Widget

- **Estimated Duration:** 3 month
- **FTE:**  3 FTE - frontend dev + designer + (PM/QA + Designer)
- **Costs:** 30,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both inline documentation of the code and documentation on how someone can use or integrate the Widget |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. |
| 0d. | Docker | Yes |
| 0e. | Article | We will create a blog + social media posts announcing this feature |
| 1. | Widget UX/UI design + development | We will design and create a widget to be used anywhere as a window into the Skyline ecosystem allowing customers to access our services directly from the Mintbase wallet. Services include: Onboarding, Login, 3rd party payments in fiat currencies |  
| 2. | Integrate widget into Mintbase wallet | We will create a Mintbase module that will load our widget to access Skyline services directly |  

Mockups:
![skylinePaymentsFlowWithSmartContract (1)](https://github.com/Skyline-Digital-AG/Grants-Program/assets/575479/e9b7cbfa-63c9-441f-980a-1aecd2ea78e9)
<img width="1281" alt="Screen Shot 2023-12-11 at 21 07 37" src="https://github.com/Skyline-Digital-AG/Grants-Program/assets/575479/bc84c71a-e216-49ee-865c-7c808ae2e4c0">


## Future Plans

Please include here

- how you intend to use, enhance, promote and support your project in the short term, and

_We plan to promote this via our social networks, blog posts and overall marketing strategies.
We will also send newsletters to our existing customer base and investors to show how we are helping the community with more access to TradeFi which is not easily accessible._

- the team's long-term plans and intentions in relation to it.

_Our long term goals are to democratize access to all financial products for any crypto native, from accessing bank accounts and payments, to cards, loans and many other services that aren’t easy to access or even at all available to the general public._

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Personal recommendation.

- Financially backed by Stake Capital, Funfair Ventures, Nemo ventures
- We are partnering with Otonomos/OtoCo allowing their onchain entities to access TradeFi (FYI this could be a future deal for them to setup entities on the NEAR chain too)
