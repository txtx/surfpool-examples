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

// 连接到本地测试网
const connection = new Connection('http://127.0.0.1:8899', 'confirmed');


async function testDLMMArbitrage() {
    try {

        // 1. 加载密钥对
        const payer = await getOrCreateKeypair('~/.config/solana/id.json');
        console.log(`Wallet: ${payer.publicKey.toBase58()}`);

        // 2. 确保有足够的SOL
        await ensureSufficientBalance(connection, payer, 10);

        // 3. 设置代币信息
        const baseMint = COMMON_TOKENS.SOL; // SOL
        const testMint = new PublicKey('4eDf52YYzL6i6gbZ6FXqrLUPXbtP61f1gPSFM66M4XHe'); // 测试代币

        // 4. 批量创建ATA
        const ataMap = await setupATAs(connection, payer, [baseMint, testMint]);
        const userBaseMintAcc = ataMap.get(baseMint.toBase58())!;
        const userTestMintAcc = ataMap.get(testMint.toBase58())!;

        console.log("用户 SOL ATA:", userBaseMintAcc.toBase58());
        console.log("用户测试代币 ATA:", userTestMintAcc.toBase58());

        // 5. 设置 DLMM 相关账户（这些需要从实际的 DLMM 池获取）
        const dlmmProgramId = COMMON_PROGRAM_IDS.DLMM;
        const dlmmEventAuthority = new PublicKey('D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6'); // 需要实际的事件权限地址

        // DLMM 池相关账户（示例地址，实际使用时需要从真实池获取）
        const lbPair = new PublicKey('FBdKuQ5WxRgDMYFLq2NtJ16gkcWAoSwEH6wjZ8fCmpD3'); // LB Pair 地址
        const reserveX = new PublicKey('58dFe79BtN2oDPQpfAjx2efhRmcQfAHMj92rBrdnd72h'); // X储备金账户
        const reserveY = new PublicKey('3awiBqZdTfGRZ5Fw2cAc7CzXVaSTS3XcmwKXUqyD8FhZ'); // Y储备金账户
        const oracle = new PublicKey('5UueU2R5BqkafwLqUGqGu5a3pDWvE52qnY2MRNS7Axhm'); // 预言机账户

        // Bin 账户（DLMM 的流动性箱）
        const bin1 = new PublicKey('CSnujRLUDracMiPjtSUbKf3gQfj5PK37cJ4YPqZGLA6J');
        const bin2 = new PublicKey('GjA7YQGpWiWNvHHnVP1gnynZ5TaPk7bdKScWEAKCAfUg');
        const bin3 = new PublicKey('6R15VDKFzTrHv8vfU9JDKd3gWUGw5gA8wwpaGQsmKAiJ');

        // 6. 创建 DLMM 账户参数
        const accounts: DLMMSwapInstructionAccounts = {
            signerAcc: payer.publicKey,
            baseMint: baseMint,
            feeCollectorAcc: userBaseMintAcc, // 使用用户作为费用收集器
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

        // 7. 创建指令数据
        const arbArgs: DLMMSwapInstructionArgs = {
            arbitrageIxData: {
                dex: SupportDex.DLMM,
                maxBinToProcess: new BN(20), // 最大处理的 bin 数量
                minProfitThreshold: new BN(10), // 最小利润阈值
                noFailure: true, // 不允许失败
            },
        };

        // 8. 创建指令
        const ix = createDLMMSwapInstruction(accounts, arbArgs);

        ix.data = Buffer.from([0, ...ix.data.slice(1)]);

        console.log('创建的 DLMM 指令:', {
            programId: ix.programId.toBase58(),
            dataLength: ix.data.length,
            accountsCount: ix.keys.length,
        });

        // 9. 创建交易并发送
        const limitIx = createComputeUnitLimitInstruction(400000);
        // const priceIx = createComputeUnitPriceInstruction(10000);
        const arbTransaction = new Transaction();
        arbTransaction.add(limitIx, ix);

        console.log('发送 DLMM 交易...');
        const txSignature = await sendAndConfirmTransaction(
            connection,
            arbTransaction,
            [payer],
            {commitment: 'confirmed', skipPreflight: true}
        );

        console.log(`✅ DLMM Swap Success: ${txSignature}`);
        console.log(`🔍 tx: https://solscan.io/tx/${txSignature}?cluster=custom&customUrl=https://engine.mirror.ad/rpc/246317e7-f583-4769-b724-8f45cbf45f4b`);

    } catch (error) {
        console.error('❌ 执行 DLMM 交易时出错:', error);

        if (error instanceof Error) {
            console.error('错误堆栈:', error.stack);
        }
    }
}

/**
 * 获取真实的 DLMM 池信息（可选功能）
 * 这个函数展示了如何获取真实的 DLMM 池数据
 */
async function getDLMMPoolInfo(poolAddress: PublicKey) {
    try {
        console.log('📊 获取 DLMM 池信息...');

        // 这里应该调用 DLMM SDK 或直接查询链上数据
        // 示例：查询池账户信息
        const poolAccountInfo = await connection.getAccountInfo(poolAddress);

        if (poolAccountInfo) {
            console.log('池账户信息:', {
                owner: poolAccountInfo.owner.toBase58(),
                lamports: poolAccountInfo.lamports,
                dataLength: poolAccountInfo.data.length,
            });

            // 这里可以添加解析池数据的逻辑
            // const poolData = parsePoolData(poolAccountInfo.data);

        } else {
            console.log('未找到池账户信息');
        }
    } catch (error) {
        console.error('获取池信息时出错:', error);
    }
}

/**
 * 主函数
 */
async function main() {
    console.log('🚀 启动 DLMM 程序...');
    console.log('程序 ID:', PROGRAM_ID.toBase58());
    console.log('DLMM 程序 ID:', COMMON_PROGRAM_IDS.DLMM.toBase58());

    await testDLMMArbitrage();

    console.log('✨ DLMM 程序执行完成');
}

// 运行主函数
if (require.main === module) {
    main().catch(console.error);
}

// 导出函数供其他模块使用
export {
    testDLMMArbitrage,
    getDLMMPoolInfo,
};
