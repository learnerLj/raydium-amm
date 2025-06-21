//! Structured logging and event emission utilities
//!
//! This module provides functionality for emitting structured logs and events
//! from the AMM program. Events are encoded as base64 strings and emitted
//! through Solana's program logs, making them easy to parse and monitor
//! off-chain.
//!
//! ## Event Types
//! - Init: Pool initialization events
//! - Deposit: Liquidity provision events  
//! - Withdraw: Liquidity removal events
//! - SwapBaseIn: Token swaps with exact input
//! - SwapBaseOut: Token swaps with exact output
//!
//! All events include relevant pool state and operation parameters
//! for comprehensive tracking and analytics.

use arrform::{arrform, ArrForm};
use serde::{Deserialize, Serialize};
use solana_program::{
    msg,
    // entrypoint::ProgramResult,
    pubkey::Pubkey,
};

/// Maximum size for formatted log messages
pub const LOG_SIZE: usize = 256;

/// Assertion macro for validating account keys with detailed logging
///
/// This macro checks if two values are equal and logs a detailed error message
/// if they don't match. It's primarily used for validating that account
/// public keys match expected values during instruction processing.
///
/// # Arguments
/// * `$input` - The actual value to check
/// * `$expected` - The expected value
/// * `$msg` - Error message prefix
/// * `$err` - Error type to return on mismatch
#[macro_export]
macro_rules! check_assert_eq {
    ($input:expr, $expected:expr, $msg:expr, $err:expr) => {
        if $input != $expected {
            log_keys_mismatch(concat!($msg, " mismatch:"), $input, $expected);
            return Err($err.into());
        }
    };
}

/// Logs a key mismatch error with detailed information
///
/// This function formats and emits a log message when account validation
/// fails, showing both the actual and expected public keys for debugging.
///
/// # Arguments
/// * `msg` - Error message describing the context
/// * `input` - The actual public key received
/// * `expected` - The expected public key
pub fn log_keys_mismatch(msg: &str, input: Pubkey, expected: Pubkey) {
    msg!(arrform!(
        LOG_SIZE,
        "ray_log: {} input:{}, expected:{}",
        msg,
        input,
        expected
    )
    .as_str());
}

/// Event type enumeration for structured logging
///
/// This enum defines the different types of events that can be logged
/// by the AMM program. Each event type corresponds to a major operation
/// and has its own associated data structure.
#[derive(Debug)]
pub enum LogType {
    /// Pool initialization event
    Init,
    /// Liquidity deposit event
    Deposit,
    /// Liquidity withdrawal event
    Withdraw,
    /// Token swap with exact input amount
    SwapBaseIn,
    /// Token swap with exact output amount
    SwapBaseOut,
}

impl LogType {
    pub fn from_u8(log_type: u8) -> Self {
        match log_type {
            0 => LogType::Init,
            1 => LogType::Deposit,
            2 => LogType::Withdraw,
            3 => LogType::SwapBaseIn,
            4 => LogType::SwapBaseOut,
            _ => unreachable!(),
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            LogType::Init => 0u8,
            LogType::Deposit => 1u8,
            LogType::Withdraw => 2u8,
            LogType::SwapBaseIn => 3u8,
            LogType::SwapBaseOut => 4u8,
        }
    }
}

/// Pool initialization event data
///
/// This structure captures all relevant information when a new AMM pool
/// is initialized, including token configurations and initial liquidity.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InitLog {
    /// Event type identifier (0 for Init)
    pub log_type: u8,
    /// Timestamp of pool creation
    pub time: u64,
    /// Quote token decimal places
    pub pc_decimals: u8,
    /// Base token decimal places
    pub coin_decimals: u8,
    /// Quote token lot size for OpenBook integration
    pub pc_lot_size: u64,
    /// Base token lot size for OpenBook integration
    pub coin_lot_size: u64,
    /// Initial quote token amount deposited
    pub pc_amount: u64,
    /// Initial base token amount deposited
    pub coin_amount: u64,
    /// Associated OpenBook market public key
    pub market: Pubkey,
}

