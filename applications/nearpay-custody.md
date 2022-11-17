# Mintbase Grant Proposal

- **Project Name:** NearPay Custody
- **Team Name:** NearPay
- **Payment Address:** kikimora.sputnikdao.near
- **[Level](../README.md#level_slider-levels):** 2

## Project Overview :page_facing_up:

NearPay Custody service is helping merchants to have a complex custodial solution for NEAR Protocol blockchain. 
It provides opportunity to create, securely store, and process customer wallets with [like a fireblocks](https://www.fireblocks.com/blog/hot-vs-warm-vs-cold-which-crypto-wallet-is-right-for-me/) hot-warm-cold wallets strategy.

### Overview

The Project is already written to work with [NearPay](https://nearpay.co/), but we decided to modify it to be used by any another merchants. 
Our results for today:
- ~30k wallets
- ~50k NEAR common turnover.

Service provides secured GRPC interface, wallets are stored in Hashicorp Vault. 
Project has it's own indexer, which scans blockchain to find new deposits to customer wallets. Periodically automatically tokens from customer wallets are transferred to warm wallet. 
In addition, there is a possibility to notify merchant about new deposit to customer wallet.
Project is developed with Java & Spring technological stack, and ready to use in kubernetes environment. 

Our advantage over [fireblocks](https://www.fireblocks.com/):
- cheaper
- can flexible integrate with mintbase or [roketo](https://roke.to/) because of good integration with NEAR ecosystem

### Project Details

<details>
  <summary>GRPC Interface</summary>

```protobuf
syntax = "proto3";

package co.nearpay;

service NearpayCustody {

    // Hot wallet methods
    rpc CreateTransfer(CreateTransferRequest) returns (Transfer);
    rpc GetTransfer(GetTransferRequest) returns (Transfer);
    rpc EstimateNetworkFee(EstimateNetworkFeeRequest) returns (NetworkFeeEstimation);

    // Warm wallet methods
    rpc CreateAddress(Empty) returns (Address);
    rpc ListDeposits(ListDepositsRequest) returns (ListDepositsResponse);
    rpc ListDepositsV2(ListDepositsV2Request) returns (ListDepositsV2Response);

    // Management methods
    rpc GetBalances(Empty) returns (WalletBalances);
    rpc FreezeWallet(FreezeWalletRequest) returns (Success);
    rpc ListFungibleTokens(Empty) returns (ListFungibleTokensResponse);
}

message Empty {}

message CreateTransferRequest {
    string amount = 1; // required transfer amount
    string address = 2; // required destination address
    string currency = 3; // NEAR or some NEP-141 token. Default is NEAR
}

message GetTransferRequest {
    string txid = 1; // required transaction hash
}

message EstimateNetworkFeeRequest {
    string currency = 1; // NEAR or some NEP-141 token. NEAR by default
}

message ListDepositsRequest {
    string txid = 1; // required one of: this, or destination_address
    string destination_address = 2; // required one of: this, or txid
    int64 start_timestamp = 3; // timestamp of the first transaction for offset. No offset if null or zero. Deposits sorted ascending by timestamp (oldest first)
    int32 limit = 4; // maximum count of transactions in response, zero is unlimited, default is 10
}

message ListDepositsV2Request {
    repeated Receiver receivers = 1; // required receivers list
}

message FreezeWalletRequest {
    bool freeze = 1;
    WalletType walletType = 2;
}

message Receiver {
    string id = 1; // required
    int64 earliest_timestamp = 2; // timestamp of the first transaction for offset. No offset if null or zero. Deposits sorted ascending by timestamp (oldest first)
    int32 limit = 3; // maximum count of transactions in response, zero is unlimited, default is 10
    repeated string currencies = 4; // all currencies if empty. Returns list of transactions for NEAR or specified NEP-141 currencies
}

message Transfer {
    string amount = 1;
    string address = 2; // destination address
    string txid = 3; // transaction hash
    DepositStatus status = 4; // transaction status
    string network_fee = 5;
    string raw = 6; // raw data from blockchain
    int64 block_timestamp = 7; // transaction timestamp
    string currency = 8; // NEAR or some NEP-141 token
}

message NetworkFeeEstimation {
    string amount = 1;
}

message Address {
    string address = 1; // Hex representation of created address
}

message ListDepositsResponse {
    repeated Deposit deposits = 1;
}

message ListDepositsV2Response {
    repeated ReceiverToDeposits receiver_to_deposits = 1;
}

message ReceiverToDeposits {
    string destination_address = 1;
    repeated Deposit deposits = 2;
}

message Deposit {
    string txid = 1; // transaction hash
    string amount = 2; // deposit amount
    DepositStatus status = 3; // transaction status
    string source_address = 4;
    string destination_address = 5;
    int64 block_timestamp = 6; // transaction timestamp
    string currency = 8; // NEAR or some NEP-141 token
}

enum DepositStatus {
    UNKNOWN = 0;
    FAILURE = 1;
    SUCCESS_VALUE = 2;
    SUCCESS_RECEIPT_ID = 3;
}

message WalletBalances {
    WalletBalance hot = 1;
    WalletBalance warm = 2;
}

message WalletBalance {
    string address = 1;
    string balance = 2;
    WalletType walletType = 3;
}

enum WalletType {
    HOT = 0;
    WARM = 1;
}

message ListFungibleTokensResponse {
    repeated FungibleToken token = 1;
}

message FungibleToken {
    string name = 1;
}

message Success {
    bool success = 1;
}

// callback events
message Event {
    oneof data {
        TransferEvent transfer_changed = 1;
        DepositEvent deposit_changed = 2;
    }
}

message TransferEvent {
    string address = 1;
    string txid = 2;
    string status = 3;
    string currency = 4; // NEAR or some NEP-141 token
}

message DepositEvent {
    string address = 1;
    string txid = 2;
    string status = 3;
    string currency = 4; // NEAR or some NEP-141 token
}
```
  
</details>

- Used Spring boot to easily assemble production-ready applications. It provides opportunity to use 3rd-party libraries like a already configured bricks, which means faster development with less proprietary code. 
- Modified spring batch used for all periodic tasks, like wallets scan for new deposits, or new transactions scan in blockchain. It has a lot of embedded useful components for that.

Our service is already used by [NearPay](https://nearpay.co/) for NEAR cryptocurrency.

## Team :busts_in_silhouette:

### Team members

- Kirill Arutyunov - CTO
- Roman Molchanov - Lead developer
- Ilya Romanov - Adviser / Marketing consultant

### Contact

- **Contact Name:** Roman Molchanov
- **Contact Email:** romanmolchanov@nearpay.co
- **Website:** [nearpay.co](https://nearpay.co/)

### Team's experience

We developed NearPay Custody from scratch. Now it works for [nearpay.co](https://nearpay.co/).

### Team Code Repos

Now a code is not open-sourced, it's located inside private git repositories. 

### Team LinkedIn Profiles (if available)

- https://www.linkedin.com/in/kirillarutyunov/
- https://www.linkedin.com/in/roman-molchanov-57532093/
- https://www.linkedin.com/in/ilyshka/

## Development Roadmap :nut_and_bolt:

### Overview

- **Total Estimated Duration:** 3 Months
- **Full-Time Equivalent (FTE):** 2,33 FTE
- **Total Costs:** 50000 USD

### Milestone 1: Code & infrastructure adaptation

- **Estimated duration:** 1 month
- **FTE:** 2
- **Costs:** 15000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | GPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a merchant can use custodial service, and show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains benefits of usage custodial service, and how it can be used. |
| 1. | Code make up | We will bring our current code base in a state, applicable to move into github repository |  
| 2. | SAAS | We will make project usable by any external merchants |
| 3. | Infrastructure | We will make required manifests/charts to deploy applications into kubernetes environment |
| 4. | CI/CD | We will adapt CD process to new infrastracture |
| 5. | Project integrations | Applications of our project will get information from indexer-application, open NEAR RPC node, vault, NEAR public S3 indexer repo, and will not interact between themselves |  

### Milestone 2: Production environment preparations and marketing

- **Estimated Duration:** 1 month
- **FTE:** 3
- **Costs:** 20000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | GPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a merchant can use custodial service, and show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains benefits of usage custodial service, and how it can be used. |
| 1. | Rate-limiter | We will implement rate-limiting feature to limit application load |
| 2. | Vault instance | We will add and configure Hashicorp Vault instance to our environment to store customer's wallets there |
| 3. | RabbitMQ instance | We will add and configure RabbitMQ instance to our environment to provide callbacks functionality if deposit received to a customer's wallet |
| 4. | Marketing | We will place advertise and sell service to potential merchants |
| 5. | Production deploy | We will deliver and configure project to production environment |

### Milestone 3: Support and merchants adaptation

- **Estimated Duration:** 1 month
- **FTE:** 2
- **Costs:** 15000 USD

| Number | Deliverable | Specification |
| -----: | ----------- | ------------- |
| 0a. | License | GPLv3 |
| 0b. | Documentation | We will provide both **inline documentation** of the code and a basic **tutorial** that explains how a merchant can use custodial service, and show how the new functionality works. |
| 0c. | Testing Guide | Core functions will be fully covered by unit tests to ensure functionality and robustness. In the guide, we will describe how to run these tests. |
| 0d. | Docker | We will provide a Dockerfile(s) that can be used to test all the functionality delivered with this milestone. |
| 0e. | Article | We will publish an **article**/workshop that explains benefits of usage custodial service, and how it can be used. |
| 1. | Merchants connect | We will support merchants to how to use our service, and write guides and FAQs on the way |
| 2. | Metrics | We will make affordable for merchants usage metrics |
| 3. | Hot-improvements | We will make some immediate improvements by requests from our merchants |


## Future Plans

- Implement 2-of-3 multisig strategy to make transfers
- Staking support
- Indexer improvements to store data in ClickHouse, to easily check supposes about sevice usage 
