import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";
import { BN } from "bn.js";
import {
  airdropSol,
  LAMPORTS_FEE,
  program,
  provider,
  user1,
  user2,
} from "./setup";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { expect } from "chai";

describe("Token", () => {
  before(async () => {
    // Airdrop some SOL to the user1 before tests
    await airdropSol(user1.publicKey, 2);
    await airdropSol(user2.publicKey, 1);
  });

  it("Creates token and pays fee to factory treasury!", async () => {
    const TOKEN_NAME = "MyToken";
    const TOKEN_SYMBOL = "MTK";
    const TOKEN_DECIMALS = 6;
    const TOKEN_INITIAL_SUPPLY = 1_000_000;
    const TOKEN_INITIAL_SUPPLY_BN = new BN(TOKEN_INITIAL_SUPPLY).mul(
      new BN(10).pow(new BN(TOKEN_DECIMALS))
    );
    const TOKEN_URI = "https://example.com/token-metadata.json";

    const mint = anchor.web3.Keypair.generate();

    await program.methods
      .createToken(
        TOKEN_NAME,
        TOKEN_SYMBOL,
        TOKEN_URI,
        TOKEN_DECIMALS,
        TOKEN_INITIAL_SUPPLY_BN
      )
      .accounts({
        mint: mint.publicKey,
        payer: user1.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([mint])
      .rpc();

    const payer_ata = await getAssociatedTokenAddress(
      mint.publicKey,
      user1.publicKey
    );

    const payer_ata_info = await getAccount(provider.connection, payer_ata);

    expect(payer_ata_info.amount).to.eql(
      BigInt(TOKEN_INITIAL_SUPPLY_BN.toString())
    );
  });
});
