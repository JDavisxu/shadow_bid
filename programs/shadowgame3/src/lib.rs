// programs/shadowgame/src/lib.rs
use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("HxhbYFaQzCD6qdV3ZR6WQ5wY2oCZnHfn99t27Usjcdfs");

#[program]
pub mod shadowgame {
    use super::*;

    /// Initializes the vault PDA that holds game config and SOL.
    /// Only needs to be run once by the admin.
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        min_stake: u64,
        max_stake: u64,
        vrf_cost: u64,
    ) -> Result<()> {
        initialize_vault::handler(ctx, min_stake, max_stake, vrf_cost)
    }

    /// Allows the admin to update game parameters like min/max stake,
    /// VRF cost, and paused state.
    pub fn edit_game(
        ctx: Context<EditGame>,
        min_stake: Option<u64>,
        max_stake: Option<u64>,
        vrf_cost: Option<u64>,
        paused: Option<bool>,
    ) -> Result<()> {
        edit_game::handler(ctx, min_stake, max_stake, vrf_cost, paused)
    }

    /// Called by players to start a game round.
    /// Records their bid and stake, transfers SOL to the vault.
    pub fn play_shadow_bid(ctx: Context<PlayShadowBid>, bid: u16, stake: u64) -> Result<()> {
        play_shadow_bid::handler(ctx, bid, stake)
    }

    /// Called after randomness is fulfilled.
    /// Compares the player's guess to the shadow number and pays out if close.
    pub fn reveal_shadow(ctx: Context<RevealShadow>) -> Result<()> {
        reveal_shadow::handler(ctx)
    }

    /// Admin-only instruction to withdraw SOL from the vault account.
    pub fn withdraw_from_vault(ctx: Context<WithdrawFromVault>, amount: u64) -> Result<()> {
        withdraw_from_vault::handler(ctx, amount)
    }

    /// Allows admin to pause or unpause the game.
    /// When paused, users cannot play new rounds.
    pub fn pause_game(ctx: Context<PauseGame>, paused: bool) -> Result<()> {
        pause_game::handler(ctx, paused)
    }
}