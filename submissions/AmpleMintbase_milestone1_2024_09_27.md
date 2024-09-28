# Ample x Mintbase Streaming Royalty Claiming, Gated Function and Legal Doc Generator Builds
## Milestone 1: Smart Contract Royalty Claim & Gated Streaming Access Modules

### Deliverables 

| Number | Deliverable | Link | 
| ------------- | ------------- | ------------- | 
| - | Main repo | https://github.com/AmpleProtocol/launchpad |
| 0a. | License | https://github.com/AmpleProtocol/launchpad/tree/master/LICENSE   | 
| 0b. | Documentation | - [main](https://github.com/AmpleProtocol/launchpad/blob/master/README.md) <br> - [core](https://github.com/AmpleProtocol/launchpad/blob/master/core/README.md) <br> - [ui](https://github.com/AmpleProtocol/launchpad/blob/master/ui/README.md) <br> - [client](https://github.com/AmpleProtocol/launchpad/blob/master/client/README.md) <br> - [server](https://github.com/AmpleProtocol/launchpad/blob/master/server/README.md) <br> - [contracts](https://github.com/AmpleProtocol/launchpad/blob/master/contracts/README.md) |
| 0c. | Testing guide | https://github.com/AmpleProtocol/launchpad?tab=readme-ov-file#testing-guide | 
| 0d. | Docker | https://hub.docker.com/r/menurivera/ample-launchpad-server | 
| 0e. | Article + video | https://youtu.be/H4EEH8ep2VU | 
| 1. | Mintbase module: Streaming Royalty Percentage Claims | https://github.com/AmpleProtocol/launchpad/blob/master/ui/src/components/Royalty.tsx | 
| 2. | Mintbase module: Gated Streaming Access Module | - https://github.com/AmpleProtocol/launchpad/blob/master/ui/src/components/Launch.tsx <br> - https://github.com/AmpleProtocol/launchpad/blob/master/ui/src/components/Player.tsx <br> - https://github.com/AmpleProtocol/launchpad/blob/master/ui/src/hooks/useStreaming.ts | 
| 3. | ~~NEAR chain integration~~ <br> NPM packages | (We agreed upon an npm packages based approach, here are the links to them) <br> - [core](https://www.npmjs.com/package/@ample-launchpad/core) <br> - [client](https://www.npmjs.com/package/@ample-launchpad/client) <br> - [ui](https://www.npmjs.com/package/@ample-launchpad/ui) |

### Formtatted code 

#### TypeScript 
1. `camelCase` for variables, functions and properties
2. `PascalCase` for Classes
3. `PascalCalse` with an `I` for interfaces (like `ISomeInterface`)
4. `CONSTANT_CASE` for constants
5. `camelCase` for files

#### Rust 
1. `snake_case` for variables, functions and properties
2. `PascalCase` for structs and impls 
3. `CONSTANT_CASE` for constants
4. `snake_case` for files
