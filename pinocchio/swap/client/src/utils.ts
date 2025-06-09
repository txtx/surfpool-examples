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
 * 从文件加载或创建新的密钥对
 * @param path 密钥对文件路径
 */
export async function getOrCreateKeypair(path: string): Promise<Keypair> {
    try {
        if (fs.existsSync(path)) {
            const keypairData = JSON.parse(fs.readFileSync(path, 'utf-8'));
            return Keypair.fromSecretKey(new Uint8Array(keypairData));
        } else {
            const keypair = Keypair.generate();
            fs.writeFileSync(path, JSON.stringify(Array.from(keypair.secretKey)));
            return keypair;
        }
    } catch (error) {
        console.error('加载密钥对时出错:', error);
        const keypair = Keypair.generate();
        return keypair;
    }
}

/**
 * 请求空投SOL
 * @param connection Solana连接对象
 * @param publicKey 接收空投的公钥
 * @param amount 空投数量（SOL）
 */
export async function requestAirdrop(
    connection: Connection, 
    publicKey: PublicKey, 
    amount: number = 2
): Promise<void> {
    console.log(`请求空投 ${amount} SOL 到 ${publicKey.toBase58()}`);
    const signature = await connection.requestAirdrop(publicKey, amount * LAMPORTS_PER_SOL);
    await connection.confirmTransaction(signature);
    console.log(`成功获得空投: ${signature}`);
}

/**
 * 确保钱包有足够的SOL余额
 * @param connection Solana连接对象
 * @param payer 支付账户
 * @param minBalance 最小余额（SOL）
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
 * 创建关联代币账户的指令（如果不存在）
 * @param connection Solana连接对象
 * @param payer 支付账户
 * @param mint 代币铸造地址
 * @param owner 代币账户所有者
 * @returns 创建ATA的指令（如果需要），否则返回null
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
 * 批量创建ATA并执行交易
 * @param connection Solana连接对象
 * @param payer 支付账户
 * @param mints 需要创建ATA的代币列表
 * @param owner ATA所有者
 * @returns ATA地址映射
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
    
    // 如果有ATA创建指令，执行交易
    if (transaction.instructions.length > 0) {
        console.log('批量创建ATA...');
        const { blockhash } = await connection.getLatestBlockhash();
        transaction.recentBlockhash = blockhash;
        transaction.feePayer = payer.publicKey;
        
        const signature = await connection.sendTransaction(transaction, [payer], {
            preflightCommitment: 'confirmed'
        });
        await connection.confirmTransaction(signature);
        console.log(`ATA创建成功: ${signature}`);
    }
    
    return ataMap;
}

/**
 * 常用代币地址
 */
export const COMMON_TOKENS = {
    SOL: new PublicKey('So11111111111111111111111111111111111111112'), // 原生SOL包装代币
    USDC: new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'), // USDC
    USDT: new PublicKey('Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB'), // USDT
};

/**
 * 常用程序ID
 */
export const COMMON_PROGRAM_IDS = {
    TOKEN_PROGRAM: TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM: ASSOCIATED_TOKEN_PROGRAM_ID,
    SYSTEM_PROGRAM: SystemProgram.programId,
    PUMP_AMM: new PublicKey('pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA'),
    DLMM: new PublicKey('LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo'), // Meteora DLMM 程序
};

/**
 * 将 SOL 包装成 wSOL 并转入关联代币账户
 * @param connection Solana连接对象
 * @param payer 支付账户
 * @param amount 要包装的SOL数量
 * @param owner wSOL ATA的所有者（默认为支付者）
 * @returns wSOL ATA地址
 */
export async function wrapSOLToATA(
    connection: Connection,
    payer: Keypair,
    amount: number,
    owner: PublicKey = payer.publicKey
): Promise<PublicKey> {
    console.log(`将 ${amount} SOL 包装成 wSOL...`);
    
    // 获取或创建 wSOL ATA
    const { instruction: createATAInstruction, address: wsolATA } = await createATAIfNeeded(
        connection,
        payer.publicKey,
        COMMON_TOKENS.SOL,
        owner
    );
    
    const transaction = new Transaction();
    
    // 如果需要创建 ATA，添加创建指令
    if (createATAInstruction) {
        transaction.add(createATAInstruction);
    }
    
    // 添加转账指令（将 SOL 转到 wSOL ATA）
    transaction.add(
        SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: wsolATA,
            lamports: amount * LAMPORTS_PER_SOL,
        })
    );
    
    // 执行交易
    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = payer.publicKey;
    
    try {
        const signature = await connection.sendTransaction(transaction, [payer], {
            preflightCommitment: 'confirmed'
        });
        await connection.confirmTransaction(signature);
        console.log(`成功将 ${amount} SOL 包装成 wSOL: ${signature}`);
        console.log(`wSOL ATA 地址: ${wsolATA.toBase58()}`);
        return wsolATA;
    } catch (error) {
        console.error('包装 SOL 时出错:', error);
        throw error;
    }
}

