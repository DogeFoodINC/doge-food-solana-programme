use anchor_lang::prelude::*;

use crate::{
    constants::{BPS, CONFIG_PDA_SEED},
    error::DogeFoodContractError,
    state::Global,
};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [CONFIG_PDA_SEED],
        bump
    )]
    pub global: Box<Account<'info, Global>>,

    #[account(
        address = global.owner
    )]
    pub owner: Signer<'info>,
}

pub fn update_config(
    ctx: Context<UpdateConfig>,
    recipt_authority: Option<Pubkey>,
    team_wallet: Option<Pubkey>,
    charity_wallet: Option<Pubkey>,
    charity_portion_bps: Option<u16>,
    new_owner: Option<Pubkey>,
) -> Result<()> {
    let global_config = &mut ctx.accounts.global;

    require!(
        global_config.initialized,
        DogeFoodContractError::NotInitialized
    );

    if let Some(new_value) = recipt_authority {
        global_config.recipt_authority = new_value;
    }

    if let Some(new_value) = team_wallet {
        global_config.team_wallet = new_value;
    }

    if let Some(new_value) = charity_wallet {
        global_config.charity_wallet = new_value;
    }

    if let Some(new_value) = charity_portion_bps {
        require!(new_value <= BPS, DogeFoodContractError::InvalidBps);

        global_config.charity_portion_bps = new_value;
    }

    if let Some(new_value) = new_owner {
        global_config.owner = new_value;
    }

    Ok(())
}
