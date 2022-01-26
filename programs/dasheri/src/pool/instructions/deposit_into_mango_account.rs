use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use mango::instruction;
use solana_program::program::invoke_signed;

use crate::pool::state::Pool;

#[derive(Accounts)]
pub struct DepositIntoMangoAccount<'info> {
    pub mango_program: UncheckedAccount<'info>,

    pub mango_group: UncheckedAccount<'info>,

    pub mango_cache: UncheckedAccount<'info>,

    pub root_bank: UncheckedAccount<'info>,

    #[account(mut)]
    pub node_bank: UncheckedAccount<'info>,

    #[account(mut)]
    pub node_bank_vault: UncheckedAccount<'info>,

    #[account(mut)]
    pub owner_token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub mango_account: UncheckedAccount<'info>,

    pub pool: Box<Account<'info, Pool>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DepositIntoMangoAccount>, quantity: u64) -> ProgramResult {
    let instruction = instruction::deposit(
        &ctx.accounts.mango_program.key(),
        &ctx.accounts.mango_group.key(),
        &ctx.accounts.mango_account.key(),
        &ctx.accounts.pool.key(),
        &ctx.accounts.mango_cache.key(),
        &ctx.accounts.root_bank.key(),
        &ctx.accounts.node_bank.key(),
        &ctx.accounts.node_bank_vault.key(),
        &ctx.accounts.owner_token_account.key(),
        quantity,
    )
    .unwrap();

    let pool_admin_key = &ctx.accounts.pool.admin.key();
    let seeds = &[
        b"pool".as_ref(),
        pool_admin_key.as_ref(),
        &[ctx.accounts.pool.bump],
    ];
    invoke_signed(
        &instruction,
        &[
            ctx.accounts.mango_program.to_account_info().clone(),
            ctx.accounts.mango_group.to_account_info().clone(),
            ctx.accounts.mango_account.to_account_info().clone(),
            ctx.accounts.pool.to_account_info().clone(),
            ctx.accounts.mango_cache.to_account_info().clone(),
            ctx.accounts.root_bank.to_account_info().clone(),
            ctx.accounts.node_bank.to_account_info().clone(),
            ctx.accounts.node_bank_vault.to_account_info().clone(),
            ctx.accounts.owner_token_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            ctx.accounts.token_program.to_account_info().clone(),
        ],
        &[&seeds[..]],
    )?;
    Ok(())
}
