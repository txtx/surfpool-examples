use crate::dex::{pump_amm::PumpAmmSwap, DLMMSwap, BUY_DISCRIMINATOR, SWAP_DISCRIMINATOR};
use crate::state::{load_ix_data, DataLen};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use pinocchio_log::log;

use super::SupportDex;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, shank::ShankType)]
pub struct ArbitrageIxData {
    pub dex: SupportDex,
    pub max_bin_to_process: u64,
    pub min_profit_threshold: u64,
    pub no_failure: bool,
}

impl DataLen for ArbitrageIxData {
    const LEN: usize = core::mem::size_of::<ArbitrageIxData>() - 7 * 2;
}

pub fn process_execute_arbitrage(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    log!(
        "ArbitrageIxData len {}",
        core::mem::size_of::<ArbitrageIxData>()
    );
    // 解析指令数据
    let ix_data = unsafe { load_ix_data::<ArbitrageIxData>(data)? };

    // 提取最低收益阈值
    let min_profit_threshold = ix_data.min_profit_threshold;

    let (ix_disc, instruction_data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match SupportDex::try_from(ix_disc)? {
        SupportDex::PumpAmm => {
            log!("PumpAmm");
            // buy:
            // base_amount_out    : u64
            // max_quote_amount_in : u64
            let mut inst_data = [0u8; 24];
            let base_amount_out: u64 = 10_000_000;
            let max_quote_amount_in: u64 = 10_000_00;
            inst_data[0..8].copy_from_slice(&BUY_DISCRIMINATOR);
            inst_data[8..16].copy_from_slice(&(base_amount_out).to_le_bytes());
            inst_data[16..24].copy_from_slice(&(max_quote_amount_in).to_le_bytes());
            PumpAmmSwap::execute_swap(accounts, &inst_data)?;
        }
        SupportDex::DLMM => {
            log!("DLMM");
            // swap:
            // AmountIn     *uint64
            // MinAmountOut *uint64
            let mut inst_data = [0u8; 24];
            let amount_in: u64 = 100;
            let min_amount_out: u64 = 0;
            inst_data[0..8].copy_from_slice(&SWAP_DISCRIMINATOR);
            inst_data[8..16].copy_from_slice(&(amount_in).to_le_bytes());
            inst_data[16..24].copy_from_slice(&(min_amount_out).to_le_bytes());

            DLMMSwap::execute_swap(accounts, &inst_data)?;
        }
        _ => {}
    }

    Ok(())
}
