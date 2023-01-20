use std::str::FromStr;
use anchor_lang::prelude::*;
use crate::data::*;

impl ProgramContract {
    pub const MAX_ACTIVE_GAMES : usize = 10;
    pub const MAX_WAGERS_PER_GAME : usize = 20;
    pub const MIN_WAGER_AMOUNT : usize = 1000;
    //taxes 8 + active games (4 + size * amount)
    pub const MAX_SIZE : usize = 8 + (4 + ProgramContract::MAX_ACTIVE_GAMES * Game::MAX_SIZE);
    //percent of each wager that goes to the Onwers account
    pub const OWNER_CUT : u64 = 5;

    //only owner is allowed to call certain functions
    pub fn owner_key() -> Pubkey {
        Pubkey::from_str("4BXdw9SoHpzaZCMR5tvEhjm7qQiCsjUAfmjJTHmTmEVC").unwrap()
    } 
}

impl UserStats {
    pub const MAX_HISTORY : usize = 10;
    //bump 1 + wagers (4 + size * amount)
    pub const MAX_SIZE : usize = 1 + (4 + Wager::SIZE * UserStats::MAX_HISTORY);
}

impl Wager {
    //id 32 + lamports 8 + prediction 1 + collected 1
    pub const SIZE : usize = 32 + 8 + 1 + 1;
}

impl WagerSummary {
    //game_id 4 + bet 8 + won 8 + predicted 1 + actuall 2
    pub const SIZE : usize = 4 + 8 + 8 + 1 + 2;
}

impl Game {
    //id 4 + state 1 + result 2 + wagers (4 + size * amount)
    pub const MAX_SIZE : usize = 4 + 1 + 2 + (4 + ProgramContract::MAX_WAGERS_PER_GAME * Wager::SIZE);
}

impl ProgramWallet {
    //bump 1
    pub const MAX_SIZE: usize = 1;
}

impl GameResult {
    pub fn from_str(input: &str) -> Result<GameResult> {
        match input {
            "HomeVictory"  => Ok(GameResult::HomeVictory),
            "AwayVictory"  => Ok(GameResult::AwayVictory),
            "Tie"          => Ok(GameResult::Tie),
            _              => Err(ProgramErrorCode::InvalidEnumType.into()),
        }
    }

    pub fn from_str_opt(input: &str) -> Result<Option<GameResult>> {
        match input {
            "HomeVictory"  => Ok(Some(GameResult::HomeVictory)),
            "AwayVictory"  => Ok(Some(GameResult::AwayVictory)),
            "Tie"          => Ok(Some(GameResult::Tie)),
            "None"         => Ok(None),
            _              => Err(ProgramErrorCode::InvalidEnumType.into()),
        }
    }
}

impl GameState {
    pub fn from_str(input: &str) -> Result<GameState> {
        match input {
            "Scheduled"  => Ok(GameState::Scheduled),
            "Live"       => Ok(GameState::Live),
            "Finished"   => Ok(GameState::Finished),
            "Cancelled"  => Ok(GameState::Cancelled),
            _            => Err(ProgramErrorCode::InvalidEnumType.into()),
        }
    }
}