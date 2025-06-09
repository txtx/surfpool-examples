import * as os from 'os';
import * as path from 'path';
import {
    Connection,
    Keypair,
    PublicKey,
    Transaction,
    SystemProgram,
    LAMPORTS_PER_SOL,
    TransactionInstruction,
    ComputeBudgetProgram,
} from '@solana/web3.js';
import {
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
    getAssociatedTokenAddress,
    createAssociatedTokenAccountInstruction,
    getAccount,
    createCloseAccountInstruction,
} from '@solana/spl-token';
import * as fs from 'fs';

/**
 * Load or create new keypair from file
 * @param filePath Path to keypair file
 */
export async function getOrCreateKeypair(filePath: string): Promise<Keypair> {
    try {
        // Resolve '~' to home directory
        const resolvedPath = filePath.replace(/^~/, os.homedir());
        const absolutePath = path.resolve(resolvedPath);

        if (fs.existsSync(absolutePath)) {
            const keypairData = JSON.parse(fs.readFileSync(absolutePath, 'utf-8'));
            return Keypair.fromSecretKey(new Uint8Array(keypairData));
        } else {
            console.log(`Keypair file not found at ${absolutePath}, generating new one...`);
            const keypair = Keypair.generate();
            // Ensure directory exists
            fs.mkdirSync(path.dirname(absolutePath), { recursive: true });
            fs.writeFileSync(absolutePath, JSON.stringify(Array.from(keypair.secretKey)));
            return keypair;
        }
    } catch (error) {
        console.error('Error loading keypair:', error);
        throw error; // Let's throw the error instead of silently generating new keypair
    }
}

/**
 * Request SOL airdrop
 * @param connection Solana connection object
 * @param publicKey Public key to receive airdrop
 * @param amount Airdrop amount (SOL)
 */
export async function requestAirdrop(
    connection: Connection, 
    publicKey: PublicKey, 
    amount: number = 2
): Promise<void> {
    console.log(`Requesting airdrop of ${amount} SOL to ${publicKey.toBase58()}`);
    const signature = await connection.requestAirdrop(publicKey, amount * LAMPORTS_PER_SOL);
    await connection.confirmTransaction(signature);
    console.log(`Airdrop successful: ${signature}`);
}

/**
 * Ensure wallet has sufficient SOL balance
 * @param connection Solana connection object
 * @param payer Payer account
 * @param minBalance Minimum balance (SOL)
 */
export async function ensureSufficientBalance(
    connection: Connection,
    payer: Keypair,
    minBalance: number = 1000
): Promise<void> {
    const balance = await connection.getBalance(payer.publicKey);
    console.log(`${payer.publicKey.toBase58()} SOL balance: ${balance / LAMPORTS_PER_SOL} SOL`);
    
    const wsolATA = await getAssociatedTokenAddress(COMMON_TOKENS.SOL, payer.publicKey);
    const wsolBalance = await connection.getBalance(wsolATA);
    console.log(`${wsolATA.toBase58()}  wSOL balance: ${wsolBalance / LAMPORTS_PER_SOL} SOL`);
    
    if (balance < minBalance * LAMPORTS_PER_SOL) {
        await requestAirdrop(connection, payer.publicKey, minBalance);
    }

    if (wsolBalance < minBalance * LAMPORTS_PER_SOL) {
        await requestAirdrop(connection, wsolATA, minBalance);
    }
}

/**
 * Create Associated Token Account instruction (if it doesn't exist)
 * @param connection Solana connection object
 * @param payer Payer account
 * @param mint Token mint address
 * @param owner Token account owner
 * @returns ATA creation instruction (if needed), otherwise returns null
 */
