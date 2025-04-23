use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub bump: u8,
    pub min_stake: u64,
    pub max_stake: u64,
    pub vrf_cost: u64,
    pub paused: bool,
    pub total_earned: u64,
    // pub total_burned: u64, // optional
}

impl Vault {
    pub const SEED: &'static [u8] = b"vault";
    pub const LEN: usize = 74; // or 82 if total_burned added
}
