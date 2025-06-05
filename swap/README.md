
```bash
cd program
cargo build-sbf
cp swap_program-keypair.json ./program/target/deploy/swap_program-keypair.json
```

- After build is successful get the program pubkey and replace with the pinocchio_pubkey::declare_id!(...)

```bash
solana address -k target/deploy/swap_program-keypair.json
```

### 4. Running Tests
```bash
yarn run pump-test
yarn run dlmm-test
```
