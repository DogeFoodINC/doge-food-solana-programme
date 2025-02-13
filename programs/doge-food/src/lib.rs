pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("Hjz8SGLuX6TrzENSusARf2i8TCgXK3aKzDaedXWq5KCJ");

#[program]
pub mod doge_food {
    use super::*;

    pub fn init(
        ctx: Context<InitAccounts>,
        recipt_authority: Pubkey,
        team_wallet: Pubkey,
        charity_wallet: Pubkey,
        charity_portion_bps: u16,
    ) -> Result<()> {
        instructions::init(
            ctx,
            recipt_authority,
            team_wallet,
            charity_wallet,
            charity_portion_bps,
        )
    }

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        recipt_authority: Option<Pubkey>,
        team_wallet: Option<Pubkey>,
        charity_wallet: Option<Pubkey>,
        charity_portion_bps: Option<u16>,
        new_owner: Option<Pubkey>,
    ) -> Result<()> {
        instructions::update_config(
            ctx,
            recipt_authority,
            team_wallet,
            charity_wallet,
            charity_portion_bps,
            new_owner,
        )
    }

    pub fn update_payment_config(
        ctx: Context<UpdatePaymentConfigAccounts>,
        payment_config_id: Vec<u8>,
        token_programme: Pubkey,
        token_mint: Pubkey,
        is_enabled: bool,
    ) -> Result<()> {
        instructions::update_payment_config(
            ctx,
            payment_config_id,
            token_programme,
            token_mint,
            is_enabled,
        )
    }

    pub fn pay_order(
        ctx: Context<PayAccounts>,
        order_id: Vec<u8>,
        payment_config_id: Vec<u8>,
        expired_ts: u64,
        payment_amount: u64,
    ) -> Result<()> {
        instructions::pay_order(ctx, order_id, payment_config_id, expired_ts, payment_amount)
    }

    pub fn close_order_ts(ctx: Context<CloseAccounts>, order_ids: Vec<Vec<u8>>) -> Result<()> {
        instructions::close_order_ts(ctx, order_ids)
    }

    pub fn create_contract_ata(
        ctx: Context<CreateContractAtaAccounts>,
        payment_config_id: Vec<u8>,
    ) -> Result<()> {
        instructions::create_contract_ata(ctx, payment_config_id)
    }
}
