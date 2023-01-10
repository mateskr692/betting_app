pub mod data;
pub mod logic;

use crate::data::*;
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod betting_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.active_games = Vec::new();

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
        contract.add_scheduled_game(game_id)
    }

    pub fn place_wager(ctx: Context<PlaceWager>, game_id: u32, amount: u64, prediction: GameResult) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.place_wager(ctx.accounts.payer.key(), game_id, amount, prediction)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, address = ProgramContract::OWNER_KEY @ ProgramErrorCode::InstructionNotPermitted)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = ProgramContract::MAX_SIZE + 8)]
     pub contract: Account<'info, ProgramContract>,

     pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, payer = user, space = UserStats::MAX_SIZE + 8, seeds = [b"user-stats", user.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserStats>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddScheduledGame<'info> {
    #[account(mut, address = ProgramContract::OWNER_KEY @ ProgramErrorCode::InstructionNotPermitted)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceWager<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    pub system_program: Program<'info, System>,
}