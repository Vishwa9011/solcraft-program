import {
  getAssociatedTokenAddress,
  getAccount,
  getMint,
} from "@solana/spl-token";
import { BN } from "bn.js";
import * as anchor from "@coral-xyz/anchor";
import { airdropSol, program, provider, user1, user2 } from "./setup";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { expect } from "chai";

describe("Token", () => {
  let mintPubKey: anchor.web3.PublicKey;

  const TOKEN_NAME = "MyToken";
  const TOKEN_SYMBOL = "MTK";
  const TOKEN_DECIMALS = 6;
  const TOKEN_INITIAL_SUPPLY = 1_000_000;
  const TOKEN_INITIAL_SUPPLY_BN = new BN(TOKEN_INITIAL_SUPPLY).mul(
    new BN(10).pow(new BN(TOKEN_DECIMALS))
  );

  before(async () => {
    // Airdrop some SOL to the user1 before tests
    await airdropSol(user1.publicKey, 2);
    await airdropSol(user2.publicKey, 1);
  });

  it("Creates token and pays fee to factory treasury!", async () => {
    const TOKEN_URI = "https://example.com/token-metadata.json";

    const mint = anchor.web3.Keypair.generate();
    mintPubKey = mint.publicKey;

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

  it("Mint more tokens to token admin", async () => {
    const payer_ata = await getAssociatedTokenAddress(
      mintPubKey,
      user1.publicKey
    );

    const payer_ata_info_before = await getAccount(
      provider.connection,
      payer_ata
    );

    const MINT_AMOUNT = 500_000;
    const MINT_AMOUNT_BN = new BN(MINT_AMOUNT).mul(
      new BN(10).pow(new BN(6)) // Assuming 6 decimals
    );

    await program.methods
      .mintTokens(MINT_AMOUNT_BN)
      .accounts({
        mint: mintPubKey,
        recipient: user1.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const payer_ata_info_after = await getAccount(
      provider.connection,
      payer_ata
    );

    expect(payer_ata_info_after.amount).to.eql(
      BigInt(payer_ata_info_before.amount.toString()) +
        BigInt(MINT_AMOUNT_BN.toString())
    );
  });

  it("Transfer mint authority to another user", async () => {
    const mintInfoBefore = await getMint(provider.connection, mintPubKey);

    await program.methods
      .transferMintAuthority(user2.publicKey)
      .accounts({
        currentAuthority: user1.publicKey,
        mint: mintPubKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const mintInfoAfter = await getMint(provider.connection, mintPubKey);

    expect(mintInfoBefore.mintAuthority?.toBase58()).to.eql(
      user1.publicKey.toBase58()
    );
    expect(mintInfoAfter.mintAuthority?.toBase58()).to.eql(
      user2.publicKey.toBase58()
    );
  });

  it("Revoke mint authority", async () => {
    const mintInfoBefore = await getMint(provider.connection, mintPubKey);

    await program.methods
      .transferMintAuthority(null)
      .accounts({
        currentAuthority: user2.publicKey,
        mint: mintPubKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user2])
      .rpc();

    const mintInfoAfter = await getMint(provider.connection, mintPubKey);

    expect(mintInfoBefore.mintAuthority?.toBase58()).to.eql(
      user2.publicKey.toBase58()
    );
    expect(mintInfoAfter.mintAuthority).to.eql(null);
  });

  it("Fails to mint tokens after revoking mint authority", async () => {
    try {
      const MINT_AMOUNT = 100_000;
      const MINT_AMOUNT_BN = new BN(MINT_AMOUNT).mul(
        new BN(10).pow(new BN(6)) // Assuming 6 decimals
      );

      await program.methods
        .mintTokens(MINT_AMOUNT_BN)
        .accounts({
          mint: mintPubKey,
          recipient: user1.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      expect.fail("Minting should have failed due to revoke authority");
    } catch (error: any) {
      expect(error.message).to.include(
        "the total supply of this token is fixed"
      );
    }
  });
});
