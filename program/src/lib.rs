// #![deny(missing_docs)]

//! Raydium Automated Market Maker (AMM) Program
//!
//! A Uniswap-style constant product AMM implemented for the Solana blockchain.
//! This program enables decentralized token trading through liquidity pools,
//! integrated with OpenBook (formerly Serum) for enhanced liquidity sharing.
//!
//! ## Key Features
//! - Constant product market maker (x * y = k formula)
//! - Integration with OpenBook for order book liquidity sharing
//! - Fibonacci sequence-based limit order placement
//! - Fee collection and distribution mechanisms
//! - Administrative controls and parameter updates
//! - Slippage protection for swaps and liquidity operations
//!
//! ## Module Organization
//! - `entrypoint`: Program entry point and instruction routing
//! - `processor`: Core business logic and instruction processing
//! - `instruction`: Instruction definitions and serialization
//! - `state`: Account state structures and validation
//! - `math`: Mathematical operations for AMM calculations
//! - `error`: Comprehensive error definitions
//! - `invokers`: Cross-program invocation utilities
//! - `log`: Structured logging and event emission

#[macro_use]
pub mod log;

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod invokers;
pub mod math;
pub mod processor;
pub mod state;

// Export current solana-sdk types for downstream users who may also be building with a different solana-sdk version
pub use solana_program;

// Security disclosure information for on-chain programs
// This metadata is embedded in the program binary and provides
// contact information for security researchers
#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "raydium-amm",
    project_url: "https://raydium.io",
    contacts: "link:https://immunefi.com/bounty/raydium",
    policy: "https://immunefi.com/bounty/raydium",
    source_code: "https://github.com/raydium-io/raydium-amm",
    preferred_languages: "en",
    auditors: "https://github.com/raydium-io/raydium-docs/blob/master/audit/MadSheild%20Q2%202023/Raydium%20updated%20orderbook%20AMM%20program%20%26%20OpenBook%20migration.pdf"
}

// Program ID declarations for different deployment environments
// The program ID determines the address at which this program is deployed

/// Devnet program ID for testing and development
#[cfg(feature = "devnet")]
solana_program::declare_id!("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8");

/// Mainnet program ID for production deployment
#[cfg(not(feature = "devnet"))]
solana_program::declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
