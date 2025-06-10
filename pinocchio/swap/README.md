# DEX Swap Example 🔄

This example demonstrates token swapping functionality across different DEXs, featuring **DLMM swap operations** and **Pump AMM buy operations**.


## 🎯 Core Features

### 1. DLMM Swap Operations  
- Execute swap operations on Meteora DLMM (Dynamic Liquidity Market Maker)
- Liquidity bins-based price discovery
- Concentrated liquidity trading support

### 2. Pump AMM Buy Operations
- Execute token buy operations on Pump.fun AMM
- Support for SOL and other token swaps
- Automatic protocol and creator fee handling



## 📁 Project Structure

```
swap/
├── program/                # Rust program source
│   └── src/
│       ├── dex/            # DEX implementation modules
│       │   ├── dlmm/       # DLMM implementation
│       │   └── pump_amm/   # Pump AMM implementation
│       ├── instruction/    # Instruction handling
│       └── state/          # State management
├── client/                 # TypeScript client
│   └── src/
│       ├── arb_dlmm_test.ts    # DLMM test example
│       ├── arb_pump_test.ts    # Pump AMM test example
│       ├── utils.ts            # Utility functions
│       └── generated/          # Auto-generated types and instructions
└── README.md
```

## 🚀 Quick Start

### 1. Build Program

Ensure you are in the "program" folder
```sh
cd program
```

Build program
```sh
cargo build-sbf
```

Copy and change the keypair in "target" folder from the newly generated one to the one that came with this example
```sh
cp ../swap_program-keypair.json target/deploy/swap_program-keypair.json
```

Generate program address from the keypair copied above
```sh
solana address -k target/deploy/swap_program-keypair.json
```

Update the program ID with address generated from the keypair in /program/src/lib.rs
```bash
pinocchio_pubkey::declare_id!("D7Nv2Yt9i7r1xSGgTZo9zGHgZ8wwiAX13nFodBXdpox4");
```

### 2. Deploy Program To Localnet (Surfnet) With Surfpool

```sh
surfpool start --watch
```

### 3. Running Tests

Ensure you are in the "client" folder
```sh
cd client
```

Install dependencies
```sh
yarn install
```

#### DLMM Test  
```sh
yarn run dlmm-test
```

#### Pump AMM Test
```sh
yarn run pump-test
```


## 💡 Usage Examples

### Pump AMM Buy Example

```typescript
// Setup account parameters
const accounts: PumpAMMSwapInstructionAccounts = {
    signerAcc: payer.publicKey,
    baseMint: COMMON_TOKENS.SOL,
    mint: testTokenMint,
    userBaseMintAcc: userSolAccount,
    userMintAcc: userTestTokenAccount,
    pumpProgramId: PUMP_PROGRAM_ID,
    poolAcc: pumpPoolAddress,
    // ... other required accounts
};

// Create swap instruction
const swapArgs: PumpAMMSwapInstructionArgs = {
    arbitrageIxData: {
        dex: SupportDex.PumpAmm,
        maxBinToProcess: new BN(20),
        minProfitThreshold: new BN(10),
        noFailure: true,
    },
};

// Execute transaction
const ix = createPumpAMMSwapInstruction(accounts, swapArgs);
const transaction = new Transaction().add(ix);
```

### DLMM Swap Example

```typescript
// Setup DLMM accounts
const accounts: DLMMSwapInstructionAccounts = {
    signerAcc: payer.publicKey,
    lbPair: dlmmPairAddress,
    reserveX: reserveXAccount,
    reserveY: reserveYAccount,
    oracle: oracleAccount,
    bin1: binAccount1,
    bin2: binAccount2,
    bin3: binAccount3,
    // ... other accounts
};

// Create swap instruction
const swapArgs: DLMMSwapInstructionArgs = {
    arbitrageIxData: {
        dex: SupportDex.DLMM,
        maxBinToProcess: new BN(20),
        minProfitThreshold: new BN(10),
        noFailure: true,
    },
};

// Execute transaction
const ix = createDLMMSwapInstruction(accounts, swapArgs);
```

## 🔧 Key Parameters

- `maxBinToProcess`: Maximum number of liquidity bins to process (recommended: 20)
- `minProfitThreshold`: Minimum profit threshold (in lamports)
- `noFailure`: Whether to allow partial failures (recommended: true)

## 🛡️ Important Notes

1. **Slippage Protection**: Always set reasonable slippage limits
2. **Account Validation**: Verify all accounts validity and permissions
3. **Sufficient Balance**: Ensure adequate SOL and token balances

