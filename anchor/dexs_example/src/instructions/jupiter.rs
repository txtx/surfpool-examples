use crate::prelude::*;
use solana_instruction::{Instruction, account_meta::{AccountMeta}};
use solana_sysvar_id::ID as sysvarID;
use spl_associated_token_account::get_associated_token_address;
use crate::constants::{JUP_AGGV6_PROGRAM};



pub fn create_jupiter_swap_ix(
    user: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    amount: u64,
) -> Instruction {
   unimplemented!()
}