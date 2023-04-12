import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Kivo } from "../target/types/kivo";
import { initialize_user } from "./test-helpers"
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
  });
});
