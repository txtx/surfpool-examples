use pinocchio::program_error::ProgramError;

pub mod arb;

pub use arb::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub enum SupportDex {
    Pump,
    PumpAmm,
    RaydiumAmm,
    RaydiumCP,
    RaydiumCLMM,
    DLMM,
    WhirlPool,
}

impl TryFrom<&u8> for SupportDex {
    type Error = ProgramError;
    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(SupportDex::Pump),
            1 => Ok(SupportDex::PumpAmm),
            2 => Ok(SupportDex::RaydiumAmm),
            3 => Ok(SupportDex::RaydiumCP),
            4 => Ok(SupportDex::RaydiumCLMM),
            5 => Ok(SupportDex::DLMM),
            6 => Ok(SupportDex::WhirlPool),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

#[repr(u8)]
pub enum LossProgramInstruction {
    DexSwap,
}

impl TryFrom<&u8> for LossProgramInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(LossProgramInstruction::DexSwap),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

mod idl_gen {
    use super::{ArbitrageIxData};

    #[derive(shank::ShankInstruction)]
    enum _MyProgramInstruction {
        #[account(0, writable, signer, name = "signer_acc", desc = "Fee payer account")]
        #[account(1, name = "base_mint", desc = "SOL/USDC mint address")]
        #[account(
            2,
            writable,
            name = "fee_collector_acc",
            desc = "Fee collector account"
        )]
        #[account(3, writable, name = "base_mint_acc", desc = "Base mint ata account")]
        #[account(4, name = "token_program_id", desc = "Token program account")]
        #[account(5, name = "system_program_id", desc = "System program account")]
        #[account(
            6,
            name = "associated_token_program_id",
            desc = "Associated token program account"
        )]
        #[account(7, name = "mint", desc = "mint account")]
        #[account(8, writable, name = "user_mint_acc", desc = "user mint ata account")]
        #[account(9, name = "pump_program_id", desc = "Pump program account")]
        #[account(
            10,
            name = "pump_global_config_acc",
            desc = "Pump global config account"
        )]
        #[account(
            11,
            name = "pump_event_authority_acc",
            desc = "Pump event authority account"
        )]
        #[account(12, name = "protocol_fee_recipient", desc = "Pump fee wallet account")]
        #[account(13, name = "pool_acc", desc = "Pool pool account")]
        #[account(
            14,
            writable,
            name = "pool_base_token_acc",
            desc = "Pool token vault account"
        )]
        #[account(
            15,
            writable,
            name = "pool_quote_token_acc",
            desc = "Pool sol vault account"
        )]
        #[account(
            16,
            writable,
            name = "protocol_fee_recipient_token_acc",
            desc = "fee token wallet account"
        )]
        #[account(
            17,
            writable,
            name = "coin_creator_vault_ata",
            desc = "Pool coin creator vault ata account"
        )]
        #[account(
            18,
            name = "coin_creator_vault_authority",
            desc = "Pool coin creator vault authority account"
        )]
        PumpAMMSwap(ArbitrageIxData),

        #[account(0, writable, signer, name = "signer_acc", desc = "Fee payer account")]
        #[account(1, name = "base_mint", desc = "SOL/USDC mint address")]
        #[account(
            2,
            writable,
            name = "fee_collector_acc",
            desc = "Fee collector account"
        )]
        #[account(3, writable, name = "base_mint_acc", desc = "Base mint ata account")]
        #[account(4, name = "token_program_id", desc = "Token program account")]
        #[account(5, name = "system_program_id", desc = "System program account")]
        #[account(
            6,
            name = "associated_token_program_id",
            desc = "Associated token program account"
        )]
        #[account(7, name = "mint", desc = "mint account")]
        #[account(8, writable, name = "user_mint_acc", desc = "user mint ata account")]
        #[account(9, name = "dlmm_program_id", desc = "dlmm program account")]
        #[account(10, name = "dlmm_event_authority", desc = "dlmm program account")]
        #[account(11, writable, name = "lb_pair", desc = "dlmm pool account")]
        #[account(12, writable, name = "reserve_x", desc = "x token account")]
        #[account(13, writable, name = "reserve_y", desc = "y token account")]
        #[account(14, writable, name = "oracle", desc = "oracle token account")]
        #[account(15, writable, name = "bin_1", desc = "bin 1 token account")]
        #[account(16, writable, name = "bin_2", desc = "bin 2 token account")]
        #[account(17, writable, name = "bin_3", desc = "bin 3 token account")]
        DLMMSwap(ArbitrageIxData),
    }
}
