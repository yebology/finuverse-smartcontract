import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Smartcontract } from "../target/types/smartcontract";
import { SystemProgram, PublicKey } from "@solana/web3.js";
import { assert } from "chai";

describe("smartcontract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Smartcontract as Program<Smartcontract>;
  const systemProgram = SystemProgram.programId;
  const user = provider.wallet;

  it("can create a course and buy a course!", async () => {
    const now = Math.floor(new Date().getTime() / 1000);
    const courseId = new anchor.BN(now);
    const name: string = "Introduction to Web Development";
    const description: string = "Learn the basics of web development, including HTML, CSS, and JavaScript.";
    const price = new anchor.BN(100);
    const thumbnail: string = "https://example.com/course-thumbnail.jpg";
    const section_title: string[] = [
      "Getting Started with HTML",
      "CSS Fundamentals",
      "Introduction to JavaScript"
    ];
    const section_description: string[] = [
      "This section covers the basics of HTML, including tags.",
      "Learn how to style your web pages using CSS, including selectors.",
      "Understand the basics of JavaScript, including variables."
    ];
    const section_duration = [new anchor.BN(600), new anchor.BN(1200), new anchor.BN(1800)];
    const section_video: string[] = [
      "https://example.com/video.mp4",
      "https://example.com/video.mp4",
      "https://example.com/video.mp4"
    ];
    const question_list: string[] = [
      "What?",
      "What?",
      "What?"
    ];
    const answer_list: string[] = [
      "HTML is used for.",
      "CSS stands.",
      "DOM stands for."
    ];
    const first_answer_options: string[] = [
      "To style a web page",
      "Hypertext Markup Language",
      "CSS Selectors",
      "HTML is used for."
    ];
    const second_answer_options: string[] = [
      "To add interactivity",
      "Cascading Style Sheets",
      "JavaScript Functions",
      "CSS stands for."
    ];
    const third_answer_options: string[] = [
      "To manage databases",
      "Document Object Model",
      "JSON",
      "DOM stands for Document."
    ];

    const id = courseId.toBuffer("le", 8);
    const [coursePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("course"), user.publicKey.toBuffer(), id],
      program.programId
    );
    console.log(coursePda)

    await program.methods
      .createCourse(courseId, name, description, price, thumbnail, section_title, section_description, section_duration, section_video, question_list, answer_list, first_answer_options, second_answer_options, third_answer_options)
      .accounts({
        course: coursePda,
        creator: user.publicKey,
        systemProgram: systemProgram
      })
      .rpc();

    const [buyPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("buy"), user.publicKey.toBuffer(), id],
      program.programId
    );

    const fetchCourse = await program.account.course.fetch(coursePda);
    const to = fetchCourse.creator

    await program.methods
      .buyCourse(courseId)
      .accounts({
        buy: buyPda,
        course: coursePda,
        buyer: user.publicKey,
        to: to,
        systemProgram: systemProgram,
      })
      .rpc();

    const account = await program.account.buy.fetch(buyPda);
    assert.strictEqual(account.courseId.toString(), courseId.toString());
    assert.strictEqual(account.buyer.toString(), user.publicKey.toString());
  });
});