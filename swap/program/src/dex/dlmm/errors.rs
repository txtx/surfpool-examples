use pinocchio::program_error::ProgramError;

/// DLMM 特定错误
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DlmmError {
    /// 流动性不足
    InsufficientLiquidity,
    /// 滑点过大
    SlippageExceeded,
    /// 非法的代币组合
    InvalidTokenPair,
    /// 未找到活跃的 Bin
    NoBinActive,
    /// 超出最大 Bin 限制
    BinLimitExceeded,
}

impl From<DlmmError> for ProgramError {
    fn from(e: DlmmError) -> Self {
        ProgramError::Custom(500 + e as u32)
    }
} 