use std::io::stdout;
use constants::{SOLFI_MARKET, SOLFI_PROGRAM, USDC, WSOL, DEFAULT_SWAP_AMOUNT, USDC_DECIMALS};
use crate::instructions::solfi::create_solfi_swap_ix;
use crate::utils::get_token_account_balance;
use crate::prelude::*;


struct LatestBlockhash {
    blockhash: RwLock<Hash>,
    slot: AtomicU64,
}

pub fn simulate_solfi(amount: Option<u64>) -> Result<()> {
    let user_keypair = Keypair::new();
    let user = user_keypair.pubkey();
    let rpc_url = "http://127.0.0.1:8899";
    let rpc_client = Arc::new(RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    ));

    let rpc_client_clone = rpc_client.clone();
    
    let latest_blockhash = Arc::new(LatestBlockhash {
        blockhash: RwLock::new(Hash::default()),
        slot: AtomicU64::new(0),
    });

    let latest_blockhash_clone = latest_blockhash.clone();
    tokio::spawn(async move {
        loop {
            if let Ok((blockhash, slot)) =
                rpc_client_clone.get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
            {
                let mut blockhash_write = latest_blockhash_clone.blockhash.write().await;
                *blockhash_write = blockhash;
                latest_blockhash_clone.slot.store(slot, Ordering::Relaxed);
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }); 
    
    let amount: f64 = 0.1;

    let swap_amount_in_lamports = sol_to_lamports(amount);
    
    let wsol_ata = get_associated_token_address(&user, &WSOL);
    let usdc_ata = get_associated_token_address(&user, &USDC);

    let usdc_starting = get_token_account_balance(&usdc_ata, None)
        .await
        .unwrap()
        .ui_amount;

    loop {
        let slot = latest_blockhash.slot.load(Ordering::Relaxed);
        if slot != 0 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    let recent_blockhash = latest_blockhash.blockhash.read().await;

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
                    create_swap_ix(&SOLFI_MARKET, &user, &WSOL, &USDC, swap_amount_in_lamports),
                ],
                Some(&user),
            ),
            *recent_blockhash
        );

        let sol_in = lamports_to_sol(swap_amount_in_lamports);

        match rpc_client_clone.send_transaction(&tx) {
            Ok(sig) => {
                let usdc_out = get_token_account_balance(&usdc_ata, None) - usdc_starting;
                println!(
                    "Market: {}, SOL In: {:.4}, USDC Out: {:.4}, Tx Hash: {}",
                    SOLFI_MARKET,
                    sol_in,
                    usdc_out as f64 / 10f64.powi(USDC_DECIMALS),
                    sig,
                );
            }
            Err(err) => {
                    println!(
                        "Market: {}, SOL In: {:.4}, USDC Out: None, Error: {}",
                        SOLFI_MARKET,
                        sol_in,
                        err.err
                    );
            }
        }
}