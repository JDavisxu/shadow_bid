use anchor_lang::prelude::*;
use crate::state::vault::Vault;
use crate::constants::SEED_VAULT;
use crate::error::CustomError;

#[derive(Accounts)]
pub struct PauseGame<'info> {
    #[account(
        mut,
        seeds = [SEED_VAULT],
        bump = vault.bump,
        has_one = admin
    )]
    pub vault: Account<'info, Vault>,

    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<PauseGame>, paused: bool) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    require_keys_eq!(
        ctx.accounts.admin.key(),
        vault.admin,
        CustomError::Unauthorized
    );

    vault.paused = paused;

    msg!("Game pause state changed by {}: paused = {}", vault.admin, paused);

    Ok(())
}
