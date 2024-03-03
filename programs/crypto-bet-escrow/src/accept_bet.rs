use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AcceptBet<'info> {
    #[account(mut)]
    pub acceptor: Signer<'info>,
    #[account(
        mut,
        seeds = [ESCROW_SEED, escrow_account.initializer.as_ref()],
        bump,
        constraint = escrow_account.acceptor == Pubkey::default()
    )]
    pub escrow_account: Account<'info, EscrowState>,
    pub system_program: Program<'info, System>,
}

pub fn accept_bet_handler(ctx: Context<AcceptBet>) -> Result<()> {
    let escrow_account = &mut ctx.accounts.escrow_account;
    escrow_account.acceptor = *ctx.accounts.acceptor.key;

    escrow_account.is_active = true;
    Ok(())
}
