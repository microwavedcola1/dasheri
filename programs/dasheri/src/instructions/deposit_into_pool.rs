use crate::state::{Pool, PoolAccount};
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Token, TokenAccount};
use mango::instruction;
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct DepositIntoPool<'info> {
    #[account(
        seeds = [b"pool".as_ref(), pool.admin.key().as_ref()],
        bump = pool.bump,
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        associated_token::authority = pool,
        associated_token::mint = pool.deposit_mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"pool_account".as_ref(), pool.key().as_ref()],
        bump = pool_account.bump,
        constraint = pool_account.owner == payer.key()
    )]
    pub pool_account: Box<Account<'info, PoolAccount>>,

    #[account(
        mut,
        constraint = deposit_token.owner == payer.key(),
    )]
    pub deposit_token: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> DepositIntoPool<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = token::Transfer {
            from: self.deposit_token.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.payer.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

pub fn handler(ctx: Context<DepositIntoPool>, amount: u64) -> ProgramResult {
    token::transfer(ctx.accounts.transfer_ctx(), amount)?;

    Ok(())
}
