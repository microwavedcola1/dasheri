use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod dasheri {
    use super::*;

    pub fn create_mango_account(
        ctx: Context<CreateMangoAccount>,
        account_num: u64,
    ) -> ProgramResult {
        instructions::create_mango_account::create_mango_account(ctx, account_num)
    }
}
