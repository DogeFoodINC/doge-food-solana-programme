use anchor_lang::prelude::*;

#[account]
pub struct OrderTs {
    pub ts:u64,
    pub user: Pubkey
}