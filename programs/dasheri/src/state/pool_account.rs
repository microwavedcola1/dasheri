use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PoolAccount {
    pub bump: u8,
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub padding: [u8; 31],
}
const_assert!(std::mem::size_of::<PoolAccount>() == 1 + 32 + 32 + 31);
