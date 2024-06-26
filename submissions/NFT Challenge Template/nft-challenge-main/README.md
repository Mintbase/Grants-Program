# nft-challenge

Smart contract for NFT challenges. NFT Challenges are ways to reward users NFTs(or even RWAs) for completing challenges, where challenge "components" are represented as NFTS they've collected. Once they collect all challenge piece NFTs, their eligible to complete the challenge and be marked as a winner!

## Important: 
If you plan on burning challenge NFTs on completion, make sure users who try to complete the challenge **give the challenge contract transfer approval for their challenge piece NFTs**, so that the challenge contract can burn them.
If you plan on minting the reward NFT through the challenge contract, ensure you make the **challenge contract a minter of the reward NFT contract**.


## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Iteract with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
