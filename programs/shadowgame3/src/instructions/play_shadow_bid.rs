use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::{vault::Vault, play_ticket::PlayTicket};
use crate::constants::SEED_TICKET;
use crate::error::CustomError;

#[derive(Accounts)]
#[instruction(bid: u16, stake: u64)]
pub struct PlayShadowBid<'info> {
    #[account(mut, seeds = [Vault::SEED], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = player,
        space = PlayTicket::LEN,
        seeds = [SEED_TICKET, player.key().as_ref()],
        bump
    )]
    pub play_ticket: Account<'info, PlayTicket>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PlayShadowBid>, bid: u16, stake: u64) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let ticket = &mut ctx.accounts.play_ticket;

    // Game pause and stake validation
    require!(!vault.paused, CustomError::GamePaused);
    require!(stake >= vault.min_stake, CustomError::StakeTooLow);
    require!(stake <= vault.max_stake, CustomError::StakeTooHigh);

    // Transfer stake to vault
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.player.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    system_program::transfer(cpi_ctx, stake)?;

    // Store ticket data
    ticket.player = ctx.accounts.player.key();
    ticket.bid = bid;
    ticket.stake = stake;
    ticket.is_revealed = false;
    ticket.bump = ctx.bumps.play_ticket;

    Ok(())
}
