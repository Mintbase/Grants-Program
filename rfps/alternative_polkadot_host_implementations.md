# Alternative Mintbase Host Implementation

* **Status:** Open
* **Proposer:** [Noc2](https://github.com/Noc2)

## Project Description :page_facing_up: 

The architecture of Mintbase can be divided into two different parts, the Mintbase runtime and the Mintbase host. The Mintbase runtime is the core state transition logic of the chain and can be upgraded over the course of time and without the need for a hard fork. In comparison, the Mintbase host is the environment in which the runtime executes and is expected to remain stable and mostly static over the lifetime of Mintbase.

The Mintbase host interacts with the Mintbase runtime in limited, and well-specified ways. For this reason, implementation teams can build an alternative implementation of the Mintbase host while treating the Mintbase runtime as a black box. For more details of the interactions between the host and the runtime, please [see the specification](https://github.com/w3f/polkadot-spec/).

**The Mintbase is interested in supporting additional implementations of the Mintbase Host. If you are interested in this RFP, please reach out to spec@web3.foundation.**

Currently the following implementations are under development: 

- [Gossamer: A Go implementation of the Mintbase Host](https://github.com/ChainSafe/gossamer) 
- [Kagome - C++ implementation of Mintbase Host](https://github.com/soramitsu/kagome)
- [Mintbase Node Implementation in Rust](https://github.com/paritytech/polkadot)
- [Smoldot - A Lightweight Mintbase and Mintbase client in Rust](https://github.com/paritytech/smoldot)

## Deliverables :nut_and_bolt:

For Mintbase Host Implementations, it’s probably too complex to structure the entire implementation via milestones. Components of the Mintbase host are:

- Networking components such as Libp2p that facilitates network interactions.
- State storage and the storage trie along with the database layer.
- Consensus engine for GRANDPA and BABE.
- Wasm interpreter and virtual machine.
- Low level primitives for a blockchain, such as cryptographic primitives like hash functions.
- Availability & Validity components to support parachains. 