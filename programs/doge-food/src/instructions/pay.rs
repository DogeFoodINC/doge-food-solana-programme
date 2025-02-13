use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::token::{spl_token, transfer, Mint, TokenAccount, Transfer};

use crate::{
    constants::{BPS, CONFIG_PDA_SEED, CONTRACT_ATA, ORDER_TS_PDA, PAYMENT_CONFIG_PDA_SEED},
    error::DogeFoodContractError,
    events::OrderPaymentProcessed,
    state::{Global, OrderTs, PaymentTokenMintConfig},
};
use std::mem::size_of;

#[derive(Accounts)]
#[instruction(order_id: Vec<u8>, payment_config_id: Vec<u8>)]
pub struct PayAccounts<'info> {
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

    #[account(
        address = global.recipt_authority
    )]
    pub recipt_authority: Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [ORDER_TS_PDA, &order_id],
        bump,
        space = 8 + size_of::<OrderTs>(),
        rent_exempt = enforce
    )]
    pub order_ts: Box<Account<'info, OrderTs>>,

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
        mut,
        associated_token::authority = user,
        associated_token::mint = token_mint,
        associated_token::token_program = token_programme
    )]
    pub user_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [&token_programme.key.to_bytes(), &token_mint.key().to_bytes(), CONTRACT_ATA],
        bump,
        token::authority = global,
        token::mint = token_mint,
        token::token_program = token_programme
    )]
    pub contract_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::authority = global.team_wallet,
        associated_token::mint = token_mint,
        associated_token::token_program = token_programme
    )]
    pub team_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::authority = global.charity_wallet,
        token::mint = token_mint,
        token::token_program = token_programme
    )]
    pub charity_ata: Box<Account<'info, TokenAccount>>,

    /// CHECK: not required
    #[account(
        executable,
        address = crate::id()
    )]
    pub pay_processor: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn pay_order(
    ctx: Context<PayAccounts>,
    order_id: Vec<u8>,
    payment_config_id: Vec<u8>,
    expired_ts: u64,
    payment_amount: u64,
) -> Result<()> {
    let global_config = &ctx.accounts.global;
    let payment_token_config = &ctx.accounts.payment_token_config;
    let clock = Clock::get()?;

    require!(
        global_config.initialized,
        DogeFoodContractError::NotInitialized
    );
    require!(
        payment_token_config.is_enabled,
        DogeFoodContractError::PaymentNotEnabled
    );
    require!(
        clock.unix_timestamp as u64 <= expired_ts,
        DogeFoodContractError::OrderExpired
    );

    transfer(
        CpiContext::new(
            ctx.accounts.token_programme.to_account_info(),
            Transfer {
                from: ctx.accounts.user_ata.to_account_info().clone(),
                to: ctx.accounts.contract_ata.to_account_info().clone(),
                authority: ctx.accounts.user.to_account_info().clone(),
            },
        ),
        payment_amount,
    )?;

    let charity_amount = payment_amount * global_config.charity_portion_bps as u64 / (BPS as u64);
    let global_ata_seeds: &[&[&[u8]]] = &[&[
        CONFIG_PDA_SEED,
        &[ctx.bumps.global]
    ]];
   
    transfer(
        CpiContext::new(
            ctx.accounts.token_programme.to_account_info(),
            Transfer {
                from: ctx.accounts.contract_ata.to_account_info().clone(),
                to: ctx.accounts.charity_ata.to_account_info().clone(),
                authority: ctx.accounts.global.to_account_info().clone(),
            },
        ).with_signer(global_ata_seeds),
        charity_amount,
    )?;

    transfer(
        CpiContext::new(
            ctx.accounts.token_programme.to_account_info(),
            Transfer {
                from: ctx.accounts.contract_ata.to_account_info().clone(),
                to: ctx.accounts.team_ata.to_account_info().clone(),
                authority: ctx.accounts.global.to_account_info().clone(),
            },
        ).with_signer(global_ata_seeds),
        payment_amount - charity_amount,
    )?;

    let order_ts_acc = &mut ctx.accounts.order_ts;
    order_ts_acc.user = *ctx.accounts.user.key;
    order_ts_acc.ts = expired_ts;

    msg!(
        "Payment. OID: {}, PCFG: {}, payer: {}, a: {}, ca: {}",
        order_id
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>(),
        payment_config_id
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>(),
        *ctx.accounts.user.key,
        payment_amount,
        charity_amount
    );

    emit!(OrderPaymentProcessed {
        order_id: order_id,
        payment_config_id: payment_config_id,
        payer: *ctx.accounts.user.key,
        amount: payment_amount,
        charity_amount: charity_amount,
    });

    Ok(())
}
