use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Pool {
    pub bump: u8,
    pub deposit_mint: Pubkey,
    pub vault: Pubkey,
    pub admin: Pubkey,
    pub padding: [u8; 31],
}
const_assert!(std::mem::size_of::<Pool>() == 1 + 32 + 32 + 32 + 31);
