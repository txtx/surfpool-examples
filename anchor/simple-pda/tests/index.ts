import * as anchor from "@coral-xyz/anchor";
import { SimplePda } from "../target/types/simple_pda";
import { Program, Wallet } from "@coral-xyz/anchor";
import { assert } from "chai";

describe("Call Run", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.SimplePda as Program<SimplePda>;
  const signer = provider.wallet as Wallet;

  it("Call it", async () => {
    const [pda, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("custom"), signer.publicKey.toBuffer()],
      program.programId
    );

    console.log("PDA: ", pda.toBase58());
    console.log("Bump: ", bump);

    const tx = await program.methods
      .run()
      .accountsStrict({
        sender: signer.publicKey,
        custom: pda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc({ commitment: "confirmed" });

    console.log("Your transaction signature", tx);
  });
});

describe("Call Run Again", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.SimplePda as Program<SimplePda>;
  const signer = provider.wallet as Wallet;

  it("Call it again", async () => {
    const [pda, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("custom"), signer.publicKey.toBuffer()],
      program.programId
    );

    console.log("PDA: ", pda.toBase58());
    console.log("Bump: ", bump);

    const tx = await program.methods
      .run()
      .accountsStrict({
        sender: signer.publicKey,
        custom: pda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc({ commitment: "confirmed" });

    console.log("Your transaction signature", tx);
  });

  it("Call it again bad", async () => {
    const [pda, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bad"), signer.publicKey.toBuffer()],
      program.programId
    );

    console.log("PDA: ", pda.toBase58());
    console.log("Bump: ", bump);

    try {
      const tx = await program.methods
        .run()
        .accountsStrict({
          sender: signer.publicKey,
          custom: pda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      assert(false, "Should have failed");
    } catch (e) {
      console.log("Expected error");
      return;
    }
  });
});
