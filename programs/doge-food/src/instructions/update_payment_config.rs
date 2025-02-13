use anchor_lang::prelude::*;

use crate::{
    constants::{CONFIG_PDA_SEED, PAYMENT_CONFIG_PDA_SEED},
    error::DogeFoodContractError,
    state::{Global, PaymentTokenMintConfig},
};
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(payment_config_id: Vec<u8>)]
pub struct UpdatePaymentConfigAccounts<'info> {
    #[account(
        seeds = [CONFIG_PDA_SEED],
        bump,
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(
        mut,
        address = global.owner
    )]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        seeds = [PAYMENT_CONFIG_PDA_SEED, &payment_config_id],
        bump,
        space = 8 + size_of::<PaymentTokenMintConfig>(),
        rent_exempt = enforce
    )]
    pub payment_token_config: Box<Account<'info, PaymentTokenMintConfig>>,

    pub system_program: Program<'info, System>,
}

pub fn update_payment_config(
    ctx: Context<UpdatePaymentConfigAccounts>,
    _payment_config_id: Vec<u8>,
    token_programme: Pubkey,
    token_mint: Pubkey,
    is_enabled: bool,
) -> Result<()> {
    let global_config = &ctx.accounts.global;
    require!(
        global_config.initialized,
        DogeFoodContractError::NotInitialized
    );

    let payment_token_config = &mut ctx.accounts.payment_token_config;

    payment_token_config.token_programme = token_programme;
    payment_token_config.token_mint = token_mint;
    payment_token_config.is_enabled = is_enabled;

    Ok(())
}
