import * as anchor from "@coral-xyz/anchor";
import { Buffer } from "buffer";
import { Program, BN } from "@coral-xyz/anchor";
import { sha256 } from 'js-sha256';
import { Web3Regitser } from "../target/types/web3Regitser";
import testPayerKeypair from "/home/f/wallet/test1.json";
import { PublicKey, Keypair } from "@solana/web3.js";

export const WEB3_NAME_SERVICE_ID = new PublicKey("F6PrVaeL2TegV2fmXFGW1dQeCXCCBZffn1JT5PUEiPMM");

describe("web3Register", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Web3Regitser as Program<Web3Regitser>;

    const web3NameServiceKey = new PublicKey("F6PrVaeL2TegV2fmXFGW1dQeCXCCBZffn1JT5PUEiPMM");

    const payerSecretKey = Uint8Array.from(testPayerKeypair);
    const payer = Keypair.fromSecretKey(payerSecretKey);

    const create_name = "iu";
    const hashed_create = getHashedName(create_name);
    const { nameAccountKey } = getSeedAndKey(
      WEB3_NAME_SERVICE_ID, getHashedName(create_name), null);
    
    const ownerstr = payer.publicKey.toBase58().toString();
    const { nameAccountKey: recordaccountKey } = getSeedAndKey(
      WEB3_NAME_SERVICE_ID, getHashedName(ownerstr), null);

    const ipfsHash = "QmPu4ZT2zPfyVY8CA2YBzqo9HfAV79nDuuf177tMrQK1py";
    const ipfsBytes = Buffer.from(ipfsHash, 'utf-8');


    const storageData = {
        name: "iu",
        owner: payer.publicKey,
        ipfs: null,
    };


    const [centralStatePDA, bump] = PublicKey.findProgramAddressSync(
      [Buffer.from("central_state")], // 必须与Anchor的seeds完全一致
      program.programId,
    );

    it("this is register test", async () => {
        console.log("Name Account Key:", nameAccountKey.toBase58());
        console.log("web3nameservice:", web3NameServiceKey);
        const tx = await program.methods
            .createDomain(storageData)
            .accounts({
                web3NameService: web3NameServiceKey,
                nameAccount: nameAccountKey,
                buyer: payer.publicKey,
                domainRecord: recordaccountKey,
                feePayer: payer.publicKey,
                rootDomainAccount: null,
                central_state: centralStatePDA,
            })
            .signers([payer])
            .rpc();

        console.log("Transaction created:", tx);
    });
});




function getHashedName(name: string){
  const HASH_PREFIX = "WEB3 Name Service";
  const rawHash = HASH_PREFIX + name;
  const hashValue = sha256(rawHash);
  return new Uint8Array(Buffer.from(hashValue, 'hex'));
}

function getSeedAndKey(
  programid: PublicKey, hashedName: Uint8Array, rootOpt: null | PublicKey ){
  
  let seeds = new Uint8Array([...hashedName]);
  
  const rootDomain = rootOpt || PublicKey.default;
  seeds = new Uint8Array([...seeds, ...rootDomain.toBytes()]);

  const seedChunks = [];
  for (let i = 0; i < seeds.length; i += 32) {
      const chunk = seeds.slice(i, i + 32);
      seedChunks.push(chunk);
  }

  const [nameAccountKey, bump] = PublicKey.findProgramAddressSync(
      seedChunks,
      programid
  );

  seeds = new Uint8Array([...seeds, bump]);

  return {nameAccountKey, seeds};
}
