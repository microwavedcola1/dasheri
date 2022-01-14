use anchor_lang::prelude::*;
use mango::instruction;
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct Deposit {}

pub fn handler(ctx: Context<Deposit>) -> ProgramResult {
    Ok(())
}
