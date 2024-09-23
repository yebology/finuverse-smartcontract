import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartcontract } from "../target/types/smartcontract";
import { SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("smartcontract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Smartcontract as Program<Smartcontract>;
  const systemProgram = SystemProgram.programId;
  const user = provider.wallet;

  it("can rate a course!", async () => {
    const now = Math.floor(new Date().getTime() / 1000);
    const id = new anchor.BN(now);
    const rating = new anchor.BN(4);

    const courseId = id.toBuffer("le", 8);
    const [ratePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rate"), user.publicKey.toBuffer(), courseId],
      program.programId
    );

    await program.methods
      .rateCourse(id, rating)
      .accounts({
        rate: ratePda,
        user: user.publicKey,
        systemProgram: systemProgram,
      })
      .rpc();

    const account = await program.account.rate.fetch(ratePda);
    assert.strictEqual(account.courseId.toString(), id.toString());
    assert.strictEqual(account.user.toString(), user.publicKey.toString());
    assert.strictEqual(account.rating.toString(), rating.toString());
  });

  it("throw error if rating is invalid!", async () => {
    const now = Math.floor(new Date().getTime() / 1000);
    const id = new anchor.BN(now);
    const rating = new anchor.BN(0);

    const courseId = id.toBuffer("le", 8);
    const [ratePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rate"), user.publicKey.toBuffer(), courseId],
      program.programId
    );

    try {
      await program.methods
        .rateCourse(id, rating)
        .accounts({
          rate: ratePda,
          user: user.publicKey,
          systemProgram: systemProgram,
        })
        .rpc();
        assert(false)
    } catch (err) {
      const errMsg = err.error.errorMessage;
      assert.strictEqual(errMsg, "Invalid rating.")
    }
  });

  it("throw error if course id is not valid!", async () => {
    const id = new anchor.BN(0);
    const rating = new anchor.BN(3);

    const courseId = id.toBuffer("le", 8);
    const [ratePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rate"), user.publicKey.toBuffer(), courseId],
      program.programId
    );

    try {
      await program.methods
        .rateCourse(id, rating)
        .accounts({
          rate: ratePda,
          user: user.publicKey,
          systemProgram: systemProgram,
        })
        .rpc();
        assert(false);
    } catch (err) {
      const errMsg = err.error.errorMessage;
      assert.strictEqual(errMsg, "Invalid course id.")
    }
  });
});
