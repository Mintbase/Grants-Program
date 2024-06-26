# NFT Challenge Template

Generate blockchain challenges that validate if users have all the challenge pieces! Capabilities include burning challenge pieces on completion, as well as minting a reward piece on completion. Users can also set an expiration date for their challenge, as well as set the maximum amount of winners for the challenge.

[![Demo](https://img.shields.io/badge/Demo-Visit%20Demo-brightgreen)](https://nft-challenge-creator.vercel.app/)


## Important: 
If you plan on burning challenge NFTs on completion, make sure users who try to complete the challenge **give the challenge contract transfer approval for their challenge piece NFTs**, so that the challenge contract can burn them.
If you plan on minting the reward NFT through the challenge contract, ensure you make the **challenge contract a minter of the reward NFT contract**.



## Run the project

    yarn install

    yarn run dev

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

We have set the environment variables necessary to get the template working on testnet right out of the box for you in the `env.example` file, simply rename it `env.local` and you'll be good to go! If you want to use the template on mainnet or have your own deployed challenge factory contract, change the environment variable values as desired.

## Demo videos
[Demo Video Part 1](https://drive.google.com/file/d/1-bRR35WkdvPea_CV7A29NWSJ_wWAGqBn/view?usp=sharing)



[Demo Video Part 2](https://drive.google.com/file/d/1oxsuym7PGw_IojSCQH6prk1IpT-r2dpW/view?usp=sharing)

Below is an example of a test challenge which was created to illustrate this template:

NFT Challenge Page: [https://nft-challenge-creator.vercel.app/challenges/tenamint-blackdragon1?transactionHashes=EpU3RWCnWTuXetECqdJQQ48XTBKGRUZZzrUjEziwNPBx](https://nft-challenge-creator.vercel.app/challenges/tenamint-blackdragon1?transactionHashes=EpU3RWCnWTuXetECqdJQQ48XTBKGRUZZzrUjEziwNPBx)
 Challenge NFT 1: [[https://testnet.mintbase.xyz/contract/tmxbdxbaseball.mintspace2.testnet/nfts/all/0](https://testnet.mintbase.xyz/contract/tmxbdxbaseball.mintspace2.testnet/nfts/all/0)]
 Challenge NFT 2: [[https://testnet.mintbase.xyz/contract/tmxbdxhockey.mintspace2.testnet/nfts/all/0](https://testnet.mintbase.xyz/contract/tmxbdxhockey.mintspace2.testnet/nfts/all/0)]
 Challenge NFT 3: [[https://testnet.mintbase.xyz/contract/tmxbdxbasketball.mintspace2.testnet/nfts/all/0](https://testnet.mintbase.xyz/contract/tmxbdxbasketball.mintspace2.testnet/nfts/all/0)]

Claim all 3 and then visit this link to complete the challenge claim your reward NFT: 
[[https://nft-challenge-creator.vercel.app/challenges/tenamint-blackdragon1?transactionHashes=EpU3RWCnWTuXetECqdJQQ48XTBKGRUZZzrUjEziwNPBx](https://nft-challenge-creator.vercel.app/challenges/tenamint-blackdragon1?transactionHashes=EpU3RWCnWTuXetECqdJQQ48XTBKGRUZZzrUjEziwNPBx)]

## Notes on Using V0 for AI generated components

A lot of the components for this template uses components generated by [v0](https://v0.dev), an AI frontend component generator. Here's a list of components I generated for this project, you can play around with the prompts to modify the components.

- [Choosing a reward NFT](https://v0.dev/t/VjZbaEsFCvv)
- [Choosing number of challenge nfts and current challenge info](https://v0.dev/r/2WXHinKdaFF)
- [Choosing termination details](https://v0.dev/r/gsYzCa15AWT)
- [Network Toggle](https://v0.dev/r/Tyu9vrlRBxZ)
- [NFT Challenge page](https://v0.dev/r/BRmDtqSddRR)