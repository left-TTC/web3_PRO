import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import { Web3Regitser } from "../target/types/web3Regitser";
import idl from "../target/idl/web3Regitser.json";
import { Idl } from '@project-serum/anchor';

import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

const VAULT_ACCOUNT = new PublicKey("GcWEQ9K78FV7LEHteFVciYApERk5YvQuFDQPk1yYJVXi");
const ROOT_DOMAIN_ACCOUNT = new PublicKey("58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx");
const AUCTION_PROGRAM_ID = new PublicKey("AVWV7vdWbLqXiLKFaP19GhYurhwxaLp2qRBSjT5tR5vT");
const WEB_NAMING_SERVICE = new PublicKey("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");

describe("web3register", () => {
  //set local net
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  //load program
  //const program = anchor.workspace.Web3Register as Program<Web3Regitser>;


  // //method to create a account
  // const createAccount = async (keypair: Keypair, space: number, ifsys: boolean) => {
  //   //get rent exempt
  //   //in solana, accounts must have sufficient SOL to be considered for rent exemption
  //   const lamports = await provider.connection.getMinimumBalanceForRentExemption(space);

  //   let ownership;
  //   if (ifsys){
  //     ownership = SystemProgram.programId;
  //     console.log("sysid:", ownership.toBase58());
  //   }else{
  //     ownership = program.programId;
  //   }

  //   const createAccountTx = await SystemProgram.createAccount({
  //     //payer to create the account
  //     fromPubkey: provider.wallet.publicKey,
  //     newAccountPubkey: keypair.publicKey,
  //     lamports,
  //     space,
  //     //maybe decide the owner
  //     programId:ownership,
  //   });

  //   const transaction = new anchor.web3.Transaction().add(createAccountTx);
  //   await provider.sendAndConfirm(transaction, [keypair]);
  // }

  //try to create a domian
  it("try to create domain", async () => {
    const web3NameService = Keypair.generate();
    const rootDomainAccount = Keypair.generate();
    const nameAccount = Keypair.generate();
    const vault = Keypair.generate();
    const buyer = Keypair.generate();
    const state = Keypair.generate();
    const reverseLookup = Keypair.generate();
    const centralState = Keypair.generate();
    const baseData = Keypair.generate();
    const initData = Keypair.generate();
    const splTokenProgram = Keypair.generate();
    const rentSysvar = Keypair.generate();
    const feePayer = provider.wallet; 
    const buyerTokenSource = Keypair.generate();
    const referrerOpt = Keypair.generate();
    
    console.log("already create")
    
  })

  it("should run a simple test", async () => {
    console.log("Anchor provider is set up.", provider);
    expect(true).to.equal(true); // 简单的断言，确保没有错误
    console.log("Available workspace keys:", anchor.workspace);

  });

  it("should successfully connect to the local Solana cluster", async () => {
    const connection = provider.connection;
    const version = await connection.getVersion();
    console.log("Solana cluster version:", version);
    expect(version).to.have.property("solana-core"); // 确保连接成功并返回版本信息
  });
});


