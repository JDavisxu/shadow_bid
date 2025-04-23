use anchor_lang::prelude::*;

#[account]
pub struct PlayTicket {
    pub player: Pubkey,
    pub bid: u16,
    pub stake: u64,
    pub is_revealed: bool,
    pub bump: u8,
}

impl PlayTicket {
    pub const LEN: usize = 52;
}
