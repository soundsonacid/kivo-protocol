import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Kivo } from "../target/types/kivo"
import * as assert from "assert";

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Kivo as Program<Kivo>;

// Creates a User with the provided name as their username with the provided Keypair's public key as the account's "owner"
// The actual account is controlled by a PDA derived from the user's name.
// This enforces uniqueness because the PDA generation will fail if the user's name is the same as a previously created user as the PDA will have already been created.
export async function initialize_user(name : string, client: anchor.web3.Keypair) {
  const seeds = [Buffer.from("user"), client.publicKey.toBuffer()];
  const [userPDA, _] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
  
    await program.methods
          .initializeUser(name)
          .accounts({
            owner: client.publicKey,
            userAccount: userPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([client])
          .rpc();

    const user = await program.account.user.fetch(userPDA);

    assert.equal(user.pubkey.toBase58(), userPDA.toBase58());
    console.log(`PDA: ${userPDA.toBase58()}`)
    console.log(`Client: ${client.publicKey.toBase58()}`)

    return user
}

