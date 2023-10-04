import { BN } from "bn.js";
import { AddressLookupTableAccount, Connection, PublicKey, TransactionInstruction, Transaction, VersionedTransaction, MessageCompiledInstruction } from "@solana/web3.js";

const API_ENDPOINT = "https://quote-api.jup.ag/v6";

export function u32ToLittleEndianBytes(value) {
  if (value < 0 || value > 4294967295) {
    throw new Error("Value out of range for u32");
  }

  const buffer = new ArrayBuffer(4);
  const view = new DataView(buffer);
  
  view.setUint32(0, value, true);  // The 'true' here sets it to little-endian
  
  return new Uint8Array(buffer);
}

export const ToDecimal = (value: number) => {
  return new BN(value, 10);
};

export const UsernameToBytes = (username: string): number[] => {
  let usernameBytes = new Array(16).fill(0);

  for (let character = 0; character < username.length; character++) {
      usernameBytes[character] = username.charCodeAt(character);
}
  return usernameBytes;
};

export const getSwapIx = async (
  user: PublicKey,
  outputAccount: PublicKey,
  quote: any
) => {
  const data = {
    quoteResponse: quote,
    userPublicKey: user.toBase58(),
    destinationTokenAccount: outputAccount.toBase58(),
  };
  return fetch(`${API_ENDPOINT}/swap-instructions`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  }).then((response) => response.json());
};

export const getQuote = async (
  fromMint: PublicKey,
  toMint: PublicKey,
  amount: number
) => {
  return fetch(
    `${API_ENDPOINT}/quote?outputMint=${toMint.toBase58()}&inputMint=${fromMint.toBase58()}&amount=${amount}&slippage=0.5&onlyDirectRoutes=true&maxAccounts=50`
  ).then((response) => response.json());
};

export const instructionDataToTransactionInstruction = (
  instructionPayload: any
) => {
  if (instructionPayload === null) {
    return null;
  }

  return new TransactionInstruction({
    programId: new PublicKey(instructionPayload.programId),
    keys: instructionPayload.accounts.map((key) => ({
      pubkey: new PublicKey(key.pubkey),
      isSigner: key.isSigner,
      isWritable: key.isWritable,
    })),
    data: Buffer.from(instructionPayload.data, "base64"),
  });
};

export const getAddressLookupTableAccounts = async (
  keys: string[],
  connection: Connection
): Promise<AddressLookupTableAccount[]> => {
  const addressLookupTableAccountInfos =
    await connection.getMultipleAccountsInfo(
      keys.map((key) => new PublicKey(key))
    );

  return addressLookupTableAccountInfos.reduce((acc, accountInfo, index) => {
    const addressLookupTableAddress = keys[index];
    if (accountInfo) {
      const addressLookupTableAccount = new AddressLookupTableAccount({
        key: new PublicKey(addressLookupTableAddress),
        state: AddressLookupTableAccount.deserialize(accountInfo.data),
      });
      acc.push(addressLookupTableAccount);
    }

    return acc;
  }, new Array<AddressLookupTableAccount>());
};

export const getSignersFromTransaction = (transaction: VersionedTransaction): string[] => {
  const signers: string[] = [];

  // transaction.message.staticAccountKeys.forEach((key) => {
  //   if ()
  // })

  for (let i = 0; i < transaction.message.staticAccountKeys.length; i++) {
    if (transaction.message.isAccountSigner(i)) {
      signers.push(transaction.message.staticAccountKeys[i].toBase58())
    }
  }

  return signers;
};

    // await program.methods.handleFreeze(ToDecimal(LAMPORTS_PER_SOL / 2))
    //     .accounts({
    //         groupSolVault: GROUP_KP2_WSOL_VAULT,
    //         solBalance: USER1_WSOL_GROUP_KP2_BALANCE,
    //         lstBalance: USER1_LST_GROUP_KP2_BALANCE,
    //         lstMint: LST_MINT,
    //         stakePool: LST_POOL,
    //         stakePoolWithdrawAuthority: ,
    //         reserveStakeAccount: ,
    //         poolTokensTo: GROUP_KP2_LST_VAULT,
    //         managerFeeAccount: ,
    //         referrerPoolTokensAccount: ,
    //         group: GROUP2_KEYPAIR.publicKey,
    //         payer: KEYPAIR1.publicKey,
    //         stakePoolProgram: STAKE_POOL,
    //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //         tokenProgram: TOKEN_PROGRAM_ID,
    //         systemProgram: SystemProgram.programId
    //     })
    //     .signers([GROUP2_KEYPAIR, KEYPAIR1])
    //     .rpc()
    //     .then((sig) => console.log(`Successfully froze 0.5 SOL to LST from USER1: ${sig}`))
    //     .catch((err) => console.error(`Failed to freeze 0.5 SOL to LST from USER1: ${err}`))