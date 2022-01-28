#[macro_use]
extern crate static_assertions;

use anchor_lang::prelude::*;
use error::*;
use iou::instructions::*;
use pool::instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod error;
mod ids;
mod iou;
mod pool;

#[program]
pub mod dasheri {
    use super::*;
    /// iou

    pub fn iou_init_gateway(
        ctx: Context<IouInitGateway>,
        _gateway_bump: u8,
        _deposit_iou_mint_bump: u8,
    ) -> ProgramResult {
        iou::instructions::init_gateway::handler(ctx, _gateway_bump, _deposit_iou_mint_bump)
    }

    pub fn iou_deposit_into_mango_account(
        ctx: Context<IouDepositIntoMangoAccount>,
        quantity: u64,
    ) -> ProgramResult {
        iou::instructions::deposit_into_mango_account::handler(ctx, quantity)
    }

    /// pool

    pub fn pool_create_pool(ctx: Context<PoolCreatePool>, bump: u8) -> ProgramResult {
        create_pool::handler(ctx, bump)
    }

    pub fn pool_create_pool_account(
        ctx: Context<PoolCreatePoolAccount>,
        bump: u8,
    ) -> ProgramResult {
        create_pool_account::handler(ctx, bump)
    }

    pub fn pool_deposit_into_pool(ctx: Context<PoolDepositIntoPool>, amount: u64) -> ProgramResult {
        deposit_into_pool::handler(ctx, amount)
    }

    pub fn pool_create_mango_account(
        ctx: Context<PoolCreateMangoAccount>,
        account_num: u64,
        bump: u8,
    ) -> ProgramResult {
        create_mango_account::handler(ctx, account_num, bump)
    }

    pub fn pool_deposit_into_mango_account(
        ctx: Context<PoolDepositIntoMangoAccount>,
        quantity: u64,
    ) -> ProgramResult {
        pool::instructions::deposit_into_mango_account::handler(ctx, quantity)
    }
}
