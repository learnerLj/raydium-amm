//! Program entrypoint definitions
//!
//! This module defines the entry point for the Raydium AMM program on Solana.
//! The entrypoint is the first function called when the program is invoked,
//! and it's responsible for routing instruction processing to the appropriate handlers.

#![cfg(not(feature = "no-entrypoint"))]

use crate::{error::AmmError, processor::Processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

// Define the program's entry point using Solana's entrypoint macro
entrypoint!(process_instruction);

/// Main entry point for all program instructions
///
/// This function serves as the single entry point for all incoming instructions
/// to the Raydium AMM program. It acts as a dispatcher that forwards all
/// instruction processing to the main Processor module.
///
/// # Arguments
/// * `program_id` - The public key of this program
/// * `accounts` - Array of account infos that this instruction can access
/// * `instruction_data` - Raw instruction data containing the operation to perform
///
/// # Returns
/// * `ProgramResult` - Success (Ok(())) or error details
///
/// # Error Handling
/// If the Processor returns an error, this function:
/// 1. Prints the error using Solana's logging system for debugging
/// 2. Returns the error to the runtime for proper transaction failure handling
fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    // Delegate all instruction processing to the main Processor
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // Log the error for debugging purposes before returning it
        // This helps developers understand what went wrong during execution
        error.print::<AmmError>();
        return Err(error);
    }
    Ok(())
}
