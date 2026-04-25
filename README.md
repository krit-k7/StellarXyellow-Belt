# Stellar Crowdfund - Yellow Belt Submission

A decentralized crowdfunding platform built on the Stellar network with multi-wallet integration, smart contract deployment, and real-time event handling.

![Stellar Crowdfund](https://img.shields.io/badge/Stellar-Testnet-blue)
![Soroban](https://img.shields.io/badge/Soroban-Smart%20Contract-green)

## 🚀 Live Demo

**Live Application**: [Stellar Crowdfund on Render](https://stellar-yellowbelt.onrender.com) ✅ **LIVE**

**GitHub Repository**: 

## Features

- **Multi-Wallet Support** - Connect via Freighter, Albedo, or xBull wallets
- **Smart Contract** - Soroban crowdfunding contract deployed on Stellar Testnet
- **Campaign Management** - Create campaigns with title, description, target amount, and deadline
- **Real XLM Transfers** - Donations send actual XLM to campaign creators via payment operations
- **Target Amount Validation** - Prevents donations when campaign reaches target amount
- **Transaction Status** - Visual tracking of pending/success/fail states with explorer links
- **Real-time Updates** - Live balance and campaign progress updates
- **Error Handling** - 3 error types: Wallet Not Found, Transaction Rejected, Insufficient Balance
- **Testnet Friendbot** - One-click account funding for testing

## Screenshots

### Wallet Options Available
When clicking "Connect Wallet", users see three wallet options:

![Multiple Wallet Options](/public/multiple_wallets.jpeg)

- 🚀 **Freighter** - Browser extension wallet
- 🌟 **Albedo** - Web-based wallet  
- 🐂 **xBull** - Multi-platform wallet

### Transaction Status Tracking
Transactions show real-time status: Pending → Success/Fail with explorer links.

## Deployed Contract

- **Contract Address**: `CCEWBXDQJ2YHQ6NVRQW3OLAJ6MGH2FSDSEQW6L4GSEUPZQRLIFK3UW3F`
- **Network**: Stellar Testnet
- **Explorer**: [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCEWBXDQJ2YHQ6NVRQW3OLAJ6MGH2FSDSEQW6L4GSEUPZQRLIFK3UW3F)

### Contract Initialization Transaction
- **Transaction Hash**: `1c0171b55172e5699e5ac4553cc312578273a5ffebb2a826fe18a5188d354c95`
- **Verify on Explorer**: [View Transaction](https://stellar.expert/explorer/testnet/tx/1c0171b55172e5699e5ac4553cc312578273a5ffebb2a826fe18a5188d354c95)

## How It Works

### Donation Flow (Real XLM Transfer)
1. User enters donation amount and clicks "Donate"
2. **Transaction 1**: Payment operation sends XLM to campaign creator
3. **Transaction 2**: Contract records donation on-chain
4. Both transactions must be approved in Freighter wallet
5. Balance updates automatically after ~8 seconds

### Target Amount Validation
- Frontend checks if campaign has reached target before allowing donation
- Contract enforces target limit on-chain
- Smart error messages show remaining amount needed

## Tech Stack

- **Frontend**: Next.js 14, TypeScript, TailwindCSS
- **Blockchain**: Stellar Testnet, Soroban Smart Contracts
- **Wallets**: Freighter, Albedo, xBull
- **SDK**: @stellar/stellar-sdk
- **UI**: Lucide Icons, Sonner (toast notifications)

## Error Handling

| Error Type | Trigger | User Feedback |
|---|---|---|
| `WalletNotFoundError` | Wallet extension not installed | Toast with install link |
| `TransactionRejectedError` | User declines in wallet | Toast notification |
| `InsufficientBalanceError` | Not enough XLM balance | Toast with required vs available |

## Setup Instructions

### Prerequisites
- Node.js 18+
- A Stellar wallet extension (Freighter recommended)

### Installation

```bash
# Clone the repository
git clone <your-repo-url>
cd stellar-crowdfund

# Install dependencies
npm install

# Run development server
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser.

### Smart Contract (Soroban)

The contract source is in `contracts/crowdfund/src/lib.rs` with tests in `contracts/crowdfund/src/test.rs`.

To build and deploy the contract yourself:

```bash
# Install Soroban CLI
cargo install soroban-cli

# Build the contract
cd contracts/crowdfund
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Deploy to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/crowdfund.wasm \
  --network testnet \
  --source deployer

# Initialize the contract
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  --source deployer \
  -- init
```

### Contract Functions

- `init()` - Initialize the contract
- `create(creator, title, desc, target, deadline)` - Create a new campaign
- `donate(donor, campaign_id, amount)` - Donate to a campaign (with target validation)
- `get_campaign(campaign_id)` - Get campaign details
- `get_count()` - Get total number of campaigns
- `claim(campaign_id)` - Claim funds after campaign ends (creator only)

## Project Structure

```
stellar-crowdfund/
├── src/
│   ├── app/
│   │   ├── layout.tsx          # Root layout with Toaster
│   │   ├── page.tsx            # Main page component
│   │   └── globals.css         # Global styles + Tailwind
│   ├── components/
│   │   ├── WalletConnect.tsx   # Multi-wallet connection modal
│   │   ├── CampaignCard.tsx    # Campaign display + donate
│   │   ├── CreateCampaign.tsx  # Campaign creation form
│   │   ├── EventFeed.tsx       # Real-time event feed
│   │   └── TransactionStatus.tsx # TX status indicator
│   └── lib/
│       ├── constants.ts        # Network config, contract ID
│       ├── errors.ts           # Custom error classes (3 types)
│       ├── wallet.ts           # Multi-wallet integration
│       └── contract.ts         # Soroban contract interactions
├── contracts/
│   └── crowdfund/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # Soroban smart contract
│           └── test.rs         # Contract unit tests
├── package.json
├── tailwind.config.ts
└── README.md
```

## Requirements Checklist

- [x] 3 error types handled (WalletNotFound, TransactionRejected, InsufficientBalance)
- [x] Contract deployed on testnet with test.rs file
- [x] Contract called from the frontend (create campaign, donate)
- [x] Transaction status visible (pending/success/fail with explorer links)
- [x] Real XLM transfers via payment operations
- [x] Target amount validation (frontend + contract)
- [x] Multiple meaningful commits with proper git history
- [x] Multi-wallet app with deployed contract and real-time updates

## License

MIT
