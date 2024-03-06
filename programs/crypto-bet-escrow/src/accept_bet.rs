use crate::{deposit::deposit_acceptor_handler, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AcceptBet<'info> {
    #[account(mut)]
    pub acceptor: Signer<'info>,
    #[account(
        mut,
        seeds = [ESCROW_SEED, acceptor.key().as_ref()],
        bump,
        constraint = escrow_account.acceptor == Pubkey::default()
    )]
    pub escrow_account: Box<Account<'info, EscrowState>>,
    pub system_program: Program<'info, System>,
}

pub fn accept_bet_handler(ctx: Context<AcceptBet>) -> Result<()> {
    ctx.accounts.escrow_account.acceptor = *ctx.accounts.acceptor.key;
    let escrow_amount = ctx.accounts.escrow_account.escrow_amount;

    deposit_acceptor_handler(ctx, escrow_amount)?;
    Ok(())
}
