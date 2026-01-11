import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solcraft } from "../target/types/solcraft";
import { BN } from "bn.js";
import { expect } from "chai";
import { airdropSol, LAMPORTS_FEE, user1, user2, program } from "./setup";

function getFactoryPDA(program: Program<Solcraft>) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("factory_config")],
    program.programId
  )[0];
}
function getTreasuryPDA(program: Program<Solcraft>) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("factory_treasury")],
    program.programId
  )[0];
}

describe("Factory", () => {
  before(async () => {
    // Airdrop some SOL to the user1 before tests
    await airdropSol(user1.publicKey, 2);
    await airdropSol(user2.publicKey, 1);
  });

  it("Initialized!", async () => {
    await program.methods
      .initializeFactory(new BN(LAMPORTS_FEE))
      .accounts({
        admin: user1.publicKey,
      })
      .rpc();

    const factoryConfig = await program.account.factoryConfig.fetch(
      getFactoryPDA(program)
    );

    expect(factoryConfig.admin.toBase58()).to.equal(user1.publicKey.toBase58());
    expect(factoryConfig.creationFeeLamports.toNumber()).to.equal(LAMPORTS_FEE);
  });

  it("Updates fee!", async () => {
    const NEW_FEE = LAMPORTS_FEE + 1000;

    await program.methods
      .updateCreationFee(new BN(NEW_FEE))
      .accounts({
        admin: user1.publicKey,
      })
      .rpc();

    const factoryConfig = await program.account.factoryConfig.fetch(
      getFactoryPDA(program)
    );

    expect(factoryConfig.creationFeeLamports.toNumber()).to.equal(NEW_FEE);
  });

  it("Fails to update fee from non-admin!", async () => {
    const NEW_FEE = LAMPORTS_FEE + 2000;
    try {
      await program.methods
        .updateCreationFee(new BN(NEW_FEE))
        .accounts({
          admin: user2.publicKey,
        })
        .signers([user2])
        .rpc({ commitment: "confirmed" });
      expect.fail("The transaction should have failed");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code;
      console.warn("Error code:", code);
    }
  });

  it("Pauses factory!", async () => {
    await program.methods
      .pauseFactory()
      .accounts({
        admin: user1.publicKey,
      })
      .rpc();
  });

  it("Unpauses factory!", async () => {
    await program.methods
      .unpauseFactory()
      .accounts({
        admin: user1.publicKey,
      })
      .rpc();
  });

  it("Fails to pause factory from non-admin!", async () => {
    try {
      await program.methods
        .pauseFactory()
        .accounts({
          admin: user2.publicKey,
        })
        .signers([user2])
        .rpc({ commitment: "confirmed" });
      expect.fail("The transaction should have failed");
    } catch (err: any) {
      const code = err?.error?.errorCode?.code;
      console.warn("Error code:", code);
    }
  });

  it("Withdraws fees!", async () => {
    const treasuryPDA = getTreasuryPDA(program);

    const depositIx = anchor.web3.SystemProgram.transfer({
      fromPubkey: user2.publicKey,
      toPubkey: treasuryPDA,
      lamports: LAMPORTS_FEE,
    });
    await program.provider.sendAndConfirm(
      new anchor.web3.Transaction().add(depositIx),
      [user2]
    );

    const initialAdminBalance = await program.provider.connection.getBalance(
      user1.publicKey
    );
    const treasuryBalance = await program.provider.connection.getBalance(
      treasuryPDA
    );

    await program.methods
      .withdrawFees()
      .accounts({
        admin: user1.publicKey,
      })
      .rpc();

    const finalAdminBalance = await program.provider.connection.getBalance(
      user1.publicKey
    );
    const finalTreasuryBalance = await program.provider.connection.getBalance(
      treasuryPDA
    );

    expect(finalTreasuryBalance).to.equal(0);
    expect(finalAdminBalance).to.be.greaterThan(initialAdminBalance);
  });
});
