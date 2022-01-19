use anchor_lang::prelude::*;
use mango::instruction;
use solana_program::program::invoke;

#[derive(Accounts)]
#[instruction(account_num: u64)]
pub struct CreateMangoAccount<'info> {
    pub mango_program: AccountInfo<'info>,

    #[account(mut)]
    pub mango_group: AccountInfo<'info>,

    #[account(mut)]
    pub mango_account: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMangoAccount>, account_num: u64) -> ProgramResult {
    let instruction = instruction::create_mango_account(
        ctx.accounts.mango_program.key,
        ctx.accounts.mango_group.to_account_info().key,
        ctx.accounts.mango_account.to_account_info().key,
        ctx.accounts.owner.to_account_info().key,
        ctx.accounts.system_program.to_account_info().key,
        account_num,
    )
    .unwrap();

    invoke(
        &instruction,
        &[
            ctx.accounts.mango_program.to_account_info().clone(),
            ctx.accounts.mango_group.to_account_info().clone(),
            ctx.accounts.mango_account.to_account_info().clone(),
            ctx.accounts.owner.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        ],
    )?;

    Ok(())
}