export async function createATAIfNeeded(
    connection: Connection,
    payer: PublicKey,
    mint: PublicKey,
    owner: PublicKey
): Promise<{ instruction: any; address: PublicKey } | { instruction: null; address: PublicKey }> {
    const ataAddress = await getAssociatedTokenAddress(mint, owner);
    
    try {
        await getAccount(connection, ataAddress);
        console.log(`ATA exist: ${ataAddress.toBase58()}`);
        return { instruction: null, address: ataAddress };
    } catch (error) {
        console.log(`create ATA: ${ataAddress.toBase58()}`);
        const instruction = createAssociatedTokenAccountInstruction(
            payer,
            ataAddress,
            owner,
            mint
        );
        return { instruction, address: ataAddress };
    }
}

/**
 * Batch create ATAs and execute transaction
 * @param connection Solana connection object
 * @param payer Payer account
 * @param mints Token mint list for ATA creation
 * @param owner ATA owner
 * @returns ATA address mapping
 */
export async function setupATAs(
    connection: Connection,
    payer: Keypair,
    mints: PublicKey[],
    owner: PublicKey = payer.publicKey
): Promise<Map<string, PublicKey>> {
    const transaction = new Transaction();
    const ataMap = new Map<string, PublicKey>();
    
    for (const mint of mints) {
        const { instruction, address } = await createATAIfNeeded(
            connection,
            payer.publicKey,
            mint,
            owner
        );
        
        ataMap.set(mint.toBase58(), address);
        
        if (instruction) {
            transaction.add(instruction);
        }
    }
    
    // If there are ATA creation instructions, execute transaction
    if (transaction.instructions.length > 0) {
        console.log('Creating batch ATAs...');
        const { blockhash } = await connection.getLatestBlockhash();
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = payer.publicKey;
        
        const signature = await connection.sendTransaction(transaction, [payer], {
            preflightCommitment: 'confirmed'
        });
        await connection.confirmTransaction(signature);
        console.log(`ATA creation successful: ${signature}`);
    }
    
    return ataMap;
}

/**
 * Common token addresses
 */
export const COMMON_TOKENS = {
    SOL: new PublicKey('So11111111111111111111111111111111111111112'), // Native SOL wrapped token
    USDC: new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'), // USDC
    USDT: new PublicKey('Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB'), // USDT
};

/**
 * Common program IDs
 */
export const COMMON_PROGRAM_IDS = {
    TOKEN_PROGRAM: TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM: ASSOCIATED_TOKEN_PROGRAM_ID,
    SYSTEM_PROGRAM: SystemProgram.programId,
    PUMP_AMM: new PublicKey('pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA'),
    DLMM: new PublicKey('LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo'), // Meteora DLMM Program
};

/**
 * Wrap SOL to wSOL and transfer to associated token account
 * @param connection Solana connection object
 * @param payer Payer account
 * @param amount Amount of SOL to wrap
 * @param owner wSOL ATA owner (defaults to payer)
 * @returns wSOL ATA address
 */
export async function wrapSOLToATA(
    connection: Connection,
    payer: Keypair,
    amount: number,
    owner: PublicKey = payer.publicKey
): Promise<PublicKey> {
    console.log(`Wrapping ${amount} SOL to wSOL...`);
    
    // Get or create wSOL ATA
    const { instruction: createATAInstruction, address: wsolATA } = await createATAIfNeeded(
        connection,
        payer.publicKey,
        COMMON_TOKENS.SOL,
        owner
    );
    
    const transaction = new Transaction();
    
    // If ATA creation needed, add instruction
    if (createATAInstruction) {
        transaction.add(createATAInstruction);
    }
    
    // Add transfer instruction (Transfer SOL to wSOL ATA)
    transaction.add(
        SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: wsolATA,
            lamports: amount * LAMPORTS_PER_SOL,
        })
    );
    
    // Execute transaction
    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = payer.publicKey;
    
    try {
        const signature = await connection.sendTransaction(transaction, [payer], {
            preflightCommitment: 'confirmed'
        });
        await connection.confirmTransaction(signature);
        console.log(`Successfully wrapped ${amount} SOL to wSOL: ${signature}`);
        console.log(`wSOL ATA address: ${wsolATA.toBase58()}`);
        return wsolATA;
    } catch (error) {
        console.error('Error wrapping SOL:', error);
        throw error;
    }
}

