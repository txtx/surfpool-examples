import {
    Connection,
    PublicKey,
    Transaction,
    SystemProgram,
    LAMPORTS_PER_SOL,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import {
    PROGRAM_ID,
    createPumpAMMSwapInstruction,
    PumpAMMSwapInstructionAccounts,
    PumpAMMSwapInstructionArgs, 
    SupportDex,
} from './generated';
import { BN } from 'bn.js';
import {
    getOrCreateKeypair,
    ensureSufficientBalance,
    setupATAs,
    COMMON_TOKENS,
    COMMON_PROGRAM_IDS, createComputeUnitLimitInstruction,
} from './utils';

// Connect to local testnet
const connection = new Connection('http://127.0.0.1:8899', 'confirmed');

// These functions have been moved to utils.ts

// Main function
async function main() {
    try {
        console.log('=== Starting Pump AMM Test ===');
        
        // 1. Load keypair
        const payer = await getOrCreateKeypair('~/.config/solana/id.json');
        console.log(`Using wallet address: ${payer.publicKey.toBase58()}`);

        // 2. Ensure sufficient SOL
        await ensureSufficientBalance(connection, payer, 10);

        // 3. Setup token information
        const baseMint = COMMON_TOKENS.SOL; // Native SOL wrapped token
        const testMint = new PublicKey('FtTSDNLD5mMLn3anqEQpy44cRdrtAJRrLX2MKXxfpump'); // Test token

        // 4. Batch create ATAs
        const ataMap = await setupATAs(connection, payer, [baseMint, testMint]);
        const userBaseMintAcc = ataMap.get(baseMint.toBase58())!;
        const userTestMintAcc = ataMap.get(testMint.toBase58())!;

        console.log("User SOL ATA:", userBaseMintAcc.toBase58());
        console.log("User test token ATA:", userTestMintAcc.toBase58());

        // Simulate Pump AMM parameters
        const pumpProgramId = new PublicKey('pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA');
        const pumpGlobalConfigAcc = new PublicKey('ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw'); // Example
        const pumpEventAuthorityAcc = new PublicKey('GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR'); // Example
        const protocolFeeRecipient = new PublicKey('JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU'); // Example

        // Simulate pool accounts
        const poolAcc = new PublicKey('6j6b2bG7MTbWjAgCBv4sLEFevqvRhiHAWKpe6Dz7PJnj'); // Example
        const poolBaseTokenAcc = new PublicKey('HA4w7y2zGiMVttUFZe9HdDzbNpZh2YBv4MNMr2tsBWnq'); // Example
        const poolQuoteTokenAcc = new PublicKey('B4YCF385oipgk4QAQM4q9zzPLPS8whW7NZ2Ebg1Wrros'); // Example
        const protocolFeeRecipientTokenAcc = new PublicKey('DWpvfqzGWuVy9jVSKSShdM2733nrEsnnhsUStYbkj6Nn'); // Example
        const coinCreatorVaultAta = new PublicKey('CnaQqEc5sbbFGNhn4Hde8b63CUPoYGKPuUNzfhbfU3UN'); // Example
        const coinCreatorVaultAuthority = new PublicKey('2oY61WmvjLiBDe6A8n5w3hTsqt25upisRcbbXs5xRg95'); // Example

        // 5. Create account parameters
        const accounts: PumpAMMSwapInstructionAccounts = {
            signerAcc: payer.publicKey,
            baseMint: baseMint,
            feeCollectorAcc: payer.publicKey, // Use user as fee collector
            baseMintAcc: userBaseMintAcc,
            tokenProgramId: COMMON_PROGRAM_IDS.TOKEN_PROGRAM,
            systemProgramId: COMMON_PROGRAM_IDS.SYSTEM_PROGRAM,
            associatedTokenProgramId: COMMON_PROGRAM_IDS.ASSOCIATED_TOKEN_PROGRAM,
            mint: testMint,
            userMintAcc: userTestMintAcc,
            pumpProgramId,
            pumpGlobalConfigAcc,
            pumpEventAuthorityAcc,
            protocolFeeRecipient,
            poolAcc,
            poolBaseTokenAcc,
            poolQuoteTokenAcc,
            protocolFeeRecipientTokenAcc,
            coinCreatorVaultAta,
            coinCreatorVaultAuthority,
        };

        // 6. Create instruction data
        const arbArgs: PumpAMMSwapInstructionArgs = {
            arbitrageIxData: {
                dex: SupportDex.PumpAmm,
                maxBinToProcess: new BN(20),
                minProfitThreshold: new BN(10),
                noFailure: true,
            },
        };

        // 7. Create instruction
        const ix = createPumpAMMSwapInstruction(accounts, arbArgs);

        // Manually modify instruction data to ensure instruction code 0 is used
        ix.data = Buffer.from([0, ...ix.data.slice(1)]);

        console.log('Created Pump AMM instruction:', {
            programId: ix.programId.toBase58(),
            dataLength: ix.data.length,
            accountsCount: ix.keys.length,
        });

        // 8. Create and send transaction
        const limitIx = createComputeUnitLimitInstruction(400000);
        // const priceIx = createComputeUnitPriceInstruction(10000);
        const arbTransaction = new Transaction();
        arbTransaction.add(limitIx, ix);
        console.log('Sending Pump AMM transaction...');
        const txSignature = await sendAndConfirmTransaction(
            connection,
            arbTransaction,
            [payer],
            {commitment: 'confirmed', skipPreflight: true}
        );

        console.log(`✅ Pump AMM transaction successful: ${txSignature}`);
        console.log(`🔍 View transaction: https://explorer.solana.com/tx/${txSignature}?cluster=custom&customUrl=http://localhost:8899`);

    } catch (error) {
        console.error('❌ Error executing Pump AMM transaction:', error);
        
        // Output detailed error information
        if (error instanceof Error) {
            // console.error('Error details:', error.message);
            console.error('Error stack:', error.stack);
        }
    }
}

/**
 * Main function entry
 */
async function mainEntry() {
    console.log('🚀 Starting Pump AMM program...');
    console.log('Program ID:', PROGRAM_ID.toBase58());
    console.log('Pump AMM Program ID:', COMMON_PROGRAM_IDS.PUMP_AMM.toBase58());
    
    await main();
    
    console.log('✨ Pump AMM program execution completed');
}

// Run main function
if (require.main === module) {
    mainEntry().catch(console.error);
}

// Export functions for use by other modules
export {
    main as testPumpAMMArbitrage,
};