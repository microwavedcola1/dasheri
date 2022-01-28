use anchor_lang::prelude::*;

/// A pool account is a user account attached to a pool, it will do bookkeeping
/// for owner, and additionally other metadata e.g. how much funds did the user
/// deposit in pool etc.
#[account]
#[derive(Default)]
pub struct PoolAccount {
    pub bump: u8,
    pub pool: Pubkey,
    pub owner: Pubkey,
    pub padding: [u8; 31],
}
const_assert!(std::mem::size_of::<PoolAccount>() == 1 + 32 + 32 + 31);
