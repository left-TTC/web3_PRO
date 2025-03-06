use anchor_lang::prelude::*;

#[cfg(not(feature = "devnet"))]
pub mod Constants{
    use super::*;
    //all not ready
    pub const VAULT_ACCOUNT: Pubkey = pubkey!("GcWEQ9K78FV7LEHteFVciYApERk5YvQuFDQPk1yYJVXi");

    pub const ROOT_DOMAIN_ACCOUNT: Pubkey = pubkey!("58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx");

    pub const AUCTION_PROGRAM_ID: Pubkey = pubkey!("AVWV7vdWbLqXiLKFaP19GhYurhwxaLp2qRBSjT5tR5vT");
}

#[cfg(feature = "devnet")]
pub mod Constants{
    use super::*;
    //all not ready
    pub const VAULT_ACCOUNT: Pubkey = pubkey!("GcWEQ9K78FV7LEHteFVciYApERk5YvQuFDQPk1yYJVXi");

    pub const ROOT_DOMAIN_ACCOUNT: Pubkey = pubkey!("58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx");

    pub const AUCTION_PROGRAM_ID: Pubkey = pubkey!("AVWV7vdWbLqXiLKFaP19GhYurhwxaLp2qRBSjT5tR5vT");
}
