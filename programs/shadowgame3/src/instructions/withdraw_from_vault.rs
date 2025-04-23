use anchor_lang::prelude::*;
use crate::state::vault::Vault;
use crate::constants::SEED_VAULT;
use crate::error::CustomError;

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {
    #[account(
        mut,
        seeds = [SEED_VAULT],
        bump = vault.bump,
        has_one = admin
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<WithdrawFromVault>, amount: u64) -> Result<()> {
    let vault_info = ctx.accounts.vault.to_account_info();
    let admin_info = ctx.accounts.admin.to_account_info();

    require!(
        **vault_info.lamports.borrow() >= amount,
        CustomError::NoPayoutAvailable
    );

    **vault_info.try_borrow_mut_lamports()? -= amount;
    **admin_info.try_borrow_mut_lamports()? += amount;

    msg!("Withdrew {} lamports to {}", amount, admin_info.key);

    Ok(())
}
