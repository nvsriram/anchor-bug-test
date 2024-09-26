import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorBugTest } from "../target/types/anchor_bug_test";
import { MainProgram } from "../target/types/main_program";
import { AnchorExample } from "../target/types/anchor_example";
import { BN } from "bn.js";
import { assert } from "chai";

describe("anchor-bug-test", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const signer = provider.wallet;

  const mainProgram = anchor.workspace.MainProgram as Program<MainProgram>;
  const ownerProgram = anchor.workspace.AnchorBugTest as Program<AnchorBugTest>;
  const exampleProgram = anchor.workspace
    .AnchorExample as Program<AnchorExample>;

  const [pdaAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("init-seed"), signer.publicKey.toBuffer()],
    ownerProgram.programId
  );
  const pool = anchor.web3.Keypair.generate();

  it("Is working!", async () => {
    const tx = await exampleProgram.methods
      .createPool()
      .accountsStrict({
        poolAccount: pool.publicKey,
        user: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([pool])
      .rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", tx);

    assert(
      (
        await ownerProgram.account.myData.fetch(pdaAccount, "confirmed")
      ).data.toNumber() == 42
    );
  });

  it("Is initialized!", async () => {
    const data = new BN(42);
    const tx = await ownerProgram.methods
      .initialize(data)
      .accountsStrict({
        pdaAccount,
        signer: signer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", tx);

    assert(
      (
        await ownerProgram.account.myData.fetch(pdaAccount, "confirmed")
      ).data.toNumber() == 42
    );
  });

  it("Is Valid!", async () => {
    const tx = await mainProgram.methods
      .initialize()
      .accountsStrict({
        pdaAccount,
        ownerProgram: ownerProgram.programId,
      })
      .rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", tx);
  });

  it("Is Set!", async () => {
    const data = new BN(1);
    const tx = await ownerProgram.methods
      .setData(data)
      .accountsStrict({
        pdaAccount,
        signer: signer.publicKey,
      })
      .rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", tx);

    assert(
      (
        await ownerProgram.account.myData.fetch(pdaAccount, "confirmed")
      ).data.toNumber() == 1
    );
  });

  it("Is Invalid!", async () => {
    const tx = await mainProgram.methods
      .initialize()
      .accountsStrict({
        pdaAccount,
        ownerProgram: ownerProgram.programId,
      })
      .rpc({ commitment: "confirmed" });
    console.log("Your transaction signature", tx);
  });
});
