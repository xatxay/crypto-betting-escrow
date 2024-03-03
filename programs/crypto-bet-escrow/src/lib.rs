pub mod accept_bet;
pub mod custom_errors;
pub mod deposit;
pub mod initialize_bet;
pub mod state;
pub use accept_bet::*;

pub use initialize_bet::*;

use anchor_lang::prelude::*;

declare_id!("Da9ntLekG2xkGhZPxSZwxnwZqh43q8KZCaHHbutq9BMM");

#[program]
pub mod crypto_bet_escrow {
    use super::*;

    pub fn initialize_bet(
        ctx: Context<InitializeBet>,
        win_price: u64,
        lose_price: u64,
        escrow_amount: u64,
    ) -> Result<()> {
        initialize_bet_handler(ctx, win_price, lose_price, escrow_amount)
    }

    pub fn accept_bet(ctx: Context<AcceptBet>) -> Result<()> {
        accept_bet_handler(ctx)
    }
}
