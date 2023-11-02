import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sysvar } from "../target/types/sysvar";

describe("sysvar", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sysvar as Program<Sysvar>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        // pass the RecentBlockhashes sysvar public address to our instruction
        recentBlockhashes: anchor.web3.SYSVAR_SLOT_HISTORY_PUBKEY,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
// recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
// stakeHistory: anchor.web3.SYSVAR_STAKE_HISTORY_PUBKEY,
