mod errors;
mod swap;

use pinocchio::account_info::AccountInfo;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::ProgramResult;
use pinocchio_log::log;
// 仅导出必要的内容
pub use errors::PumpAmmError;
pub use swap::PumpAmmSwap;

pub const BUY_DISCRIMINATOR: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];

// Discriminator for sell operation
pub const SELL_DISCRIMINATOR: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

