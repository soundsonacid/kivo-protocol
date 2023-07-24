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

  });
});
