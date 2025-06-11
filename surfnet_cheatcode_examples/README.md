### Surfnet Cheatcodes

This repo contains examples of how to use various surfnet cheeatcodes to do various activities such as 

- Setting an account in the local state with `surfnet_setAccount`
- Setting a token account for a pubkey to mock it having a balance of the particular token with `surfnet_setTokenAccount`
- Profile a transaction to get the compute Units used with `surfnet_profileTransaction`
- Retrieve the profile results for a particular transaction using `surfnet_getProfileResults`


---

## ✅ How to Run Tests

Run `surfpool start` and a local test validator instance will be started at `http://localhost:8899`.

```bash
cargo test --package surfnet_cheatcodes_examples

cargo test test_set_account
cargo test test_set_token_account
cargo test test_profile_transaction
cargo test test_get_profile_results
