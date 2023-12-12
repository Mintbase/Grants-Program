# Mintbase Grant Proposal

- **Project Name:** Monad NFT Multi-Chanel Gated Ticketing, Creator Minting and Marketplaces
- **Team Name:** Monad.Social
- **Payment Address:** monadsocial.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### Overview

- Monad.Social is a subscription-based, virtual live performance streaming experience and community powered by web3. 
- Monad.Social allows for creators to manage and monetize their own fanbase and offers a number of benefits for those fanbases from a variety of functional NFTs to Live Streaming performance, sophisticated chat and feeds, badging, community recognition and much more. For more information, see Monad deck: https://docs.google.com/presentation/d/1LgFlNbeyC9v7SMxqyp1zpqShieDY2Wk9A7NlELU-_Lg/edit?usp=sharing
- As part of our prior NEAR Grant, we have implemented a basic NFT system for POAPs, tipping benefit and earning Monad FTs. Next steps are to scale, make creation easier, and a number of other improvements to the NFT system and its overall integration into our experience. A core part of our goal is to make these technologies have value for the average "fan", make it very easy to engage with and use, and drive greater creator/fan engagement and satisfaction (leading to continued subscription).
- Monad already received and completed a $75k NEAR Grant to build out all the necessary infrastructure and to launch the World's First "dance to earn app" we call the Virtual Dance Floor. We use the gyro and accelerometer in mobile devices to translate dancing movements into pulsing avatars that earn Monad Tokens. Phase II of this roadmap is to integrate Mintbase in order to expand and improve the UX for both the Creators and Fans. We will implement the ability of Artist/Creators to directly upload their NFT art that will be used for sale of NFTs that purchased by fans as tips for music performance and POAPs. For more information on prior NEAR Grant, see Trello Board https://trello.com/invite/b/VJ1pxt4j/ATTI2d3bc270064cf3bdadde51cd55778fb6277FEF7C/near-grant-milestones
- In addition we want all these NFTs to be searchable and discoverable by regular web search. We will also implement the ability for Fans earning Monad Tokens via the Virtual Dance Floor, and other user behaviours, to be able to spend these Monad Tokens to purchase NFTs and to tie all of this into a NFT Gated Ticketing solution that can be used both in Virtual Events andy by IRL Venues.
- The goal here is to create an economic ecosystem here that drives customer acquisition and retention behaviour for the masses - not to create speculative assets.
- In order to accomplish this, we need to make the UX, especially for Creators, as simple and intuitive as possible. In our discussions with Mintbase executives (Paul Kuveke and Nate Geieir), we believe in aggregate your solutions are the best option for us to achieve these goals.
- These products and developments will allow us to complete the last step in this phase which is NFT Gated Ticking. Ticketing A Huge Problem in the music industry ecosystem in particular is the loss of control by the Artists/Creators once a ticket is Initially Sold. Many bots purchase tickets to be marked up tremendously then resold on StubHub, etc. Not only do “real fans” often miss the opportunity to purchase these tickets, which upsets all artists, but of course none this money for this ‘ticket scalping’ goes to the artist / creators / event producers. This is on top of the exorbitant fees charged by these ticketing services the vast majority of which is in the 30% to 50%+ range.  Not only will we solve these problems, by using our Ticketing Platform as another extension of Tokenization, Gamification this will be another important element in customer acquisition and retention for Monad that will increase total revenues but not at the expense of artists, creators and our ticketing customers.
- Completing this buildout will provide the foundation for the next phase of comleting our AR Virtual Venues Portals built on Unity with tokenizing and implementing these featues via the NEAR/Unity SDK (more below and in Trello).

Adding Royalty Payments will further level the playing field for Ticket Sellers and will further drive transaction volume since these Sellers will even promote the opportunity to buy these secondary tickets via Monad/MB/NEAR Marketplace.

### Project Details

- Mockups/designs of any UI components
  - We have a complete, fully functioning MVP with customers and generating revenue now. The UI/UX for users both on the front-end side and for Creators to upload art is already completed and live. We can provide access to any Mintbase people since the content is behind a paywall.
  - Fan UX, Purchase of NFTs as tips, POAPs - https://monad.social/circle/djsneak/
  - POAP Upload UI: https://monad.social/wp-content/uploads/2023/12/MonadSocial-Create-PoAP-NFT.png
  - NFTs for tipping: https://monad.social/wp-content/uploads/2023/12/MonadSocial-Create-Tipping-NFT.png
  - Data models / API specifications of the core functionality
    
- An overview of the technology stack to be used
  - Smart Contracts: Rust, NEAR SDK, Mintbase API
  - Front End: React, HTML, CSS, JavaScript
  - Back End: We are Google Cloud for Startups Partner so all infrastructure on GCP with a WordPress MySQL DB.

