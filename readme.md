# 🚀 Solana Fork Simulation Engine

A powerful Solana network fork simulation engine that allows developers to test dApps and protocols in isolated environments with real mainnet state. Built with Rust and LiteSVM.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🌟 Features

- **🔗 Mainnet State Forking** - Create isolated forks synced with the latest Solana mainnet header
- **👤 Multi-User Isolation** - Each user gets their own independent fork environment
- **⏰ Auto-Cleanup** - Forks automatically expire after 15 minutes to manage resources
- **🔌 JSON-RPC Compatible** - Standard Solana RPC endpoint for wallet and dApp integration
- **💰 Balance Manipulation** - Update SOL and SPL token balances for testing
- **📦 Account Loading** - Load real accounts and token accounts from mainnet
- **🔄 Transaction Support** - Full transaction execution with proper state updates
- **🎯 Program Deployment** - Deploy and test custom Solana programs
- **🛡️ Fault Isolation** - Failures in one fork don't affect others

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [API Documentation](#api-documentation)
- [Usage Examples](#usage-examples)
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [License](#license)

## 🔧 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/solana-fork-engine.git
cd solana-fork-engine

# Build the project
cargo build --release

# Run the server
cargo run --release
```

The server will start on `http://0.0.0.0:8899`

## 🚀 Quick Start

### 1. Create a Fork

Create a new fork synced with mainnet and load some accounts:

```bash
curl -X POST http://localhost:8899/api/v1/forks/mainnet \
  -H "Content-Type: application/json" \
  -d '{
    "accounts": ["11111111111111111111111111111111"]
  }'
```

**Response:**

```json
{
  "fork_id": "fork-abc123-1234567890",
  "user_id": "abc123-def456-ghi789",
  "created_at": "1234567890",
  "expires_at": "1234568790",
  "mainnet_slot": 376970830,
  "mainnet_blockhash": "4EVZgNRMgzkg5xm424NcWBrN1PQHdVWCAqpVzsL6iGti",
  "accounts_loaded": 1,
  "loaded_addresses": ["11111111111111111111111111111111"]
}
```

Save the `user_id` for subsequent requests.

### 2. Check Balance

```bash
curl -X POST http://localhost:8899/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "abc123-def456-ghi789",
    "address": "11111111111111111111111111111111"
  }'
```

### 3. Airdrop SOL for Testing

```bash
curl -X POST http://localhost:8899/api/v1/fork/airdrop \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "abc123-def456-ghi789",
    "address": "YourWalletAddress",
    "sol": 100.0
  }'
```

### 4. Connect a Wallet or dApp

Point your Solana wallet or dApp to the fork's RPC endpoint:

```
http://localhost:8899/rpc?user_id=abc123-def456-ghi789
```

**Example with @solana/web3.js:**

```javascript
import { Connection } from "@solana/web3.js";

const connection = new Connection(
  "http://localhost:8899/rpc?user_id=abc123-def456-ghi789"
);

// Now all operations use your isolated fork!
const balance = await connection.getBalance(publicKey);
const slot = await connection.getSlot();
```

## 📚 API Documentation

### Fork Management

#### Create Fork with Mainnet Sync

```http
POST /api/v1/forks/mainnet
Content-Type: application/json

{
  "accounts": ["address1", "address2"],
  "rpc_endpoint": "optional-custom-rpc-url"
}
```

#### Get Fork Info

```http
GET /api/v1/fork/info?user_id=YOUR_USER_ID
```

#### List All Forks

```http
GET /api/v1/forks
```

### Account Operations

#### Load Account from Mainnet

```http
POST /api/v1/fork/load-account
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "address": "ACCOUNT_ADDRESS"
}
```

#### Load Multiple Accounts

```http
POST /api/v1/fork/load-accounts
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "addresses": ["address1", "address2"]
}
```

#### Load Token Accounts

```http
POST /api/v1/fork/load-token-accounts
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "owner": "OWNER_ADDRESS"
}
```

### Balance Operations

#### Set SOL Balance

```http
POST /api/v1/fork/balance/set
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "address": "WALLET_ADDRESS",
  "lamports": 1000000000
}
```

#### Get SOL Balance

```http
POST /api/v1/fork/balance/get
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "address": "WALLET_ADDRESS"
}
```

#### Airdrop SOL

```http
POST /api/v1/fork/airdrop
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "address": "WALLET_ADDRESS",
  "sol": 10.0
}
```

### Token Operations

#### Create Token Mint

```http
POST /api/v1/token/create-mint
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "payer_keypair": "BASE58_ENCODED_KEYPAIR",
  "decimals": 9
}
```

#### Mint Tokens

```http
POST /api/v1/token/mint
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "mint_authority_keypair": "BASE58_ENCODED_KEYPAIR",
  "mint_address": "MINT_ADDRESS",
  "destination_account": "TOKEN_ACCOUNT",
  "amount": 1000000
}
```

#### Transfer Tokens

```http
POST /api/v1/token/transfer
Content-Type: application/json

{
  "user_id": "YOUR_USER_ID",
  "from_keypair": "BASE58_ENCODED_KEYPAIR",
  "source_account": "SOURCE_TOKEN_ACCOUNT",
  "destination_account": "DEST_TOKEN_ACCOUNT",
  "amount": 1000
}
```

### JSON-RPC Endpoint

Standard Solana JSON-RPC compatible endpoint:

```http
POST /rpc?user_id=YOUR_USER_ID
Content-Type: application/json

{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBalance",
  "params": ["WALLET_ADDRESS"]
}
```

**Supported RPC Methods:**

- `getBalance` - Get SOL balance
- `getAccountInfo` - Get account details
- `getSlot` - Get current slot
- `getLatestBlockhash` - Get recent blockhash
- `getBlockHeight` - Get block height
- `getHealth` - Check RPC health
- `getVersion` - Get version info

## 💡 Usage Examples

### Example 1: Testing a Token Swap

```bash
# 1. Create fork with necessary accounts
curl -X POST http://localhost:8899/api/v1/forks/mainnet \
  -H "Content-Type: application/json" \
  -d '{
    "accounts": [
      "YourWalletAddress",
      "RaydiumPoolAddress",
      "TokenAccountA",
      "TokenAccountB"
    ]
  }'

# 2. Airdrop SOL for transaction fees
curl -X POST http://localhost:8899/api/v1/fork/airdrop \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "YOUR_USER_ID",
    "address": "YourWalletAddress",
    "sol": 10.0
  }'

# 3. Execute swap transaction
# (Use your wallet or dApp connected to the fork RPC)
```

### Example 2: Testing Program Deployment

```bash
# 1. Create fork
curl -X POST http://localhost:8899/api/v1/forks/mainnet \
  -H "Content-Type: application/json" \
  -d '{}'

# 2. Deploy program
curl -X POST "http://localhost:8899/api/v1/program/deploy?user_id=YOUR_USER_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "payer_keypair": "BASE58_KEYPAIR",
    "program_keypair": "BASE58_KEYPAIR",
    "program_data": "BASE64_ENCODED_PROGRAM"
  }'

# 3. Invoke program
curl -X POST "http://localhost:8899/api/v1/program/invoke?user_id=YOUR_USER_ID" \
  -H "Content-Type: application/json" \
  -d '{
    "payer_keypair": "BASE58_KEYPAIR",
    "program_id": "PROGRAM_ADDRESS",
    "instruction_data": "BASE64_DATA"
  }'
```

### Example 3: Using with Solana CLI

```bash
# Configure Solana CLI to use your fork
solana config set --url "http://localhost:8899/rpc?user_id=YOUR_USER_ID"

# Check cluster info
solana cluster-version

# Check balance
solana balance YOUR_WALLET_ADDRESS

# All CLI commands now use your fork!
```

### Example 4: Integration with TypeScript/JavaScript

```typescript
import { Connection, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

// Create connection to fork
const connection = new Connection(
  "http://localhost:8899/rpc?user_id=YOUR_USER_ID",
  "confirmed"
);

// Check balance
const publicKey = new PublicKey("YOUR_WALLET_ADDRESS");
const balance = await connection.getBalance(publicKey);
console.log(`Balance: ${balance / LAMPORTS_PER_SOL} SOL`);

// Get slot
const slot = await connection.getSlot();
console.log(`Current slot: ${slot}`);

// Get latest blockhash
const { blockhash } = await connection.getLatestBlockhash();
console.log(`Blockhash: ${blockhash}`);
```

## 🏗️ Architecture

### System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     Solana Fork Engine                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐ │
│  │   Fork 1     │    │   Fork 2     │    │   Fork N     │ │
│  │  (User A)    │    │  (User B)    │    │  (User N)    │ │
│  │              │    │              │    │              │ │
│  │  LiteSVM     │    │  LiteSVM     │    │  LiteSVM     │ │
│  │  Instance    │    │  Instance    │    │  Instance    │ │
│  └──────────────┘    └──────────────┘    └──────────────┘ │
│         ▲                   ▲                   ▲          │
│         │                   │                   │          │
│         └───────────────────┴───────────────────┘          │
│                             │                              │
│                   ┌─────────▼──────────┐                   │
│                   │   Fork Manager     │                   │
│                   │   (Thread-Safe)    │                   │
│                   └────────────────────┘                   │
│                             │                              │
│         ┌───────────────────┴───────────────────┐          │
│         ▼                                       ▼          │
│  ┌─────────────┐                        ┌─────────────┐   │
│  │ REST API    │                        │ JSON-RPC    │   │
│  │ Endpoints   │                        │ Endpoint    │   │
│  └─────────────┘                        └─────────────┘   │
│         ▲                                       ▲          │
└─────────┼───────────────────────────────────────┼──────────┘
          │                                       │
          │                                       │
     ┌────▼─────┐                           ┌────▼─────┐
     │ HTTP API │                           │ Wallets  │
     │ Clients  │                           │  & dApps │
     └──────────┘                           └──────────┘
              ▲
              │
              │
       ┌──────▼──────┐
       │   Mainnet   │
       │  (Read Only)│
       └─────────────┘
```

### Key Components

1. **Fork Manager**: Thread-safe manager that handles fork lifecycle

   - Creates and tracks forks
   - Maps users to their forks
   - Cleans up expired forks

2. **Fork**: Individual isolated blockchain instance

   - Contains LiteSVM instance
   - Tracks slot, blockhash, and transaction count
   - Synced with mainnet at creation time

3. **Mainnet Client**: Fetches data from real Solana mainnet

   - Account state
   - Token accounts
   - Slot and blockhash

4. **API Layer**: Axum-based HTTP server
   - RESTful endpoints for custom operations
   - JSON-RPC endpoint for standard Solana compatibility

### Data Flow

1. **Fork Creation**: User → API → Fork Manager → Create Fork + Fetch Mainnet State
2. **Account Loading**: User → API → Mainnet Client → Fork (set account)
3. **Transaction**: User → API/RPC → Fork (LiteSVM) → Execute → Update State
4. **Cleanup**: Background Task → Fork Manager → Remove Expired Forks

## ⚙️ Configuration

### Environment Variables

```bash
# Server address (default: 0.0.0.0:8899)
SERVER_ADDR=0.0.0.0:8899

# Fork lifetime in seconds (default: 900 = 15 minutes)
FORK_LIFETIME=900

# Cleanup interval in seconds (default: 60)
CLEANUP_INTERVAL=60

# Default Solana RPC endpoint (default: https://api.mainnet-beta.solana.com)
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
```

### Constants

Edit `src/constants.rs` to change default values:

```rust
pub const DEFAULT_SERVER_ADDR: &str = "0.0.0.0:8899";
pub const FORK_LIFETIME_SECONDS: u64 = 15 * 60; // 15 minutes
pub const CLEANUP_INTERVAL_SECONDS: u64 = 60;   // 1 minute
```

## 🧪 Testing

### Run Tests

```bash
cargo test
```

### Manual Testing

```bash
# Start the server
cargo run

# In another terminal, run the test script
./scripts/test.sh
```

## 🔒 Security Considerations

- **Local Development Only**: This engine is designed for local testing and development
- **No Authentication**: There is no user authentication - anyone can create/access forks
- **Resource Limits**: Forks auto-expire after 15 minutes to prevent resource exhaustion
- **Network Isolation**: Forks are isolated from each other to prevent interference

**⚠️ Do NOT expose this server to the public internet without proper authentication and rate limiting!**

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash
# Clone your fork
git clone https://github.com/yourusername/solana-fork-engine.git
cd solana-fork-engine

# Install development dependencies
cargo build

# Run in development mode with auto-reload
cargo watch -x run

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [LiteSVM](https://github.com/LiteSVM/litesvm) for fast Solana simulation
- Powered by [Axum](https://github.com/tokio-rs/axum) for the HTTP server
- Inspired by [Tenderly](https://tenderly.co/) forks for Ethereum

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/solana-fork-engine/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/solana-fork-engine/discussions)

## 🗺️ Roadmap

- [ ] Add more RPC methods (`sendTransaction`, `simulateTransaction`, etc.)
- [ ] Persistent fork snapshots
- [ ] Fork sharing between users
- [ ] Web UI for fork management
- [ ] Performance metrics and monitoring
- [ ] Docker support
- [ ] Transaction history and replay
- [ ] Custom slot progression speed

---

**Made with ❤️ for the Solana developer community**
