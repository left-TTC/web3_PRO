import { sha256 } from 'js-sha256';
import { Buffer } from 'buffer'; // To handle binary data in Node.js
import { PublicKey, Connection } from '@solana/web3.js'; 

const HASH_PREFIX = "WEB3 Name Service";

const DEVNET_URL = "https://api.devnet.solana.com";

const ROOT_CLASS = new PublicKey("52F3LuKrH19f8JATdXn1w9F3kFQceK3n5ticQmbjVs78");

export const WEB3_NAME_SERVICE_ID = new PublicKey("BWK7ZQWjQ9fweneHfsYmof7znPr5GyedCWs2J8JhHxD3");

export const WEB3_ROOT = new PublicKey("52F3LuKrH19f8JATdXn1w9F3kFQceK3n5ticQmbjVs78");


/**
 * 
 * @param {string} name 
 * @returns 
 */
function getHashedName(name){
    const rawHash = HASH_PREFIX + name;
    const hashValue = sha256(rawHash);
    return new Uint8Array(Buffer.from(hashValue, 'hex'));
}

/**
 * 
 * @param {PublicKey} programid 
 * @param {Uint8Array} hashedName 
 * @param {PublicKey} domainClass 
 * @param {PublicKey | null} rootOpt 
 * @returns 
 */
export function getSeedAndKey(
    programid, hashedName, domainClass, rootOpt=null){
    
    let seeds = new Uint8Array([...hashedName]);
    seeds = new Uint8Array([...seeds, ...domainClass.toBytes()]);
    
    const rootDomain = rootOpt || PublicKey.default;
    seeds = new Uint8Array([...seeds, ...rootDomain.toBytes()]);

    const [nameAccountKey, bump] = PublicKey.findProgramAddressSync(
        [seeds],
        programid
    );

    seeds = new Uint8Array([...seeds, bump]);

    return { nameAccountKey, seeds };
}

/**
 * 
 * @param {string} name 
 * @param {PublicKey} programid 
 * @param {PublicKey} domainClass 
 * @param {PublicKey | null} rootOpt 
 * @returns 
 */
export async function query_domain(
    name, programid, domainClass, rootOpt=null,) {
    try{
        const hashedName = getHashedName(name);
        const {key} = getSeedAndKey(
            programid, hashedName, domainClass, rootOpt);
        console.log("PDA:", key.toBase58());
        
        const connection = new Connection(DEVNET_URL, 'confirmed');
        const accountInfo = await connection.getAccountInfo(key);

        if(accountInfo){
            console.log("exist");
            return accountInfo;
        }else{
            console.log("not exist");
            return "";
        }
    }catch(err){
        console.log("err happened when quering:", err)
    }
}

/**
 * 
 * @param {string} className 
 * @returns 
 */
export function checkClass(className){
    return ROOT_CLASS;
}


export function isCheckDomain(){
    return false
}

