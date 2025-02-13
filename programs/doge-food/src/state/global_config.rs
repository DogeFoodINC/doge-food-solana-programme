use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

#[account]
pub struct Global {
    pub initialized: bool,
    pub recipt_authority: Pubkey,
    pub team_wallet: Pubkey,
    pub charity_wallet: Pubkey,
    pub charity_portion_bps: u16,
    pub owner: Pubkey,
}