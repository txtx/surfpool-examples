use pinocchio::instruction::{AccountMeta, Instruction};
use pinocchio::{
    account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};
use pinocchio_log::log;

// buy:
// base_amount_out    : u64
// max_quote_amount_in : u64
// sell:
// base_amount_in : u64
// min_quote_amount_out : u64
// [0] = [] pool
//
// [1] = [WRITE, SIGNER] user
//
// [2] = [] global_config
//
// [3] = [] base_mint
//
// [4] = [] quote_mint
//
// [5] = [WRITE] user_base_token_account
//
// [6] = [WRITE] user_quote_token_account
//
// [7] = [WRITE] pool_base_token_account
//
// [8] = [WRITE] pool_quote_token_account
//
// [9] = [] protocol_fee_recipient
//
// [10] = [WRITE] protocol_fee_recipient_token_account
//
// [11] = [] base_token_program
//
// [12] = [] quote_token_program
//
// [13] = [] system_program
//
// [14] = [] associated_token_program
//
// [15] = [] event_authority
//
// [16] = [] program
//
// [17] = [] coin creator vault ata
//
// [18] = [] coin creator vault authority
/// Pump AMM 交换实现
pub struct PumpAmmSwap;

impl PumpAmmSwap {
    /// only buy
    pub fn execute_swap(accounts: &[AccountInfo], inst_data: &[u8]) -> ProgramResult {
        // 基本验证
        let [signer_acc, base_mint, _fee_collector_acc, base_mint_acc, token_program_id, system_program_id, associated_token_program_id, mint, user_mint_acc, pump_program_id, pump_global_config_acc, pump_event_authority_acc, protocol_fee_recipient, pool_acc, pool_base_token_acc, pool_quote_token_acc, protocol_fee_recipient_token_acc, coin_creator_vault_ata, coin_creator_vault_authority, ..] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        if !signer_acc.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // 创建账户元数据
        let account_infos = [
            pool_acc,
            signer_acc,
            pump_global_config_acc,
            mint,
            base_mint,
            user_mint_acc,
            base_mint_acc,
            pool_base_token_acc,
            pool_quote_token_acc,
            protocol_fee_recipient,
            protocol_fee_recipient_token_acc,
            token_program_id,
            token_program_id,
            system_program_id,
            associated_token_program_id,
            pump_event_authority_acc,
            pump_program_id,
            coin_creator_vault_ata,
            coin_creator_vault_authority,
        ];
        let account_metas = [
            AccountMeta::readonly(pool_acc.key()),
            AccountMeta::writable_signer(signer_acc.key()),
            AccountMeta::readonly(pump_global_config_acc.key()),
            AccountMeta::readonly(mint.key()),
            AccountMeta::readonly(base_mint.key()),
            AccountMeta::writable(user_mint_acc.key()),
            AccountMeta::writable(base_mint_acc.key()),
            AccountMeta::writable(pool_base_token_acc.key()),
            AccountMeta::writable(pool_quote_token_acc.key()),
            AccountMeta::readonly(protocol_fee_recipient.key()),
            AccountMeta::writable(protocol_fee_recipient_token_acc.key()),
            AccountMeta::readonly(token_program_id.key()),
            AccountMeta::readonly(token_program_id.key()),
            AccountMeta::readonly(system_program_id.key()),
            AccountMeta::readonly(associated_token_program_id.key()),
            AccountMeta::readonly(pump_event_authority_acc.key()),
            AccountMeta::readonly(pump_program_id.key()),
            AccountMeta::writable(coin_creator_vault_ata.key()),
            AccountMeta::readonly(coin_creator_vault_authority.key()),
        ];
        log!("inst data pump {}", inst_data);
        // 创建指令
        let instruction = Instruction {
            program_id: pump_program_id.key(),
            accounts: &account_metas,
            data: inst_data,
        };
        msg!("call cpi");

        pinocchio::cpi::invoke(&instruction, &account_infos)?;

        log!("Pump AMM交换CPI调用成功完成");
        Ok(())
    }
}
