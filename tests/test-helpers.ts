import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Kivo } from "../target/types/kivo"
import * as assert from "assert";
import { PublicKey } from "@solana/web3.js";
import { getAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Kivo as Program<Kivo>;

// Creates a User with the provided name as their username at the provided Keypair
// Does not yet check for duplicates... 
// May not need to check for duplicates if PDA mapping user <-> address is implemented.
export async function initialize_user(name : string, userAccount: anchor.web3.Keypair) {
    // const userAccount = anchor.web3.Keypair.generate();

    await program.methods
          .initializeUser(name)
          .accounts({
            owner: program.provider.publicKey,
            userAccount: userAccount.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([userAccount])
          .rpc();

    const user = await program.account.user.fetch(userAccount.publicKey);

    assert.equal(user.pubkey.toBase58(), userAccount.publicKey.toBase58());

    return user;
}

// Creates a Token Account at the provided Keypair from the provided mint and transfers the authority to the provided PublicKey.
export async function initialize_vault(mint: string, authority: PublicKey, vaultAccount: anchor.web3.Keypair) {
    // const vaultAccount = anchor.web3.Keypair.generate();

    await program.methods
          .initializeVault(authority)
          .accounts({
            vault: vaultAccount.publicKey,
            mint: mint,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([vaultAccount])
          .rpc();

    const vault = await getAccount(program.provider.connection, vaultAccount.publicKey);

    assert.equal(vault.address.toBase58(), vaultAccount.publicKey.toBase58());
    assert.equal(vault.mint, mint);

    return vault;
}