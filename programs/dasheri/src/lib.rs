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

    pub fn create_mango_account(
        ctx: Context<CreateMangoAccount>,
        account_num: u64,
    ) -> ProgramResult {
        instructions::create_mango_account::handler(ctx, account_num)
    }

    pub fn deposit(ctx: Context<DepositIntoMangoAccount>, quantity: u64) -> ProgramResult {
        instructions::deposit_into_mango_account::handler(ctx, quantity)
    }
}