### Ecosystem Fit

- Where and how does your project fit into the ecosystem?
  - We had always planned after "Phase I" of our first NEAR Grant was to continue building out our Product Roadmap with NEAR and partners. After substantial research and discussions with the Mintbase team, we believe this is the best solution for us to carry on with our roadmap since we have built sunbstantial aspects of the underlying required architecture, it makes to continue building upon this foundation.
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
  - We have 2 target audiences - Music Creators and their Fans. Music Creators indcludes artist, record labels, festivals, and venues. We are working with one of the 3 Major Record labels amd other artists to help them migrate their 30 million YouTube subscribers to Monad.
- What need(s) does your project meet?
  - We need to create simple, elegant solutions to lower the barriers for Creators and Fans to supply art that can be monetized through tokenization and digital asset sales.  Then to in turn create entertaining Fan Experiences that will encourage usage and therefore be an excellent customer retention tool. We also see an opening in the market for a low cost NFT Gated Ticketing solution that will combine access to our Virtual Venues with IRL Venues.
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  - Sweatcoin has done something similar of course with the "exercise to earn" aspect of what we're doing. We are unaware of other Web3 versions of Twitch for Music which is what we are building out.

## Team :busts_in_silhouette:

### Team members

- Brett Hawkins - CEO Product Lead
- Greg Johnson - Chief Digital Strategist
- Jiandon Wei - CTO
- Anvar Bagiyev - Lead Designer/UI/UX

### Contact

- **Contact Name:** Brett Hawkins
- **Contact Email:** Brett@monad.social
- **Website:** https://monad.social/

### Legal Structure

- **Registered Address:** 920 California Ave., Suite 3, Venice, CA 90291
- **Registered Legal Entity:** Monad.Social PBC

### Team's experience

Our Team has been building complex, consumer facing internet applications together for over a decade. In addition to having received the prior NEAR Grant of $75,000 and completed that buildout, we are also a Google Cloud for Startups partner. This team built an early e-sports live streaming platform called Global Gaming League that we took from a napkin to 10 Million MAUs. We are applying a lot from that experience to apply video game and Web3 technologies to the music industry and broader.


### Team Code Repos

- https://github.com/monadsocial
  - Project 1: https://github.com/monadsocial/NEAR-NFT
  - Project 2: https://github.com/monadsocial/Near-Tipping
  - Project 3: https://github.com/monadsocial/MonadFT

### Team LinkedIn Profiles

- https://www.linkedin.com/in/bretthawkinsjr/
- https://www.linkedin.com/in/gregrjohnson/
- https://www.linkedin.com/in/jiandong-wei-3a602158/
- https://www.linkedin.com/in/anvarbagi/

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 3.5 Months
- **Full-Time Equivalent (FTE):**  3 FTE (Lead Engineer + Front End Engineer + UI/UX/Design + Strategy/Marketing, 7 people)
- **Total Costs:** $45,000

### Milestone 1  — NFT Frontend and Minting Interaction

- **Estimated duration:** 1.5 month
- **FTE:**  3
- **Costs:** 15,000 USD

### Milestone 2  — Build Fan NFT Front End and Minting Interaction

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 15,000 USD

### Milestone 3  — Build Storefront and Legacy NFT Integration

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 15,000 USD

Details of what will be delivered in each milestone is available at https://trello.com/invite/b/rYDEt1eP/ATTI5e27b825ab7be33b11cf362652ebd14b9031C994/mintbase-grant-milestones

## Future Plans

The next steps in our 2024 Roadmap after completion of these milestones will include:

- Complete the creation, buildout and infrastructure development for our Tokenizaed AR Portal Virtual Venues (video demo here: https://vimeo.com/861100716/1c938a25fd?share=copy ) including gamification via building out / customizing the Unity SDK for NEAR that carries over from the current Virtual Dance Floor App that’s already been built on NEAR. This will include earn and burn mechanics, use of NFT based Avatars, minting, tracking and allocating FTs in AR App and related gamification features and built to be compatible with AR/VR Headsets (though not required).
  
- When Twitter / X SDK available for Crypto Payments (we don’t see access yet), add this to our payment alternative of PayPal, Stripe and we’re working with other payment processors like Pix in Brazil.
  
- Further build out of NFT Gated Ticketing solutions to provide more features for IRL Venues combining their ticketing with Monad Virtual Venus and experiences. We see this as a very large opportunity and discussing desired Use Cases and features for our partners at Warner Music, festivals like Tomorrowalnd, ADE and others.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?**

Mintbase directly as introduced by NEAR and mentioned above.

- We have already received grants from NEAR $75k, Google Cloud for Startups $200k+ and Celo Camp $20k
