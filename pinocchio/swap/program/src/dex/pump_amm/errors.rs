use pinocchio::program_error::ProgramError;

/// Pump AMM 特定错误
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PumpAmmError {
    /// 池中流动性不足
    InsufficientLiquidity,
    /// 滑点过大
    SlippageExceeded,
    /// 非法的代币组合
    InvalidTokenPair,
    /// 无效的池账户
    InvalidPoolAccount,
}

impl From<PumpAmmError> for ProgramError {
    fn from(e: PumpAmmError) -> Self {
        ProgramError::Custom(400 + e as u32)
    }
} 