import { BN } from "bn.js";
import { expect } from "chai";
import * as anchor from "@coral-xyz/anchor";
import { airdropSol, user1, user2, program } from "./setup";
import {
  createMint,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  getAccount,
} from "@solana/spl-token";
import { getFaucetPDA } from "./pdas";

describe("Faucet", () => {
  let mint: anchor.web3.PublicKey;
  let user1Ata: anchor.web3.PublicKey;
  let faucetPda: anchor.web3.PublicKey;
  let treasuryAta: anchor.web3.PublicKey;

  before(async () => {
    // Airdrop some SOL to the user1 before tests
    await airdropSol(user1.publicKey, 2);
    await airdropSol(user2.publicKey, 2);

    mint = await createMint(
      program.provider.connection,
      user1.payer,
      user1.publicKey,
      null,
      6
    );

    const user1AtaAccount = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      user1.payer,
      mint,
      user1.publicKey
    );
    user1Ata = user1AtaAccount.address;

    await mintTo(
      program.provider.connection,
      user1.payer,
      mint,
      user1Ata,
      user1.publicKey,
      BigInt(10_000_000_000)
    );
  });

  it("Initialized!", async () => {
    faucetPda = getFaucetPDA(program);
    treasuryAta = await getAssociatedTokenAddress(mint, faucetPda, true);

    await program.methods
      .initializeFaucet()
      .accounts({
        mint: mint,
        owner: user1.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const faucetConfig = await program.account.faucetConfig.fetch(
      getFaucetPDA(program)
    );

    expect(faucetConfig.mint.toBase58()).to.eql(mint.toBase58());
    expect(faucetConfig.owner.toBase58()).to.eql(user1.publicKey.toBase58());
  });

  it("Deposit tokens into faucet!", async () => {
    const DEPOSIT_AMOUNT = new BN(5_000_000_000);

    await program.methods
      .depositToFaucet(DEPOSIT_AMOUNT)
      .accounts({
        depositor: user1.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const treasuryAccount = await getAccount(
      program.provider.connection,
      treasuryAta
    );

    expect(treasuryAccount.amount).to.eql(BigInt(DEPOSIT_AMOUNT.toString()));
  });

  it("Withdraw tokens from faucet!", async () => {
    const WITHDRAW_AMOUNT = new BN(2_000_000_000);

    const user1AtaInfoBefore = await getAccount(
      program.provider.connection,
      user1Ata
    );

    await program.methods
      .withdrawFromFaucet(WITHDRAW_AMOUNT)
      .accounts({
        recipient: user1.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    const treasuryAccountAfter = await getAccount(
      program.provider.connection,
      treasuryAta
    );

    const user1AtaInfoAfter = await getAccount(
      program.provider.connection,
      user1Ata
    );

    expect(treasuryAccountAfter.amount).to.eql(
      BigInt(3_000_000_000) // 5_000_000_000 - 2_000_000_000
    );

    expect(user1AtaInfoAfter.amount).to.eql(
      BigInt(user1AtaInfoBefore.amount + BigInt(WITHDRAW_AMOUNT.toString()))
    );
  });

  it("Any User can claim from faucet!", async () => {
    const user2AtaAccount = await getOrCreateAssociatedTokenAccount(
      program.provider.connection,
      user2,
      mint,
      user2.publicKey
    );
    const user2Ata = user2AtaAccount.address;

    const user2AtaInfoBefore = await getAccount(
      program.provider.connection,
      user2Ata
    );

    await program.methods
      .claimFromFaucet()
      .accounts({
        recipient: user2.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user2])
      .rpc();

    const treasuryAccountAfter = await getAccount(
      program.provider.connection,
      treasuryAta
    );

    const user2AtaInfoAfter = await getAccount(
      program.provider.connection,
      user2Ata
    );

    expect(treasuryAccountAfter.amount).to.eql(
      BigInt(2_000_000_000) // 300_000_000 - 100_000_000
    );

    expect(user2AtaInfoAfter.amount).to.eql(
      BigInt(user2AtaInfoBefore.amount + BigInt(1_000_000_000))
    );
  });
});
