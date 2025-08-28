use std::io::stdout;
use crate::constants::{SOLFI_SOL_USDC_MARKET, SOLFI_PROGRAM, USDC, WSOL, DEFAULT_SWAP_AMOUNT, USDC_DECIMALS};
use crate::instructions::solfi::create_solfi_swap_ix;
use crate::utils::get_token_account_balance;
use crate::prelude::{Result};
use crate::prelude::*;


pub async fn simulate_solfi(amount: Option<f64>) -> Result<()> {

    let swap_amount_in_lamports = sol_to_lamports(amount.unwrap_or(DEFAULT_SWAP_AMOUNT));
    
    let wsol_ata = get_associated_token_address(&user, &WSOL);
    let usdc_ata = get_associated_token_address(&user, &USDC);

    let tx = Transaction::new(
            &[&user_keypair],
            Message::new(
                &[
                    create_associated_token_account_idempotent(
                        &user,
                        &user,
                        &WSOL,
                        &spl_token::id(),
                    ),
                    create_associated_token_account_idempotent(
                        &user,
                        &user,
                        &USDC,
                        &spl_token::id(),
                    ),
                    transfer(&user, &wsol_ata, swap_amount_in_lamports),
                    sync_native(&spl_token::id(), &wsol_ata).unwrap(),
                    create_solfi_swap_ix(&SOLFI_SOL_USDC_MARKET, &user, &WSOL, &USDC, swap_amount_in_lamports),
                ],
                Some(&user),
            ),
            *recent_blockhash
        );

        let sol_in = lamports_to_sol(amount.unwrap_or(DEFAULT_SWAP_AMOUNT) as u64);
        let usd_ata = get_token_account_balance(&usdc_ata, None).await;

        match rpc_client.send_transaction(&tx) {
            Ok(sig) => {
                let usdc_out = usd_ata - usdc_starting;
                println!(
                    "Market: {}, SOL In: {:.4}, USDC Out: {:.4}, Tx Hash: {}",
                     SOLFI_SOL_USDC_MARKET,
                    sol_in,
                    usdc_out as f64 / 10f64.powi(USDC_DECIMALS),
                    sig,
                );
            }
            Err(err) => {
                    println!(
                        "Market: {}, SOL In: {:.4}, USDC Out: None, Error: {:?}",
                         SOLFI_SOL_USDC_MARKET,
                        sol_in,
                        err
                    );
            }
        }
    Ok(())
}