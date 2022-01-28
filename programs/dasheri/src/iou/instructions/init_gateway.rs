use crate::iou::state::Gateway;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Mint, Token};

#[derive(Accounts)]
#[instruction(gateway_bump: u8, deposit_iou_mint_bump: u8)]
pub struct IouInitGateway<'info> {
    #[account(
        init,
        seeds = [b"gateway".as_ref(), admin.key().as_ref()],
        bump = gateway_bump,
        payer = admin
    )]
    pub gateway: Box<Account<'info, Gateway>>,

    // todo: create mint for borrowing iou
    /// The mint for iou which will represent user deposits
    #[account(
        init,
        mint::decimals = token_mint.decimals,
        mint::authority = gateway,
        mint::freeze_authority = gateway,
        seeds = [token_mint.key().as_ref()],
        bump = deposit_iou_mint_bump,
        payer = admin
    )]
    pub deposit_iou_mint: Box<Account<'info, Mint>>,

    /// The mint for the token deposited via the gateway
    pub token_mint: Account<'info, Mint>,

    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<IouInitGateway>,
    gateway_bump: u8,
    deposit_iou_mint_bump: u8,
) -> ProgramResult {
    let gateway = &mut ctx.accounts.gateway;
    gateway.token_mint = ctx.accounts.token_mint.key();
    gateway.bump = gateway_bump;
    gateway.deposit_iou_mint_bump = deposit_iou_mint_bump;
    gateway.deposit_iou_mint = ctx.accounts.deposit_iou_mint.key();
    gateway.admin = ctx.accounts.admin.key();

    Ok(())
}
