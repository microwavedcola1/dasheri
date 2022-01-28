use anchor_lang::prelude::*;

use crate::pool::state::{Pool, PoolAccount};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct PoolCreatePoolAccount<'info> {
    #[account(
        init,
        seeds = [b"pool_account".as_ref(), pool.key().as_ref()],
        bump = bump,
        payer = user,
        space = 8 + std::mem::size_of::<PoolAccount>(),
    )]
    pub pool_account: Box<Account<'info, PoolAccount>>,

    #[account(
        seeds = [b"pool".as_ref(), pool.admin.key().as_ref()],
        bump = pool.bump,
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PoolCreatePoolAccount>, bump: u8) -> ProgramResult {
    let pool_account = &mut ctx.accounts.pool_account;
    pool_account.bump = bump;
    pool_account.pool = ctx.accounts.pool.key();
    pool_account.owner = ctx.accounts.user.key();

    Ok(())
}
