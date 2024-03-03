use crate::accept_bet::AcceptBet;
use crate::custom_errors::*;
use crate::state::*;
use crate::InitializeBet;
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [ESCROW_SEED, escrow_account.initializer.as_ref()],
        bump
    )]
    pub escrow_account: Account<'info, EscrowState>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_initializer_handler(ctx: Context<InitializeBet>, escrow_amount: u64) -> Result<()> {
    msg!("Depositing funds into escrow account...");
    msg!("Amount: {}", &escrow_amount,);

    let is_initializer = ctx.accounts.initializer.key() == ctx.accounts.escrow_account.initializer;

    if !is_initializer {
        require!(
            ctx.accounts.escrow_account.acceptor != Pubkey::default(),
            EscrowErrorCode::AcceptorNotSet
        )
    }

    let transfer_ix = transfer(
        &ctx.accounts.initializer.key(),
        &ctx.accounts.escrow_account.key(),
        escrow_amount,
    );

    invoke(
        &transfer_ix,
        &[
            ctx.accounts.initializer.to_account_info(),
            ctx.accounts.escrow_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    msg!("Transfer completed!");

    Ok(())
}

pub fn deposit_acceptor_handler(ctx: &Context<AcceptBet>, escrow_amount: u64) -> Result<()> {
    msg!("Depositing funds into escrow account...");
    msg!("Amount: {}", &escrow_amount,);

    let is_initializer = ctx.accounts.acceptor.key() == ctx.accounts.escrow_account.initializer;

    require!(is_initializer, EscrowErrorCode::NoInitializer);

    if is_initializer {
        require!(
            ctx.accounts.escrow_account.acceptor != Pubkey::default(),
            EscrowErrorCode::AcceptorNotSet
        )
    }

    let transfer_ix = transfer(
        &ctx.accounts.acceptor.key(),
        &ctx.accounts.escrow_account.key(),
        escrow_amount,
    );

    invoke(
        &transfer_ix,
        &[
            ctx.accounts.acceptor.to_account_info(),
            ctx.accounts.escrow_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    msg!("Transfer completed!");

    Ok(())
}
