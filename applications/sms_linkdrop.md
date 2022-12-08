# Mintbase Grant Proposal

- **Project Name:** NEAR NFTs SMS linkdrop 
- **Team Name:** HERE Wallet
- **Payment Address:** neafiol.near
- **[Level](../README.md#level_slider-levels):** 1


### Overview

SMS Sender is a service that allows companies and dApps to send NFT to users who do not yet have a NEAR wallet by phone number. The NFT will go to a smart contract for temporary storage while it's being sent. Once the user confirms that they own the number they will be able to retrieve the NFT. This will allow to attract a lot of web2 users, for example through NFT tickets or games.

In order to do this, we're going to make

- A smartcontact that can store, give and proxy NFTs after confirming the phone number.
- A backend server to send notifications and phone number confirmation.
- update the wallet selector and py-near library so that web3 projects could easily integrate sending NFT to users
- make a security system to hide comments and phone numbers and make the phone number verification process transparent.

 We make a HERE wallet and onboarding web2 users is the main task we have to solve. We think that if we make an independent and open-source service for sending NFT via sms and make convenient libraries for using it, we can create a new channel to attract web2 users.

### Project Details

Sending NFT by phone number consists of 3 basic steps.

1. Sending NFT to a contract
2. Sending sms notification
3. Confirming the number and receiving the NFT

We will make a handy tool for each of these steps to make it easy for third party developers to use this feature.

1. We'll make client libraries in python and java scripts so developers can send NFT to the phone number in one line of code

2. We will make it possible to pay to send sms to NEAR. The project will be able to attach them to the translation of NFT and not to make its own service for notifications.

3. we will add a possibility to get NFT by phone number on phone.herewallet.app and in client libraries.  We'll also add the ability to encrypt comments.


Features we will add to send NFTs
- return by expiration date
- possibility of recurring sms notifications
- NEAR payment for sending sms
- encrypted comments to transfers  
- python and java script libraries 

**Backend:** open source repository on fast api using twilio api
**Smart Contact:** open source repository on rust using near-rust-sdk
**Libraries:** PR into wallet-selector and py-near repositories

### Ecosystem Fit


#### Where and how does your project fit into the ecosystem?
Our project can be integrated into existing NFT marketplaces and cryptocurrencies on NEAR protol. It can be used for sending gifts, NFT bought for fiat, raffles.

####  Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
Our target audience is developers for whom we will make a simple tool to attract web2 users

Also, our target audience is the web2 users who will be able to easily receive and submit NFTs even without understanding many web3 concepts

#### What need(s) does your project meet?

many users are now stopping at the point of creating a wallet and topping up their balance to pay for gas. If they can get NFT right away then anyone can start using NEAR. The motivation of a user to create a wallet if they already have NFT is much higher and we projects will be able to get more users.


#### Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
KeyPom

we do sms instead of a link, this will allow you to send money to any user and guarantee anti-spam protection. We are also doing not only smart contract, but js/python libraries so any developer can add NFT to web2 project. 

## Team

### Team members

- Volnov Petr
- Patrik Duksin
- Andrey Zevlakov

### Contact

- **Contact Name:** Volnov Petr
- **Contact Email:** petr@herewallet.app
- **Website:** herewallet.app

### Legal Structure

- **Registered Address:** HERE Wallet, Inc 8 The Green Suite #4655 Dover, DE 19901
- **Registered Legal Entity:** HERE Wallet inc

### Team's experience

**Petr Volnov** Rust NEAR/Backend dev at HERE Wallet, ex-backend developer in [osai.ai](http://osai.ai/) (sport CV recognition), founder fora.vision rust developer in Wannaspin (p2p game on NEAR ). Computer Science at ITMO University

**Patrik Duksin:** IOS/web developer at HERE Wallet, developed iOS app at Oops Finance (gen-Z finance manager), iOS dev at Mobileup (outsource team for ‎DMV Genie app).

**Andrey Zhevlakov** IOS/web developer at HERE Wallet, ex-frontend engineer in the tool development department at Dasha.ai ex. senior frontend at StudyWorld

Our team has extensive experience working on NEAR projects, including first place on [Etherium Global SF](https://ethglobal.com/showcase/app-clip-instant-wallet-i3p3j), NEAR Hacker House hackaton in New York

### Team Code Repos

- https://github.com/here-wallet
- https://github.com/here-wallet/phone.herewallet.app
- https://github.com/here-wallet/phone-access-contract

Please also provide the GitHub accounts of all team members. If they contain no activity, references to projects hosted elsewhere or live are also fine.

- https://github.com/pvolnov
- https://github.com/elro-root
- https://github.com/AZbang

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/pvolnov/
- https://www.linkedin.com/in/patrik-duksin-a966581a3/
- https://www.linkedin.com/in/azbang

## Development Status :open_book:

We have worked on the task of sending FT by phone number before. Including we have made a project FT by phone which is open source here

https://github.com/here-wallet/phone.herewallet.app

and available for demo here

https://phone.herewallet.app/

we also made a smart contract for temporary storage of FT tokens, and it's open-source too

https://github.com/here-wallet/phone-access-contract

we really have a lot of work in this direction and have been doing FT by sms for over 2 months now. Now we want to focus on features for NFT sending and build libraries and documentation so 3rd party projects could use NFT sending via phone number.

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 1 months
- **Full-Time Equivalent (FTE):**  3 FTE
- **Total Costs:** 10,000 USD
### Milestone 1 Example — Implement Mintbase Modules

- **Estimated duration:** 1 month
- **FTE:**  3
- **Costs:** 10,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a user can (for example) spin up one of our Mintbase nodes and send test transactions, which will show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article** that explains how to use nft-sms python/js libs in web2 projects
| 1. | Smart contact | We will create smart contact with NEAR rust sdk that can store, give and proxy NFTs after confirming the phone number |  
| 2. | Backend + twilio | We will create backend server on python FastApi to send transfer notifications and phone number confirmation via twilio |  
| 3. | py-near | We will create PR to py-near library with NFT-sms api |  
| 4. | wallet selector | We will create PR to wallet selector library with NFT-sms api |  
| 5. | Security | We will add the ability to encrypt comments in smart contact calls to transfers so that only the sender and the recipient can read them. |


## Future Plans

- we will add the ability to unsubscribe NFT by phone number to the HERE Wallet app
- we will integrate this feature together with mintbase team and meteor wallet


## Additional Information

**How did you hear about the Grants Program?** personal recommendation from Maria Magdalena

We talked to the Mintbase team on the HERE integration and they offered to apply for a grant to put the NFT by texting in the public domain and make tools so that other projects could also use this functionality.
