import { PublicKey, Connection, clusterApiUrl, SystemProgram, Transaction, Keypair } from '@solana/web3.js';
//used to convert parameter to a bytes array
import { serialize } from 'borsh';
import { Buffer } from 'buffer'; 
//import { NameServiceClient } from '@solana/spl-name-service'


//define a test prams
//mainnet-beta
const USING_NET = 'devnet';
//global params: SPL_Name_Service's programID
const SPL_Name_ServiceID = new PublicKey('namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX');
//root domain
const root_domain = ".sol";

/**
 * calculate the PDA of domain,nd query solana to confirm whether it has been registered
 * @params  {string} domian  --  Domain name to query
 * @returns  -- state
 */
export async function check_domain_availability(domain) {
    const max_length = 15;
    console.log("ready to query:", domain);
    //check if 
    if( !(domain.length <= max_length) || !(domain.endsWith(root_domain))){
        console.log("invalid domain format")
        return ""
    };
    console.log("ready to try")
    try {
        console.log("start try");
        //convert domian to buty array
        //TextEncoder()：Constructor for encoding a string into a byte array in Web API
        const domain_bytes = new TextEncoder().encode(domain);
        //calculate the PDA
        //await: Waiting for an asynchronous operation
        const [PDA, _] =  PublicKey.findProgramAddressSync([domain_bytes], SPL_Name_ServiceID);
        console.log("PDA:", PDA.toBase58())
        //create solana network connect
        const connection = new Connection(clusterApiUrl(USING_NET));
        //query
        const accountInfo = await connection.getAccountInfo(PDA);
        if (accountInfo === null){
            console.log("valid domain");
            return PDA.toBase58();
        }else{
            console.log("invalid domain");
            return "";
        }
    }catch(error){
        console.log("error happened:", error);
        return "";
    }
}


/**
 * 
 * 
 */

//constructor: used to save params when creating a domain name
class create_domian_params {
    constructor({ name, space, referrer_idx_opt = null}){
        this.name = name;
        this.space = space;
        //in contract: it's optional
        this.referrer_idx_opt = referrer_idx_opt;
    }
}

//define the Borsh's structure pattern,
//tell borsh how to serialize the create_domian_params class in borsh
//Map allows us to store key-value pairs
const CreateParamsSchema = new Map([
    [
        create_domian_params,
        {
            kind: 'struct',
            fields: [
                ['name', 'string'],
                ['sapce', 'u32'],
                ['referrer_idx_opt', {kind: 'option', type: 'u16'}],
            ],
        },
    ],
])

//function to create a Serialized parameters
function create_domian_parameters(name, space, referrer_idx_opt = null){
    const params = new create_domian_params({name, space, referrer_idx_opt});
    if (typeof window !== "undefined") {
        window.Buffer = Buffer;
    }
    const data = serialize (CreateParamsSchema,params); 
    return data;
}


/**
 * 
 * 
 * 
 */

//define constant
//get contract address
const web3_registerID = new PublicKey("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");
//get root_domain address
const root_domain_key = new PublicKey("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");
//get centralstate address
const cantral_state_key = new PublicKey("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");
//get vault address
const vault_key = new PublicKey("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");
//get token address
const token_key = new PublicKey("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");

export async function call_web3_register(domain_name, name_account_key) {
    console.log("entry web3_register:",domain_name,name_account_key );
    const connection = new Connection(clusterApiUrl('devnet'));
    console.log("connect dev")
    const data = create_domian_parameters(domain_name,1024);
    console.log("create param")
    const buyer_wallet = get_connecting_wallet();
    console.log("get wallet")
    //fristly create a account info list
    const account = [
        {pubkey: root_domain_key, isSigner: false, isWritable: false},
        {pubkey: name_account_key, isSigner: false, isWritable: true},
        //{pubkey: reverse_lookup_key, isSigner: false, isWritable: false},
        {pubkey: cantral_state_key, isSigner: false, isWritable: false},
        {pubkey: buyer_wallet.publicKey, isSigner: true, isWritable: true},
        //{pubkey: buyer_token_source_key, isSigner: false, isWritable: true},
        {pubkey: vault_key, isSigner: false, isWritable: false},
        {pubkey: token_key, isSigner: false, isWritable: false},
    ]
    console.log("create account")
    //create a transaction
    const transaction = new Transaction().add({
        keys:account,
        programId: web3_registerID,
        data: Buffer.from(data),
    })
    console.log("create transaction")

    try{const response = await fetch(`https://api.bonfida.com/v1/domain/resolve/f.sol`);
    const response_data = await response.json();
    console.log(`域名 ${domain_name} 解析的地址:`, response_data.address);
    }catch(error){
        console.log("fetch err:",error)
    }
        // try{
    //     const signature = await connection.sendTransaction(transaction, [buyer_wallet], {
    //         skipPreflight: false,
    //         preflightCommitment: 'confirmed',
    //       });
    //     console.log("transaction start:", signature);
    // }catch(error){
    //     console.log("error:", error)
    // }
}



/**
 * get connecting wallet pubkey
 * 
 */
function get_connecting_wallet(){
    const wallet =  new PublicKey("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");
    return Keypair.generate();
}
