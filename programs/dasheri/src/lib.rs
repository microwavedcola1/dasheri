use anchor_lang::prelude::*;
use mango::instruction::MangoInstruction;
use solana_program::instruction::AccountMeta;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dasheri {
    use solana_program::program::invoke;

    use super::*;

    pub fn create_mango_account(
        ctx: Context<CreateMangoAccount>,
        account_num: u64,
    ) -> ProgramResult {
        invoke(
            &solana_program::instruction::Instruction {
                program_id: *ctx.accounts.mango_program_ai.key,
                accounts: vec![
                    AccountMeta::new(*ctx.accounts.mango_group.to_account_info().key, false),
                    AccountMeta::new(*ctx.accounts.mango_account.to_account_info().key, false),
                    AccountMeta::new(*ctx.accounts.owner.to_account_info().key, true),
                    AccountMeta::new_readonly(
                        *ctx.accounts.system_program.to_account_info().key,
                        false,
                    ),
                ],
                data: MangoInstruction::CreateMangoAccount {
                    account_num: account_num,
                }
                .pack(),
            },
            &[
                ctx.accounts.mango_program_ai.to_account_info().clone(),
                ctx.accounts.mango_group.to_account_info().clone(),
                ctx.accounts.mango_account.to_account_info().clone(),
                ctx.accounts.owner.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info().clone(),
            ],
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(account_num: u64)]
pub struct CreateMangoAccount<'info> {
    pub mango_program_ai: AccountInfo<'info>,

    #[account(mut)]
    pub mango_group: AccountInfo<'info>,

    #[account(mut)]
    pub mango_account: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
