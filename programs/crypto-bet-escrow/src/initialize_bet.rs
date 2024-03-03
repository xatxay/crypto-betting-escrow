use crate::{deposit::deposit_initializer_handler, state::*};
use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

#[derive(Accounts)]
pub struct InitializeBet<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init,
        seeds = [ESCROW_SEED, initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = std::mem::size_of::<EscrowState>() + 8
    )]
    pub escrow_account: Account<'info, EscrowState>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_bet_handler(
    ctx: Context<InitializeBet>,
    win_price: u64,
    lose_price: u64,
    escrow_amount: u64,
) -> Result<()> {
    let escrow_state = &mut ctx.accounts.escrow_account;
    escrow_state.initializer = *ctx.accounts.initializer.key;
    escrow_state.acceptor = Pubkey::default();
    escrow_state.escrow_amount = escrow_amount * LAMPORTS_PER_SOL;
    escrow_state.win_price = win_price;
    escrow_state.lose_price = lose_price;

    deposit_initializer_handler(ctx, escrow_amount)?;

    Ok(())
}
