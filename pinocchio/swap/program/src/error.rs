use pinocchio::program_error::ProgramError;

/// 自定义 Loss Program 错误
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LossProgramError {
    /// PDA 不匹配
    PdaMismatch = 0,
    /// 算术错误
    ArithmeticError = 1,
    /// 无效的池账户
    InvalidPoolAccount = 2,
    /// 无效的指令数据
    InvalidInstructionData = 3,
    /// 滑点超过限制
    SlippageExceeded = 4,
    /// 流动性不足
    InsufficientLiquidity = 5,
    /// 无效的代币对
    InvalidTokenPair = 6,
    /// 池子被禁用
    PoolDisabled = 7,
    /// 交易数量过小
    AmountTooSmall = 8,
    /// 交易数量过大
    AmountTooLarge = 9,
    /// 授权错误
    AuthorizationError = 10,

    InvalidOwner = 11,
}

impl From<LossProgramError> for ProgramError {
    fn from(e: LossProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
