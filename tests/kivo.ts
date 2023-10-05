import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Kivo } from "../target/types/kivo";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddress, createSyncNativeInstruction } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionMessage, VersionedTransaction, sendAndConfirmTransaction } from "@solana/web3.js";
import { u64ToLEBytes, u32ToLittleEndianBytes, getAddressLookupTableAccounts, ToDecimal, UsernameToBytes, getQuote, getSwapIx, instructionDataToTransactionInstruction } from "./test-helpers";

describe("kivo", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Kivo as Program<Kivo>;

  const WSOL_MINT = new PublicKey("So11111111111111111111111111111111111111112");
  const USDC_MINT = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
  const USDT_MINT = new PublicKey("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
  const UXD_MINT = new PublicKey("7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT");
  const BONK_MINT = new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");
  const LST_MINT = new PublicKey("LSTxxxnJzKDFSLr4dUkPcmCf5VyryEqzPLz5j4bpxFp");
  const OPOS_MINT = new PublicKey("BqVHWpwUDgMik5gbTciFfozadpE2oZth5bxCDrgbDt52");

  const secret1 = new Uint8Array([254,155,255,45,248,92,179,39,148,200,207,243,81,75,194,89,240,63,239,3,62,168,147,119,234,21,7,0,166,42,180,49,249,255,33,255,71,10,137,238,240,90,142,143,198,50,220,221,99,40,12,96,140,68,77,192,61,117,136,34,62,157,192,83]);
  const KEYPAIR1 = Keypair.fromSecretKey(secret1);
  const USER1 = PublicKey.findProgramAddressSync([Buffer.from("user"), KEYPAIR1.publicKey.toBuffer()], program.programId)[0];

  const secret2 = new Uint8Array([92,38,5,51,107,45,43,74,235,80,128,156,138,166,3,32,237,199,117,15,0,16,165,135,168,17,145,33,225,72,220,211,212,121,255,19,101,207,234,186,28,72,199,15,59,119,233,18,114,177,146,138,14,167,1,153,72,180,164,209,67,221,72,234]);
  const KEYPAIR2 = Keypair.fromSecretKey(secret2);
  const USER2 = PublicKey.findProgramAddressSync([Buffer.from("user"), KEYPAIR2.publicKey.toBuffer()], program.programId)[0];

  const USER1_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, USER1, true);
  const USER1_USDC_VAULT = await getAssociatedTokenAddress(USDC_MINT, USER1, true);
  const USER1_USDT_VAULT = await getAssociatedTokenAddress(USDT_MINT, USER1, true);
  const USER1_UXD_VAULT = await getAssociatedTokenAddress(UXD_MINT, USER1, true);
  const USER1_BONK_VAULT = await getAssociatedTokenAddress(BONK_MINT, USER1, true);
  const USER1_LST_VAULT = await getAssociatedTokenAddress(LST_MINT, USER1, true);

  const USER2_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, USER2, true);
  const USER2_USDC_VAULT = await getAssociatedTokenAddress(USDC_MINT, USER2, true);
  const USER2_USDT_VAULT = await getAssociatedTokenAddress(USDT_MINT, USER2, true);
  const USER2_UXD_VAULT = await getAssociatedTokenAddress(UXD_MINT, USER2, true);
  const USER2_BONK_VAULT = await getAssociatedTokenAddress(BONK_MINT, USER2, true);
  const USER2_LST_VAULT = await getAssociatedTokenAddress(LST_MINT, USER2, true);

  const secret3 = new Uint8Array([219,183,179,120,94,178,96,177,69,181,243,107,222,83,42,166,46,231,244,28,132,254,17,91,126,75,3,46,19,231,169,23,245,23,163,83,60,103,20,163,59,174,205,187,156,70,243,140,10,128,171,216,41,86,172,33,200,73,71,229,78,111,205,146]);
  const GROUP2_KEYPAIR = Keypair.fromSecretKey(secret3);

  const GROUP_KP2_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_USDC_VAULT = await getAssociatedTokenAddress(USDC_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_USDT_VAULT = await getAssociatedTokenAddress(USDT_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_UXD_VAULT = await getAssociatedTokenAddress(UXD_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_BONK_VAULT = await getAssociatedTokenAddress(BONK_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_LST_VAULT = await getAssociatedTokenAddress(LST_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_OPOS_VAULT = await getAssociatedTokenAddress(OPOS_MINT, GROUP2_KEYPAIR.publicKey, false);

  const USER1_WSOL_GROUP_KP2_BALANCE = PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), WSOL_MINT.toBuffer()], program.programId)[0];
  const USER1_USDC_GROUP_KP2_BALANCE = PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), USDC_MINT.toBuffer()], program.programId)[0];
  const USER1_LST_GROUP_KP2_BALANCE =  PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), LST_MINT.toBuffer()], program.programId)[0];
  const USER1_OPOS_GROUP_KP2_BALANCE =  PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), OPOS_MINT.toBuffer()], program.programId)[0];

  const USER2_WSOL_GROUP_KP2_BALANCE = PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), WSOL_MINT.toBuffer()], program.programId)[0];
  const USER2_USDC_GROUP_KP2_BALANCE = PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), USDC_MINT.toBuffer()], program.programId)[0];
  const USER2_LST_GROUP_KP2_BALANCE =  PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), LST_MINT.toBuffer()], program.programId)[0];

  const KIVO_USDC_VAULT = new PublicKey("3VtZGaCBUges4R54DuWNM795wAfg6ChChvk4TFq34asj");
  const KIVO_WSOL_VAULT = new PublicKey("GrE2qLGwfbE9fnVfLjJiBKT8WN3fSrCF29hhcTPArmij");
  const KIVO_LST_VAULT = new PublicKey('3yhUfYWZmoKoCMQQ2LMjbuz232zxP3tne8U1JCiRkp7v');
  const KIVO_OPOS_VAULT = new PublicKey('GWnUtew1TFP6jxmsX4BuETQ41n1AVqQ6e9YwYdYpjCYG');

  const JUPITER = new PublicKey("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

  it.skip("Initializes USER1 & USER2", async () => {
    await program.methods.handleInitializeUser(UsernameToBytes("userone"))
        .accounts({
            userAccount: USER1,
            payer: KEYPAIR1.publicKey,
            owner: KEYPAIR1.publicKey,
            systemProgram: SystemProgram.programId,
        })
        .signers([KEYPAIR1])
        .rpc()
        .then(() => console.log("USER1 initialized successfully \n"))
        .catch((err) => console.error(`Failed to initialize USER1: ${err} \n`));

    await program.methods.handleInitializeUserVaults()
        .accounts({
            userAccount: USER1,
            wsolMint: WSOL_MINT,
            wsolVault: USER1_WSOL_VAULT,
            usdcMint:  USDC_MINT,
            usdcVault: USER1_USDC_VAULT,
            usdtMint: USDT_MINT,
            usdtVault: USER1_USDT_VAULT,
            uxdMint:  UXD_MINT,
            uxdVault: USER1_UXD_VAULT,
            bonkMint: BONK_MINT,
            bonkVault: USER1_BONK_VAULT,
            lstMint: LST_MINT,
            lstVault: USER1_LST_VAULT,
            payer: KEYPAIR1.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([KEYPAIR1])
        .rpc()
        .then(() => console.log("USER1 vaults initialized successfully \n"))
        .catch((err) => console.error(`Failed to initialize USER1 vaults: ${err} \n`));

    await program.methods.handleInitializeUser(UsernameToBytes("usertwo"))
        .accounts({
            userAccount: USER2,
            payer: KEYPAIR2.publicKey,
            owner: KEYPAIR2.publicKey,
            systemProgram: SystemProgram.programId,
        })
        .signers([KEYPAIR2])
        .rpc()
        .then(() => console.log("USER2 initialized successfully \n"))
        .catch((err) => console.error(`Failed to initialize USER2: ${err} \n`));

    await program.methods.handleInitializeUserVaults()
        .accounts({
            userAccount: USER2,
            wsolMint: WSOL_MINT,
            wsolVault: USER2_WSOL_VAULT,
            usdcMint:  USDC_MINT,
            usdcVault: USER2_USDC_VAULT,
            usdtMint: USDT_MINT,
            usdtVault: USER2_USDT_VAULT,
            uxdMint:  UXD_MINT,
            uxdVault: USER2_UXD_VAULT,
            bonkMint: BONK_MINT,
            bonkVault: USER2_BONK_VAULT,
            lstMint: LST_MINT,
            lstVault: USER2_LST_VAULT,
            payer: KEYPAIR2.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([KEYPAIR2])
        .rpc()
        .then(() => console.log("USER2 vaults initialized successfully \n"))
        .catch((err) => console.error(`Failed to initialize USER2 vaults: ${err} \n`));
  })

  it.skip("Deposits 0.5 SOL to USER1_WSOL_VAULT & USER2_WSOL_VAULT", async () => {
    const lamports = LAMPORTS_PER_SOL * 0.1;

    const user1TransferInstruction = SystemProgram.transfer({
        fromPubkey: KEYPAIR1.publicKey,
        toPubkey: USER1_WSOL_VAULT,
        lamports: lamports,
    });

    const user1SyncNative = createSyncNativeInstruction(USER1_WSOL_VAULT);

    const user1Transaction = new Transaction();

    user1Transaction.add(user1TransferInstruction);
    user1Transaction.add(user1SyncNative);

    const { blockhash: blockhash1 } = await program.provider.connection.getLatestBlockhash();

    user1Transaction.recentBlockhash = blockhash1;
    user1Transaction.feePayer = KEYPAIR1.publicKey;

    await sendAndConfirmTransaction(program.provider.connection, user1Transaction, [KEYPAIR1])
        .then((sig) => console.log(`Successfully funded USER1 with 0.5 SOL: ${sig}`))
        .catch((err) => console.error(`Failed to fund USER1 with 0.5 SOL: ${err}`))

    // const user2TransferInstruction = SystemProgram.transfer({
    //     fromPubkey: KEYPAIR2.publicKey,
    //     toPubkey: USER2_WSOL_VAULT,
    //     lamports: lamports,
    // });

    // const user2SyncNative = createSyncNativeInstruction(USER2_WSOL_VAULT);

    // const user2Transaction = new Transaction();

    // user2Transaction.add(user2TransferInstruction);
    // user2Transaction.add(user2SyncNative);

    // const { blockhash: blockhash2 } = await program.provider.connection.getLatestBlockhash();

    // user2Transaction.recentBlockhash = blockhash2;
    // user2Transaction.feePayer = KEYPAIR2.publicKey;

    // await sendAndConfirmTransaction(program.provider.connection, user2Transaction, [KEYPAIR2])
    // .then((sig) => console.log(`Successfully funded USER2 with 0.5 SOL: ${sig}`))
    // .catch((err) => console.error(`Failed to fund USER2 with 0.5 SOL: ${err}`))

  })

  it.skip("Initializes GROUP2_KEYPAIR as GROUP2 signed by GROUP2_KEYPAIR & USER1", async () => {
    await program.methods.handleGroupCreate()
        .accounts({
            groupAdmin: USER1,
            group: GROUP2_KEYPAIR.publicKey,
            payer: KEYPAIR1.publicKey,
            systemProgram: SystemProgram.programId
        })
        .signers([GROUP2_KEYPAIR, KEYPAIR1])
        .rpc()
        .then((sig) => console.log(`Successfully created GROUP2 signed by USER1: ${sig}`))
        .catch((err) => console.error(`Failed to create GROUP2 signed by USER1: ${err}`))


    await program.methods.handleGroupVaultsInit()
        .accounts({
            wsolVault: GROUP_KP2_WSOL_VAULT,
            usdcVault: GROUP_KP2_USDC_VAULT,
            usdtVault: GROUP_KP2_USDT_VAULT,
            uxdVault: GROUP_KP2_UXD_VAULT,
            bonkVault: GROUP_KP2_BONK_VAULT,
            lstVault: GROUP_KP2_LST_VAULT,
            wsolMint: WSOL_MINT,
            usdcMint: USDC_MINT,
            usdtMint: USDT_MINT,
            uxdMint: UXD_MINT,
            bonkMint: BONK_MINT,
            lstMint: LST_MINT,
            group: GROUP2_KEYPAIR.publicKey,
            payer: KEYPAIR1.publicKey,
            systemProgram: SystemProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([GROUP2_KEYPAIR, KEYPAIR1])
        .rpc()
        .then((sig) => console.log(`Successfully initialized vaults for GROUP2 signed by USER1: ${sig}`))
        .catch((err) => console.error(`Failed to initialized vaults for GROUP2 signed by USER1: ${err}`))
  })

  it.skip("Deposits 0.5 SOL to GROUP_KP2_WSOL_VAULT from USER1 & USER2", async () => {
    const deposit = ToDecimal(LAMPORTS_PER_SOL * 0.5)

    await program.methods.handleGroupDeposit(deposit)
        .accounts({
            group: GROUP2_KEYPAIR.publicKey,
            user: USER1,
            groupVault: GROUP_KP2_WSOL_VAULT,
            userVault: USER1_WSOL_VAULT,
            balance: USER1_WSOL_GROUP_KP2_BALANCE,
            mint: WSOL_MINT,
            payer: KEYPAIR1.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([KEYPAIR1])
        .rpc()
        .then((sig) => console.log(`USER1 successfully deposited 0.5 SOL to GROUP: ${sig}`))
        .catch((err) => console.error(`USER1 failed to deposit 0.5 SOL to GROUP: ${err}`))

    await program.methods.handleGroupDeposit(deposit)
        .accounts({
            group: GROUP2_KEYPAIR.publicKey,
            user: USER2,
            groupVault: GROUP_KP2_WSOL_VAULT,
            userVault: USER2_WSOL_VAULT,
            balance: USER2_WSOL_GROUP_KP2_BALANCE,
            mint: WSOL_MINT,
            payer: KEYPAIR2.publicKey,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([KEYPAIR2])
        .rpc()
        .then((sig) => console.log`USER2 successfully deposited 0.5 SOL to GROUP: ${sig}`)
        .catch((err) => console.error(`USER2 failed to deposit 0.5 SOL to GROUP: ${err}`))
  })

  it.skip("Swaps 0.5 SOL for USDC from USER1 to GROUP_KP2_USDC_VAULT", async () => {
    const quote = await getQuote(USDC_MINT, OPOS_MINT, 500000);

    const res = await getSwapIx(GROUP2_KEYPAIR.publicKey, GROUP_KP2_OPOS_VAULT, quote);

    if ("error" in res) {
        console.log({ res });
        return res;
    }

    const { computeBudgetInstructions, swapInstruction, addressLookupTableAddresses } = res;

    let swapIx = instructionDataToTransactionInstruction(swapInstruction);

    const instructions = [
        ...computeBudgetInstructions.map(instructionDataToTransactionInstruction),
        await program.methods.handleApe(ToDecimal(500000), swapIx.data)
            .accounts({
                groupVault: GROUP_KP2_USDC_VAULT,
                kivoVault: KIVO_OPOS_VAULT,
                groupOutputVault: GROUP_KP2_OPOS_VAULT,
                user: USER1,
                userInputBalance: USER1_USDC_GROUP_KP2_BALANCE,
                userOutputBalance: USER1_OPOS_GROUP_KP2_BALANCE,
                inputMint: USDC_MINT,
                outputMint: OPOS_MINT,
                group: GROUP2_KEYPAIR.publicKey,
                payer: KEYPAIR1.publicKey,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                jupiterProgram: JUPITER,
                systemProgram: SystemProgram.programId,
            })
            .signers([GROUP2_KEYPAIR, KEYPAIR1])
            .remainingAccounts(swapIx.keys)
            .instruction(),
    ];
    
    const { blockhash } = await program.provider.connection.getLatestBlockhash();

    const addressLookupTableAccounts = await getAddressLookupTableAccounts(
        addressLookupTableAddresses, program.provider.connection
    )
    const messageV0 = new TransactionMessage({
        payerKey: KEYPAIR1.publicKey,
        recentBlockhash: blockhash,
        instructions,
    }).compileToV0Message(addressLookupTableAccounts)

    const transaction = new VersionedTransaction(messageV0);

    transaction.sign([GROUP2_KEYPAIR, KEYPAIR1]);

    await program.provider.connection.sendTransaction(transaction)
        .then((sig) => console.log(`Successfully swapped 0.5 SOL to USDC from USER1: ${sig}`))
        .catch((err) => console.error(`Failed to swap 0.5 SOL to USDC from USER1: ${err}`))
  });

  it.skip("Preferred TX 0.3 SOL USER1 -> USDC USER2", async () => {
    const quote = await getQuote(WSOL_MINT, USDC_MINT, (LAMPORTS_PER_SOL * 0.3))

    const res = await getSwapIx(KEYPAIR1.publicKey, USER1_USDC_VAULT, quote);

    if ('error' in res) {
        console.log({ res })
        return res;
    }

    const { computeBudgetInstructions, swapInstruction, addressLookupTableAddresses } = res;

    let swapIx = instructionDataToTransactionInstruction(swapInstruction);

    const USER1_ACC = program.account.user.fetch(USER1);
    const USER2_ACC = program.account.user.fetch(USER2);
  
    const USER1_TX_ACCOUNT = PublicKey.findProgramAddressSync([Buffer.from('outgoing_tx'), USER1.toBuffer(), u32ToLittleEndianBytes((await USER1_ACC).incomingTx)], program.programId)[0];
    const USER2_TX_ACCOUNT = PublicKey.findProgramAddressSync([Buffer.from('incoming_tx'), USER2.toBuffer(), u32ToLittleEndianBytes((await USER2_ACC).incomingTx)], program.programId)[0];

    const KEYPAIR1_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, KEYPAIR1.publicKey, false);

    const instructions = [
        ...computeBudgetInstructions.map(instructionDataToTransactionInstruction),
        await program.methods.handlePreferredTxExec(ToDecimal(LAMPORTS_PER_SOL * 0.3), swapIx.data)
            .accounts({
                user: USER1,
                sourceVault: USER1_WSOL_VAULT,
                inputVault: KEYPAIR1_WSOL_VAULT,
                outputVault: USER1_USDC_VAULT,
                destinationOwner: USER2,
                destinationVault: USER2_USDC_VAULT,
                kivoVault: KIVO_USDC_VAULT,
                payerTxAccount: USER1_TX_ACCOUNT,
                receiverTxAccount: USER2_TX_ACCOUNT,
                inputMint: WSOL_MINT,
                payer: KEYPAIR1.publicKey,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                jupiterProgram: JUPITER,
                systemProgram: SystemProgram.programId,
            })
            .signers([KEYPAIR1])
            .remainingAccounts(swapIx.keys)
            .instruction(),
    ];

    const { blockhash } = await program.provider.connection.getLatestBlockhash();

    const addressLookupTableAccounts = await getAddressLookupTableAccounts(
        addressLookupTableAddresses, program.provider.connection
    );

    const messageV0 = new TransactionMessage({
        payerKey: KEYPAIR1.publicKey,
        recentBlockhash: blockhash,
        instructions,
    }).compileToV0Message(addressLookupTableAccounts);

    const transaction = new VersionedTransaction(messageV0);

    transaction.sign([KEYPAIR1])

    await program.provider.connection.sendTransaction(transaction)
        .then((sig) => console.log(`Successfully sent preferred TX 0.3 SOL USER1 -> USDC USER2: ${sig}`))
        .catch((err) => console.error(`Failed to send preferred TX 0.3 SOL USER1 -> USDC USER2: ${err}`))
  })

  it.skip("Splits 0.4 SOL swapped to USDC from USER2 to USER1", async () => {
    const quote = await getQuote(WSOL_MINT, USDC_MINT, (LAMPORTS_PER_SOL * 0.4));

    const res = await getSwapIx(GROUP2_KEYPAIR.publicKey, GROUP_KP2_USDC_VAULT, quote);

    if ('error' in res) {
        console.log({ res })
        return res;
    }

    const { computeBudgetInstructions, swapInstruction, addressLookupTableAddresses } = res;

    let swapIx = instructionDataToTransactionInstruction(swapInstruction);

    const instructions = [
        ...computeBudgetInstructions.map(instructionDataToTransactionInstruction),
        await program.methods.handleSwapSplit(ToDecimal(LAMPORTS_PER_SOL * 0.4), swapIx.data)
            .accounts({
                groupInputVault: GROUP_KP2_WSOL_VAULT,
                kivoVault: KIVO_USDC_VAULT,
                groupOutputVault: GROUP_KP2_USDC_VAULT,
                outputVault: USER1_USDC_VAULT,
                inputBalance: USER2_WSOL_GROUP_KP2_BALANCE,
                sender: USER2,
                receiver: USER1,
                inputMint: WSOL_MINT,
                outputMint: USDC_MINT,
                group: GROUP2_KEYPAIR.publicKey,
                payer: KEYPAIR2.publicKey,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                jupiterProgram: JUPITER,
                systemProgram: SystemProgram.programId
            })
            .signers([GROUP2_KEYPAIR, KEYPAIR2])
            .remainingAccounts(swapIx.keys)
            .instruction(),
    ];

    const { blockhash } = await program.provider.connection.getLatestBlockhash();

    const addressLookupTableAccounts = await getAddressLookupTableAccounts(
        addressLookupTableAddresses, program.provider.connection
    )

    const messageV0 = new TransactionMessage({
        payerKey: KEYPAIR2.publicKey,
        recentBlockhash: blockhash,
        instructions
    }).compileToV0Message(addressLookupTableAccounts);

    const transaction = new VersionedTransaction(messageV0);

    transaction.sign([GROUP2_KEYPAIR, KEYPAIR2]);

    await program.provider.connection.sendTransaction(transaction)
        .then((sig) => console.log(`Successfully split 0.4 SOL to USDC from USER2 to USER1: ${sig}`))
        .catch((err) => console.error(`Failed to split 0.4 SOL to USDC from USER2 to USER1: ${err}`))
  })

  it.skip("Freezes 3 USDC for LST from USER1 & GROUP_KP2_LST_VAULT", async () => {
    const quote = await getQuote(USDC_MINT, LST_MINT, 3000000);

    const res = await getSwapIx(GROUP2_KEYPAIR.publicKey, GROUP_KP2_LST_VAULT, quote);

    if ('error' in res) {
        console.log({ res })
        return res;
    }

    const { computeBudgetInstructions, swapInstruction, addressLookupTableAddresses } = res;

    let swapIx = instructionDataToTransactionInstruction(swapInstruction);

    const instructions = [
        ...computeBudgetInstructions.map(instructionDataToTransactionInstruction),
        await program.methods.handleApe(ToDecimal(3000000), swapIx.data)
            .accounts({
                groupVault: GROUP_KP2_USDC_VAULT,
                kivoVault: KIVO_LST_VAULT,
                groupOutputVault: GROUP_KP2_LST_VAULT,
                user: USER1,
                userInputBalance: USER1_USDC_GROUP_KP2_BALANCE,
                userOutputBalance: USER1_LST_GROUP_KP2_BALANCE,
                inputMint: USDC_MINT,
                outputMint: LST_MINT,
                group: GROUP2_KEYPAIR.publicKey,
                payer: KEYPAIR1.publicKey,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                jupiterProgram: JUPITER,
                systemProgram: SystemProgram.programId,
            })
            .signers([GROUP2_KEYPAIR, KEYPAIR1])
            .remainingAccounts(swapIx.keys)
            .instruction(),
    ];

    const { blockhash } = await program.provider.connection.getLatestBlockhash();

    const addressLookupTableAccounts = await getAddressLookupTableAccounts(
        addressLookupTableAddresses, program.provider.connection
    )

    const messageV0 = new TransactionMessage({
        payerKey: KEYPAIR1.publicKey,
        recentBlockhash: blockhash,
        instructions
    }).compileToV0Message(addressLookupTableAccounts);

    const transaction = new VersionedTransaction(messageV0);

    transaction.sign([GROUP2_KEYPAIR, KEYPAIR1]);

    await program.provider.connection.sendTransaction(transaction)
        .then((sig) => console.log(`Successfully froze 3 USDC to LST from USER1: ${sig}`))
        .catch((err) => console.error(`Failed to freeze 3 USDC to LST from USER1: ${err}`))
  })

  it.skip("Splits 2 USDC from USER1 to USER2", async () => {
    await program.methods.handleSplit(ToDecimal(2000000))
        .accounts({
            groupVault: GROUP_KP2_USDC_VAULT,
            sender: USER1,
            receiver: USER2,
            destinationVault: USER2_USDC_VAULT,
            userBalance: USER1_USDC_GROUP_KP2_BALANCE,
            mint: USDC_MINT,
            group: GROUP2_KEYPAIR.publicKey,
            payer: KEYPAIR1.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
        })
        .signers([GROUP2_KEYPAIR, KEYPAIR1])
        .rpc()
        .then((sig) => console.log(`Successfully split 2 USDC from USER1 to USER2: ${sig}`))
        .catch((err) => console.error(`Failed to split 2 USDC from USER1 to USER2: ${err}`))
  })

  it.skip("Withdraws all tokens from USER1 & USER2", async () => {
        const user1Vaults = [  
            USER2_USDC_VAULT, 
            USER2_USDT_VAULT, 
            USER2_UXD_VAULT, 
            USER2_BONK_VAULT,
            USER1_LST_VAULT
        ]

        const user2Vaults = [ 
            USER2_USDC_VAULT, 
            USER2_USDT_VAULT, 
            USER2_UXD_VAULT, 
            USER2_BONK_VAULT, 
            USER2_LST_VAULT
        ]

        const mints = [
            USDC_MINT,
            USDT_MINT,
            UXD_MINT,
            BONK_MINT,
            LST_MINT
        ]

        for (let i = 0; i < user1Vaults.length; i++) {
            await program.methods.handleWithdrawal(ToDecimal(1), true)
                .accounts({
                    withdrawer: KEYPAIR1.publicKey,
                    withdrawerTokenAccount: await getAssociatedTokenAddress(mints[i], KEYPAIR1.publicKey, false),
                    userAccount: USER1,
                    pdaTokenAccount: user1Vaults[i],
                    mint: mints[i],
                    payer: KEYPAIR1.publicKey,
                    systemProgram: SystemProgram.programId,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    tokenProgram: TOKEN_PROGRAM_ID
                })
                .signers([KEYPAIR1])
                .rpc()
                .then((sig) => console.log(`Successfully withdrew all ${mints[i]} from USER1: ${sig}`))
                .catch((err) => console.error(`Failed to withdraw all ${mints[i]} from USER1: ${err} `));
        }

        for (let i = 0; i < user2Vaults.length; i++) {
            await program.methods.handleWithdrawal(ToDecimal(1), true)
                .accounts({
                    withdrawer: KEYPAIR2.publicKey,
                    withdrawerTokenAccount: await getAssociatedTokenAddress(mints[i], KEYPAIR2.publicKey, false),
                    userAccount: USER2,
                    pdaTokenAccount: user2Vaults[i],
                    mint: mints[i],
                    payer: KEYPAIR2.publicKey,
                    systemProgram: SystemProgram.programId,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    tokenProgram: TOKEN_PROGRAM_ID
                })
                .signers([KEYPAIR2])
                .rpc()
                .then((sig) => console.log(`Successfully withdrew all ${mints[i]} from USER1: ${sig}`))
                .catch((err) => console.error(`Failed to withdraw all ${mints[i]} from USER1: ${err} `));
        }

        const USER1_ACC = program.account.user.fetch(USER1);
        const USER2_ACC = program.account.user.fetch(USER2);
      
        const USER1_TEMP_TOKEN_ACCOUNT = PublicKey.findProgramAddressSync([Buffer.from("unwrap"), USER1.toBuffer(), u64ToLEBytes(BigInt((await USER1_ACC).totalWithdraws))], program.programId)[0];
        const USER2_TEMP_TOKEN_ACCOUNT = PublicKey.findProgramAddressSync([Buffer.from("unwrap"), USER2.toBuffer(), u64ToLEBytes(BigInt((await USER2_ACC).totalWithdraws))], program.programId)[0];

        
        await program.methods.handleUnwrapWithdrawal(ToDecimal(1), true)
            .accounts({
                withdrawer: KEYPAIR1.publicKey,
                userAccount: USER1,
                userTokenAccount: USER1_WSOL_VAULT,
                temporaryTokenAccount: USER1_TEMP_TOKEN_ACCOUNT,
                mint: WSOL_MINT,
                payer: KEYPAIR1.publicKey,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID
            })
            .signers([KEYPAIR1])
            .rpc()
            .then((sig) => console.log(`Successfully withdrew all WSOL from USER1: ${sig}`))
            .catch((err) => console.error(`Failed to withdraw all WSOL from USER1: ${err} `));

        await program.methods.handleUnwrapWithdrawal(ToDecimal(1), true)
            .accounts({
                withdrawer: KEYPAIR2.publicKey,
                userAccount: USER2,
                userTokenAccount: USER2_WSOL_VAULT,
                temporaryTokenAccount: USER2_TEMP_TOKEN_ACCOUNT,
                mint: WSOL_MINT,
                payer: KEYPAIR2.publicKey,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID
            })
            .signers([KEYPAIR2])
            .rpc()
            .then((sig) => console.log(`Successfully withdrew all WSOL from USER2: ${sig}`))
            .catch((err) => console.error(`Failed to withdraw all WSOL from USER2: ${err} `));
  })

  it("Withdraw USDC", async () => {
//     await program.methods.handleGroupWithdrawal(ToDecimal(37000000), null)
//         .accounts({
//             groupVault: GROUP_KP2_USDC_VAULT,
//             user: USER1,
//             userVault: USER1_USDC_VAULT,
//             userBalance: USER1_USDC_GROUP_KP2_BALANCE,
//             mint: USDC_MINT,
//             group: GROUP2_KEYPAIR.publicKey,
//             payer: KEYPAIR1.publicKey,
//             tokenProgram: TOKEN_PROGRAM_ID,
//             associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
//             systemProgram: SystemProgram.programId,
//         })
//         .signers([GROUP2_KEYPAIR, KEYPAIR1])
//         .rpc()
//         .then((sig) => console.log(`${sig}`))
//         .catch((err) => console.error(`${err}`))

    await program.methods.handleWithdrawal(ToDecimal(0), true)
        .accounts({
            withdrawer: KEYPAIR1.publicKey,
            withdrawerTokenAccount: new PublicKey("6XZ4XSz6C7NSaG1xeN5R18x6GKs2wivANGrJSEytwMRN"),
            userAccount: USER1,
            pdaTokenAccount: USER1_USDC_VAULT,
            mint: USDC_MINT,
            payer: KEYPAIR1.publicKey,
            systemProgram: SystemProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([KEYPAIR1])
        .rpc()
        .then((sig) => console.log(`${sig}`))
        .catch((err) => console.error(`${err}`))
  })
});
