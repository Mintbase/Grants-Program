# nft-challenge-factory

A factory smart contract for creating NFT challenges, the factory keeps track of nft challenges it's created. Note, if you make any changes to the [challenge contract](https://github.com/TENAMINT/nft-challenge), you'll need to 
get the compiled `target/wasm32-unknown-unknown/release/nft_challenge.wasm` file within the  [challenge contract repo](https://github.com/TENAMINT/nft-challenge) after building and replace the existing `nft-challenge.wasm` file at `wasm/nft-challenge.wasm` location in this repo. Note you need to rename `nft_challenge.wasm` to `nft-challenge.wasm` as well.

## Important: 
The challenges generated will be at the account id `{ID_PREFIX}.{ID OF THIS FACTORY CONTRACT}` For example if I created a challenge with the id prefix of my-challenge, and the challenge factory contract was deployed at challenge-factory.testnet, then the challenge contract would be deployed at account `my-challenge.challenge-factory.testnet`


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
