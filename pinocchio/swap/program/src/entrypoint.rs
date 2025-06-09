#![allow(unexpected_cfgs)]

use crate::instruction::{self, LossProgramInstruction};
use pinocchio::{
    account_info::AccountInfo, default_panic_handler, msg, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;
use pinocchio_pubkey::pubkey;

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();
// Use the no_std panic handler.
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    log!("data {}",instruction_data);
    log!("accounts {}", accounts.len());

    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match LossProgramInstruction::try_from(ix_disc)? {
        LossProgramInstruction::DexSwap => {
            msg!("Ix:Swap");
            instruction::process_execute_arbitrage(accounts, instruction_data)
        }
    }
}
