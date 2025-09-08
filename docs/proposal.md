# P2P Energy Trading Platform: Smart Contract System Proposal

## 1. Introduction

This document outlines the architecture and design of the smart contract system for the P2P Energy Trading Platform. The system is built on a modular, multi-contract architecture, ensuring separation of concerns, security, and upgradeability. It consists of four core contracts that work in concert to facilitate a decentralized energy marketplace.

The core components are:
- **Registry Contract**: The single source of truth for identity and asset management.
- **GridToken (GRID) Contract**: A PSP22-compliant fungible token for value exchange.
- **Trading Contract**: The decentralized marketplace for energy trading.
- **Oracle Client Contract**: A secure bridge to off-chain data and automation.

This proposal details the purpose, features, and key functions of each contract.

---

## 2. Registry Contract (`registry`)

The Registry contract is the foundation of the system, managing the identities of all participants and their associated assets (smart meters).

### 2.1. Purpose

- To register and verify all users (Prosumers and Consumers).
- To manage the assignment of smart meters to users.
- To maintain a list of authorized REC (Renewable Energy Certificate) Regulators who can administer the registry.

### 2.2. Key Features

- **Role-Based Access Control**: Only authorized REC Regulators can modify the registry.
- **User and Asset Management**: Provides a canonical record of all participants and their devices.
- **Status Tracking**: Manages the status of users (Active, Suspended, Deactivated).

### 2.3. Core Data Structures

- **`UserInfo`**: Stores details about a user, including their type (`Prosumer`/`Consumer`), location, and status.
- **`UserType`**: Enum representing user capabilities:
  - `Prosumer`: Can both generate and consume energy (recommended for most users)
  - `Consumer`: Can only consume energy (for users without generation capabilities)
- **`UserStatus`**: Enum for the current state of a user account.

**Design Note**: In practice, most users should be registered as `Prosumer` type since they may install solar panels in the future. The `Consumer` type is primarily for users who will never have energy generation capabilities (e.g., renters).

### 2.4. Key Functions (Messages)

- `register_user(user, user_type, location)`: Registers a new user. (Regulator only)
  - **Best Practice**: Register most users as `Prosumer` for flexibility
- `assign_meter(meter_id, owner)`: Links a smart meter to a registered user. (Regulator only)
- `update_user_status(user, new_status)`: Changes a user's status. (Regulator only)
- `update_user_type(user, new_type)`: Allows upgrading Consumer → Prosumer. (Regulator only)
- `add_rec_regulator(new_regulator)`: Adds a new administrator. (Regulator only)
- `is_user_verified(user)`: Publicly checks if a user is registered and active.
- `is_prosumer(user)`: Checks if a user can create sell orders.
- `get_meter_owner(meter_id)`: Returns the owner of a specific meter.

### 2.5. Events

- `UserRegistered`: Emitted when a new user is added.
- `MeterAssigned`: Emitted when a meter is linked to a user.
- `UserStatusUpdated`: Emitted on user status changes.
- `RecRegulatorAdded`: Emitted when a new regulator is authorized.

---

## 3. GridToken Contract (`grid-token`)

The GridToken (GRID) contract implements the platform's native utility token, used for all economic activities.

### 3.1. Purpose

- To provide a PSP22-compliant fungible token for settling energy trades.
- To represent generated energy, where **1 GRID = 1 kWh**.
- To manage the minting (creation) and burning (destruction) of tokens in a controlled manner.

### 3.2. Key Features

- **PSP22 Standard**: Fully compatible with the Polkadot standard for fungible tokens, ensuring interoperability.
- **Controlled Supply**: Only authorized "Minter" accounts (like the Oracle Client) can create new tokens, and only authorized "Burner" accounts can destroy them.
- **Registry Integration**: Can be configured to check the Registry contract to ensure tokens are only minted for verified users.

### 3.3. Core Data Structures

- The contract primarily uses `ink::storage::Mapping` to store `balances`, `allowances`, `minters`, and `burners`.

### 3.4. Key Functions (Messages)

- `mint(to, amount, meter_id)`: Creates new tokens and assigns them to a user, typically after verifying energy generation data. (Minter only)
- `burn(from, amount)`: Destroys tokens from a user's balance, representing energy consumption. (Burner only)
- `transfer(to, value, data)`: Standard PSP22 function to transfer tokens between users.
- `approve(spender, value)`: Standard PSP22 function to approve a third party (like the Trading contract) to spend tokens on a user's behalf.
- `add_minter(minter)` / `add_burner(burner)`: Authorizes new accounts to mint or burn tokens. (Owner/Admin only)
- `set_registry_contract(registry_contract)`: Links the token to the Registry for user verification.

### 3.5. Events

