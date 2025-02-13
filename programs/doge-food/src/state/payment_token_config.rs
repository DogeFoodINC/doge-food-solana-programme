use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

#[account]
pub struct PaymentTokenMintConfig {
    pub token_programme: Pubkey,
    pub token_mint: Pubkey,
    pub is_enabled: bool,
}
