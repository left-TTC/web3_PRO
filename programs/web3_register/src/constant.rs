use anchor_lang::prelude::*;


#[cfg(not(feature = "devnet"))]
pub mod Constants{
    use super::*;
    //all not ready
    pub const VAULT_ACCOUNT: Pubkey = pubkey!("GcWEQ9K78FV7LEHteFVciYApERk5YvQuFDQPk1yYJVXi");

    pub const ROOT_DOMAIN_ACCOUNT: Pubkey = pubkey!("58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx");

    pub const AUCTION_PROGRAM_ID: Pubkey = pubkey!("AVWV7vdWbLqXiLKFaP19GhYurhwxaLp2qRBSjT5tR5vT");

    pub const WEB_NAMEING_SERVICE: Pubkey = pubkey!("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");

    pub const HASH_PREFIX: &str = "WEB3 Name Service";
    //5% discount
    pub const REFERRER_FEE_PCT: u64 = 5;
}

#[cfg(feature = "devnet")]
pub mod Constants{
    use super::*;
    //all not ready
    //use my account as the test wallet
    pub const VAULT_ACCOUNT: Pubkey = pubkey!("BHDoJEpVvJgXdTEutoxQT2BR7aVRGtmxJG49ov3eDdET");

    //this should be create at name
    pub const ROOT_DOMAIN_ACCOUNT: Pubkey = pubkey!("58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx");

    pub const AUCTION_PROGRAM_ID: Pubkey = pubkey!("AVWV7vdWbLqXiLKFaP19GhYurhwxaLp2qRBSjT5tR5vT");
    //change it to web3 name service
    pub const WEB_NAMEING_SERVICE: Pubkey = pubkey!("F6PrVaeL2TegV2fmXFGW1dQeCXCCBZffn1JT5PUEiPMM");

    pub const HASH_PREFIX: &str = "WEB3 Name Service";

    pub const REFERRER_FEE_PCT: u64 = 5;

    pub const CENTRAL_STATE_SEED: &[u8] = b"central_state";

}
