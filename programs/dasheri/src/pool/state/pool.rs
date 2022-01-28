use anchor_lang::prelude::*;

/// A pool represents a collective entity. Initialized by an admin,
/// users can create accounts within the pool, and deposit assets
/// of a said deposit mint. Pool would have a corresponding mango account
/// to carry out collective activities e.g.
/// 1) run liquidator using all the deposited
/// amounts
/// 2) run a fund which does trading and distributes the profits to all users etc.
#[account]
#[derive(Default)]
pub struct Pool {
    pub bump: u8,
    pub deposit_iou_mint: Pubkey,
    pub vault: Pubkey,
    pub admin: Pubkey,
    pub padding: [u8; 31],
}
const_assert!(std::mem::size_of::<Pool>() == 1 + 32 + 32 + 32 + 31);
