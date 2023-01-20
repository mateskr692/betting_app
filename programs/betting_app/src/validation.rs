use crate::data::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, payer = user, space = UserStats::MAX_SIZE + 8, seeds = [b"user-stats", user.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserStats>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceWager<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    #[account(mut, seeds = [b"program-wallet", contract.key().as_ref()], bump)]
    pub program_wallet: Account<'info, ProgramWallet>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawWager<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    #[account(mut, seeds = [b"program-wallet", contract.key().as_ref()], bump)]
    pub program_wallet: Account<'info, ProgramWallet>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CollectWager<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    #[account(mut, seeds = [b"program-wallet", contract.key().as_ref()], bump)]
    pub program_wallet: Account<'info, ProgramWallet>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, address = ProgramContract::owner_key() @ ProgramErrorCode::InstructionNotPermitted)]
    pub owner: Signer<'info>,

    #[account(init, payer = owner, space = ProgramContract::BASE_SIZE + 8)]
     pub contract: Account<'info, ProgramContract>,

     #[account(init, payer = owner, space = ProgramWallet::MAX_SIZE + 8, seeds = [b"program-wallet", contract.key().as_ref()], bump)]
     pub program_wallet: Account<'info, ProgramWallet>,

     pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ReserveSpace<'info> {
    #[account(mut, address = ProgramContract::owner_key() @ ProgramErrorCode::InstructionNotPermitted)]
    pub owner: Signer<'info>,

    #[account(mut, realloc = ProgramContract::total_size(contract.max_games + 1) + 8, realloc::payer = owner, realloc::zero = false)]
     pub contract: Account<'info, ProgramContract>,

     pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddScheduledGame<'info> {
    #[account(mut, address = ProgramContract::owner_key() @ ProgramErrorCode::InstructionNotPermitted)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CollectTaxes<'info> {
    #[account(mut, address = ProgramContract::owner_key() @ ProgramErrorCode::InstructionNotPermitted)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    #[account(mut, seeds = [b"program-wallet", contract.key().as_ref()], bump)]
    pub program_wallet: Account<'info, ProgramWallet>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetGameState<'info> {
    #[account(mut, address = ProgramContract::owner_key() @ ProgramErrorCode::InstructionNotPermitted)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteGame<'info> {
    #[account(mut, address = ProgramContract::owner_key() @ ProgramErrorCode::InstructionNotPermitted)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub contract: Account<'info, ProgramContract>,

    pub system_program: Program<'info, System>,
}