/**
 * 从 wSOL ATA 中解包装 SOL
 * @param connection Solana连接对象
 * @param payer 支付账户
 * @param owner wSOL ATA的所有者（默认为支付者）
 * @returns 交易签名
 */
export async function unwrapSOLFromATA(
    connection: Connection,
    payer: Keypair,
    owner: PublicKey = payer.publicKey
): Promise<string> {
    console.log('解包装 wSOL 为 SOL...');
    
    const wsolATA = await getAssociatedTokenAddress(COMMON_TOKENS.SOL, owner);
    
    try {
        // 检查 ATA 是否存在
        await getAccount(connection, wsolATA);
    } catch (error) {
        throw new Error(`wSOL ATA 不存在: ${wsolATA.toBase58()}`);
    }
    
    const transaction = new Transaction();
    
    // 添加关闭账户指令（这会将剩余的 wSOL 转换回 SOL）
    transaction.add(
        createCloseAccountInstruction(
            wsolATA,
            owner,
            owner,
            []
        )
    );
    
    // 执行交易
    const { blockhash } = await connection.getLatestBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = payer.publicKey;
    
    try {
        const signature = await connection.sendTransaction(transaction, [payer], {
            preflightCommitment: 'confirmed'
        });
        await connection.confirmTransaction(signature);
        console.log(`成功解包装 wSOL: ${signature}`);
        return signature;
    } catch (error) {
        console.error('解包装 wSOL 时出错:', error);
        throw error;
    }
}

/**
 * 创建设置计算单元限制的指令
 * @param units 计算单元限制数量
 * @returns 设置计算单元限制的指令
 */
export function createComputeUnitLimitInstruction(units: number): TransactionInstruction {
    console.log(`设置计算单元限制: ${units}`);
    return ComputeBudgetProgram.setComputeUnitLimit({
        units: units,
    });
}

/**
 * 创建设置计算单元价格的指令
 * @param microLamports 每个计算单元的价格（微lamports）
 * @returns 设置计算单元价格的指令
 */
export function createComputeUnitPriceInstruction(microLamports: number): TransactionInstruction {
    console.log(`设置计算单元价格: ${microLamports} micro-lamports`);
    return ComputeBudgetProgram.setComputeUnitPrice({
        microLamports: microLamports,
    });
}

/**
 * 为交易添加计算单元优化指令
 * @param transaction 要优化的交易
 * @param computeUnitLimit 计算单元限制（可选）
 * @param computeUnitPrice 计算单元价格（微lamports，可选）
 * @returns 添加了CU指令的交易
 */
export function addComputeUnitInstructions(
    transaction: Transaction,
    computeUnitLimit?: number,
    computeUnitPrice?: number
): Transaction {
    // 添加计算单元限制指令（如果提供）
    if (computeUnitLimit !== undefined) {
        const limitInstruction = createComputeUnitLimitInstruction(computeUnitLimit);
        transaction.instructions.unshift(limitInstruction); // 添加到开头
    }
    
    // 添加计算单元价格指令（如果提供）
    if (computeUnitPrice !== undefined) {
        const priceInstruction = createComputeUnitPriceInstruction(computeUnitPrice);
        transaction.instructions.unshift(priceInstruction); // 添加到开头
    }
    
    return transaction;
}

/**
 * 计算单元的常用预设值
 */
export const COMPUTE_UNIT_CONFIGS = {
    // 计算单元限制预设
    LIMITS: {
        LOW: 200000,      // 低复杂度交易
        MEDIUM: 400000,   // 中等复杂度交易
        HIGH: 800000,     // 高复杂度交易
        MAX: 1400000,     // 最大限制
    },
    // 计算单元价格预设（微lamports）
    PRICES: {
        NORMAL: 0,        // 正常网络状况
        FAST: 10000,      // 快速确认
        TURBO: 50000,     // 优先确认
        ULTRA: 100000,    // 超高优先级
    }
};
