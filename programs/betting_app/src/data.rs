use anchor_lang::prelude::*;

#[account]
pub struct UserStats {
    pub history : Vec<WagerSummary>,
    pub bump : u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct WagerSummary {
    pub game_id: u32,
    pub lamports_bet : u64,
    pub lamports_won : u64,
    pub predicted_result : GameResult,
    pub actuall_result : GameResult,
}

#[account]
pub struct ProgramContract {
    pub active_games : Vec<Game>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Game {
    pub id: u32,
    pub wagers : Vec<Wager>,
    pub state : GameState,
    pub result : Option<GameResult>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct Wager {
    pub user : Pubkey,
    pub lamports : u64,
    pub prediction : GameResult
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Scheduled,
    Live,
    InPlay,
    Paused,
    Finished,
    Postponed,
    Suspended,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum GameResult {
    HomeVictory,
    AwayVictory,
    Tie,
}

#[error_code]
pub enum ProgramErrorCode {
    #[msg("Game with given Id doesn't exist or is already finished")]
    InvalidGameId,

    #[msg("Game with given Id has already started, cannot place a bet")]
    GameAlreadyStarted,

    #[msg("Game with given Id already exists in the pool of active games")]
    GameAlreadyExists,

    #[msg("Maximum amount of wagers for this game has been reached, cannot place a bet")]
    MaxWagersPerGameReached,

    #[msg("Maximum amount of active games has been reached, cannot open a bet for this game")]
    MaxActiveGamesReached,

    #[msg("A wager must be at least X lamparts")]
    MinimalWagerAmount,

    #[msg("No permission to call this instruction, only the owner is allowed")]
    InstructionNotPermitted,
}