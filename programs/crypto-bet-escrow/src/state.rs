use anchor_lang::prelude::*;

pub const ESCROW_SEED: &[u8] = b"CRYPTO_ESCROW";

#[account]
pub struct EscrowState {
    pub initializer: Pubkey,
    pub acceptor: Pubkey,
    pub win_price: u64,
    pub lose_price: u64,
    pub escrow_amount: u64,
    pub is_active: bool,
}
