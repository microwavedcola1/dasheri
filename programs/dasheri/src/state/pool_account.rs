use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct PoolAccount {
    pub pool: Pubkey,
}
const_assert!(std::mem::size_of::<PoolAccount>() == 32);
