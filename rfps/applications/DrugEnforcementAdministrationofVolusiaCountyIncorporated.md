# Overview 

This is a proposal for a DAO Agent to be developed and hence added to the current list of supported agents for Metor wallet. 
Interacting with Sputnik DAO contracts right now has 2 major problems. 
-  Needs other UX and css for prefilled templates for moving tokens and making swaps between the ref.finance contract. 
-  Does not have an option to create credentials for agents.
A DAO agent on Metor wallet would make it easy for users to interact with Sputnik V2 contracts. 

## Functionality 	

### Search Functionality 
**DAOs**
- Retrieve a list of all DAOs 
- Search for a specific DAO using keywords (e.g., 'education', 'sustainability'), contract addresses (e.g., 'dao123.near'), or names (e.g., 'Volusia DAO')

**Proposals**
- Fetch all proposals for a given DAO 
- Retrieve detailed information for a specific proposal within a DAO, including vote counts and status (pending, done, approved, rejected) 

**User-specific**
- List all DAOs a user is part of and what permissions they have.
- Display all proposals created by a user within a specific DAO or all DAOs.
- Show all proposals a user is eligible to vote on

### Voting 
- Enable voting on a specific proposal ID, integrated with the proposal search functionality

### Proposal Creation 
- Create Near Transfer proposals (with refinements to existing functionality) 
- Generate Function Call proposals: 
    - Transfer stable coins
    - Swap Near for stable coins via Ref
    - Token swaps (implementation to be carefully considered)
    - Transfer any FT token, add liquidity to pools for following tokens: usnr.tkn.near,usnd.tkn.near,usnrd.tkn.near,rnc.tkn.near,dnc.tkn.near,nfc.tkn.near
    - Use the rainbow bridge to push FT to other blockchains
 

- Add or remove DAO members 

## Deliverables 

### Release 1 
1. List of all DAOs a user is part of. (Default to current logged in user.) 
2. List of top n(or all) proposals in a DAO. 
3. List proposals the user is eligible to vote on. 
4. Specific Information for a given proposal. 
5. Voting on a given proposal. 
6. Create a Near Transfer proposal. 

### Release 2 
7. Fetch all DAOs. 
8. Fetch a single DAO using specific keywords. 
9. List proposals only created by the user. (An admin can check the status of their proposals )
10. Proposal for Stable Coin transfers. USDC and USDT for now. (Can merge with FT proposals but separate APIs would ensure safety.) 

### Release 3 
11. Proposals for Near to stable coin swap. 
12. Add/Remove member 
13. Swap A with B. (Not sure of this for now.)

**Budget**: 6000 USD (Release based payment)
**Timeline**: 45 days from approval. (A release every 15 days with  2000 USD payment.)
