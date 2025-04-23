use anchor_lang::prelude::*;
use crate::state::vault::Vault;

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = admin,
        space = Vault::LEN,
        seeds = [Vault::SEED],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeVault>,
    min_stake: u64,
    max_stake: u64,
    vrf_cost: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    vault.admin = ctx.accounts.admin.key();
    vault.bump = ctx.bumps.vault;
    vault.min_stake = min_stake;
    vault.max_stake = max_stake;
    vault.vrf_cost = vrf_cost;
    vault.paused = false;
    vault.total_earned = 0;

    Ok(())
}
