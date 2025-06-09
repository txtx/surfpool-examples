# DEX Swap Example 🔄

This example demonstrates token swapping functionality across different DEXs, featuring **Pump AMM buy operations** and **DLMM swap operations**.

## 🎯 Core Features

### 1. Pump AMM Buy Operations
- Execute token buy operations on Pump.fun AMM
- Support for SOL and other token swaps
- Automatic protocol and creator fee handling

### 2. DLMM Swap Operations  
- Execute swap operations on Meteora DLMM (Dynamic Liquidity Market Maker)
- Liquidity bins-based price discovery
- Concentrated liquidity trading support

## 📁 Project Structure

```
swap/
├── program/              # Rust program source
│   └── src/
│       ├── dex/         # DEX implementation modules
│       │   ├── pump_amm/    # Pump AMM implementation
│       │   └── dlmm/        # DLMM implementation
│       ├── instruction/ # Instruction handling
│       └── state/       # State management
├── client/              # TypeScript client
│   └── src/
│       ├── arb_pump_test.ts    # Pump AMM test example
│       ├── arb_dlmm_test.ts    # DLMM test example
│       ├── utils.ts            # Utility functions
│       └── generated/          # Auto-generated types and instructions
└── README.md
```

## 🚀 Quick Start

### 1. Setup Environment
```bash
# Install dependencies
yarn install

# Build program
cd program
cargo build-sbf
```

### 2. Deploy Program
```bash
# Copy keypair
cp swap_program-keypair.json ./program/target/deploy/swap_program-keypair.json

# Get program address
solana address -k target/deploy/swap_program-keypair.json

# Update program ID in lib.rs
```

### 3. Running Tests

#### Pump AMM Test
```bash
yarn run pump-test
```

#### DLMM Test  
```bash
yarn run dlmm-test
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

---
*Built with Surfpool Framework*