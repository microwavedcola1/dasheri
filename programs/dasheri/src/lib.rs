use anchor_lang::prelude::*;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod error;
pub mod ids;
pub mod instructions;
pub mod state;

#[macro_use]
extern crate static_assertions;

#[program]
pub mod dasheri {
    use super::*;
    use anchor_spl::associated_token::Create;

    pub fn create_pool(ctx: Context<CreatePool>, bump: u8) -> ProgramResult {
        instructions::create_pool::handler(ctx, bump)
    }

    pub fn create_pool_account(ctx: Context<CreatePoolAccount>, bump: u8) -> ProgramResult {
        instructions::create_pool_account::handler(ctx, bump)
    }

    pub fn deposit_into_pool(ctx: Context<DepositIntoPool>, amount: u64) -> ProgramResult {
        instructions::deposit_into_pool::handler(ctx, amount)
    }

    pub fn create_mango_account(
        ctx: Context<CreateMangoAccount>,
        account_num: u64,
        bump: u8,
    ) -> ProgramResult {
        instructions::create_mango_account::handler(ctx, account_num, bump)
    }

    pub fn deposit(ctx: Context<DepositIntoMangoAccount>, quantity: u64) -> ProgramResult {
        instructions::deposit_into_mango_account::handler(ctx, quantity)
    }
}
