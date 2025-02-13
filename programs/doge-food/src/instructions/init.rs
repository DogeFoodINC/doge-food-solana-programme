use anchor_lang::prelude::*;

use crate:: {
    constants::{BPS, CONFIG_PDA_SEED}, error::DogeFoodContractError, state::Global
};
use std::mem::size_of;

#[derive(Accounts)]
pub struct InitAccounts<'info> { 
    #[account(
        init,
        payer = owner,
        seeds = [CONFIG_PDA_SEED],
        bump,
        space = 8 + size_of::<Global>(),
        rent_exempt = enforce
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn init(
    ctx: Context<InitAccounts>,
    recipt_authority: Pubkey,
    team_wallet: Pubkey,
    charity_wallet: Pubkey,
    charity_portion_bps: u16,
) -> Result<()> {
    // TODO: ensure that owner is our wallet

    let global_config = &mut ctx.accounts.global;

    require!(global_config.initialized == false, DogeFoodContractError::AlreadyInitialized);

    global_config.recipt_authority = recipt_authority;
    global_config.team_wallet = team_wallet;
    global_config.charity_wallet = charity_wallet;

    require!(charity_portion_bps <= BPS, DogeFoodContractError::InvalidBps);
    global_config.charity_portion_bps = charity_portion_bps;
    global_config.owner = *ctx.accounts.owner.key;
    global_config.initialized = true;

    Ok(())
}