/// Liquidity deposit event data
///
/// This structure captures information about liquidity provision operations,
/// including user inputs, pool state, and calculated results.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DepositLog {
    /// Event type identifier (1 for Deposit)
    pub log_type: u8,
    // === User Input Parameters ===
    /// Maximum base token amount user willing to deposit
    pub max_coin: u64,
    /// Maximum quote token amount user willing to deposit
    pub max_pc: u64,
    /// Which token side used as base for calculation
    pub base: u64,
    // === Pool State Information ===
    /// Pool's base token balance
    pub pool_coin: u64,
    /// Pool's quote token balance
    pub pool_pc: u64,
    /// Total LP token supply
    pub pool_lp: u64,
    /// Calculated PnL for base token side
    pub calc_pnl_x: u128,
    /// Calculated PnL for quote token side
    pub calc_pnl_y: u128,
    // === Operation Results ===
    /// Actual base token amount deducted from user
    pub deduct_coin: u64,
    /// Actual quote token amount deducted from user
    pub deduct_pc: u64,
    /// LP tokens minted to user
    pub mint_lp: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct WithdrawLog {
    pub log_type: u8,
    // input
    pub withdraw_lp: u64,
    // user info
    pub user_lp: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    pub pool_lp: u64,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    // calc result
    pub out_coin: u64,
    pub out_pc: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SwapBaseInLog {
    pub log_type: u8,
    // input
    pub amount_in: u64,
    pub minimum_out: u64,
    pub direction: u64,
    // user info
    pub user_source: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    // calc result
    pub out_amount: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SwapBaseOutLog {
    pub log_type: u8,
    // input
    pub max_in: u64,
    pub amount_out: u64,
    pub direction: u64,
    // user info
    pub user_source: u64,
    // pool info
    pub pool_coin: u64,
    pub pool_pc: u64,
    // calc result
    pub deduct_in: u64,
}

/// Encodes and emits a structured log event
///
/// This function serializes a log structure to binary format, encodes it
/// as base64, and emits it through Solana's program logging system.
/// Off-chain services can parse these logs to track AMM operations.
///
/// # Arguments
/// * `log` - The log structure to emit (must implement Serialize)
///
/// # Format
/// Events are emitted as "ray_log: <base64_encoded_data>" for easy parsing
pub fn encode_ray_log<T: Serialize>(log: T) {
    // Serialize the log structure to binary format
    let bytes = bincode::serialize(&log).unwrap();
    let mut out_buf = Vec::new();
    out_buf.resize(bytes.len() * 4 / 3 + 4, 0);
    // Encode binary data as base64 for safe transmission in logs
    let bytes_written = base64::encode_config_slice(bytes, base64::STANDARD, &mut out_buf);
    out_buf.resize(bytes_written, 0);
    let msg_str = unsafe { std::str::from_utf8_unchecked(&out_buf) };
    // Emit the encoded log through Solana's logging system
    msg!(arrform!(LOG_SIZE, "ray_log: {}", msg_str).as_str());
}

/// Decodes and prints a ray_log event (utility function for debugging)
///
/// This function takes a base64-encoded log string and decodes it back
/// to the original log structure for inspection. Primarily used for
/// debugging and testing purposes.
///
/// # Arguments
/// * `log` - Base64-encoded log string to decode
///
/// # Behavior
/// Prints the decoded log structure to stdout based on the log type
pub fn decode_ray_log(log: &str) {
    let bytes = base64::decode_config(log, base64::STANDARD).unwrap();
    match LogType::from_u8(bytes[0]) {
        LogType::Init => {
            let log: InitLog = bincode::deserialize(&bytes).unwrap();
            println!("{:?}", log);
        }
        LogType::Deposit => {
            let log: DepositLog = bincode::deserialize(&bytes).unwrap();
            println!("{:?}", log);
        }
        LogType::Withdraw => {
            let log: WithdrawLog = bincode::deserialize(&bytes).unwrap();
            println!("{:?}", log);
        }
        LogType::SwapBaseIn => {
            let log: SwapBaseInLog = bincode::deserialize(&bytes).unwrap();
            println!("{:?}", log);
        }
        LogType::SwapBaseOut => {
            let log: SwapBaseOutLog = bincode::deserialize(&bytes).unwrap();
            println!("{:?}", log);
        }
    }
}
