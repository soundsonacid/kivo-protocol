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
    const userAccount = anchor.web3.Keypair.generate();
    let user = await initialize_user(name, userAccount);
    // assert.equal(user.owner.toBase58(), program.provider.publicKey.toBase58());
    // assert.equal(user.name, name);  

    // Create a second test user
    // const name2 = "test2"
    // const userAccount2 = anchor.web3.Keypair.generate();
    // const user2 = await initialize_user(name2, userAccount2);

    // // assert.equal(user2.owner.toBase58(), program.provider.publicKey.toBase58());
    // assert.equal(user.name, name);

    // Find a PDA to control vault pools
    // let [vaultAuthority, _] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault_authority")], program.programId);

    // // Create a USDC vault for the Program
    // const usdc_mint = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU" 
    // const usdc_vaultAccount = anchor.web3.Keypair.generate();
    // const usdc_vault = await initialize_vault(usdc_mint, vaultAuthority, usdc_vaultAccount);

    // assert.equal(usdc_vault.owner.toBase58(), vaultAuthority.toBase58());

    // // Create a wSOL vault for the Program 
    // const wsol_mint = "So11111111111111111111111111111111111111112"
    // const wsol_vaultAccount = anchor.web3.Keypair.generate();
    // const wsol_vault = await initialize_vault(wsol_mint, vaultAuthority, wsol_vaultAccount);

    // assert.equal(wsol_vault.owner.toBase58(), vaultAuthority.toBase58());

    // BN representing the amount we are "depositing"
    let amount = new BN(5.00);

    // assert.equal(amount.toNumber(), 5);
    // "Deposit" into a user account
    await program.methods
          .handleDeposit(amount)
          .accounts({
            userAccount: user.pubkey,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .rpc()

    user = await program.account.user.fetch(userAccount.publicKey);

    assert.equal(user.totalDeposits.toNumber(), amount.toNumber());
  });

});
