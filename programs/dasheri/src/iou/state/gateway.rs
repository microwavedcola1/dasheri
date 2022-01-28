use anchor_lang::prelude::*;

/// A gateway acts a middleware for depositing and borrowing. Depositing in mango
/// doesn't return iou tokens.
/// One can use a deposit mint to mint and return iou
/// tokens back to depositor.
#[account]
#[derive(Default)]
pub struct Gateway {
    pub token_mint: Pubkey,
    pub bump: u8,
    pub deposit_iou_mint_bump: u8,
    pub deposit_iou_mint: Pubkey,
    pub admin: Pubkey,
    pub padding: [u8; 31],
}
const_assert!(std::mem::size_of::<Gateway>() == 1 + 1 + 32 + 32 + 32 + 31);
