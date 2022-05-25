# Mintbase Grant Proposal

- **Project Name:** the-auction.io
- **Team Name:** MagicPowered
- **Payment Address:** the-auction.near
- **[Level](../README.md#level_slider-levels):** 3

## Project Overview :page_facing_up:

### Overview

the-auction.io is an NFT secondary marketplace where everyone can put their NFT on auction sale or buy the NFT at The Auction. We are providing a place where users can buy and sell their NFTs and a platform for NFT projects to sell their NFTs at the highest price possible. We have a "Projects" section, and any NFT project can sign in and publish their NFTs on their own page.
Currently, our platform supports all NEP-171 tokens available on the Near blockchain, and we leverage the nft_approve method to manage transfers to the auction winner.

### Project Details

The Auction as a platform is already finished in it's first working beta.
We are seeking for a grant to implement tight integration between The Auction and Mintbase.

We are extending "Projects" functionality to provide more powerful tools for an artists:

1. Ability to create a Project in The Auction:
    1. Mint new NFT collection (Create Store and mint tokens)
        1. Create new Mintbase store on the Project creation wizzard
        2. Mint tokens in the new store
        3. Provide necessary metadata for the tokens
        4. Create a Project page in The Auction with the link to the Mintbase collection (Store)
        5. Upload Project related information: Branding, Descriptions, Links, etc
    2. Create new The Auction Project page and link existing Mintbase store into it.
2. Tight integration with Mintbase contract to show all additional token information:
    1. attributes stored on Arweave
    2. all additional token metadata from the token itself
    3. Display transaction history for the tokens

Current Projects MVP implemented in a way that the project is just a Near wallet and the list of the NFTs that this
wallet sell at the moment, example: https://the-auction.io/user/beta-the-auction.near
Any user can convert their profile to the project profile and it will appear in the "Projects"
section: https://the-auction.io/projects

By extending the "Projects" functionality we separate "project" entity from the user wallet and will be able to:

1. Show Mintbase collections on "Projects" page instead of user's wallets.
2. Provide all time statistics for each Mintbase collection separately.

### Ecosystem Fit

The Auction is the first general secondary NFT marketplace.
It means that we are not limited to single NFT platform and we can provide buy and sell capabilities for any NEP-171
tokens in Near ecosystem.
It also means that we have very wide target audience: from artists and nft projects who want to sell their own
masterpieces to random folks who want to buy and sell their own NFTs.

There are huge portion of our beta users and NFT projects who requesting minting functionality on the platform.
It is make sense to mint NFT on the marketplace platform where the tokens could be sold and where each project can have
it's own space.

We think that it does make sense to integrate The Auction with the Mintbase to provide an option for projects to choose
a convenient way to mint their NFTs.

Also, knowing that Mintbase is the one of the larges NFT Platforms on Near ecosystem we expect that a big portion of the
tokens that will be sold on the auction will be minted on Mntabase so it also make sense to provide better visualisation
of the all token metadata that we can receive from the Mintbase API and increase retention over Mintbase tokens.

## Team :busts_in_silhouette:

### Team members

- Vicky Plashevska - Product Owner
- Anatolii Petrovskyi - Technical Lead
- Maksym Khaiuk - Frontend developer
- Eduard Kalosha - Frontend developer
- Oleh Kalenyk - Smart contract developer / API developer
- Illia Mazurin - QA Engineer

### Contact

- **Contact Name:** Petrovskyi Anatolii
- **Contact Email:** anatolii@magicpowered.io
- **Website:** the-auction.io

### Legal Structure

- **Registered Address:** 3520 Spruce Hill Rd, Ottsville, PA, USA 18942
- **Registered Legal Entity:** Magic Powered Inc

### Team's experience

The team was involved in different projects that is listed here: https://magicpowered.io
The Auction itself is the grant project from Near Foundation https://near.foundation/

### Team Code Repos

There are no active opensource profiles of the team yet.

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/anatolii-petrovskyi-76905a64/

## Development Status :open_book:

The auction currently is in the public beta (MVP) state. Everyone can access it and use it's
functionality: https://the-auction.io
The code is still closed in the private repositories, but soon we will opensource our smart contract.

The Mintbase integration and the Projects functionality extention now is in design stage.

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 2 Months and 1 week
- **Full-Time Equivalent (FTE):**  4 FTE
- **Total Costs:** 60,000 USD

### Milestone 1 — Fetch Mintbase token metadata and visualise it on the auction page

- **Estimated duration:** 1 weeks
- **FTE:**  2
- **Costs:** 5,000 USD

| Number | Deliverable                                           | Specification                                                                                                                                                 |
|-------:|-------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     1. | Show full Mintbase token metadata on the auction page | When user will open the auction page to place a bid they will be able to see the whole list of available token metadata with the attributes stored on Arweave |  

### Milestone 2 — Create NFT project with existing NFT collection

- **Estimated Duration:** 1 month
- **FTE:** 4
- **Costs:** 20,000 USD

| Number | Deliverable                                                | Specification                                                                                                                                                                         |
|-------:|------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     1. | Project creation wizzard                                   | User can create a project in The Auction and set themselfs as a project admin                                                                                                         |
|     2. | Import mintbase store                                      | user can choose "import Mintbase store" and specify the link to the mintbase store. The Auction will download all existing tokens from the store contract to show on the Project page |
|     3. | Project page shows all tokens minted in the Mintbase store | The Auction automatically fetch any changes happened with the tokens on the store: mint, burn, transfer                                                                               |

### Milestone 3 — Create NFT project and mint NFT using Mintbase Contract

- **Estimated Duration:** 1 month
- **FTE:** 4
- **Costs:** 35,000 USD

| Number | Deliverable                                             | Specification                                                                                                                                                    |
|-------:|---------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|     1. | Create Mintbase store option on project creation wizard | User can choose "create Mintbase store" when creating the project. The Auction will create the new store using Mintbase Contract                                 |
|     2. | NFT Minting flow on the Project admin space             | User can mint new NFT on the "Project Admin" space on The Auction. That include hte full minting flow: upload media, set attributes, royalties and revenue split |


## Future Plans

We plan maintain and support this integration in the future, upgrade our code to comply to the Mintbase contract and API changes

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?**: personal recommendation

https://the-auction.io/about
