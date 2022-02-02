use crate::error::ErrorCode::NotAProgram;
use crate::pool::state::Pool;
use anchor_lang::prelude::*;
use mango::instruction;
use solana_program::program::invoke_signed;

#[derive(Accounts)]
#[instruction(account_num: u64, bump: u8)]
pub struct PoolCreateMangoAccount<'info> {
    // todo: check target program key
    pub mango_program: UncheckedAccount<'info>,
    // todo: check target group key
    #[account(mut)]
    pub mango_group: UncheckedAccount<'info>,

    // Note: seed contraint checkw will be done in mango
    #[account(mut)]
    pub mango_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool.admin.key().as_ref()],
        bump = pool.bump,
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PoolCreateMangoAccount>, account_num: u64, bump: u8) -> ProgramResult {
    require!(ctx.accounts.mango_program.executable, NotAProgram);

    let instruction = instruction::create_mango_account(
        ctx.accounts.mango_program.key,
        &ctx.accounts.mango_group.key(),
        &ctx.accounts.mango_account.key(),
        &ctx.accounts.pool.key(),
        &ctx.accounts.system_program.key(),
        &ctx.accounts.payer.key(),
        account_num,
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
            ctx.accounts.system_program.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
        ],
        &[&seeds[..]],
    )?;

    Ok(())
}
