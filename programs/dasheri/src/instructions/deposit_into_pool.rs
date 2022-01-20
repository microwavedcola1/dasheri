use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use mango::instruction;
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct DepositIntoPool<'info> {
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DepositIntoPool>) -> ProgramResult {
    Ok(())
}
