use pinocchio::instruction::{AccountMeta, Instruction};
use pinocchio::{
    account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;

/// DLMM 交换实现
// amount_in     : u64
// min_amount_out : u64

// [0] = [WRITE] lbPair
//
// [1] = [] binArrayBitmapExtension
//
// [2] = [WRITE] reserveX
//
// [3] = [WRITE] reserveY
//
// [4] = [WRITE] userTokenIn
//
// [5] = [WRITE] userTokenOut
//
// [6] = [] tokenXMint
//
// [7] = [] tokenYMint
//
// [8] = [WRITE] oracle
//
// [9] = [WRITE] hostFeeIn
//
// [10] = [SIGNER] user
//
// [11] = [] tokenXProgram
//
// [12] = [] tokenYProgram
//
// [13] = [] eventAuthority
//
// [14] = [] program
pub struct DLMMSwap ;

impl DLMMSwap {
    /// only buy
    pub fn execute_swap( accounts: &[AccountInfo], inst_data: &[u8]) -> ProgramResult {
        // 基本验证
        let [signer_acc, base_mint, fee_collector_acc, base_mint_acc, token_program_id, system_program_id, associated_token_program_id, mint, user_mint_acc, dlmm_program_id, dlmm_event_authority, lb_pair, reserve_x, reserve_y, oracle, bin_1, bin_2, bin_3, ..] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        msg!("swap");
        if !signer_acc.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // 创建账户元数据
        let account_infos = [
            // 1
            lb_pair,
            //  2
            dlmm_program_id,
            //  3
            reserve_x,
            // 4
            reserve_y,
            // 5
            base_mint_acc,
            // 6
            user_mint_acc,
            // 7
            mint,
            // 8
            base_mint,
            // 9
            oracle,
            // 10
            dlmm_program_id,
            signer_acc,
            token_program_id,
            token_program_id,
            dlmm_event_authority,
            dlmm_program_id,
            bin_1,
            bin_2,
            bin_3,
        ];
        let account_metas = [
            AccountMeta::writable(lb_pair.key()),
            AccountMeta::readonly(dlmm_program_id.key()),
            AccountMeta::writable(reserve_x.key()),
            AccountMeta::writable(reserve_y.key()),
            AccountMeta::writable(base_mint_acc.key()),
            AccountMeta::writable(user_mint_acc.key()),
            AccountMeta::readonly(mint.key()),
            AccountMeta::readonly(base_mint.key()),
            AccountMeta::writable(oracle.key()),
            AccountMeta::readonly(dlmm_program_id.key()),
            AccountMeta::writable_signer(signer_acc.key()),
            AccountMeta::readonly(token_program_id.key()),
            AccountMeta::readonly(token_program_id.key()),
            AccountMeta::readonly(dlmm_event_authority.key()),
            AccountMeta::readonly(dlmm_program_id.key()),
            AccountMeta::writable(bin_1.key()),
            AccountMeta::writable(bin_2.key()),
            AccountMeta::writable(bin_3.key()),
        ];

        // 创建指令
        let instruction = Instruction {
            program_id: dlmm_program_id.key(),
            accounts: &account_metas,
            data: inst_data,
        };
        msg!("call cpi");

        pinocchio::cpi::invoke(&instruction, &account_infos)?;

        log!("DLMM Swap CPI调用成功完成");

        Ok(())
    }
}
