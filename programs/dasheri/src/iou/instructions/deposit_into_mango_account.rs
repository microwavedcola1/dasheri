use crate::iou::state::Gateway;
use crate::pool::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::accessor::amount;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use mango::instruction;
use mango::state::MangoAccount;
use solana_program::program::invoke;

#[derive(Accounts)]
pub struct IouDepositIntoMangoAccount<'info> {
    // todo: check target program key
    pub mango_program: UncheckedAccount<'info>,
    // todo: check target group key
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

    // todo: be careful with using init_if_needed (re-initialization attack),
    // ideally split creation of deposit_iou_account into another ix
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
    require!(ctx.accounts.mango_program.executable, NotAProgram);

    log_deposits(&ctx, "mango_account deposits before deposit");

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

    log_deposits(&ctx, "mango_account deposits after deposit");

    token::mint_to(
        ctx.accounts.iou_mint_context().with_signer(&[&[
            "gateway".as_ref(),
            ctx.accounts.gateway.admin.key().as_ref(),
            &[ctx.accounts.gateway.bump],
        ]]),
        // todo: quantity should be based on current deposit index for corresponding asset
        // this is just an over simplification using quantity as is
        quantity,
    )?;

    // note: to compute how interest rate could be computed, see...
    // https://github.com/blockworks-foundation/mango-v3/blob/3583fa19a909aaa4113bcdb23b35c5bede6866ae/program/src/state.rs#L410

    Ok(())
}

fn log_deposits(ctx: &Context<IouDepositIntoMangoAccount>, msg_string: &'static str) {
    let mango_account_deposits = MangoAccount::load_checked(
        &ctx.accounts.mango_account.to_account_info(),
        ctx.accounts.mango_program.key,
        ctx.accounts.mango_group.key,
    )
    .unwrap()
    .deposits;

    msg!("{:?} {:?}", msg_string, mango_account_deposits);
}
