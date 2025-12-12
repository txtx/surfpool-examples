// use anchor_litesvm::{AnchorContext, AnchorLiteSVM, Pubkey, Signer, TestHelpers};
// use litesvm_utils::{AssertionHelpers, TestHelpers};
// use solana_sdk::signature::Signer;
// use surfpool_types::TimeseriesSurfnetSnapshot;

// use crate::hello_pyth;

// pub struct SurfpoolSnapshotLoader {
//     snapshot: TimeseriesSurfnetSnapshot,
// }

// impl SurfpoolSnapshotLoader {
//     pub fn load(snapshot_path: &str, ctx: &mut AnchorLiteSVM) -> Self {
//         let bytes = include_bytes!(AnchorContext);
//         let starting_slot = ctx.svm.get_current_slot();
//         let mut snapshot: TimeseriesSurfnetSnapshot = serde_json::from_slice(&bytes)
//             .expect(format!("Failed to deserialize snapshot at {snapshot_path}"));

//         snapshot
//             .iter_mut()
//             .for_each(|(slot, _)| *slot += starting_slot);
//         Self { snapshot }
//     }

//     pub fn next_slot(&mut self, ctx: &mut AnchorContext) {
//         if let Some(overrides) = self.snapshot.remove(&ctx.svm.get_current_slot()) {
//             for (pubkey, account) in overrides {
//                 ctx.svm
//                     .set_account(
//                         anchor_litesvm::Pubkey::from_str_const(pubkey),
//                         account.into(),
//                     )
//                     .unwrap();
//             }
//         }
//         ctx.svm.advance_slot(1);
//     }
// }

// #[test]
// fn test_my_program() {
//     // 1. One-line setup - no mock RPC
//     let mut ctx = AnchorLiteSVM::build_with_program(
//         hello_pyth::ID,
//         include_bytes!("../target/deploy/hello_pyth.so"),
//     );

//     // 2. Create accounts with built-in helpers
//     let user = ctx.svm.create_funded_account(10_000_000_000).unwrap();

//     ctx.svm.set_account(pubkey, data).unwrap();

//     let report = ctx.svm.get_pda(
//         &hello_pyth::client::accounts::report_pda_seed(&user.pubkey()),
//         hello_pyth::ID,
//     );

//     // 3. Build instruction (simplified syntax - similar to anchor client)
//     let ix = ctx
//         .program()
//         .accounts(hello_pyth::client::accounts::UpdateTriangularArbitrage {
//             sender: user.pubkey(),
//             report,
//             starting_pair_account: Pubkey::from_str_const(
//                 "4cSM2e6rvbGQUFiJbqytoVMi5GgghSMr8LwVrT9VPSPo",
//             ),
//             bridging_pair_account: Pubkey::from_str_const(
//                 "5JwbqPPMNpzE2jVAdobWo6m5gkhsDhRdGBo3FYbSfmaK",
//             ),
//             crossing_pair_account: Pubkey::from_str_const(
//                 "42amVS4KgzR9rA28tkVYqVXjq9Qa8dcZQMbH5EYFX6XC",
//             ),
//             system_program: solana_sdk::system_program::id(),
//         })
//         .args(hello_pyth::client::args::UpdateTriangularArbitrage {
//             starting_pair_feed_id:
//                 "0xe62df6c8b4a85fe1a67db44dc12de5db330f7ac66b72dc658afedf0f4a415b43".to_string(),
//             bridging_pair_feed_id:
//                 "0xc96458d393fe9deb7a7d63a0ac41e2898a67a7750dbd166673279e06c868df0a".to_string(),
//             crossing_pair_feed_id:
//                 "0xff61491a931112ddf1bd8147cd1b641375f79f5825126d665480874634fd0ace".to_string(),
//         })
//         .instruction()
//         .unwrap();

//     // 4. Execute and verify
//     ctx.execute_instruction(ix, &[&user])
//         .unwrap()
//         .assert_success();

//     ctx.svm.assert_account_exists(&user.pubkey());
// }
