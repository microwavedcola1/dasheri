use crate::iou::state::Gateway;
use crate::pool::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::accessor::amount;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use mango::instruction;
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct IouDepositIntoMangoAccount<'info> {
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

    #[account(
        seeds = [b"gateway".as_ref(), gateway.admin.key().as_ref()],
        bump = gateway.bump,
        has_one = deposit_iou_mint,
        has_one = token_mint
    )]
    pub gateway: Box<Account<'info, Gateway>>,

    #[account(
        mut,
        seeds = [token_mint.key().as_ref()],
        bump = gateway.deposit_iou_mint_bump,
    )]
    pub deposit_iou_mint: AccountInfo<'info>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        associated_token::authority = payer,
        associated_token::mint = deposit_iou_mint,
        payer = payer
    )]
    pub deposit_iou_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> IouDepositIntoMangoAccount<'info> {
    fn iou_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                to: self.deposit_iou_account.to_account_info(),
                mint: self.deposit_iou_mint.to_account_info(),
                authority: self.gateway.to_account_info(),
            },
        )
    }
}

pub fn handler(ctx: Context<IouDepositIntoMangoAccount>, quantity: u64) -> ProgramResult {
    let instruction = instruction::deposit(
        &ctx.accounts.mango_program.key(),
        &ctx.accounts.mango_group.key(),
        &ctx.accounts.mango_account.key(),
        &ctx.accounts.payer.key(),
        &ctx.accounts.mango_cache.key(),
        &ctx.accounts.root_bank.key(),
        &ctx.accounts.node_bank.key(),
        &ctx.accounts.node_bank_vault.key(),
        &ctx.accounts.owner_token_account.key(),
        quantity,
    )
    .unwrap();

    invoke(
        &instruction,
        &[
            ctx.accounts.mango_program.to_account_info().clone(),
            ctx.accounts.mango_group.to_account_info().clone(),
            ctx.accounts.mango_account.to_account_info().clone(),
            ctx.accounts.payer.to_account_info().clone(),
            ctx.accounts.mango_cache.to_account_info().clone(),
            ctx.accounts.root_bank.to_account_info().clone(),
            ctx.accounts.node_bank.to_account_info().clone(),
            ctx.accounts.node_bank_vault.to_account_info().clone(),
            ctx.accounts.owner_token_account.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            ctx.accounts.token_program.to_account_info().clone(),
        ],
    )?;

    token::mint_to(
        ctx.accounts.iou_mint_context().with_signer(&[&[
            "gateway".as_ref(),
            ctx.accounts.gateway.admin.key().as_ref(),
            &[ctx.accounts.gateway.bump],
        ]]),
        quantity,
    )?;

    Ok(())
}
