pub mod data;
pub mod logic;
pub mod validation;

use crate::data::*;
use crate::validation::*;
use anchor_lang::prelude::*;

declare_id!("41jcANndXq56ztpCtvJa9mKkzxaunn9nyYVbCFihcxT7");

#[program]
pub mod betting_app {
    use anchor_lang::system_program;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.active_games = Vec::new();
        let wallet = &mut ctx.accounts.program_wallet;
        wallet.bump = *ctx.bumps.get("program_wallet").unwrap();

        Ok(())
    }

    pub fn create_user_stats(ctx: Context<CreateUserStats>) -> Result<()> {
		let user_stats = &mut ctx.accounts.user_stats;
        user_stats.bump = *ctx.bumps.get("user_stats").unwrap();
        user_stats.history = Vec::new();

        Ok(())
    }

    pub fn add_scheduled_game(ctx: Context<AddScheduledGame>, game_id : u32) -> Result<()> {
        let contract = &mut ctx.accounts.contract;

        if contract.active_games.len() >= ProgramContract::MAX_ACTIVE_GAMES {
            return Err(ProgramErrorCode::MaxActiveGamesReached.into());
        }
        if let Some(_) = contract.active_games.iter().find(|&g| g.id == game_id) {
            return Err(ProgramErrorCode::GameAlreadyExists.into());
        }

        contract.active_games.push(Game {
            id: game_id,
            state: GameState::Scheduled,
            result: None,
            wagers: Vec::new(),
        });
        
        Ok(())
    }

    pub fn place_wager(ctx: Context<PlaceWager>, game_id: u32, amount: u64, prediction_str: String) -> Result<()> {
        let prediction = GameResult::from_str(&prediction_str)?;
        let contract = &mut ctx.accounts.contract;
        let sys_program = &mut ctx.accounts.system_program;
        let wallet = &mut ctx.accounts.program_wallet;
        let user = &mut ctx.accounts.user;

        let game = if let Some(game) = contract.active_games.iter_mut().find(|g| g.id == game_id) {game} else {
            return Err(ProgramErrorCode::InvalidGameId.into());
        };
        if game.state != GameState::Scheduled {
            return Err(ProgramErrorCode::GameAlreadyStarted.into());
        }
        if game.wagers.len() >= ProgramContract::MAX_WAGERS_PER_GAME {
            return Err(ProgramErrorCode::MaxWagersPerGameReached.into());
        }
        if let Some(_wager) = game.wagers.iter().find(|&w| w.user == user.key() ) {
            return Err(ProgramErrorCode::WagerAlreadyPlaced.into());
        }

        let transfer_cpi = CpiContext::new(
            sys_program.to_account_info(), 
            system_program::Transfer { 
                from: user.to_account_info(),
                to: wallet.to_account_info(),
            }
        );
        system_program::transfer(transfer_cpi, amount)?;

        let owner_cut = amount * ProgramContract::OWNER_CUT / 100;
        let wager_amount = amount - owner_cut;

        game.wagers.push(Wager {
            user: user.key(),
            prediction,
            lamports: wager_amount,
            collected_reward: false,
        });
        contract.taxes_accumulated += owner_cut;

        let user_history =  &mut ctx.accounts.user_stats.history;
        if user_history.len() >= UserStats::MAX_HISTORY {
            user_history.remove(0);
        }
        user_history.push(WagerSummary {
            game_id,
            lamports_bet: wager_amount,
            lamports_won: 0,
            predicted_result: prediction,
            actuall_result: None,
        });

        Ok(())
    }

    pub fn withdraw_wager(ctx: Context<WithdrawWager>, game_id : u32) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let _sys_program = &mut ctx.accounts.system_program;
        let wallet = &mut ctx.accounts.program_wallet;
        let user = &mut ctx.accounts.user;

        let game = if let Some(game) = contract.active_games.iter_mut().find(|g| g.id == game_id) {game} else {
            return Err(ProgramErrorCode::InvalidGameId.into());
        };
        if game.state != GameState::Scheduled {
            return Err(ProgramErrorCode::GameAlreadyStarted.into());
        }
        let index = if let Some(index) = game.wagers.iter().position(|&w| w.user == user.key() ) {index} else {
            return Err(ProgramErrorCode::WagerNotPlaced.into());
        };
        
        let amount = game.wagers[index].lamports;
        **wallet.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        game.wagers.remove(index);

        let user_history =  &mut ctx.accounts.user_stats.history;
        if let Some(index) = user_history.iter().position(|&w| w.game_id == game_id ) {
            user_history.remove(index);
        }
        
        Ok(())
    }

    pub fn collect_taxes(ctx: Context<CollectTaxes>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let contract_info = contract.to_account_info();
        let sys_program = &mut ctx.accounts.system_program;
        let owner = &mut ctx.accounts.owner;

        if contract.taxes_accumulated == 0 {
            return Err(ProgramErrorCode::NoAmountOwed.into());
        }

        let transfer_cpi = CpiContext::new(
            sys_program.to_account_info(), 
            system_program::Transfer { 
                from: contract_info,
                to: owner.to_account_info(),
            }
        );
        system_program::transfer(transfer_cpi, contract.taxes_accumulated)?;
        contract.taxes_accumulated = 0;

        Ok(())
    }

    pub fn set_game_state(ctx: Context<SetGameState>, game_id : u32, state_str: String, result_str: String ) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let result = GameResult::from_str_opt(&result_str)?;
        let state = GameState::from_str(&state_str)?;
        let game = if let Some(game) = contract.active_games.iter_mut().find(|g| g.id == game_id) {game} else {
            return Err(ProgramErrorCode::InvalidGameId.into());
        };
        game.state = state;
        game.result = result;

        Ok(())
    }

    pub fn delete_game(ctx: Context<DeleteGame>, game_id : u32) -> Result<()> {
        let contract = &mut ctx.accounts.contract;

        let game_idx = if let Some(idx) = contract.active_games.iter().position(|g| g.id == game_id) {idx} else {
            return Err(ProgramErrorCode::InvalidGameId.into());
        };
        let game = &contract.active_games[game_idx];
        if game.state == GameState::Scheduled || game.state == GameState::Live {
            return Err(ProgramErrorCode::GameNotFinished.into());
        }

        let mut total_pool : u64 = 0;
        let mut victorious_pool : u64 = 0;
        for wager in game.wagers.iter() {
            total_pool += wager.lamports;
            if game.result.is_some() && wager.prediction == game.result.unwrap() {
                victorious_pool += wager.lamports;
            }
        }

        let mut tax_pool : u64 = 0;
        for wager in game.wagers.iter() {
            let won_amount = if game.result.is_none() {
                wager.lamports
            } else if wager.prediction == game.result.unwrap() {
                wager.lamports * total_pool / victorious_pool
            } else { 
                0
            };

            if !wager.collected_reward {
                tax_pool += won_amount;
            }
        }
        contract.active_games.remove(game_idx);
        contract.taxes_accumulated += tax_pool;
        
        Ok(())
    }

    pub fn collect_wager(ctx: Context<CollectWager>, game_id : u32) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let contract_info = contract.to_account_info();
        let sys_program = &mut ctx.accounts.system_program;
        let user = &mut ctx.accounts.user;

        let game = if let Some(game) = contract.active_games.iter_mut().find(|g| g.id == game_id) {game} else {
            return Err(ProgramErrorCode::InvalidGameId.into());
        };
        if game.state == GameState::Scheduled || game.state == GameState::Live {
            return Err(ProgramErrorCode::GameNotFinished.into());
        }
        let wager_idx = if let Some(wager_idx) = game.wagers.iter().position(|&w| w.user == user.key() ) {wager_idx} else {
            return Err(ProgramErrorCode::WagerNotPlaced.into());
        };

        let user_wager = &mut game.wagers[wager_idx];
        let user_history =  &mut ctx.accounts.user_stats.history;
        if game.result.is_some() && user_wager.prediction != game.result.unwrap() {
            if let Some(summary) = user_history.iter_mut().find(|s| s.game_id == game_id) {
                summary.actuall_result = game.result;
            }
            return Err(ProgramErrorCode::NoAmountOwed.into());
        }
        if user_wager.collected_reward {
            return Err(ProgramErrorCode::NoAmountOwed.into());
        }

        let waged_amount = user_wager.lamports;
        let mut total_pool : u64 = 0;
        let mut victorious_pool : u64 = 0;
        for wager in game.wagers.iter() {
            total_pool += wager.lamports;
            if game.result.is_some() && wager.prediction == game.result.unwrap() {
                victorious_pool += wager.lamports;
            }
        }
        let won_amount = if game.result.is_none() {
            waged_amount
        } else {
            waged_amount * total_pool / victorious_pool
        };

        let transfer_cpi = CpiContext::new(
            sys_program.to_account_info(), 
            system_program::Transfer { 
                from: contract_info,
                to: user.to_account_info(),
            }
        );
        system_program::transfer(transfer_cpi, won_amount)?;
        game.wagers[wager_idx].collected_reward = true;

        if let Some(summary) = user_history.iter_mut().find(|s| s.game_id == game_id) {
            summary.actuall_result = game.result;
            summary.lamports_won = won_amount;
        }

        Ok(())
    }


}