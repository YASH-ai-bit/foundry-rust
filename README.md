# Ethereum Rust SDK Examples

A Rust project demonstrating Ethereum smart contract deployment and transactions using [ethers-rs](https://github.com/gakonst/ethers-rs).

## Features

- **Contract Deployment** - Compile and deploy Solidity contracts
- **Simple Transactions** - Send ETH between addresses
- **Local Development** - Uses Anvil for local blockchain testing
- **Solidity Compilation** - Built-in Solidity compiler support

## Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Foundry](https://getfoundry.sh/) (for Anvil local node)

## Installation

```bash
git clone https://github.com/yourusername/foundry.git
cd foundry
cargo build
```

## Usage

### Deploy a Smart Contract

Compiles and deploys the `SimpleToken` ERC20 contract to a local Anvil instance:

```bash
cargo run --bin deploy
```

This will:

1. Start a local Anvil blockchain
2. Compile the Solidity contract in `examples/`
3. Deploy the `SimpleToken` contract
4. Print the deployed contract address

### Send a Transaction

Demonstrates sending ETH between two addresses:

```bash
cargo run --bin transact
```

This will:

1. Start a local Anvil blockchain
2. Create a wallet from a mnemonic
3. Send 1000 wei to another address
4. Display updated balances

## Project Structure

```
├── Cargo.toml                    # Rust dependencies
├── examples/
│   └── SimpleToken.sol           # ERC20 token contract
└── src/
    ├── contract_deploy.rs        # Contract deployment binary
    └── simple_transactions.rs    # Transaction example binary
```

## SimpleToken Contract

A basic ERC20 token with:

- **Name:** SimpleToken
- **Symbol:** STK
- **Decimals:** 18
- **Initial Supply:** 1,000,000 tokens (minted to deployer)

### Functions

- `transfer(address to, uint256 value)` - Transfer tokens
- `approve(address spender, uint256 value)` - Approve spending
- `transferFrom(address from, address to, uint256 value)` - Transfer on behalf

## Dependencies

| Crate         | Purpose                    |
| ------------- | -------------------------- |
| `ethers`      | Ethereum library for Rust  |
| `ethers-solc` | Solidity compiler bindings |
| `tokio`       | Async runtime              |
| `eyre`        | Error handling             |

## License

MIT
