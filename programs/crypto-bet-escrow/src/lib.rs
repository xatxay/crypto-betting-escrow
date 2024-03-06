pub mod accept_bet;
pub mod custom_errors;
pub mod deposit;
pub mod initialize_bet;
pub mod state;
pub mod withdraw;
pub use accept_bet::*;
pub use withdraw::*;

pub use initialize_bet::*;

use anchor_lang::prelude::*;

declare_id!("DgAjAXVQ1Capcc8ehiqpQwqba1wzsmh4pozP389ZmQUd");

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

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw_handler(ctx)
    }
}
