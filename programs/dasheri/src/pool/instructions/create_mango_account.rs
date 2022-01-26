use crate::pool::state::Pool;
use anchor_lang::prelude::*;
use mango::instruction;
use solana_program::program::invoke_signed;

#[derive(Accounts)]
#[instruction(account_num: u64, bump: u8)]
pub struct CreateMangoAccount<'info> {
    pub mango_program: AccountInfo<'info>,

    #[account(mut)]
    pub mango_group: AccountInfo<'info>,

    #[account(mut)]
    pub mango_account: AccountInfo<'info>,

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

pub fn handler(ctx: Context<CreateMangoAccount>, account_num: u64, bump: u8) -> ProgramResult {
    let instruction = instruction::create_mango_account(
        ctx.accounts.mango_program.key,
        ctx.accounts.mango_group.to_account_info().key,
        ctx.accounts.mango_account.to_account_info().key,
        ctx.accounts.pool.to_account_info().key,
        ctx.accounts.system_program.to_account_info().key,
        ctx.accounts.payer.to_account_info().key,
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
