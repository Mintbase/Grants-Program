# Mintbase Grant Proposal


- **Project Name:** LAND OF CLANS
- **Team Name:** Indie team: Alex&Serg
- **Payment Address:** thegame_dao.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### Overview

"Land of Clans" is a v2 of popular multiplayer, strategy game on NEAR - but this time with GameFi2.0 economical model.
(v1 has a classic play-to-earn economical model.)

Our first game version "Land to Empire" launched on NEAR on February 2022. 
Our Statistic for two years online on NEAR:
  5.5K players
  197K p2p battles
  166K buildings
  33K token transfers
  27K nft transfers
  21K awards

Our first NFT collection is in TOP Mintbase Games - https://www.mintbase.xyz/explore/Gaming/0

### Project Details

"Land of clans" - is a strategy game with implemented GameFI2.0 economical model based on DeFi’s style NFT-Pools and AMM. 
We are pushing the idea of GameFi2.0 so that there will be more games with stable economies in the World.

Game landing page - https://www.landofclans.io/

GameFi2.0 = GameFi + NFT-pool + AMM

The idea is simple - we create an NFT-pool contract, and players' payments will fill this pool with liquidity. All along the lines of miners and stakers and those who add liquidity to DeFi. Someone without finances spends his time mining resources in the game (Proof-of-Game). Another one with liquidity buys up the first mined resources and adds liquidity to the pool. Others, who have just started playing and have funds, buy resources in the pool to catch up with those who have been playing for a long time.
We create the rules, and players come up with strategies to earn money, often they surprise us and it's great. It is already good, the money is publicly controlled and does not leak out of the project to the unknown.

And what we get at the start is an NFT-pool without NFTs, but with liquid tokens.

Then we connect the AMM (Automatic Market Maker) contract, which contains a price list of all types of NFTs and regulates their rates.

And then the players who minted NFTs and want to sell them - will be able to swap them for tokens at the rate of AMM.  Example how work AMM:
The price of the NFT is 200.
You sell the NFT for 200.
AMM will put it on the market for 210 and drop the purchase price to 195.
The next player sells the NFT for 195.
AMM will put it up for sale for 205 and lower the purchase price to 190.
If there are no sales for a while, AMM will raise the price to 191, then 192, and so on.
AMM's task is to adjust the price so that it reflects the value of each NFT.

If there are a lot of sales - this is a sign that the value of the resource is lower than its value and AMM will lower the price of that type of NFT. If players on the contrary buy NFTs - their value is higher than the current price, AMM will raise the price of them. 
And thus very quickly the price will be adjusted and will reflect the real value of each type of NFT in the game.

All purchased NFTs, AMM automatically puts on sale with a markup of 5-10%, which will go to the earnings of NFT-pool.

### Ecosystem Fit

The ecosystem will get open-source liquid NFT-pool and AMM contract (Automatic Market Maker).

Many projects will be able to set up such pools for themselves, as a result of which they will get a significant increase in trading operations, user interest and trust will grow by leaps and bounds, AMM will adjust nft prices accordingly to their value to users.

The Land of Clans game will be the first implementation of NFT pools on the NEAR blockchain.

We are ready to be pioneers and show the NEAR ecosystem all the benefits of the revolutionary economic model, the increase in the number of user transactions it brings and how much more stable the economic models are.

## Team :busts_in_silhouette:

### Team members

Founders are developers with 10+ years of commercial experience and startup building.
- Alex CEO co-founder
- Serg CTO co-founder
- Alexander Marketing manager
- Katerina SMM manager
- Sasha Backend Developer
- Oleg Senior Frontend

### Contact

- Alexander Kolesnikov
- thegame_dao@protonmail.com
- https://www.landofclans.io/

### Legal Structure

- **Registered Address:** N/A
- **Registered Legal Entity:** N/A

### Team's experience

Since 2015, we have become evangelists of blockchain technology. We participated in the development of AML and KYC platforms. We launched private blockchains for financial institutions. We have developed several open source libraries that have become quite popular. In addition, we launched three games, got great feedback and engagement from our users and continue to improve in this direction.

### Team Code Repos

https://github.com/land2empire
https://github.com/near-rails-guide/near_api
https://github.com/near-rails-guide/borsh-rb
https://github.com/near-rails-guide/near-rails-demo

https://github.com/2rba
https://github.com/k-o-l-e-s

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/koles/
- https://www.linkedin.com/in/serge-tyatin/

