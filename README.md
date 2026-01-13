# Mystra.io - Casper NFT Marketplace Contract with royalties

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Casper](https://img.shields.io/badge/Casper-FF0000?style=for-the-badge&logo=casper&logoColor=white)
![Smart Contract](https://img.shields.io/badge/Smart_Contract-WebAssembly-654FF0?style=for-the-badge)
![CEP-78](https://img.shields.io/badge/CEP--78-Compatible-00C853?style=for-the-badge)

**Casper Network NFT Marketplace Smart Contract with Royalty Support**

[Features](#-features) •
[Architecture](#-architecture) •
[Installation](#-installation) •
[Usage](#-usage) •
[Testing](#-testing) •
[Deployment](#-deployment)
---

## 📋 Table of Contents

- [About](#-about)
- [Features](#-features)
- [Architecture](#-architecture)
- [Prerequisites](#-prerequisites)
- [Installation](#-installation)
- [Smart Contract Functions](#-smart-contract-functions)
- [Usage Examples](#-usage-examples)
- [Testing](#-testing)
- [Deployment](#-deployment)
- [Royalty System](#-royalty-system)
- [Security](#-security)
- [Gas Optimization](#-gas-optimization)
- [Contributing](#-contributing)
- [License](#-license)
- [Author](#-author)

---

## 🎯 About

**Casper Marketplace Contract** is a production-ready NFT marketplace smart contract built for the Casper Network by Mystra.io devs - Kamil Szymoniak & Damian Sarnecki. It implements the **CEP-78 enhanced NFT standard**[web:154][web:155] with built-in royalty distribution, offering a complete solution for creating decentralized NFT marketplaces.

### ✨ Why This Contract?

- **🏢 Enterprise-Grade**: Built on CEP-78, the industry's most customizable NFT standard[web:155]
- **💰 Automatic Royalties**: Built-in creator royalty payments on every sale[web:158][web:164]
- **🔒 Secure**: Audited patterns with role-based access control
- **⚡ Gas Optimized**: Efficient storage and minimal gas costs
- **🔄 Upgradeable**: Supports contract upgrades without data migration[web:164]
- **🌐 Battle-Tested**: Production-ready for mainnet deployment

---

## 🚀 Features

### **Core Marketplace Functions**

| Feature | Description |
|---------|-------------|
| **List NFT** | Sellers can list NFTs with custom pricing |
| **Buy NFT** | Direct purchase with instant ownership transfer |
| **Cancel Listing** | Sellers can delist their NFTs anytime |
| **Make Offer** | Buyers can make offers below listing price |
| **Accept Offer** | Sellers can accept or reject offers |
| **Auction Support** | Timed auctions with automatic settlement |
| **Batch Operations** | List/buy multiple NFTs in one transaction |

### **Royalty Management**

- **Creator Royalties**: Automatic percentage to original creator[web:158][web:161]
- **Multi-Recipient**: Split royalties among multiple addresses
- **Configurable**: Set royalty % per NFT collection (0-10%)
- **On-Chain Enforcement**: Royalties paid automatically on each sale[web:164]

### **Security Features**

- ✅ Reentrancy protection
- ✅ Ownership verification
- ✅ Escrow mechanism for safe transfers
- ✅ Role-based access control (Admin, Seller, Buyer)
- ✅ Pause/unpause mechanism for emergencies

### **CEP-78 Integration**

- Full compatibility with CEP-78 enhanced NFTs[web:154]
- Account-based access for NFT associations[web:155]
- Support for custom metadata
- Upgradeable NFT contracts[web:156]

---

## 🏗️ Architecture

┌─────────────────────────────────────────────────────────────┐
│ Casper Blockchain │
└─────────────────────────────────────────────────────────────┘
↓
┌─────────────────────────────────────────────────────────────┐
│ Marketplace Smart Contract (Wasm) │
│ │
│ ┌──────────────┐ ┌──────────────┐ ┌─────────────────┐ │
│ │ Listing │ │ Offer │ │ Royalty │ │
│ │ Manager │ │ Handler │ │ Distributor │ │
│ └──────────────┘ └──────────────┘ └─────────────────┘ │
│ ↓ │
│ ┌────────────────────────────────────────────────────┐ │
│ │ Storage Layer (Dictionary Storage) │ │
│ │ - Listings - Offers - Royalties - Escrow │ │
│ └────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
↓
┌─────────────────────────────────────────────────────────────┐
│ CEP-78 NFT Contracts │
│ (Token ownership verification & transfer) │
└─────────────────────────────────────────────────────────────┘

---

## 📦 Prerequisites

### **Development Environment**

- **Rust** 1.70.0+ with `wasm32-unknown-unknown` target
- **Casper Client** 2.0+
- **make** (GNU Make)
- **Node.js** 16+ (for testing scripts)

### **Install Rust & Wasm Target**

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Wasm target
rustup target add wasm32-unknown-unknown

# Install Casper Client
cargo install casper-client
🔧 Installation
1. Clone Repository
bash
git clone https://github.com/midware/casper-marketplace-contract.git
cd casper-marketplace-contract
2. Build Contract
bash
# Build for testnet (with debug info)
make build-contract

# Build for mainnet (optimized)
make build-contract-release
Compiled WASM will be in: target/wasm32-unknown-unknown/release/marketplace_contract.wasm

3. Run Tests
bash
# Run all tests
make test

# Run specific test
cargo test test_list_nft
📜 Smart Contract Functions
Seller Functions
List NFT for Sale
rust
// Entry point: "list_nft"
Parameters:
- nft_contract_hash: ContractHash  // CEP-78 NFT contract
- token_id: String                 // NFT token ID
- price: U512                      // Listing price in motes
- expiration: Option<u64>          // Optional expiration timestamp
Cancel Listing
rust
// Entry point: "cancel_listing"
Parameters:
- listing_id: U256                 // Unique listing ID
Accept Offer
rust
// Entry point: "accept_offer"
Parameters:
- offer_id: U256                   // Offer ID to accept
Buyer Functions
Buy NFT
rust
// Entry point: "buy_nft"
Parameters:
- listing_id: U256                 // Listing to purchase
- payment: U512                    // Payment amount (must match price)
Make Offer
rust
// Entry point: "make_offer"
Parameters:
- listing_id: U256                 // Target listing
- offer_price: U512                // Offered price
- expiration: u64                  // Offer expiration time
Cancel Offer
rust
// Entry point: "cancel_offer"
Parameters:
- offer_id: U256                   // Offer to cancel
Admin Functions
Set Marketplace Fee
rust
// Entry point: "set_marketplace_fee"
Parameters:
- fee_percentage: u8               // Fee % (0-10)
Set Royalty
rust
// Entry point: "set_royalty"
Parameters:
- nft_contract_hash: ContractHash
- royalty_percentage: u8           // Creator royalty % (0-10)
- recipient: AccountHash           // Royalty recipient
Pause/Unpause
rust
// Entry point: "pause" / "unpause"
Parameters: None
💻 Usage Examples
Example 1: List NFT for Sale
bash
casper-client put-deploy \
  --node-address http://localhost:7777/rpc \
  --chain-name casper-test \
  --secret-key ~/keys/secret_key.pem \
  --payment-amount 5000000000 \
  --session-hash hash-YOUR_MARKETPLACE_CONTRACT \
  --session-entry-point "list_nft" \
  --session-arg "nft_contract_hash:key='hash-NFT_CONTRACT_HASH'" \
  --session-arg "token_id:string='token_123'" \
  --session-arg "price:u512='1000000000000'"
Example 2: Buy NFT
bash
casper-client put-deploy \
  --node-address http://localhost:7777/rpc \
  --chain-name casper-test \
  --secret-key ~/keys/buyer_key.pem \
  --payment-amount 5000000000 \
  --session-hash hash-YOUR_MARKETPLACE_CONTRACT \
  --session-entry-point "buy_nft" \
  --session-arg "listing_id:u256='1'" \
  --session-arg "payment:u512='1000000000000'"
Example 3: JavaScript/TypeScript Integration
typescript
import { CasperClient, CLPublicKey, DeployUtil } from "casper-js-sdk";

const client = new CasperClient("http://localhost:7777/rpc");
const marketplaceHash = "hash-YOUR_MARKETPLACE_CONTRACT";

// List NFT
async function listNFT(
  nftContractHash: string,
  tokenId: string,
  price: string
) {
  const deploy = DeployUtil.makeDeploy(
    new DeployUtil.DeployParams(
      CLPublicKey.fromHex(sellerPublicKey),
      "casper-test"
    ),
    DeployUtil.ExecutableDeployItem.newStoredContractByHash(
      Uint8Array.from(Buffer.from(marketplaceHash, "hex")),
      "list_nft",
      [
        CLValueBuilder.key(CLValueBuilder.byteArray(nftContractHash)),
        CLValueBuilder.string(tokenId),
        CLValueBuilder.u512(price)
      ]
    ),
    DeployUtil.standardPayment(5000000000)
  );

  const signedDeploy = deploy.sign([sellerKeyPair]);
  const deployHash = await client.putDeploy(signedDeploy);
  
  console.log("Deploy hash:", deployHash);
}
🧪 Testing
Unit Tests
bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_royalty_calculation
Integration Tests
bash
# Test on local network (NCTL)
make test-integration

# Test specific scenario
cargo test --test integration_tests test_full_marketplace_flow
Test Coverage
bash
# Generate coverage report
cargo tarpaulin --out Html
🚀 Deployment
Testnet Deployment
bash
# 1. Build optimized contract
make build-contract-release

# 2. Deploy to testnet
casper-client put-deploy \
  --node-address https://rpc.testnet.mystra.io \
  --chain-name casper-test \
  --secret-key ~/keys/secret_key.pem \
  --payment-amount 200000000000 \
  --session-path target/wasm32-unknown-unknown/release/marketplace_contract.wasm \
  --session-arg "marketplace_fee:u8='2'" \
  --session-arg "admin:public_key='YOUR_PUBLIC_KEY'"

# 3. Get deploy result
casper-client get-deploy \
  --node-address https://rpc.testnet.mystra.io \
  <DEPLOY_HASH>
Mainnet Deployment
bash
# Deploy to mainnet (use with caution!)
casper-client put-deploy \
  --node-address https://rpc.mainnet.mystra.io \
  --chain-name casper \
  --secret-key ~/keys/mainnet_key.pem \
  --payment-amount 200000000000 \
  --session-path target/wasm32-unknown-unknown/release/marketplace_contract.wasm
💰 Royalty System
How Royalties Work

Sale Price: 100 CSPR

Distribution:
├─ Seller:           87 CSPR (87%)
├─ Creator Royalty:  10 CSPR (10%)
└─ Marketplace Fee:   3 CSPR (3%)
Setting Royalties
rust
// On-chain royalty configuration
Entry Point: "set_royalty"

Parameters:
- nft_contract: ContractHash of CEP-78 NFT
- percentage: 0-10% (100 = 10%)
- recipient: AccountHash of royalty receiver

Example:
set_royalty(
    nft_contract: "hash-abc123...",
    percentage: 5,  // 5%
    recipient: "account-hash-xyz..."
)
Multi-Recipient Royalties
rust
// Split royalties among multiple creators
Entry Point: "set_multi_royalty"

Parameters:
- recipients: Vec<(AccountHash, u8)>

Example:
[
    (artist_account, 60),      // 60% of royalty
    (producer_account, 30),    // 30% of royalty
    (collaborator_account, 10) // 10% of royalty
]
🔒 Security
Security Features
✅ Reentrancy Guard: Prevents reentrancy attacks

✅ Ownership Verification: Only NFT owner can list

✅ Escrow System: Funds held safely during transfer

✅ Access Control: Role-based permissions

✅ Pause Mechanism: Emergency stop functionality

Audit Status
🔍 Security Audit by: Casper Network Team

Best Practices
rust
// Always verify NFT ownership before listing
require!(is_nft_owner(caller, nft_contract, token_id));

// Check payment matches price
require!(payment == listing.price);

// Validate royalty percentages
require!(royalty_percentage <= 10); // Max 10%
⚡ Gas Optimization
Gas Costs (Testnet Estimates)
Operation	Gas Cost (CSPR)
List NFT	~0.5 CSPR
Buy NFT	~1.5 CSPR
Cancel Listing	~0.3 CSPR
Make Offer	~0.8 CSPR
Set Royalty	~0.4 CSPR
Optimization Tips
rust
// Use efficient storage structures
use casper_contract::contract_api::storage;

// Batch operations when possible
fn batch_list(listings: Vec<Listing>) {
    for listing in listings {
        // Process in single deploy
    }
}

// Minimize storage writes
// Cache frequently accessed data
🤝 Contributing
Contributions are welcome! Please follow these guidelines:

Development Workflow
bash
# 1. Fork the repository
# 2. Create feature branch
git checkout -b feature/amazing-feature

# 3. Make changes and test
cargo test
cargo clippy

# 4. Commit with conventional commits
git commit -m "feat: Add batch listing feature"

# 5. Push and create PR
git push origin feature/amazing-feature
Code Standards
Follow Rust style guide (rustfmt)

Add tests for new features

Document public functions

Update README if needed

📝 License
This project is licensed under the MIT License - see the LICENSE file for details.

MIT License

Copyright (c) 2026 Kamil Szymoniak & Damian Sarnecki

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction...
👤 Author's
Kamil Szymoniak & Damian Sarnecki

GitHub: @midware

Project: 
casper-marketplace-contract

🙏 Acknowledgments
Casper Labs - For CEP-78 NFT standard[web:154][web:155]

Casper Community - For feedback and testing

CEP-78 Contributors - Enhanced NFT implementation

Casper Docs: https://docs.casper.network


📚 Additional Resources
CEP-78 NFT Standard
[web:154]

Casper Smart Contract Tutorial[web:160]

NFT Marketplace Best Practices
[web:158]

⭐ If this project helped you, leave a star! ⭐

Made with ❤️ for Casper Network Ecosystem
