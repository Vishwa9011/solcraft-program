import * as anchor from "@coral-xyz/anchor";
import { BN, min } from "bn.js";
import { airdropSol, LAMPORTS_FEE, program, user1, user2 } from "./setup";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("Token", () => {
  before(async () => {
    // Airdrop some SOL to the user1 before tests
    await airdropSol(user1.publicKey, 2);
    await airdropSol(user2.publicKey, 1);

    await program.methods
      .initializeFactory(new BN(LAMPORTS_FEE))
      .accounts({
        admin: user1.publicKey,
      })
      .rpc();
  });

  it("Creates token and pays fee to factory treasury!", async () => {
    const TOKEN_NAME = "MyToken";
    const TOKEN_SYMBOL = "MTK";
    const TOKEN_DECIMALS = 6;
    const TOKEN_INITIAL_SUPPLY = 1_000_000;
    const TOKEN_URI = "https://example.com/token-metadata.json";

    const mint = anchor.web3.Keypair.generate();
    const payerAta = anchor.web3.Keypair.generate();

    await program.methods
      .createToken(
        TOKEN_NAME,
        TOKEN_SYMBOL,
        TOKEN_URI,
        TOKEN_DECIMALS,
        new BN(TOKEN_INITIAL_SUPPLY)
      )
      .accounts({
        mint: mint.publicKey,
        payer: user1.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([mint])
      .rpc();
  });
});
