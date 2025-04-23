use anchor_lang::prelude::*;
use crate::state::vault::Vault;
use crate::error::CustomError;

#[derive(Accounts)]
pub struct EditGame<'info> {
    #[account(mut, seeds = [Vault::SEED], bump = vault.bump, has_one = admin)]
    pub vault: Account<'info, Vault>,

    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<EditGame>,
    min_stake: Option<u64>,
    max_stake: Option<u64>,
    vrf_cost: Option<u64>,
    paused: Option<bool>,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    require_keys_eq!(
        ctx.accounts.admin.key(),
        vault.admin,
        CustomError::Unauthorized
    );

    if let (Some(min), Some(max)) = (min_stake, max_stake) {
        require!(min <= max, CustomError::InvalidStakeRange);
    }

    if let Some(min) = min_stake {
        vault.min_stake = min;
    }

    if let Some(max) = max_stake {
        vault.max_stake = max;
    }

    if let Some(cost) = vrf_cost {
        vault.vrf_cost = cost;
    }

    if let Some(flag) = paused {
        vault.paused = flag;
    }

    msg!("Game settings edited by: {}", vault.admin);

    Ok(())
}
