# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is the Raydium AMM (Automated Market Maker), an on-chain smart contract built on Solana using the "constant product" model. The AMM operates in a permissionless and decentralized manner, sharing liquidity with OpenBook (the primary CLOB of Solana) through Fibonacci sequence-based limit orders.

**Program IDs:**
- Mainnet: `675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8`
- Devnet: `HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8`

## Architecture

This is a Rust-based Solana program with the following structure:

- **Workspace**: Rust workspace with `program/` as the main member
- **Core modules** (in `program/src/`):
  - `entrypoint.rs` - Program entry point
  - `processor.rs` - Main instruction processing logic
  - `instruction.rs` - Instruction definitions and parsing
  - `state.rs` - Program state structures (AmmInfo, AmmConfig, etc.)
  - `math.rs` - Mathematical operations for AMM calculations
  - `error.rs` - Custom error definitions
  - `invokers.rs` - Cross-program invocation helpers
  - `log.rs` - Logging macros and utilities

**Key architectural patterns:**
- Uses Anchor-style instruction processing with match statements
- Integrates with Serum DEX (OpenBook) for order book functionality
- Implements constant product AMM with liquidity sharing to CLOB
- Feature flags for different environments (devnet, testnet, localnet)

## Development Commands

### Building
```bash
# Navigate to program directory first
cd program

# Mainnet build
cargo build-sbf

# Devnet build  
cargo build-sbf --features devnet

# Localnet build (requires updating pubkeys in config_feature)
cargo build-sbf --features localnet
```

### Deployment
```bash
# Deploy to configured environment
solana deploy

# Check environment configuration first
solana config get
```

### Testing
The crate uses `proptest` for property-based testing (see dev-dependencies in Cargo.toml). Run tests with:
```bash
cargo test
```

## Environment Setup Requirements

1. Install Rust toolchain
2. Install Solana CLI tools
3. Generate Solana keypair: `solana-keygen new`
4. Configure Solana cluster: `solana config set --url <cluster-url>`

## Integration Notes

- Requires an existing OpenBook market before initializing AMM pools
- Uses SPL Token and Associated Token Account programs
- Integrates with serum_dex crate (OpenBook fork)
- Client library available at: https://github.com/raydium-io/raydium-library

## Important Build Notes

- The program MUST be built using Solana BPF toolchain (`cargo build-sbf`)
- Different features control program ID and configuration for different networks
- Release profile uses LTO optimization and overflow checks enabled
- Program uses `cdylib` crate type for Solana deployment