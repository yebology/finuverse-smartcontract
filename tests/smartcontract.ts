import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartcontract } from "../target/types/smartcontract";
import { SystemProgram } from "@solana/web3.js";

describe("smartcontract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Smartcontract as Program<Smartcontract>;
  const systemProgram = SystemProgram.programId;
  const user = provider.wallet;

  it("can buy a course!", async () => {
    const now = Math.floor(new Date().getTime() / 1000);
    const courseId = new anchor.BN(now);

    // const courseId = courseId.toBuffer("le", 8);
    // const [buyPda] = anchor.web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("buy"), user.publicKey.toBuffer(), courseId],
    //   program.programId
    // );

    const id = courseId.toBuffer("le", 8);
    const [coursePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("course"), creator.publicKey.toBuffer(), id],
      program.programId
    );

    // await program.methods
    //   .buyCourse(course_id)
    //   .accounts({
    //     buy: buyPda,
    //     course: cours
    //     user: user.publicKey,
    //     systemProgram: systemProgram,
    //   })
    //   .rpc();

    // const account = await program.account.rate.fetch(ratePda);
    // assert.strictEqual(account.courseId.toString(), id.toString());
    // assert.strictEqual(account.user.toString(), user.publicKey.toString());
    // assert.strictEqual(account.rating.toString(), rating.toString());
  });

});
