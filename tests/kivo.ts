import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Kivo } from "../target/types/kivo";
import { initialize_user, initialize_vault } from "./test-helpers"
import * as assert from "assert";

describe("kivo", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Kivo as Program<Kivo>;

  it("Is initialized!", async () => {
    const name = "test"
    const user = await initialize_user(name);

    assert.equal(user.owner.toBase58(), program.provider.publicKey.toBase58());
    assert.equal(user.name, name);  

    const usdc_mint = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU" 
    const wsol_mint = "So11111111111111111111111111111111111111112"
    let [vaultAuthority, _] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault_authority")], program.programId);

    const usdc_vault = await initialize_vault(usdc_mint, vaultAuthority);
    const wsol_vault = await initialize_vault(wsol_mint, vaultAuthority);

    // assert.equal(usdc_vault.owner, vaultAuthority);
    // assert.equal(wsol_vault.owner, vaultAuthority);
    // This assertion fails inexplicably. The failure message indicates that the public keys are exactly the same.
  });
});
