use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;
use crate::state::{vault::Vault, play_ticket::PlayTicket};
use crate::constants::{SEED_TICKET, SHADOW_MAX};
use crate::error::CustomError;



#[event]
pub struct GameResult {
    pub player: Pubkey,
    pub shadow: u16,
    pub bid: u16,
    pub payout: u64,
}

#[derive(Accounts)]
pub struct RevealShadow<'info> {
    #[account(mut, seeds = [Vault::SEED], bump = vault.bump)]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [SEED_TICKET, player.key().as_ref()],
        bump,
        has_one = player,
        close = player
    )]
    pub play_ticket: Account<'info, PlayTicket>,

    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut)]
    pub vault_account: AccountInfo<'info>,

    pub randomness_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RevealShadow>) -> Result<()> {
    let ticket = &mut ctx.accounts.play_ticket;

    require!(!ticket.is_revealed, CustomError::AlreadyRevealed);

    // Load randomness state
    let vrf_state = RandomnessAccountData::parse(
        ctx.accounts.randomness_account.data.borrow(),
    ).map_err(|_| CustomError::RandomnessUnavailable)?;

    // ✅ Use Solana's official Clock from Anchor re-export
    let clock = Clock::get().map_err(|_| CustomError::RandomnessNotResolved)?;
    let randomness = vrf_state
        .get_value(&clock)
        .map_err(|_| CustomError::RandomnessNotResolved)?;
    

    // Use first 2 bytes to derive a fair u16 shadow number
    require!(randomness.len() >= 2, CustomError::RandomnessNotResolved);

    let raw_shadow = u16::from_le_bytes([randomness[0], randomness[1]]);
    let shadow = (raw_shadow % SHADOW_MAX) + 1;

    // Log shadow info
    msg!(
        "{} revealed shadow = {}, bid = {}, bytes = [{}, {}]",
        ctx.accounts.player.key(),
        shadow,
        ticket.bid,
        randomness[0],
        randomness[1]
    );

    // Compute payout multiplier
    let distance = if ticket.bid > shadow {
        ticket.bid - shadow
    } else {
        shadow - ticket.bid
    };

    let multiplier = if ticket.bid == shadow {
        50.0
    } else if distance <= 5 {
        20.0
    } else if distance <= 10 {
        10.0
    } else if distance <= 25 {
        3.0
    } else if distance <= 100 {
        1.5
    } else {
        0.0
    };

    ticket.is_revealed = true;

    let payout = (ticket.stake as f64 * multiplier) as u64;

    if multiplier == 0.0 {
        emit!(GameResult {
            player: ctx.accounts.player.key(),
            shadow,
            bid: ticket.bid,
            payout: 0,
        });
        return Ok(());
    }

    // Ensure vault has enough SOL
    let vault_balance = **ctx.accounts.vault_account.try_borrow_lamports()?;
    require!(vault_balance >= payout, CustomError::NoPayoutAvailable);

    // Payout transfer
    **ctx.accounts.vault_account.try_borrow_mut_lamports()? -= payout;
    **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += payout;

    emit!(GameResult {
        player: ctx.accounts.player.key(),
        shadow,
        bid: ticket.bid,
        payout,
    });

    Ok(())
}
