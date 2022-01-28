use crate::pool::state::{Pool, PoolAccount};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct PoolDepositIntoPool<'info> {
    #[account(
        seeds = [b"pool".as_ref(), pool.admin.key().as_ref()],
        bump = pool.bump,
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        seeds = [b"pool_account".as_ref(), pool.key().as_ref()],
        bump = pool_account.bump,
        constraint = pool_account.owner == user.key()
    )]
    pub pool_account: Box<Account<'info, PoolAccount>>,

    #[account(
        mut,
        associated_token::authority = pool,
        associated_token::mint = pool.deposit_iou_mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = deposit_token.owner == user.key(),
    )]
    pub deposit_token: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> PoolDepositIntoPool<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = token::Transfer {
            from: self.deposit_token.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

pub fn handler(ctx: Context<PoolDepositIntoPool>, amount: u64) -> ProgramResult {
    token::transfer(ctx.accounts.transfer_ctx(), amount)?;

    // todo: record how much amount user deposited on pool_account
    // todo: record pool performance at the time of deposit, is important if you want
    // to support users depositing at various points in time (after pool has already e.g. started
    // trading on mango), so that you can distribute rewards fairly

    Ok(())
}
