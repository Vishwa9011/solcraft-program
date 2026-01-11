import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solcraft } from "../target/types/solcraft";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

export const user1 = provider.wallet;
export const user2 = anchor.web3.Keypair.generate();

export const program = anchor.workspace.solcraft as Program<Solcraft>;
export const LAMPORTS_FEE = 0.1 * anchor.web3.LAMPORTS_PER_SOL;

export async function airdropSol(
  publicKey: anchor.web3.PublicKey,
  amountSol: number
) {
  const sign = await provider.connection.requestAirdrop(
    publicKey,
    amountSol * anchor.web3.LAMPORTS_PER_SOL
  );

  await provider.connection.confirmTransaction(sign, "confirmed");
}
