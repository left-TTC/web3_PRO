//used to convert parameter to a bytes array
import { serialize } from 'borsh';
import { Buffer } from 'buffer'; 
import { PublicKey, Connection } from '@solana/web3.js';
import { sha256 } from "js-sha256";

//this file record the utils functions that used to query domain and pubkey


const WEB3_NAMING_SERVICE_PROGRAM_ID = new PublicKey("8urcfunkMXmN59aTzW8SLPGVc5r8Fk7LEokbnQCZXvCp");

const TEST_ACCOUNT = new PublicKey("BHDoJEpVvJgXdTEutoxQT2BR7aVRGtmxJG49ov3eDdET");

const HASH_PREFIX = "WEB3 Name Service";

const MAIN_NET = "https://api.mainnet-beta.solana.com";

const DEV_NET = "https://api.devnet.solana.com";


/**
 * 
 * @param {string} domainName the querying domain
 * @returns the hashed domain
 */
function getHashedName(domainName){
    const rawValue = HASH_PREFIX + domainName;
    return Buffer.from(sha256.arrayBuffer(rawValue));
}

/**
 * 
 * @param {PublicKey} programId 
 * @param {Buffer} hashedName 
 * @param {PublicKey | null} rootKey 
 * @returns 
 */
function getSeedAndKey(programId, hashedName, rootKey = null){
    let seeds = [hashedName];
    if(rootKey){
        seeds.push(rootKey.toBuffer());
    }
    //Convert seeds into 32-byte chunks
    const seedsChunks = seeds.flatMap(seed =>{
        const chunks = [];
        for (let i = 0; i < seed.length; i += 32) {
            chunks.push(seed.subarray(i, i + 32));
        }
        return chunks;
    })
    //bump -- Collision Factor
    const [pda, bump] = PublicKey.findProgramAddressSync(seedsChunks, programId);
    return pda;
}

/**
 * 
 * @param {string} domainName 
 * @param {PublicKey | null} rootDomainKey 
 * @returns 
 */
function getNameKey(domainName, rootDomainKey = null){
    const hashedName = getHashedName(domainName);
    const rootKey = rootDomainKey ? rootDomainKey : null;

    const domainKey = getSeedAndKey(
        WEB3_NAMING_SERVICE_PROGRAM_ID,
        hashedName,
        rootKey,
    );
    return domainKey;
}

/**
 * 
 * @param {PublicKey} queryPublicKey 
 * @returns 
 */
function getReverseKey(queryPublicKey){
    const hashedName = getHashedName(queryPublicKey.toBase58());
    const reverseKey = getSeedAndKey(
        WEB3_NAMING_SERVICE_PROGRAM_ID,
        hashedName,
        null,
    )

    return reverseKey;
}


export async function queryDomainAccountOwner(domainName, rootDomainKey) {
    //create connection
    const connection = new Connection(DEV_NET);
    console.log("connect solana");

    const accountPDA = getNameKey(domainName, rootDomainKey);
    console.log("Name Account PDA:", accountPDA.toBase58());

    const accountInfo = await connection.getAccountInfo(accountPDA);
    console.log("connect over");
    //means the domain is already registered
    if (accountInfo) {
        console.log("Account Data:", accountInfo.data);
    } else {
        console.log("Account not found");
    }
    return accountInfo;
}

/**
 * 
 * @param {PublicKey} queryPubKey 
 */
export async function queryPubkeyOwnedDomain(queryPubKey) {
    const connection = new Connection(DEV_NET);
    console.log("connect solana");

    const reversePDA = getReverseKey(queryPubKey);
    console.log("reverse Account PDA:", reversePDA.toBase58());
    
    const accountInfo = await connection.getAccountInfo(reversePDA);
    console.log("connect over");
    if (accountInfo) {
        console.log("Account Data:", accountInfo.data);
    } else {
        console.log("Account not found");
    }
    return accountInfo;
}




