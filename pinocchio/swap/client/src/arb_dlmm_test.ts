import {
    Connection,
    PublicKey,
    Transaction,
    TransactionMessage,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import {
    PROGRAM_ID,
    createDLMMSwapInstruction,
    DLMMSwapInstructionAccounts,
    DLMMSwapInstructionArgs,
    SupportDex,
} from './generated';
import {BN} from 'bn.js';
import {
    getOrCreateKeypair,
    ensureSufficientBalance,
    setupATAs,
    COMMON_TOKENS,
    COMMON_PROGRAM_IDS,
    createComputeUnitLimitInstruction,
} from './utils';

// Connect to local testnet
const connection = new Connection('http://127.0.0.1:8899', 'confirmed');

async function testDLMMArbitrage() {
    try {
        // 1. Load keypair
        const payer = await getOrCreateKeypair('~/.config/solana/id.json');
        console.log(`Wallet: ${payer.publicKey.toBase58()}`);

        // 2. Ensure sufficient SOL balance
        await ensureSufficientBalance(connection, payer, 10);

        // 3. Setup token information
        const baseMint = COMMON_TOKENS.SOL; // SOL
        const testMint = new PublicKey('4eDf52YYzL6i6gbZ6FXqrLUPXbtP61f1gPSFM66M4XHe'); // Test token

        // 4. Batch create ATAs
        const ataMap = await setupATAs(connection, payer, [baseMint, testMint]);
        const userBaseMintAcc = ataMap.get(baseMint.toBase58())!;
        const userTestMintAcc = ataMap.get(testMint.toBase58())!;

        console.log("User SOL ATA:", userBaseMintAcc.toBase58());
        console.log("User test token ATA:", userTestMintAcc.toBase58());

        // 5. Setup DLMM related accounts (these need to be fetched from actual DLMM pool)
        const dlmmProgramId = COMMON_PROGRAM_IDS.DLMM;
        const dlmmEventAuthority = new PublicKey('D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6'); // Requires actual event authority address

        // DLMM pool related accounts (example addresses, need real pool addresses for actual use)
        const lbPair = new PublicKey('FBdKuQ5WxRgDMYFLq2NtJ16gkcWAoSwEH6wjZ8fCmpD3'); // LB Pair address
        const reserveX = new PublicKey('58dFe79BtN2oDPQpfAjx2efhRmcQfAHMj92rBrdnd72h'); // X reserve account
        const reserveY = new PublicKey('3awiBqZdTfGRZ5Fw2cAc7CzXVaSTS3XcmwKXUqyD8FhZ'); // Y reserve account
        const oracle = new PublicKey('5UueU2R5BqkafwLqUGqGu5a3pDWvE52qnY2MRNS7Axhm'); // Oracle account

        // Bin accounts (DLMM liquidity bins)
        const bin1 = new PublicKey('CSnujRLUDracMiPjtSUbKf3gQfj5PK37cJ4YPqZGLA6J');
        const bin2 = new PublicKey('GjA7YQGpWiWNvHHnVP1gnynZ5TaPk7bdKScWEAKCAfUg');
        const bin3 = new PublicKey('6R15VDKFzTrHv8vfU9JDKd3gWUGw5gA8wwpaGQsmKAiJ');

        // 6. Create DLMM account parameters
        const accounts: DLMMSwapInstructionAccounts = {
            signerAcc: payer.publicKey,
            baseMint: baseMint,
            feeCollectorAcc: userBaseMintAcc, // Use user as fee collector
            baseMintAcc: userBaseMintAcc,
            tokenProgramId: COMMON_PROGRAM_IDS.TOKEN_PROGRAM,
            systemProgramId: COMMON_PROGRAM_IDS.SYSTEM_PROGRAM,
            associatedTokenProgramId: COMMON_PROGRAM_IDS.ASSOCIATED_TOKEN_PROGRAM,
            mint: testMint,
            userMintAcc: userTestMintAcc,
            dlmmProgramId,
            dlmmEventAuthority,
            lbPair,
            reserveX,
            reserveY,
            oracle,
            bin1,
            bin2,
            bin3,
        };

        // 7. Create instruction data
        const arbArgs: DLMMSwapInstructionArgs = {
            arbitrageIxData: {
                dex: SupportDex.DLMM,
                maxBinToProcess: new BN(20), // Maximum number of bins to process
                minProfitThreshold: new BN(10), // Minimum profit threshold
                noFailure: true, // Do not allow failure
            },
        };

        // 8. Create instruction
        const ix = createDLMMSwapInstruction(accounts, arbArgs);
        ix.data = Buffer.from([0, ...ix.data.slice(1)]);

        console.log('Created DLMM instruction:', {
            programId: ix.programId.toBase58(),
            dataLength: ix.data.length,
            accountsCount: ix.keys.length,
        });

        // 9. Create and send transaction
        const limitIx = createComputeUnitLimitInstruction(400000);
        const arbTransaction = new Transaction();
        arbTransaction.add(limitIx, ix);

        console.log('Sending DLMM transaction...');
        const txSignature = await sendAndConfirmTransaction(
            connection,
            arbTransaction,
            [payer],
            {commitment: 'confirmed', skipPreflight: true}
        );

        console.log(`✅ DLMM Swap Success: ${txSignature}`);
        console.log(`🔍 tx: https://solscan.io/tx/${txSignature}?cluster=custom&customUrl=http://127.0.0.1:8899/`);

    } catch (error) {
        console.error('❌ Error executing DLMM transaction:', error);

        if (error instanceof Error) {
            console.error('Error stack:', error.stack);
        }
    }
}

/**
 * Get real DLMM pool information (optional feature)
 * This function demonstrates how to fetch real DLMM pool data
 */
async function getDLMMPoolInfo(poolAddress: PublicKey) {
    try {
        console.log('📊 Getting DLMM pool information...');

        // Should call DLMM SDK or query on-chain data directly
        // Example: Query pool account information
        const poolAccountInfo = await connection.getAccountInfo(poolAddress);

        if (poolAccountInfo) {
            console.log('Pool account information:', {
                owner: poolAccountInfo.owner.toBase58(),
                lamports: poolAccountInfo.lamports,
                dataLength: poolAccountInfo.data.length,
            });

            // Add logic here to parse pool data
            // const poolData = parsePoolData(poolAccountInfo.data);

        } else {
            console.log('Pool account information not found');
        }
    } catch (error) {
        console.error('Error getting pool information:', error);
    }
}

/**
 * Main function
 */
async function main() {
    console.log('🚀 Starting DLMM program...');
    console.log('Program ID:', PROGRAM_ID.toBase58());
    console.log('DLMM Program ID:', COMMON_PROGRAM_IDS.DLMM.toBase58());

    await testDLMMArbitrage();

    console.log('✨ DLMM program execution completed');
}

// Run main function
if (require.main === module) {
    main().catch(console.error);
}

// Export functions for use by other modules
export {
    testDLMMArbitrage,
    getDLMMPoolInfo,
};