## Development Status :open_book:

Since we have already implemented and tested all the mechanisms on the WAX blockchain, we need to migrate to NEAR and rewrite some of the libraries and contracts.

## Development Roadmap :nut_and_bolt:

The first step is to research the capabilities of the tools, selecting the right tools. Then we make the missing tools.

The second stage is the launch of the testnet and a cycle of tests, improvements, and extensions of the functionality.

The third stage is the launch of the mainnet.

The fourth stage is the promotion and support of the project.

### Overview

- **Total Estimated Duration:** 6 months
- **Full-Time Equivalent (FTE):**  4 FTE
- **Total Costs:** 50'000 USD

### Milestone 1 —  Preparing the game to integration and migration to NEAR

- **Estimated duration:** 1 month
- **FTE:**  4
- **Costs:** 10,000 USD

Thanks to our previous experience with Rust, it will be easier for us to write the necessary contracts and other libraries for frontend/backend interaction with NEAR. We will publish anything that may be useful to other developers as open source.

● R&D, Architecture selection,
● Developing supporting libraries SDK,
● Frontend - integrating with NEAR wallet,
● Backend - integrating with nodes and indexers (Mintbase GraphQL Indexer),
● Gameplay features connect with API
● Social media setup (twitter, telegram group, tik-tok, youtube)
● Manual and Documentation for gamers
● Presentation material and video tutorials

### Milestone 2 — NEAR Testnet game launch 

- **Estimated Duration:** 1 month
- **FTE:**  4
- **Costs:** 10,000 USD

● Node SDK integration with gameplay backend
● Game tokens contract setup
● Creating NFT collection
● Publish NFT collection on Mintbase,
● DeFi pools setup: NEAR/Gems, Gems/Gold, Gems/Food (we need to co-operate with any of the existing DEX) Premium tokens GEMs are the most coveted among players, they allow you to speed up many processes. But since the creators of the game do not sell them, an unsatisfied demand is created, which is satisfied by other players and gets p2p exchange of tokens between players through pools, which are replenished by the players themselves.
● Contract of awards competitions and distribution
● Land lottery contract - player can start the game with a lottery and paying a small amount of money can win an award of different levels of coolness, then his start of the game will be already with a fairly pumped up building, usually it's a head start of several months to six months of intensive play.
● Premium award contract - players will be rewarded for playing non-stop every 10 days, 30 days, 90 days. According to our experience 30% of players choose to start the game through participation in the lottery. the lottery contract, after payment, gives them one of the available configurations, which the player will receive at the start of the game. If he is lucky, his start will be comparable to a player who has already been playing for 6 months. In the worst case, this head start will be 2 weeks.
● Game social media support
● Documentation upgrade

### Milestone 3 — NEAR Mainnet game launch 

- **Estimated Duration:** 1 month
- **FTE:**  4
- **Costs:** 10,000 USD

● Issues fixes
● Gamers feedback analysis
● Partnership with Mintbase - we need a partner to publish our NFT collection. So that our players can sell their in-game Nfts.
● Partnership with DEX - NEAR/Gems token pool better way when pool will be integrated in wallet NEAR like in MetaMask. A third-party service will work for us, too. GEMs are premium tokens that players will receive as rewards. They can be used to buy accelerations in the game. So players will try to accumulate them and buy the missing amount in the DEX pool. The demand for tokens will determine their price in trhe pool. In our experience, the price is stable due to the fact that the development team does not sell these tokens and their issuance is done exclusively through in-game rewards (proof-of-game).
● DEX integration with game token economy
● Presentation with Land Of Clans to the ecosystem and invitation for partnership 
● Integration and collaboration with dApps in the NEAR ecosystem.
Target by users is 1K and 15K transactions

### Milestone 4 — PR & marketing  

- **Estimated Duration:** 3 months
- **FTE:**  4
- **Costs:** 20,000 USD

● Social media activity
● Working with a core of players, handing over moderation, content generation, memes...
● Documentation upgrade with data from mainnet
● Players generated content in Youtube, Tik-tok, Instagram, Telegram
● Integration and collaboration with dApps in the ecosystem NEAR

Target by users is 10K and 250K transactions


## Future Plans

For the past three years, we have been exploring the potential of blockchain for use in games and game-based economic models. Our goal is stable models, with a long life cycle. We have good results, but we can do better, so we are not stopping.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** personal recommendation

