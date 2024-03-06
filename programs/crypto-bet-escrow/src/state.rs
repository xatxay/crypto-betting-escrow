use anchor_lang::prelude::*;

pub const ESCROW_SEED: &[u8] = b"CRYPTO_ESCROW";
pub const BTC_USD_FEED: &str = "8SXvChNYFhRq4EZuZvnhjrB3jJRQCv4k3P4W6hesH3Ee";

#[account]
pub struct EscrowState {
    pub initializer: Pubkey,
    pub acceptor: Pubkey,
    pub win_price: u64,
    pub lose_price: u64,
    pub escrow_amount: u64,
    pub is_active: bool,
}
