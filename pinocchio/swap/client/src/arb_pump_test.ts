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

// 连接到本地测试网
const connection = new Connection('http://127.0.0.1:8899', 'confirmed');

// 这些函数已移至 utils.ts

// 主函数
async function main() {
    try {
        console.log('=== 开始 Pump AMM 测试 ===');
        
        // 1. 加载密钥对
        const payer = await getOrCreateKeypair('~/.config/solana/id.json');
        console.log(`使用钱包地址: ${payer.publicKey.toBase58()}`);

        // 2. 确保有足够的SOL
        await ensureSufficientBalance(connection, payer, 10);

        // 3. 设置代币信息
        const baseMint = COMMON_TOKENS.SOL; // 原生SOL包装代币
        const testMint = new PublicKey('FtTSDNLD5mMLn3anqEQpy44cRdrtAJRrLX2MKXxfpump'); // 测试代币

        // 4. 批量创建ATA
        const ataMap = await setupATAs(connection, payer, [baseMint, testMint]);
        const userBaseMintAcc = ataMap.get(baseMint.toBase58())!;
        const userTestMintAcc = ataMap.get(testMint.toBase58())!;

        console.log("用户 SOL ATA:", userBaseMintAcc.toBase58());
        console.log("用户测试代币 ATA:", userTestMintAcc.toBase58());

        // 模拟Pump AMM参数
        const pumpProgramId = new PublicKey('pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA');
        const pumpGlobalConfigAcc = new PublicKey('ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw'); // 示例
        const pumpEventAuthorityAcc = new PublicKey('GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR'); // 示例
        const protocolFeeRecipient = new PublicKey('JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU'); // 示例

        // 模拟池账户
        const poolAcc = new PublicKey('6j6b2bG7MTbWjAgCBv4sLEFevqvRhiHAWKpe6Dz7PJnj'); // 示例
        const poolBaseTokenAcc = new PublicKey('HA4w7y2zGiMVttUFZe9HdDzbNpZh2YBv4MNMr2tsBWnq'); // 示例
        const poolQuoteTokenAcc = new PublicKey('B4YCF385oipgk4QAQM4q9zzPLPS8whW7NZ2Ebg1Wrros'); // 示例
        const protocolFeeRecipientTokenAcc = new PublicKey('DWpvfqzGWuVy9jVSKSShdM2733nrEsnnhsUStYbkj6Nn'); // 示例
        const coinCreatorVaultAta = new PublicKey('CnaQqEc5sbbFGNhn4Hde8b63CUPoYGKPuUNzfhbfU3UN'); // 示例
        const coinCreatorVaultAuthority = new PublicKey('2oY61WmvjLiBDe6A8n5w3hTsqt25upisRcbbXs5xRg95'); // 示例

        // 5. 创建账户参数
        const accounts: PumpAMMSwapInstructionAccounts = {
            signerAcc: payer.publicKey,
            baseMint: baseMint,
            feeCollectorAcc: payer.publicKey, // 使用用户作为费用收集器
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

        // 6. 创建指令数据
        const arbArgs: PumpAMMSwapInstructionArgs = {
            arbitrageIxData: {
                dex: SupportDex.PumpAmm,
                maxBinToProcess: new BN(20),
                minProfitThreshold: new BN(10),
                noFailure: true,
            },
        };

        // 7. 创建指令
        const ix = createPumpAMMSwapInstruction(accounts, arbArgs);

        // 手动修改指令数据，确保使用指令识别码 0
        ix.data = Buffer.from([0, ...ix.data.slice(1)]);

        console.log('创建的 Pump AMM 指令:', {
            programId: ix.programId.toBase58(),
            dataLength: ix.data.length,
            accountsCount: ix.keys.length,
        });

        // 8. 创建交易并发送
        const limitIx = createComputeUnitLimitInstruction(400000);
        // const priceIx = createComputeUnitPriceInstruction(10000);
        const arbTransaction = new Transaction();
        arbTransaction.add(limitIx,  ix);
        console.log('发送 Pump AMM 交易...');
        const txSignature = await sendAndConfirmTransaction(
            connection,
            arbTransaction,
            [payer],
            {commitment: 'confirmed', skipPreflight: true}
        );

        console.log(`✅ Pump AMM 交易成功: ${txSignature}`);
        console.log(`🔍 查看交易: https://explorer.solana.com/tx/${txSignature}?cluster=custom&customUrl=http://localhost:8899`);

    } catch (error) {
        console.error('❌ 执行 Pump AMM 交易时出错:', error);
        
        // 输出详细错误信息
        if (error instanceof Error) {
            // console.error('错误详情:', error.message);
            console.error('错误堆栈:', error.stack);
        }
    }
}

/**
 * 主函数入口
 */
async function mainEntry() {
    console.log('🚀 启动 Pump AMM 程序...');
    console.log('程序 ID:', PROGRAM_ID.toBase58());
    console.log('Pump AMM 程序 ID:', COMMON_PROGRAM_IDS.PUMP_AMM.toBase58());
    
    await main();
    
    console.log('✨ Pump AMM 程序执行完成');
}

// 运行主函数
if (require.main === module) {
    mainEntry().catch(console.error);
}

// 导出函数供其他模块使用
export {
    main as testPumpAMMArbitrage,
}; 