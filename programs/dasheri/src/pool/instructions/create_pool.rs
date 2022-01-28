use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::pool::state::Pool;

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct PoolCreatePool<'info> {
    #[account(
        init,
        seeds = [b"pool".as_ref(), admin.key().as_ref()],
        bump = bump,
        payer = admin
    )]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        init,
        associated_token::authority = pool,
        associated_token::mint = deposit_iou_mint,
        payer = admin
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    // todo: verify that the mint is a specific one you are expecting e.g. usdc
    // #[account(
    //     constraint = deposit_iou_mint.key() == usdc_token::ID
    // )]
    pub deposit_iou_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<PoolCreatePool>, bump: u8) -> ProgramResult {
    let pool = &mut ctx.accounts.pool;
    pool.bump = bump;
    pool.deposit_iou_mint = ctx.accounts.deposit_iou_mint.key();
    pool.vault = ctx.accounts.vault.key();
    pool.admin = ctx.accounts.admin.key();

    Ok(())
}
