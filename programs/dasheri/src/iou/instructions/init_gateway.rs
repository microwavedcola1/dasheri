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

    /// The mint for iou which will represent user deposits
    #[account(
        init,
        seeds = [token_mint.key().as_ref()],
        bump = deposit_iou_mint_bump,
        payer = admin,
        owner = token::ID,
        space = Mint::LEN
    )]
    pub deposit_iou_mint: AccountInfo<'info>,

    /// The mint for the token deposited via the gateway
    pub token_mint: Account<'info, Mint>,

    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> IouInitGateway<'info> {
    fn init_deposit_iou_mint_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, InitializeMint<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            InitializeMint {
                mint: self.deposit_iou_mint.clone(),
                rent: self.rent.to_account_info(),
            },
        )
    }

    fn init_mint(&self) -> ProgramResult {
        token::initialize_mint(
            self.init_deposit_iou_mint_context(),
            self.token_mint.decimals,
            &self.gateway.key(),
            Some(&self.gateway.key()),
        )?;

        // todo: create mint for borrowing iou

        Ok(())
    }
}

pub fn handler(
    ctx: Context<IouInitGateway>,
    gateway_bump: u8,
    deposit_iou_mint_bump: u8,
) -> ProgramResult {
    ctx.accounts.init_mint();

    let gateway = &mut ctx.accounts.gateway;
    gateway.token_mint = ctx.accounts.token_mint.key();
    gateway.bump = gateway_bump;
    gateway.deposit_iou_mint_bump = deposit_iou_mint_bump;
    gateway.deposit_iou_mint = ctx.accounts.deposit_iou_mint.key();
    gateway.admin = ctx.accounts.admin.key();

    Ok(())
}