/**
 * Unwrap SOL from wSOL ATA
 * @param connection Solana connection object
 * @param payer Payer account
 * @param owner wSOL ATA owner (defaults to payer)
 * @returns Transaction signature
 */
export async function unwrapSOLFromATA(
    connection: Connection,
    payer: Keypair,
    owner: PublicKey = payer.publicKey
): Promise<string> {
    console.log('Unwrapping wSOL to SOL...');
    
    const wsolATA = await getAssociatedTokenAddress(COMMON_TOKENS.SOL, owner);
    
    try {
        // Check if ATA exists
        await getAccount(connection, wsolATA);
    } catch (error) {
        throw new Error(`wSOL ATA does not exist: ${wsolATA.toBase58()}`);
    }
    
    const transaction = new Transaction();
    
    // Add close account instruction (This converts remaining wSOL back to SOL)
    transaction.add(
        createCloseAccountInstruction(
            wsolATA,
            owner,
            owner,
            []
        )
    );
    
    // Execute transaction
    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = payer.publicKey;
    
    try {
        const signature = await connection.sendTransaction(transaction, [payer], {
            preflightCommitment: 'confirmed'
        });
        await connection.confirmTransaction(signature);
        console.log(`Successfully unwrapped wSOL: ${signature}`);
        return signature;
    } catch (error) {
        console.error('Error unwrapping wSOL:', error);
        throw error;
    }
}

/**
 * Create compute unit limit instruction
 * @param units Compute unit limit amount
 * @returns Compute unit limit instruction
 */
export function createComputeUnitLimitInstruction(units: number): TransactionInstruction {
    console.log(`Setting compute unit limit: ${units}`);
    return ComputeBudgetProgram.setComputeUnitLimit({
        units: units,
    });
}

/**
 * Create compute unit price instruction
 * @param microLamports Price per compute unit (in micro-lamports)
 * @returns Compute unit price instruction
 */
export function createComputeUnitPriceInstruction(microLamports: number): TransactionInstruction {
    console.log(`Setting compute unit price: ${microLamports} micro-lamports`);
    return ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: microLamports,
    });
}

/**
 * Add compute unit optimization instructions to transaction
 * @param transaction Transaction to optimize
 * @param computeUnitLimit Compute unit limit (optional)
 * @param computeUnitPrice Compute unit price in micro-lamports (optional)
 * @returns Transaction with added CU instructions
 */
export function addComputeUnitInstructions(
    transaction: Transaction,
    computeUnitLimit?: number,
    computeUnitPrice?: number
): Transaction {
    // Add compute unit limit instruction (if provided)
    if (computeUnitLimit !== undefined) {
        const limitInstruction = createComputeUnitLimitInstruction(computeUnitLimit);
        transaction.instructions.unshift(limitInstruction); // Add to beginning
    }
    
    // Add compute unit price instruction (if provided)
    if (computeUnitPrice !== undefined) {
        const priceInstruction = createComputeUnitPriceInstruction(computeUnitPrice);
        transaction.instructions.unshift(priceInstruction); // Add to beginning
    }
    
    return transaction;
}

/**
 * Common compute unit configurations
 */
export const COMPUTE_UNIT_CONFIGS = {
    // Compute unit limit presets
    LIMITS: {
        LOW: 200000,      // Low complexity transactions
        MEDIUM: 400000,   // Medium complexity transactions
        HIGH: 800000,     // High complexity transactions
        MAX: 1400000,     // Maximum limit
    },
    // Compute unit price presets (micro-lamports)
    PRICES: {
        NORMAL: 0,        // Normal network conditions
        FAST: 10000,      // Fast confirmation
        TURBO: 50000,     // Priority confirmation
        ULTRA: 100000,    // Ultra-high priority
    }
};