- `Minted`: Emitted when new tokens are created.
- `Burned`: Emitted when tokens are destroyed.
- `MinterAdded` / `MinterRemoved`: Emitted on changes to the minter list.
- Standard PSP22 events like `Transfer` and `Approval`.

---

## 4. Trading Contract (`trading`)

The Trading contract is the heart of the marketplace, where users can post buy and sell orders for energy.

### 4.1. Purpose

- To provide a decentralized order book for P2P energy trading.
- To facilitate the matching of buy and sell orders based on price and time priority.
- To settle trades by orchestrating token transfers.

### 4.2. Key Features

- **Periodic Order Book**: Operates in market epochs (e.g., 15 minutes), where orders are collected and then matched in a batch.
- **Automated Clearing**: A `match_orders` function, callable by authorized "Market Makers" (like the Oracle Client), executes the matching algorithm.
- **User-Centric Orders**: Users can create buy and sell orders and cancel their own active orders.

### 4.3. Core Data Structures

- **`Order`**: Contains all details of a buy or sell order, including user, type, amount, price, and status.
- **`Trade`**: Records the details of a successfully executed trade between a buyer and a seller.
- **`OrderType`**: Enum for `Buy` or `Sell`.
- **`OrderStatus`**: Enum for the state of an order (`Active`, `Filled`, `Cancelled`, etc.).

### 4.4. Key Functions (Messages)

- `create_sell_order(energy_amount, price_per_unit)`: Allows a verified prosumer to list energy for sale.
- `create_buy_order(energy_amount, max_price)`: Allows a verified user to place an order to buy energy.
- `cancel_order(order_id)`: Allows a user to cancel their own unfilled order.
- `match_orders()`: Triggers the order matching and trade settlement process. (Market Maker only)
- `add_market_maker(market_maker)`: Authorizes a new account to trigger market clearing. (Owner/Admin only)
- `get_order(order_id)`: Publicly retrieves the details of a specific order.

### 4.5. Events

- `SellOrderCreated` / `BuyOrderCreated`: Emitted when new orders are placed.
- `OrderCancelled`: Emitted when an order is cancelled.
- `TradeExecuted`: Emitted for each successful trade, providing a verifiable record.
- `MarketCleared`: Emitted after the matching process for an epoch is complete.

---

## 5. Oracle Client Contract (`oracle-client`)

The Oracle Client contract serves as the secure and reliable bridge between the on-chain smart contracts and off-chain systems.

### 5.1. Purpose

- To enable smart contracts to request data from the outside world (e.g., AMI for meter readings).
- To allow external automation services (like Chainlink Keepers) to trigger on-chain functions.
- To manage the lifecycle of data requests and responses from trusted oracle operators.

### 5.2. Key Features

- **Request-Response Model**: Manages data requests and their fulfillment by authorized oracles.
- **Upkeep Automation**: Provides an interface (`check_upkeep`, `perform_upkeep`) compatible with automation networks to trigger periodic tasks like market clearing.
- **Contract Integration**: Acts as a central coordinator, making calls to the `grid-token` contract (to mint) and the `trading` contract (to clear the market).

### 5.3. Core Data Structures

- **`OracleRequest`**: Tracks a data request's ID, requester, type, and status.
- **`RequestType`**: Enum defining the kind of data being requested (e.g., `EnergyData`, `MarketClearing`).
- **`RequestStatus`**: Enum for the state of a request (`Pending`, `Fulfilled`, `Expired`).
- **`MeterData`**: A structured format for receiving energy data from an oracle.

### 5.4. Key Functions (Messages)

- `request_energy_data(meter_id)`: Allows a user to request the latest reading for their meter.
- `fulfill_energy_data(request_id, meter_data)`: Allows an oracle operator to submit data, which triggers token minting. (Oracle Operator only)
- `check_upkeep()`: A view function used by automation networks to see if a task (like market clearing) needs to be performed.
- `perform_upkeep()`: The function called by an automation network to execute a task. (Oracle Operator only)
- `add_oracle_operator(operator)`: Authorizes a new oracle node. (Owner/Admin only)

### 5.5. Events

- `OracleRequestCreated`: Emitted when a new data request is made.
- `EnergyDataFulfilled`: Emitted when an oracle provides data, confirming the action taken (e.g., tokens minted).
- `MarketClearingChecked`: Emitted when the market clearing condition is checked.
- `AutoMarketClearingTriggered`: Emitted when the contract automatically triggers a market clear.

---

## 6. Conclusion

This four-contract system provides a robust, secure, and decentralized foundation for the P2P Energy Trading Platform. By separating concerns, each contract can focus on its core responsibility, leading to simpler, more auditable code. The interactions between the contracts are well-defined, enabling complex workflows like automated token minting from verified energy data and periodic market clearing. This architecture is designed for both immediate functionality and future extensibility.
