## Split Token Transfer 🔀

The goal of this example is to demonstrate a few key features of Surfpool:
 - on-the-fly mainnet forking
 - the `setup_surfpool` action
 - the automatic indexing of your programs via the subgraph command

### Building the example
First, ensure that surfpool is installed and up-to-date:

<details>
<summary>Initial Installation</summary>

```sh
# macOS (Homebrew)
brew tap txtx/taps
brew install surfpool

# Linux (Snapstore)
snap install surfpool
```
</details>

<details>
<summary>Update Installation</summary>

```bash
```sh
# macOS (Homebrew)
brew tap txtx/taps
brew reinstall surfpool

# Linux (Snapstore)
snap upgrade surfpool
```
</summary>
</details>

Then, run:
```sh
anchor build
```

*Note: Anchor v0.31.1 is recommended for this example*

Then, start your surfnet!
```sh
surfpool start --watch
```

The program will automatically be deployed to your local surfnet. You can executed the example by running:
```sh
surfpool run send-tokens -u
```

Or, if you want to run the supervised web UI to execute the example, run:
```sh
surfpool run send-tokens
```

This will allow you to enter in the recipient addresses, SPL token mint, token program, and the total amount of tokens to transfer via the web UI at runtime.

### Program Overview

This program takes a token mint, two recipient addresses, and some total amount of tokens to transfer. It then splits the total amount into two equal parts and transfers them to the two recipient addresses.

This is a pretty contrived example, but it showcases a few features of Surfpool.

#### Mainnet Forking
If this example was run on a local simnet that does not include mainnet forking, the SPL token that is being transferred would not exist, and the transaction would fail. However, surfpool forks mainnet out of the box, so the SPL token will exist and the transaction will succeed.

#### Setup Surfpool Action
Even with mainnet forking, there is a chance that the sender doesn't actually own any of the token being transferred. Without surfpool, in order to test transferring some token, you'd have to go through to process of downloading the SPL token account, then purchasing the token somehow.

However, surfpool includes some SVM hacks. The `setup_surfpool` action can be used to directly set an account's state, or to set an associated token account's state. For this project, in the [send-tokens](./runbooks/send-tokens.tx) runbook, we do the following:

```hcl
action "setup_surfnet" "svm::setup_surfnet" {
    set_token_account {
        public_key = signer.sender.public_key
        token = variable.mint
        amount = variable.amount
        token_program = variable.token_program
    }
}
```

This action will create an associated token account for the sender if it doesn't already exist, and then set the balance of that account to the specified amount of tokens. This allows us to test the transfer without having to go through the process of acquiring tokens.

#### Automatic Indexing
Finally, surfpool includes a subgraph command that will automatically index your programs. This is useful for querying the state of your program after running transactions.

Our auto-generated deployment runbook includes a [subgraphs.localnet.tx](./runbooks/deployment/subgraphs.localnet.tx) file with the following infrastructure as code:

```hcl
action "split_token_transfer_split_transfer_event" "svm::deploy_subgraph" {
    program_id = action.deploy_split_token_transfer.program_id
    program_idl = action.deploy_split_token_transfer.program_idl
    block_height = 0
    event {
        name = "SplitTransferEvent"
    }
}
```
This action will deploy a subgraph that listens for the `SplitTransferEvent` emitted by the `split_token_transfer` program. The subgraph will automatically index the events emitted by the program, allowing you to query the state of the program after running transactions.

When you start surfpool, it will automatically deploy the subgraph and start indexing events. You can view your subgraph at http://127.0.0.1:8900/gql/console.
