import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solcraft } from "../target/types/solcraft";
import { BN } from "bn.js";
import { expect } from "chai";

function getFactoryPDA(program: Program<Solcraft>) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("factory_config")],
    program.programId
  )[0];
}

describe("solcraft", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solcraft as Program<Solcraft>;
  const payer = provider.wallet;
  const LAMPORTS_FEE = 0.1 * anchor.web3.LAMPORTS_PER_SOL;

  it("Factory Initialized!", async () => {
    await program.methods
      .initializeFactory(new BN(LAMPORTS_FEE))
      .accounts({
        admin: payer.publicKey,
      })
      .rpc();

    const factoryConfig = await program.account.factoryConfig.fetch(
      getFactoryPDA(program)
    );

    expect(factoryConfig.admin.toBase58()).to.equal(payer.publicKey.toBase58());
  });
});
