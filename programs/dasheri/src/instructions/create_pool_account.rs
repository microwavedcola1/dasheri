use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::ids::usdc_token;
use crate::state::{Pool, PoolAccount};

#[derive(Accounts)]
#[instruction(pool_bump: u8)]
pub struct CreatePoolAccount<'info> {
    #[account(
        init,
        seeds = [b"pool_account".as_ref(), pool.key().as_ref()],
        bump = pool_bump,
        payer = payer,
        space = 8 + std::mem::size_of::<PoolAccount>(),
    )]
    pub pool_account: Box<Account<'info, PoolAccount>>,

    pub pool: Box<Account<'info, Pool>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreatePoolAccount>) -> ProgramResult {
    let pool_account = &mut ctx.accounts.pool_account;
    pool_account.pool = ctx.accounts.pool.key();

    Ok(())
}
