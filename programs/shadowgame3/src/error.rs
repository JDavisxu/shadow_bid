use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("The game is currently paused.")]
    GamePaused,

    #[msg("The stake amount is below the minimum.")]
    StakeTooLow,

    #[msg("The stake amount exceeds the maximum allowed.")]
    StakeTooHigh,

    #[msg("This ticket has already been revealed.")]
    AlreadyRevealed,

    #[msg("No winnings to distribute.")]
    NoPayoutAvailable,

    #[msg("Unauthorized action.")]
    Unauthorized,

    #[msg("Randomness unavailiable")]
    RandomnessUnavailable,

    #[msg("Randomness not resolved")]
    RandomnessNotResolved,

    #[msg("Ticket already in process.")]
    TicketAlreadyInProgress,

    #[msg("Invaild stake range")]
    InvalidStakeRange,



}
