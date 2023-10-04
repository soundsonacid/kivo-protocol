import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Kivo } from "../target/types/kivo";
import * as assert from "assert";
import { getOrCreateAssociatedTokenAccount, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddress, createSyncNativeInstruction } from "@solana/spl-token";
import { BN } from "bn.js";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionMessage, VersionedTransaction, ComputeBudgetProgram, sendAndConfirmTransaction } from "@solana/web3.js";
import { getSignersFromTransaction, getAddressLookupTableAccounts, u32ToLittleEndianBytes, ToDecimal, UsernameToBytes, getQuote, getSwapIx, instructionDataToTransactionInstruction } from "./test-helpers";

describe("kivo", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Kivo as Program<Kivo>;

  const secret1 = new Uint8Array([254,155,255,45,248,92,179,39,148,200,207,243,81,75,194,89,240,63,239,3,62,168,147,119,234,21,7,0,166,42,180,49,249,255,33,255,71,10,137,238,240,90,142,143,198,50,220,221,99,40,12,96,140,68,77,192,61,117,136,34,62,157,192,83]);
  const KEYPAIR1 = Keypair.fromSecretKey(secret1);
  const USER1 = PublicKey.findProgramAddressSync([Buffer.from("user"), KEYPAIR1.publicKey.toBuffer()], program.programId)[0];
  const USERNAME1 = PublicKey.findProgramAddressSync([Buffer.from("username"), Buffer.from(UsernameToBytes("userone"))], program.programId)[0];

  const secret2 = new Uint8Array([92,38,5,51,107,45,43,74,235,80,128,156,138,166,3,32,237,199,117,15,0,16,165,135,168,17,145,33,225,72,220,211,212,121,255,19,101,207,234,186,28,72,199,15,59,119,233,18,114,177,146,138,14,167,1,153,72,180,164,209,67,221,72,234]);
  const KEYPAIR2 = Keypair.fromSecretKey(secret2);
  const USER2 = PublicKey.findProgramAddressSync([Buffer.from("user"), KEYPAIR2.publicKey.toBuffer()], program.programId)[0];
  const USERNAME2 = PublicKey.findProgramAddressSync([Buffer.from("username"), Buffer.from(UsernameToBytes("usertwo"))], program.programId)[0];

  const GROUP = PublicKey.findProgramAddressSync([Buffer.from("group"), USER1.toBuffer(), u32ToLittleEndianBytes(0)], program.programId)[0];
  const WSOL_MINT = new PublicKey("So11111111111111111111111111111111111111112");
  const USDC_MINT = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');
  const USDT_MINT = new PublicKey("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
  const UXD_MINT = new PublicKey("7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT");
  const BONK_MINT = new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");

  const USER1_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, USER1, true);
  const USER1_USDC_VAULT = await getAssociatedTokenAddress(USDC_MINT, USER1, true);
  const USER1_USDT_VAULT = await getAssociatedTokenAddress(USDT_MINT, USER1, true);
  const USER1_UXD_VAULT = await getAssociatedTokenAddress(UXD_MINT, USER1, true);
  const USER1_BONK_VAULT = await getAssociatedTokenAddress(BONK_MINT, USER1, true);

  const USER2_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, USER2, true);
  const USER2_USDC_VAULT = await getAssociatedTokenAddress(USDC_MINT, USER2, true);
  const USER2_USDT_VAULT = await getAssociatedTokenAddress(USDT_MINT, USER2, true);
  const USER2_UXD_VAULT = await getAssociatedTokenAddress(UXD_MINT, USER2, true);
  const USER2_BONK_VAULT = await getAssociatedTokenAddress(BONK_MINT, USER2, true);

  const GROUP_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, GROUP, true);
  const GROUP_USDC_VAULT = await getAssociatedTokenAddress(USDC_MINT, GROUP, true);
  const GROUP_USDT_VAULT = await getAssociatedTokenAddress(USDT_MINT, GROUP, true);
  const GROUP_UXD_VAULT = await getAssociatedTokenAddress(UXD_MINT, GROUP, true);
  const GROUP_BONK_VAULT = await getAssociatedTokenAddress(BONK_MINT, GROUP, true);

  const USER1_WSOL_BALANCE = PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP.toBuffer(), WSOL_MINT.toBuffer()], program.programId)[0];
  const USER2_WSOL_BALANCE = PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP.toBuffer(), WSOL_MINT.toBuffer()], program.programId)[0];

  const USER1_USDC_BALANCE = PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP.toBuffer(), USDC_MINT.toBuffer()], program.programId)[0];
  const USER2_USDC_BALANCE = PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP.toBuffer(), USDC_MINT.toBuffer()], program.programId)[0];

  const JUPITER = new PublicKey("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

  const TEMP_USDC_ACCOUNT = PublicKey.findProgramAddressSync([Buffer.from("temp"), GROUP.toBuffer(), USER1.toBuffer()], program.programId)[0];
  
  const secret3 = new Uint8Array([241,255,75,187,63,206,222,62,54,179,255,163,68,167,111,67,181,31,183,11,56,251,187,203,27,169,47,211,138,186,28,89,34,95,142,175,33,148,77,126,49,196,17,207,139,55,212,254,126,8,253,91,141,233,139,169,178,118,159,99,79,60,83,7]);
  const GROUP_KEYPAIR = Keypair.fromSecretKey(secret3);

  const secret4 = new Uint8Array([219,183,179,120,94,178,96,177,69,181,243,107,222,83,42,166,46,231,244,28,132,254,17,91,126,75,3,46,19,231,169,23,245,23,163,83,60,103,20,163,59,174,205,187,156,70,243,140,10,128,171,216,41,86,172,33,200,73,71,229,78,111,205,146]);
  const GROUP2_KEYPAIR = Keypair.fromSecretKey(secret4);

  const GROUP_KP_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, GROUP_KEYPAIR.publicKey, false);
  const GROUP_KP_USDC_VAULT = await getAssociatedTokenAddress(USDC_MINT, GROUP_KEYPAIR.publicKey, false);
  const GROUP_KP_USDT_VAULT = await getAssociatedTokenAddress(USDT_MINT, GROUP_KEYPAIR.publicKey, false);
  const GROUP_KP_UXD_VAULT = await getAssociatedTokenAddress(UXD_MINT, GROUP_KEYPAIR.publicKey, false);
  const GROUP_KP_BONK_VAULT = await getAssociatedTokenAddress(BONK_MINT, GROUP_KEYPAIR.publicKey, false);

  const GROUP_KP2_WSOL_VAULT = await getAssociatedTokenAddress(WSOL_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_USDC_VAULT = await getAssociatedTokenAddress(USDC_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_USDT_VAULT = await getAssociatedTokenAddress(USDT_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_UXD_VAULT = await getAssociatedTokenAddress(UXD_MINT, GROUP2_KEYPAIR.publicKey, false);
  const GROUP_KP2_BONK_VAULT = await getAssociatedTokenAddress(BONK_MINT, GROUP2_KEYPAIR.publicKey, false);

  const USER1_WSOL_GROUP_KP_BALANCE = PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP_KEYPAIR.publicKey.toBuffer(), WSOL_MINT.toBuffer()], program.programId)[0];
  const USER1_USDC_GROUP_KP_BALANCE = PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP_KEYPAIR.publicKey.toBuffer(), USDC_MINT.toBuffer()], program.programId)[0];

  const USER2_WSOL_GROUP_KP_BALANCE = PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP_KEYPAIR.publicKey.toBuffer(), WSOL_MINT.toBuffer()], program.programId)[0];
  const USER2_USDC_GROUP_KP_BALANCE = PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP_KEYPAIR.publicKey.toBuffer(), USDC_MINT.toBuffer()], program.programId)[0];

  const USER1_WSOL_GROUP_KP2_BALANCE = PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), WSOL_MINT.toBuffer()], program.programId)[0];
  const USER1_USDC_GROUP_KP2_BALANCE = PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), USDC_MINT.toBuffer()], program.programId)[0];

  const USER2_WSOL_GROUP_KP2_BALANCE = PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), WSOL_MINT.toBuffer()], program.programId)[0];
  const USER2_USDC_GROUP_KP2_BALANCE = PublicKey.findProgramAddressSync([USER2.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), USDC_MINT.toBuffer()], program.programId)[0];

  const KIVO_USDC_VAULT = new PublicKey("3VtZGaCBUges4R54DuWNM795wAfg6ChChvk4TFq34asj");
  const KIVO_WSOL_VAULT = new PublicKey("GrE2qLGwfbE9fnVfLjJiBKT8WN3fSrCF29hhcTPArmij");
  const KIVO_LST_VAULT = new PublicKey('3yhUfYWZmoKoCMQQ2LMjbuz232zxP3tne8U1JCiRkp7v');

  const LST_MINT = new PublicKey("LSTxxxnJzKDFSLr4dUkPcmCf5VyryEqzPLz5j4bpxFp");

  const USER1_LST_GROUP_KP2_BALANCE =  PublicKey.findProgramAddressSync([USER1.toBuffer(), GROUP2_KEYPAIR.publicKey.toBuffer(), LST_MINT.toBuffer()], program.programId)[0];
  
  const GROUP_KP2_LST_VAULT = await getAssociatedTokenAddress(LST_MINT, GROUP2_KEYPAIR.publicKey, false);

  it.skip("Initializes USER1 & USER2", async () => {
    await program.methods.handleInitializeUser(UsernameToBytes("userone"), 0)
        .accounts({
            usernameAccount: USERNAME1,
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
            payer: KEYPAIR1.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .signers([KEYPAIR1])
        .rpc()
        .then(() => console.log("USER1 vaults initialized successfully \n"))
        .catch((err) => console.error(`Failed to initialize USER1 vaults: ${err} \n`));

    await program.methods.handleInitializeUser(UsernameToBytes("usertwo"), 0)
        .accounts({
            usernameAccount: USERNAME2,
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
    const lamports = LAMPORTS_PER_SOL * 0.5;

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

    const user2TransferInstruction = SystemProgram.transfer({
        fromPubkey: KEYPAIR2.publicKey,
        toPubkey: USER2_WSOL_VAULT,
        lamports: lamports,
    });

    const user2SyncNative = createSyncNativeInstruction(USER2_WSOL_VAULT);

    const user2Transaction = new Transaction();

    user2Transaction.add(user2TransferInstruction);
    user2Transaction.add(user2SyncNative);

    const { blockhash: blockhash2 } = await program.provider.connection.getLatestBlockhash();

    user2Transaction.recentBlockhash = blockhash2;
    user2Transaction.feePayer = KEYPAIR2.publicKey;

    await sendAndConfirmTransaction(program.provider.connection, user2Transaction, [KEYPAIR2])
    .then((sig) => console.log(`Successfully funded USER2 with 0.5 SOL: ${sig}`))
    .catch((err) => console.error(`Failed to fund USER2 with 0.5 SOL: ${err}`))

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
            wsolMint: WSOL_MINT,
            usdcMint: USDC_MINT,
            usdtMint: USDT_MINT,
            uxdMint: UXD_MINT,
            bonkMint: BONK_MINT,
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
    const quote = await getQuote(WSOL_MINT, USDC_MINT, LAMPORTS_PER_SOL * 0.5);

    const res = await getSwapIx(GROUP2_KEYPAIR.publicKey, GROUP_KP2_USDC_VAULT, quote);

    if ("error" in res) {
        console.log({ res });
        return res;
    }

    const { computeBudgetInstructions, swapInstruction, addressLookupTableAddresses } = res;

    let swapIx = instructionDataToTransactionInstruction(swapInstruction);

    const instructions = [
        ...computeBudgetInstructions.map(instructionDataToTransactionInstruction),
        await program.methods.handleApe(ToDecimal(LAMPORTS_PER_SOL * 0.5), swapIx.data)
            .accounts({
                groupVault: GROUP_KP2_WSOL_VAULT,
                kivoVault: KIVO_USDC_VAULT,
                groupOutputVault: GROUP_KP2_USDC_VAULT,
                user: USER1,
                userInputBalance: USER1_WSOL_GROUP_KP2_BALANCE,
                userOutputBalance: USER1_USDC_GROUP_KP2_BALANCE,
                inputMint: WSOL_MINT,
                outputMint: USDC_MINT,
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

  it.skip("Freezes 0.2 SOL for LST from USER1 & GROUP_KP2_LST_VAULT", async () => {
    const quote = await getQuote(WSOL_MINT, LST_MINT, (LAMPORTS_PER_SOL * 0.2));

    const res = await getSwapIx(GROUP2_KEYPAIR.publicKey, GROUP_KP2_LST_VAULT, quote);

    if ('error' in res) {
        console.log({ res })
        return res;
    }

    const { computeBudgetInstructions, swapInstruction, addressLookupTableAddresses } = res;

    let swapIx = instructionDataToTransactionInstruction(swapInstruction);

    const instructions = [
        ...computeBudgetInstructions.map(instructionDataToTransactionInstruction),
        await program.methods.handleApe(ToDecimal(LAMPORTS_PER_SOL * 0.2), swapIx.data)
            .accounts({
                groupVault: GROUP_KP2_WSOL_VAULT,
                kivoVault: KIVO_LST_VAULT,
                groupOutputVault: GROUP_KP2_LST_VAULT,
                user: USER1,
                userInputBalance: USER1_WSOL_GROUP_KP2_BALANCE,
                userOutputBalance: USER1_LST_GROUP_KP2_BALANCE,
                inputMint: WSOL_MINT,
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
        .then((sig) => console.log(`Successfully froze 0.2 SOL to LST from USER1: ${sig}`))
        .catch((err) => console.error(`Failed to freeze 0.2 SOL to LST from USER1: ${err}`))
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
});
