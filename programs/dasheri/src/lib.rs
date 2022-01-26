#[macro_use]
extern crate static_assertions;

use anchor_lang::prelude::*;

use pool::instructions::create_mango_account::CreateMangoAccount;
use pool::instructions::create_pool::CreatePool;
use pool::instructions::create_pool_account::CreatePoolAccount;
use pool::instructions::deposit_into_mango_account::DepositIntoMangoAccount;
use pool::instructions::deposit_into_pool::DepositIntoPool;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod ids;
pub mod pool;

#[program]
pub mod dasheri {
    use super::*;

    pub fn create_pool(ctx: Context<CreatePool>, bump: u8) -> ProgramResult {
        create_pool::handler(ctx, bump)
    }

    pub fn create_pool_account(ctx: Context<CreatePoolAccount>, bump: u8) -> ProgramResult {
        create_pool_account::handler(ctx, bump)
    }

    pub fn deposit_into_pool(ctx: Context<DepositIntoPool>, amount: u64) -> ProgramResult {
        deposit_into_pool::handler(ctx, amount)
    }

    pub fn create_mango_account(
        ctx: Context<CreateMangoAccount>,
        account_num: u64,
        bump: u8,
    ) -> ProgramResult {
        create_mango_account::handler(ctx, account_num, bump)
    }

    pub fn deposit(ctx: Context<DepositIntoMangoAccount>, quantity: u64) -> ProgramResult {
        deposit_into_mango_account::handler(ctx, quantity)
    }
}
