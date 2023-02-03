
# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** Contesty
- **Team Name:** AtomicLab
- **Payment Address:** rubycop.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

### Overview

Contesty - is a WEB3 NFT contest-based platform. It's aimed to be a connector between creators, marketplaces and users.
It'll be a NFT hub where:
- creators will be in touch with community, create quizzes, promote his NFT projects in one place. They can participating in daily/weekly/monthly contests to win prize pool.
- users decide a winner by voting during contest time. They can also predict a winner staked CNT token
- marketplaces can provide special offers for users and organize contests

Project already started on NEAR and MVP launched on testnet.
As a team we are interesing to increase NFT activity on NEAR because we are seeing low activity and problems that can be solved.

#### Main Contesty goals:
- Solve connection gaps. Contesty is aimed to be a connector between NFT creators, marketplaces and end users.
- Increase buy/sell activity. Before user decided to buy NFT he wants to know its value, popularity and "hype" around target NFT. He wants to see clear metrics and latest contest winners.
- Solve Communication lags. Contesty wil get creators to organize community around his project. Make contests,  quizzes, etc. to be in touch with community
- Make global NFT activity hub. People start voting and get CNT token for their activity - popularizing creators each time and make natural NFT arrangement.

### Project Details

Whitepaper: https://contesty.gitbook.io/contesty
Twitter: https://twitter.com/contesty_nft
AwesomeNear: https://awesomenear.com/contesty

#### Curent MVP highlevel Contract data model

```rust
pub struct Participant {
    pub id: String,
    pub owner_id: AccountId,
    pub contest_id: String,
    pub token_id: String,
    pub nft_src: String,
    pub votes_count: u8,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Voter {
    pub owner_id: String,
    pub contest_id: String,
    pub participant_id: String,
    pub reward: u32,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Prediction {
    pub owner_id: String,
    pub participant_id: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Contest {
    pub id: String,
    pub owner_id: AccountId,
    pub image: String,
    pub title: String,
    pub size: usize,
    pub entry_fee: String,
    pub currency_ft: bool,
    pub start_time: String,
    pub end_time: String,
    pub winner_ids: Vec<String>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    contests: Vec<Contest>,
    participants: Vec<Participant>,
    voters: Vec<Voter>,
    predictions: Vec<Prediction>,
    contract_ft: AccountId,
    accounts: LookupMap<AccountId, u32>,
}
```

**Technology stack:**
- Backend:
	- Node.js
	- Near sdk
- Frontend:
	- React.js
	- Tailwind CSS
- APIs
  - IPFS
  - Paras API
  - Near Social API [in future]

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- Where and how does your project fit into the ecosystem?
  
  NEAR ecosystem needs NFT evaluation platform. Also it needs app that can easily boost new creators and increase NFT activity on marketplaces. 
- Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?
  - NFT creator
  - end-users
  - marketplaces
- What need(s) does your project meet?
  - Make NFT evaluation process easy and clear
  - Aimed to increase NFT activity
- Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?
  
  There are no exactly the same competitors right now.
  Here is some similar:
  - https://drawcast.com/
  - https://www.creaticles.com/

## Team :busts_in_silhouette:

### Team members

- Andrii
- Vlad
- Olha
- Svitlana

### Contact

- **Contact Name:** Andrii Saichuk
- **Contact Email:** saychuk.andriy@gmail.com
- **Website:** https://atomic-lab.io/

### Legal Structure

### Team's experience

We already launched Zomland game on NEAR. https://zomland.com/. Won sort of hackatons last year and receive grant 5000 NEAR from Human Guild for Zomland project

**Andrii** - 9y. experienced full-stack (Ruby on Rails, React.js, Rust) developer. Was working on such company [outstaff] as CircleCI as front-end developer building UI for account page, etc. Mostly working as back-end RoR, Rust developer. You can find list of applications on linkedin profile 

**Vlad** - 12y experienced full-stack (Php, Vue/React.js, Rust, Solidity) developer. You can find list of applications on linkedin profile 

**Olha** - 5y experienced Project Coordinator, Administrative and Operation Lead

**Svitlana** - SSM manager, community manager, manual QA

**Dev hackaton works:**
- https://devpost.com/rubycop
- https://devpost.com/vlodkow

### Team Code Repos

- https://github.com/rubycop/CNT
- https://github.com/VlodkoMr/zomland [private mode]
- https://github.com/VlodkoMr/near-message
- https://github.com/VlodkoMr/near-vouchers

**Dev accounts**
- https://github.com/rubycop
- https://github.com/VlodkoMr

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/andrii-saichuk-7b8284b3/
- https://www.linkedin.com/in/vlodkow/
- https://www.linkedin.com/in/olha-saychuk-0b2810116/

## Development Status :open_book:

Testnet available here: https://test.contesty.app/
Github repo: https://github.com/rubycop/CNT

#### MVP Screenshots:

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):
<img width="1281" alt="Screenshot 2023-01-26 at 15 09 26" src="https://user-images.githubusercontent.com/9679385/216600775-2be3b2f2-3589-4560-a0d8-92094811e374.png">
<img width="1367" alt="Screenshot 2023-01-26 at 15 09 36" src="https://user-images.githubusercontent.com/9679385/216600781-6c709bdd-90a8-4a3b-bfc7-9f3970f2a2f2.png">
<img width="312" alt="Screenshot 2023-01-26 at 15 09 49" src="https://user-images.githubusercontent.com/9679385/216600785-d05c79d4-312d-48e8-9581-c33f3d0f7aa1.png">
<img width="1339" alt="Screenshot 2023-01-27 at 12 24 38" src="https://user-images.githubusercontent.com/9679385/216600788-10096dd3-5804-4246-982f-f0626c0eebb6.png">
<img width="768" alt="Screenshot 2023-01-27 at 12 26 04" src="https://user-images.githubusercontent.com/9679385/216600791-fc74e86d-7dc4-469b-b84d-3299e5875100.png">
<img width="1159" alt="Screenshot 2023-01-27 at 12 26 46" src="https://user-images.githubusercontent.com/9679385/216600796-bed44b91-172f-4de7-a21b-a3a2dbcecfe9.png">
<img width="778" alt="Screenshot 2023-01-27 at 12 59 42" src="https://user-images.githubusercontent.com/9679385/216600797-067755a1-0540-4d38-bf3c-370ec610fd7d.png">
<img width="1095" alt="Screenshot 2023-01-27 at 14 48 20" src="https://user-images.githubusercontent.com/9679385/216600801-52ac92b1-051f-4068-9267-98266b3b5508.png">

