import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Kivo } from "../target/types/kivo";
import { initialize_user } from "./test-helpers"
import * as assert from "assert";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN } from "bn.js";

describe("kivo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Kivo as Program<Kivo>;

  it("Is initialized!", async () => {
    // Create a test User
    const name = "fixthis"
    const client = anchor.web3.Keypair.generate();
    let  { user, userPDA } = await initialize_user(name, client);
    
    assert.equal(user.owner.toBase58(), client.publicKey.toBase58());
    assert.equal(user.name, name);  

    let amount = new BN(5.00);      // BN representing the amount we are "depositing"

    // "Deposit" into a user account
    await program.methods
          .handleDeposit(amount)
          .accounts({
            userAccount: user.pubkey,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .rpc()

    user = await program.account.user.fetch(userPDA);   // Refresh our user account

    assert.equal(user.totalDeposits.toNumber(), amount.toNumber());
  });
});
