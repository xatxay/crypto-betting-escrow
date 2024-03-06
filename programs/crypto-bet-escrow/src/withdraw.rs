use crate::custom_errors::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_unchecked;
use anchor_lang::solana_program::system_instruction::transfer;
use std::str::FromStr;
use switchboard_solana::AggregatorAccountData;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub winner: Signer<'info>,
    #[account(
        mut,
        seeds = [ESCROW_SEED, winner.key().as_ref()],
        bump,
        close = winner
    )]
    pub escrow_account: Box<Account<'info, EscrowState>>,
    #[account(address = Pubkey::from_str(BTC_USD_FEED).unwrap())]
    pub feed_aggregator: AccountLoader<'info, AggregatorAccountData>,
    pub system_program: Program<'info, System>,
}

pub fn determine_winner(
    current_price: f64,
    escrow_state: &Account<'_, EscrowState>,
) -> Result<Pubkey> {
    let winner: Pubkey;
    let win_price_f64: f64 = escrow_state.win_price as f64;
    let lose_price_f64: f64 = escrow_state.lose_price as f64;
    let bet_long = escrow_state.win_price > escrow_state.lose_price;
    if bet_long {
        if current_price >= win_price_f64 {
            winner = escrow_state.initializer;
        } else if current_price <= lose_price_f64 {
            winner = escrow_state.acceptor;
        } else {
            msg!("{}", EscrowErrorCode::OutcomeNotDetermined);
            return Err(error!(EscrowErrorCode::OutcomeNotDetermined));
        }
    } else {
        if current_price <= win_price_f64 {
            winner = escrow_state.initializer;
        } else if current_price >= lose_price_f64 {
            winner = escrow_state.acceptor
        } else {
            msg!("{}", EscrowErrorCode::OutcomeNotDetermined);
            return Err(error!(EscrowErrorCode::OutcomeNotDetermined));
        }
    }
    Ok(winner)
}

pub fn withdraw_handler(ctx: Context<Withdraw>) -> Result<()> {
    let feed = &ctx.accounts.feed_aggregator.load()?;
    let escrow_state = &mut ctx.accounts.escrow_account;

    if !escrow_state.is_active {
        msg!("{}", EscrowErrorCode::AcceptorNotSet);
        return Ok(());
    }

    let current_price: f64 = feed.get_result()?.try_into()?;

    feed.check_staleness(Clock::get().unwrap().unix_timestamp, 300)
        .map_err(|_| error!(EscrowErrorCode::StaleFeed))?;

    msg!("Current feed result is {}!", current_price);
    msg!(
        "Unlock price is {} or {}",
        escrow_state.win_price,
        escrow_state.lose_price
    );

    let winner = determine_winner(current_price, escrow_state)?;

    if ctx.accounts.winner.key() != winner {
        msg!("{}", EscrowErrorCode::NotWinner);
        return Err(error!(EscrowErrorCode::NotWinner));
    }

    let transfer_to_winner_ix = transfer(
        &escrow_state.to_account_info().key,
        &winner,
        escrow_state.escrow_amount,
    );

    invoke_unchecked(
        &transfer_to_winner_ix,
        &[
            escrow_state.to_account_info(),
            ctx.accounts.winner.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    **ctx
        .accounts
        .winner
        .to_account_info()
        .try_borrow_mut_lamports()? += escrow_state.to_account_info().lamports();
    **escrow_state.to_account_info().lamports.borrow_mut() = 0;

    escrow_state.is_active = false;

    Ok(())
}