## Development Roadmap :nut_and_bolt:

**January**
- [x] Project starting up.
- [x] Complete MVP with contest, voting, prediction flow
- [x] Near Social voting plugin [prototype]
- [x] Launch Testnet

**February - March**
- [x] Level-experience model. Locked/unlocked features per level.
- [ ] Quiz model for NFT creators
- [ ] "Promote my NFT" functionality
- [ ] Leaderboard info
- [ ] Extended contest models
- [ ] Head-to-head contest (1-to-1)
  - [ ] Bracket-type contests (1/16, 1/8, ... final)
- [ ] Early users reward program

**April**
- [ ] Developing public API to get NFT statistic
- [ ] Getting royalty from winning contests
- [ ] Partnerships with largest marketplaces
- [ ] Near Social integration
  - [ ] Request friend from Near Social.
  - [ ] Build near social plugin displaying leaderboard
- [ ] Marketing program

**May**
- [ ] Preparing production environment
- [ ] QA work
- [ ] Launching Mainnet
- [ ] Create NFT tickets collection

**Jun-July**
- [ ] EVM integration (Aurora)
- [ ] Launch on EVM

**Next steps**
- [ ] Aptos/Sui integrations

### Overview

- **Total Estimated Duration:** 6 month
- **Full-Time Equivalent (FTE):**  4 FTE
- **Total Costs:** 45 000 USD

### Milestone 1: Contesty core logic

- **Estimated duration:** 1 month
- **FTE:**  3
- **Costs:** 9,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0e. | Changelog | We will publish changelog with all features. Make twitter announce or write medium report.
| 1. | Public contest model | User can participating in public contest by paying entry fee. At the end of contest - user (or users) should receive prize pool minus platform fee (5%) 
| 2. | Private contest model (1-to-1)| User can participating in private contest using invited link. User can create invite link and invite his friend through Near Social (using their contract API).
| 3. | Predict contest winner| User can predict a winner by putting his bet in CNT token. He should receive reward or loose his stake in case win/loose.
| 4. | Level system| User can reach new level after he receive specific amount of XP (experience). He should get XP participating in contests, voting or making correct predictions. User can pay (using NEAR) to upgrade his level.

### Milestone 2. Quizzes, extended contests

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 9,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0e. | Changelog | We will publish changelog with all features. Make twitter announce or write medium report.
| 1. | Quiz model| User-creator can create quiz and set rules how to pass it with rewards pool.
| 2. | Recurrent and bracket-typed contests| User can participate in recurrent contests (daily/weekly/monthly) and bracked-typed (1/16, 1/8, ...final)
| 3. | Leaderboard| User can see clear and understandable leaderboard with detailed information and resource links.

### Milestone 3. Integrations, additional improvements

- **Estimated Duration:** 1 month
- **FTE:**  3
- **Costs:** 9,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0e. | Changelog | We will publish changelog with all features. Make twitter announce or write medium report.
| 1. | Near Social integration| Request friend from Near Social. Build near social plugin displaying leaderboard
| 2. | Recurrent and bracket-typed contests| User can participate in recurrent contests (daily/weekly/monthly) and bracked-typed (1/16, 1/8, ...final)
| 3. | Leaderboard| User can see clear and understandable leaderboard with detailed information and resource links.

### Milestone 4. Preparation for launch mainnet

- **Estimated Duration:** 1 month
- **FTE:**  3+1
- **Costs:** 5,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0e. | Changelog | We will publish changelog with all features. Make twitter announce or write medium report.
| 1. | QA work| test all functionality manually and runnong auto-test. Fixing bugs related to testnet
| 2. | Configure production environment| Dev-Ops works, deploying contracts, mint tokent and made initial sub-contract support
| 3. | Make NFT ticket collection| Sell tickets to support initial DEX liquidity.
| 4. | Next investements round| Cover dev work for next milestone and sell investor's token pool.
| 5. | Marketing program| Take a part on AMAs, make media promotions in twitter, discord. Involve influencers, bloggers, news channels

### Milestone 5. EVM (Aurora) integration

- **Estimated Duration:** 2 month
- **FTE:**  3
- **Costs:** 13,000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | Apache 2.0 / GPLv3 / MIT / Unlicense |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0e. | Changelog | We will publish changelog with all features. Make twitter announce or write medium report.
| 1. | QA work| test all functionality manually and runnong auto-test. Fixing bugs related to testnet
| 2. | Aurora integration| Translate all functionality onto solidity and support Aurora multichain.
| 3. | EVM launch| Prepare environment. Make marketing campaign. Launch process. Support initial dex liquidity

## Future Plans

We are going to scale into multichain solutions on other EVMs, Aptos.

## Additional Information :heavy_plus_sign:

We have business plan on 2023 and revenue model.
We also prepared tokenomic and pitch deck.

**How did you hear about the Grants Program?**
Medium
