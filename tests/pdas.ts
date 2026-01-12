import * as anchor from "@coral-xyz/anchor";
import { Solcraft } from "../target/types/solcraft";

export function getFactoryPDA(program: anchor.Program<Solcraft>) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("factory_config")],
    program.programId
  )[0];
}

export function getTreasuryPDA(program: anchor.Program<Solcraft>) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("factory_treasury")],
    program.programId
  )[0];
}

export function getFaucetPDA(program: anchor.Program<Solcraft>) {
  return anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("faucet_config")],
    program.programId
  )[0];
}
