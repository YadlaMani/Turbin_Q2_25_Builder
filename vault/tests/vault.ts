import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vault as Program<Vault>;

  it("Is initialized!", async () => {
    const tx=await program.methods.initialize().accountsPartial({
      user:getProvider.wallet.publicKey,
      vaultState,
      
    })
  });
});
