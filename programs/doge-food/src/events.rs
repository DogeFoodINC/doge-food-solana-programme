use anchor_lang::prelude::*;

#[event]
pub struct OrderPaymentProcessed {
    pub order_id: Vec<u8>,
    pub payment_config_id: Vec<u8>,
    pub payer: Pubkey,
    pub amount: u64,
    pub charity_amount: u64,
}