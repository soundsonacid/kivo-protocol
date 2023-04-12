import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Kivo } from "../target/types/kivo"
import * as assert from "assert";

export async function initialize_user(name : string) {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Kivo as Program<Kivo>;
    const userAccount = anchor.web3.Keypair.generate();

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

    assert.equal(user.pubkey.toBase58(), userAccount.publicKey.toBase58()); // Redundancy but ok for now

    return user;
}