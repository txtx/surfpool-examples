use anchor_lang::{prelude::borsh, system_program, Discriminator};
use litesvm::{types::TransactionResult, LiteSVM};
use solana_instruction::Instruction;
use solana_keypair::{Keypair, Signer};
use solana_pubkey::Pubkey;
use surfpool_types::TimeseriesSurfnetSnapshot;

use crate::hello_pyth;

pub struct SurfpoolSnapshotLoader {
    snapshot: TimeseriesSurfnetSnapshot,
}

fn get_slot(svm: &LiteSVM) -> u64 {
    svm.get_sysvar::<solana_clock::Clock>().slot
}

fn advance_slot(svm: &mut LiteSVM, slots: u64) {
    let current_slot = svm.get_sysvar::<solana_clock::Clock>().slot;
    for i in 0..slots {
        svm.warp_to_slot(current_slot + i + 1);
        svm.expire_blockhash();
    }
}

impl SurfpoolSnapshotLoader {
    pub fn load(snapshot_path: &str, svm: &mut LiteSVM) -> Self {
        let data = std::fs::read_to_string(snapshot_path)
            .expect(&format!("Failed to read snapshot at {snapshot_path}"));
        let starting_slot = get_slot(svm);
        println!("Data: {}", data);
        let mut snapshot: TimeseriesSurfnetSnapshot = serde_json::from_str(&data).expect(&format!(
            "Failed to deserialize snapshot at {snapshot_path}"
        ));

        let updated_snapshot: TimeseriesSurfnetSnapshot = snapshot
            .into_iter()
            .map(|(slot, value)| (slot + starting_slot, value))
            .collect();
        snapshot = updated_snapshot;
        Self { snapshot }
    }

    pub fn next_slot(&mut self, svm: &mut LiteSVM) {
        if let Some(overrides) = self.snapshot.remove(&get_slot(svm)) {
            for (pubkey, account) in overrides {
                svm.set_account(Pubkey::from_str_const(&pubkey), account.into())
                    .unwrap();
            }
        }
        advance_slot(svm, 1);
    }
}

#[test]
fn test_my_program() {
    // 1. One-line setup - no mock RPC
    let program_bytes = include_bytes!("../../../../target/deploy/hello_pyth.so");
    let program_id = Pubkey::new_from_array(*hello_pyth::ID.as_array());
    let mut svm = LiteSVM::new();
    svm.add_program(program_id, program_bytes).unwrap();

    let mut snapshot_loader =
        SurfpoolSnapshotLoader::load("src/tests/fixtures/snapshot.json", &mut svm);
    snapshot_loader.next_slot(&mut svm);

    let user = Keypair::new();
    // 2. Create accounts with built-in helpers
    svm.airdrop(&user.pubkey(), 10_000_000_000).unwrap();

    let (report, _) =
        Pubkey::find_program_address(&vec![b"triangle", user.pubkey().as_ref()], &program_id);

    let mut send_tx = |svm: &mut LiteSVM| -> TransactionResult {
        let data = {
            let mut data = crate::instruction::UpdateTriangularArbitrage::DISCRIMINATOR.to_vec();
            let starting_pair_feed_id_bytes =
                borsh::to_vec("0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43")
                    .unwrap();
            let bridging_pair_feed_id_bytes =
                borsh::to_vec("0xc96458d393fe9deb7a7d63a0ac41e2898a67a7750dbd166673279e06c868df0a")
                    .unwrap();
            let crossing_pair_feed_id_bytes =
                borsh::to_vec("0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace")
                    .unwrap();
            data.extend_from_slice(&starting_pair_feed_id_bytes);
            data.extend_from_slice(&bridging_pair_feed_id_bytes);
            data.extend_from_slice(&crossing_pair_feed_id_bytes);
            data
        };
        // 3. Build instruction (simplified syntax - similar to anchor client)
        let ix = Instruction {
            program_id,
            accounts: vec![
                solana_instruction::AccountMeta::new(user.pubkey(), true),
                solana_instruction::AccountMeta::new(report, false),
                solana_instruction::AccountMeta::new_readonly(
                    Pubkey::from_str_const("4cSM2e6rvbGQUFiJbqytoVMi5GgghSMr8LwVrT9VPSPo"),
                    false,
                ),
                solana_instruction::AccountMeta::new_readonly(
                    Pubkey::from_str_const("5JwbqPPMNpzE2jVAdobWo6m5gkhsDhRdGBo3FYbSfmaK"),
                    false,
                ),
                solana_instruction::AccountMeta::new_readonly(
                    Pubkey::from_str_const("42amVS4KgzR9rA28tkVYqVXjq9Qa8dcZQMbH5EYFX6XC"),
                    false,
                ),
                solana_instruction::AccountMeta::new_readonly(
                    Pubkey::new_from_array(*system_program::ID.as_array()),
                    false,
                ),
            ],
            data,
        };

        let tx = solana_transaction::Transaction::new_signed_with_payer(
            &[ix.clone()],
            Some(&user.pubkey()),
            &[&user],
            svm.latest_blockhash(),
        );
        svm.send_transaction(tx)
    };

    let res = send_tx(&mut svm);
    let res = res.expect_err("transaction with invalid snapshot should have failed");
    println!(
        "Transaction failed as expected:\n {}",
        res.meta.pretty_logs()
    );

    snapshot_loader.next_slot(&mut svm);
    let res = send_tx(&mut svm);
    let res = res.expect("transaction with valid snapshot should have succeeded");
    println!("Transaction succeeded with logs:\n{}", res.pretty_logs());
}
