use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::{
    constants::{CONFIG_PDA_SEED, CONTRACT_ATA, PAYMENT_CONFIG_PDA_SEED},
    error::DogeFoodContractError,
    state::{Global, PaymentTokenMintConfig},
};

#[derive(Accounts)]
#[instruction(payment_config_id: Vec<u8>)]
pub struct CreateContractAtaAccounts<'info> {
    #[account(
        seeds = [CONFIG_PDA_SEED],
        bump
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(
        seeds = [PAYMENT_CONFIG_PDA_SEED, &payment_config_id],
        bump,
    )]
    pub payment_token_config: Box<Account<'info, PaymentTokenMintConfig>>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: not required
    #[account(
        executable,
        address = payment_token_config.token_programme
    )]
    pub token_programme: AccountInfo<'info>,

    #[account(
        mut,
        address = payment_token_config.token_mint
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [&token_programme.key.to_bytes(), &token_mint.key().to_bytes(), CONTRACT_ATA],
        bump,
        token::authority = global,
        token::mint = token_mint,
        token::token_program = token_programme
    )]
    pub contract_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK: not required
    #[account(
        executable,
        address = crate::id()
    )]
    pub pay_processor: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_contract_ata(
    ctx: Context<CreateContractAtaAccounts>,
    _payment_config_id: Vec<u8>,
) -> Result<()> {
    let payment_token_config = &ctx.accounts.payment_token_config;

    require!(
        payment_token_config.is_enabled,
        DogeFoodContractError::PaymentNotEnabled
    );
    
    Ok(())
}
