use std::io::stdout;
use crate::constants::{SOLFI_SOL_USDC_MARKET, SOLFI_PROGRAM, USDC, WSOL, DEFAULT_SWAP_AMOUNT, USDC_DECIMALS};
use crate::instructions::solfi::create_solfi_swap_ix;
use crate::utils::get_token_account_balance;
use crate::prelude::{Result};
use crate::prelude::*;



pub async fn simulate_dex(dex: Dex, amount: Option<f64>) -> Result<()> {
    match_dex(dex, amount)
}

// pass the instruction function as an argument 


// create_solfi_swap_ix(& SOLFI_SOL_USDC_MARKET, &user, &WSOL, &USDC, swap_amount_in_lamports)

pub async simulate(dex_swap_ix: FnOnce) {
    let user_keypair = Keypair::new();
    let user = user_keypair.pubkey();

    const DEFAULT_SWAP_AMOUNT: f64 = 10.0;

    let swap_amount_in_lamports = sol_to_lamports(amount.unwrap_or(DEFAULT_SWAP_AMOUNT));
    
    let wsol_ata = get_associated_token_address(&user, &WSOL);
    let usdc_ata = get_associated_token_address(&user, &USDC);

    let usdc_starting = get_token_account_balance(&usdc_ata, None).await;

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
                    dex_swap_ix
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
                // println!(
                //     "Market: {}, SOL In: {:.4}, USDC Out: {:.4}, Tx Hash: {}",
                //     SOLFI_SOL_USDC_MARKET,
                //     sol_in,
                //     usdc_out as f64 / 10f64.powi(USDC_DECIMALS),
                //     sig,
                // );
            }
            Err(err) => {
                    // println!(
                    //     "Market: {}, SOL In: {:.4}, USDC Out: None, Error: {:?}",
                    //     SOLFI_SOL_USDC_MARKET,
                    //     sol_in,
                    //     err
                    // );
            }
        }
    Ok(())
}

// simulate --dex saros --amount 0.1

// simulate -d saros -a 0.1


fn match_dex(
    dex: &Dex,
    amount_in: u64,
) -> Result<u64> {
    let swap_function = match dex {
        Dex::StableSwap => {

        },
        Dex::Whirlpool => {

        },
        Dex::MeteoraDynamicpool => {

        },
        Dex::RaydiumSwap => {

        },
        Dex::RaydiumStableSwap => {

        }
        Dex::RaydiumClmmSwap => {

        },
        Dex::RaydiumClmmSwapV2 => {

        },
        Dex::AldrinExchangeV1 => {

        },
        Dex::AldrinExchangeV2 => {

        },
        Dex::LifinityV1 => lifinity::swap_v1,
        Dex::LifinityV2 => {

        },
        Dex::FluxBeam => {

        },
        Dex::MeteoraDlmm => {

        },
        Dex::RaydiumCpmmSwap => {

        },
        Dex::OpenBookV2 => {

        },
        Dex::WhirlpoolV2 => {

        },
        Dex::Phoenix => {

        },
        Dex::ObricV2 => {

        },
        Dex::SanctumWsolSwap => {

        }
        Dex::PumpfunBuy => pumpfun::buy,
        Dex::PumpfunSell => {
            return pumpfun::sell(
                remaining_accounts,
                amount_in,
                offset,
                hop_accounts,
                hop,
                proxy_from,
                owner_seeds,
                payer,
            )
        },
        Dex::Saros => {

        },
        Dex::StabbleSwap => {

        },
        Dex::SanctumRouter => {
           
        }
        Dex::MeteoraVaultDeposit => meteora::deposit,
        Dex::MeteoraVaultWithdraw => meteora::withdraw,
        Dex::MeteoraLst => {

        },
        Dex::Solfi => {

        },
        Dex::Zerofi => {

        },
        Dex::PumpfunammBuy => pumpfunamm::buy,
        Dex::PumpfunammSell => {
            return pumpfunamm::sell(
                remaining_accounts,
                amount_in,
                offset,
                hop_accounts,
                hop,
                proxy_from,
                owner_seeds,
                payer,
            )
        },
        Dex::Virtuals => virtuals::swap,
        Dex::VertigoBuy => vertigo::buy,
        Dex::VertigoSell => vertigo::sell,
        Dex::PerpetualsAddLiq => {
            return perpetuals::liquidity_handler(
                remaining_accounts,
                amount_in,
                offset,
                hop_accounts,
                hop,
                proxy_from,
                true,
                owner_seeds,
            );
        }
        Dex::PerpetualsRemoveLiq => {
            return perpetuals::liquidity_handler(
                remaining_accounts,
                amount_in,
                offset,
                hop_accounts,
                hop,
                proxy_from,
                false,
                owner_seeds, 
            );
        }
        Dex::PerpetualsSwap => {

        },
        Dex::RaydiumLaunchpad | Dex::LetsBonkFun => raydium_launchpad::launchpad_handler,
        Dex::Woofi => {

        },
        Dex::MeteoraDbc => {

        },
        Dex::MeteoraDlmmSwap2 => {

        },
        Dex::MeteoraDAMMV2 => {

        },
        Dex::Gavel => gavel::swap,
        Dex::BoopfunBuy => {
            return boopfun::buy(
                remaining_accounts,
                amount_in,
                offset,
                hop_accounts,
                hop,
                proxy_from,
                owner_seeds,
                payer,
            )
        },
        Dex::BoopfunSell => boopfun::sell,
        Dex::MeteoraDbc2 => {

        },
        Dex::GooseFX => {

        },
        Dex::Dooar => dooar::swap,
        Dex::Numeraire => numeraire::swap,
        Dex::SaberDecimalWrapperDeposit => saber_decimal_wrapper::deposit,
        Dex::SaberDecimalWrapperWithdraw => saber_decimal_wrapper::withdraw,
        Dex::SarosDlmm => {

        },
        Dex::OneDexSwap => {

        },
        Dex::Manifest => {

        },
        Dex::ByrealClmm => {

        },
        Dex::PancakeSwapV3Swap => pancake_swap_v3::swap,
        Dex::PancakeSwapV3SwapV2 => {

        },
        Dex::Tessera => {

        },
        Dex::SolRfq => sol_rfq::fill_order,
        Dex::PumpfunBuy2 => {
            return pumpfun::buy2(
                remaining_accounts,
                amount_in,
                offset,
                hop_accounts,
                hop,
                proxy_from,
                owner_seeds,
                payer,
            )
        },
        Dex::PumpfunammBuy2 => pumpfunamm::buy2,
    };
    swap_function(
        remaining_accounts,
        amount_in,
        offset,
        hop_accounts,
        hop,
        proxy_from,
        owner_seeds,
    )
}