use anchor_lang::*;
use anchor_lang::prelude::*;
use crate::data::*;

impl ProgramContract {
    pub const MAX_ACTIVE_GAMES : usize = 100;
    pub const MAX_WAGERS_PER_GAME : usize = 100;
    //active games (4 + size * amount)
    pub const MAX_SIZE : usize = (4 + ProgramContract::MAX_ACTIVE_GAMES * Game::MAX_SIZE);
    //only owner is allowed to call certain functions
    pub const OWNER_KEY : Pubkey = Pubkey::new_from_array(
        [0; 32]);

    pub fn add_scheduled_game(&mut self, game_id : u32) -> Result<()> {
        if self.active_games.len() >= ProgramContract::MAX_ACTIVE_GAMES {
            return Err(ProgramErrorCode::MaxActiveGamesReached.into());
        }
        if let Some(_) = self.active_games.iter().find(|&g| g.id == game_id) {
            return Err(ProgramErrorCode::GameAlreadyExists.into());
        }

        self.active_games.push(Game {
            id: game_id,
            state: GameState::Scheduled,
            result: None,
            wagers: Vec::new()
        });
        
        Ok(())
    }

    pub fn place_wager(&mut self, user: Pubkey, game_id: u32, amount: u64, prediction: GameResult) -> Result<()> {
        let game = if let Some(game) = self.active_games.iter_mut().find(|g| g.id == game_id) {game} else {
            return Err(ProgramErrorCode::InvalidGameId.into());
        };
        if game.state != GameState::Scheduled {
            return Err(ProgramErrorCode::GameAlreadyStarted.into());
        }
        if game.wagers.len() >= ProgramContract::MAX_WAGERS_PER_GAME {
            return Err(ProgramErrorCode::MaxWagersPerGameReached.into());
        }

        //TODO: make the actuall transfer

        game.wagers.push(Wager {
            user,
            prediction,
            lamports: amount,
        });

        Ok(())
    }
}

impl UserStats {
    pub const MAX_HISTORY : usize = 10;
    //bump 1 + wagers (4 + size * amount)
    pub const MAX_SIZE : usize = 1 + (4 + Wager::SIZE * UserStats::MAX_HISTORY);
}

impl Wager {
    //id 32 + lamports 8 + prediction 1
    pub const SIZE : usize = 32 + 8 + 1;
}

impl WagerSummary {
    //game_id 4 + bet 8 + won 8 + predicted 1 + actuall 1
    pub const SIZE : usize = 4 + 8 + 8 + 1 + 1;
}

impl Game {
    //id 4 + state 1 + result 2 + wagers (4 + size * amount)
    pub const MAX_SIZE : usize = 4 + 1 + 2 + (4 + ProgramContract::MAX_WAGERS_PER_GAME * Wager::SIZE);
}