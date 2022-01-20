use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Pool {
    pub vault: Pubkey,
}
const_assert!(std::mem::size_of::<Pool>() == 32);
