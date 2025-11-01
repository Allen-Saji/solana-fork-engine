# Solana Fork Simulation Engine

A high-performance Solana network fork simulation engine that enables developers to test dApps and smart contracts in isolated environments using real mainnet state. Built with Rust and LiteSVM.

## Overview

The Solana Fork Engine creates isolated, in-memory copies of Solana mainnet that can be used for safe testing and development. Each fork is synced with the latest mainnet slot and blockhash, allowing realistic testing without spending real SOL or affecting the production network.

**Key Features:**

- Fork creation synced with latest Solana mainnet header (slot + blockhash)
- Multi-user isolation - each user gets their own independent fork
- Load real accounts and programs from mainnet on-demand
- SOL and SPL token balance manipulation for testing
- Standard Solana JSON-RPC compatibility for wallet/dApp integration
- Automatic fork cleanup after 15 minutes
- Full transaction execution support
- Thread-safe concurrent request handling

## Table of Contents

- [Installation](#installation)
- [Getting Started](#getting-started)
- [API Endpoints](#api-endpoints)
- [Usage Examples](#usage-examples)
- [Architecture](#architecture)
- [Configuration](#configuration)
- [Development](#development)
- [Limitations](#limitations)
- [License](#license)

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

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

The server will start on `http://0.0.0.0:8899` by default.

### Verify Installation

```bash
curl http://localhost:8899/health
```

You should receive a `200 OK` response.

## Getting Started

### Step 1: Create Your First Fork

Create a new fork synced with mainnet:

```bash
curl -X POST http://localhost:8899/api/v1/forks/mainnet \
  -H "Content-Type: application/json" \
  -d '{"accounts": []}'
```

**Response:**

```json
{
  "fork_id": "fork-abc123...",
  "user_id": "abc123-def456-ghi789",
  "created_at": "1234567890",
  "expires_at": "1234568790",
  "mainnet_slot": 377252861,
  "mainnet_blockhash": "CKEoh...",
  "accounts_loaded": 0,
  "loaded_addresses": []
}
```

Save the `user_id` - you'll need it for all subsequent operations on this fork.

### Step 2: Airdrop Test SOL

Add SOL to any address for testing:

```bash
curl -X POST http://localhost:8899/api/v1/fork/airdrop \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "abc123-def456-ghi789",
    "address": "YourWalletAddress",
    "sol": 100.0
  }'
```

**Response:**

```json
{
  "success": true,
  "message": "Airdropped 100 SOL",
  "address": "YourWalletAddress",
  "amount_sol": 100.0,
  "amount_lamports": 100000000000
}
```

### Step 3: Check Balance

```bash
curl -X POST http://localhost:8899/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "abc123-def456-ghi789",
    "address": "YourWalletAddress"
  }'
```

**Response:**

```json
{
  "address": "YourWalletAddress",
  "lamports": 100000000000,
  "sol": 100.0
}
```

### Step 4: Load Mainnet Accounts

Load real accounts from mainnet into your fork:

```bash
curl -X POST http://localhost:8899/api/v1/fork/load-account \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "abc123-def456-ghi789",
    "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
  }'
```

### Step 5: Connect a Wallet or dApp

Point your Solana RPC client to your fork:

```javascript
import { Connection } from "@solana/web3.js";

const connection = new Connection(
  "http://localhost:8899/rpc?user_id=abc123-def456-ghi789"
);

// All operations now use your isolated fork
const balance = await connection.getBalance(publicKey);
```

## API Endpoints

### Fork Management

#### Create Fork with Mainnet Sync

```
POST /api/v1/forks/mainnet
```

**Request Body:**

```json
{
  "accounts": ["address1", "address2"],
  "user_id": "optional-user-id",
  "rpc_endpoint": "optional-custom-rpc"
}
```

**Response:**

```json
{
  "fork_id": "fork-...",
  "user_id": "...",
  "created_at": "1234567890",
  "expires_at": "1234568790",
  "mainnet_slot": 377252861,
  "mainnet_blockhash": "...",
  "accounts_loaded": 2,
  "loaded_addresses": ["address1", "address2"]
}
```

#### Get Fork Info

```
GET /api/v1/fork/info?user_id=YOUR_USER_ID
```

**Response:**

```json
{
  "fork_id": "fork-...",
  "status": "active",
  "slot": 377252861,
  "created_at": 1234567890,
  "uptime_seconds": 120,
  "transaction_count": 5
}
```

#### List All Forks

```
GET /api/v1/forks
```

### Account Operations

#### Load Single Account from Mainnet

```
POST /api/v1/fork/load-account
```

**Request Body:**

```json
{
  "user_id": "YOUR_USER_ID",
  "address": "ACCOUNT_ADDRESS",
  "rpc_endpoint": "optional-custom-rpc"
}
```

#### Load Multiple Accounts

```
POST /api/v1/fork/load-accounts
```

**Request Body:**

```json
{
  "user_id": "YOUR_USER_ID",
  "addresses": ["address1", "address2", "address3"]
}
```

#### Get Account Info

```
POST /api/v1/fork/account
```

**Request Body:**

```json
{
  "user_id": "YOUR_USER_ID",
  "address": "ACCOUNT_ADDRESS"
}
```

**Response:**

```json
{
  "address": "...",
  "lamports": 1000000000,
  "owner": "11111111111111111111111111111111",
  "executable": false,
  "rent_epoch": 0,
  "data_length": 0
}
```

### Balance Operations

#### Set Balance (Exact Amount)

```
POST /api/v1/fork/balance/set
```

**Request Body:**

```json
{
  "user_id": "YOUR_USER_ID",
  "address": "WALLET_ADDRESS",
  "lamports": 1000000000000
}
```

Sets the account balance to exactly the specified amount.

#### Airdrop SOL (Add to Balance)

```
POST /api/v1/fork/airdrop
```

**Request Body:**

```json
{
  "user_id": "YOUR_USER_ID",
  "address": "WALLET_ADDRESS",
  "sol": 100.0
}
```

Adds the specified amount of SOL to the existing balance.

#### Get Balance

```
POST /api/v1/fork/balance/get
```

**Request Body:**

```json
{
  "user_id": "YOUR_USER_ID",
  "address": "WALLET_ADDRESS"
}
```

### JSON-RPC Endpoint

Standard Solana JSON-RPC compatible endpoint:

```
POST /rpc?user_id=YOUR_USER_ID
```

**Supported Methods:**

- `getBalance` - Get SOL balance
- `getAccountInfo` - Get account details with base64/base58 encoded data
- `getSlot` - Get current slot number
- `getLatestBlockhash` - Get recent blockhash for transactions
- `getBlockHeight` - Get current block height
- `getHealth` - Health check
- `getVersion` - Get version information

**Example Request:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBalance",
  "params": ["WalletAddress"]
}
```

**Example Response:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "context": { "slot": 377252861 },
    "value": 1000000000
  }
}
```

### Program Operations

#### Load Program from Mainnet

```
POST /api/v1/program/load?user_id=YOUR_USER_ID
```

**Request Body:**

```json
{
  "user_id": "YOUR_USER_ID",
  "program_id": "PROGRAM_ADDRESS"
}
```

#### Get Program Info

```
POST /api/v1/program/info?user_id=YOUR_USER_ID
```

**Request Body:**

```json
{
  "user_id": "YOUR_USER_ID",
  "program_id": "PROGRAM_ADDRESS"
}
```

## Usage Examples

### Example 1: Testing with Multiple Forks

Create two isolated forks for different test scenarios:

```bash
# Create Fork 1
curl -X POST http://localhost:8899/api/v1/forks/mainnet \
  -H "Content-Type: application/json" \
  -d '{"accounts": []}'
# Save user_id as USER_ID_1

# Create Fork 2
curl -X POST http://localhost:8899/api/v1/forks/mainnet \
  -H "Content-Type: application/json" \
  -d '{"accounts": []}'
# Save user_id as USER_ID_2

# Airdrop different amounts to same address in different forks
curl -X POST http://localhost:8899/api/v1/fork/airdrop \
  -H "Content-Type: application/json" \
  -d '{"user_id": "USER_ID_1", "address": "TestAddress", "sol": 100.0}'

curl -X POST http://localhost:8899/api/v1/fork/airdrop \
  -H "Content-Type: application/json" \
  -d '{"user_id": "USER_ID_2", "address": "TestAddress", "sol": 500.0}'

# Verify isolation - same address, different balances
curl -X POST http://localhost:8899/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d '{"user_id": "USER_ID_1", "address": "TestAddress"}'
# Returns: 100 SOL

curl -X POST http://localhost:8899/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d '{"user_id": "USER_ID_2", "address": "TestAddress"}'
# Returns: 500 SOL
```

### Example 2: Loading and Inspecting USDC

```bash
# Create fork
curl -X POST http://localhost:8899/api/v1/forks/mainnet \
  -H "Content-Type: application/json" \
  -d '{"accounts": []}'
# Save user_id

# Load USDC mint account
curl -X POST http://localhost:8899/api/v1/fork/load-account \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "YOUR_USER_ID",
    "address": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  }'

# Get USDC account info
curl -X POST http://localhost:8899/api/v1/fork/account \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "YOUR_USER_ID",
    "address": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
  }'
```

### Example 3: Balance Manipulation

```bash
# Set exact balance
curl -X POST http://localhost:8899/api/v1/fork/balance/set \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "YOUR_USER_ID",
    "address": "TestAddress",
    "lamports": 1000000000000
  }'

# Check balance
curl -X POST http://localhost:8899/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d '{"user_id": "YOUR_USER_ID", "address": "TestAddress"}'
# Returns: 1000 SOL

# Airdrop more (adds to existing)
curl -X POST http://localhost:8899/api/v1/fork/airdrop \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "YOUR_USER_ID",
    "address": "TestAddress",
    "sol": 500.0
  }'

# Check balance again
curl -X POST http://localhost:8899/api/v1/fork/balance/get \
  -H "Content-Type: application/json" \
  -d '{"user_id": "YOUR_USER_ID", "address": "TestAddress"}'
# Returns: 1500 SOL
```

### Example 4: Using with Solana CLI

```bash
# Configure Solana CLI to use your fork
solana config set --url "http://localhost:8899/rpc?user_id=YOUR_USER_ID"

# All CLI commands now use your fork
solana balance YourWalletAddress
solana cluster-version
solana slot
```

### Example 5: TypeScript/JavaScript Integration

```typescript
import { Connection, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

// Connect to your fork
const connection = new Connection(
  "http://localhost:8899/rpc?user_id=YOUR_USER_ID",
  "confirmed"
);

// Check balance
const publicKey = new PublicKey("YourWalletAddress");
const balance = await connection.getBalance(publicKey);
console.log(`Balance: ${balance / LAMPORTS_PER_SOL} SOL`);

// Get slot (synced from mainnet)
const slot = await connection.getSlot();
console.log(`Current slot: ${slot}`);

// Get latest blockhash (from mainnet)
const { blockhash } = await connection.getLatestBlockhash();
console.log(`Blockhash: ${blockhash}`);

// All standard Solana operations work as expected
```

## Architecture

### System Design

```
┌─────────────────────────────────────────────────────────────┐
│                   Solana Fork Engine                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐ │
│  │   Fork 1     │    │   Fork 2     │    │   Fork N     │ │
│  │  (User A)    │    │  (User B)    │    │  (User N)    │ │
│  │              │    │              │    │              │ │
│  │  LiteSVM     │    │  LiteSVM     │    │  LiteSVM     │ │
│  │  Instance    │    │  Instance    │    │  Instance    │ │
│  └──────────────┘    └──────────────┘    └──────────────┘ │
│         │                   │                   │          │
│         └───────────────────┴───────────────────┘          │
│                             │                              │
│                   ┌─────────▼──────────┐                   │
│                   │   Fork Manager     │                   │
│                   │   (Thread-Safe)    │                   │
│                   └────────────────────┘                   │
│                             │                              │
│         ┌───────────────────┴───────────────────┐          │
│         │                                       │          │
│  ┌──────▼──────┐                        ┌──────▼──────┐   │
│  │ REST API    │                        │ JSON-RPC    │   │
│  │ Endpoints   │                        │ Endpoint    │   │
│  └─────────────┘                        └─────────────┘   │
└─────────────────────────────────────────────────────────────┘
                    │                           │
              ┌─────▼─────┐             ┌──────▼──────┐
              │ HTTP API  │             │  Wallets &  │
              │  Clients  │             │    dApps    │
              └───────────┘             └─────────────┘
                    │
              ┌─────▼─────┐
              │  Mainnet  │
              │(Read Only)│
              └───────────┘
```

### Key Components

**Fork Manager:**

- Thread-safe manager using Arc and Mutex
- Tracks all active forks and user mappings
- Handles fork lifecycle (creation, expiration, cleanup)
- Ensures isolation between different users

**Fork:**

- Contains isolated LiteSVM instance
- Stores mainnet sync data (slot, blockhash)
- Tracks transaction count and fork metadata
- Provides balance and account manipulation methods

**Mainnet Client:**

- Fetches account data from Solana mainnet via RPC
- Loads programs and token accounts
- Retrieves slot and blockhash information

**API Layer:**

- RESTful endpoints for custom operations
- JSON-RPC endpoint for standard Solana compatibility
- Shared state across all requests via AppState

**Background Cleanup Task:**

- Runs every 60 seconds
- Removes forks older than 15 minutes
- Prevents resource exhaustion

## Configuration

### Environment Variables

```bash
# Server address (default: 0.0.0.0:8899)
SERVER_ADDR=0.0.0.0:8899

# Fork lifetime in seconds (default: 900 = 15 minutes)
FORK_LIFETIME=900

# Cleanup interval in seconds (default: 60)
CLEANUP_INTERVAL=60

# Default Solana RPC endpoint
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
```

### Modifying Defaults

Edit `src/constants.rs`:

```rust
pub const DEFAULT_SERVER_ADDR: &str = "0.0.0.0:8899";
pub const FORK_LIFETIME_SECONDS: u64 = 15 * 60;
pub const CLEANUP_INTERVAL_SECONDS: u64 = 60;
```

```

## Limitations

### Known Limitations

**Upgradeable Programs:**
BPF Upgradeable Loader v3 programs (like Jupiter v6) require complex multi-account handling. Currently, only BPF Loader v2 programs are fully supported.

**Token Account Loading:**
The `load-token-accounts` endpoint requires `getProgramAccounts` RPC method, which is disabled on most public Solana RPC endpoints. Use premium RPC providers or load token accounts individually.

**State Persistence:**
Forks exist only in memory and are lost when the server restarts.

**Network Isolation:**
This is a local development tool. Do not expose the server to public internet without proper authentication and rate limiting.

## Security Considerations

- Local Development Only: This engine is designed for local testing
- No Authentication: There is no built-in user authentication
- Resource Limits: Forks auto-expire after 15 minutes to prevent resource exhaustion
- Fork Isolation: Each fork has its own LiteSVM instance to prevent interference

Warning: Do NOT expose this server to the public internet without implementing proper authentication, rate limiting, and access controls.

## License

This project is licensed under the MIT License.

## Acknowledgments

- Built with LiteSVM for Solana simulation
- Powered by Axum web framework
- Inspired by Tenderly forks for Ethereum

---

Built for the Solana developer community
```
