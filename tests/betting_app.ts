import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BettingApp } from "../target/types/betting_app";

describe("betting_app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BettingApp as Program<BettingApp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
