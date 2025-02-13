use anchor_lang::prelude::*;

pub const CONFIG_PDA_SEED: &[u8] = b"DOGE_COIN.CONFIG";
pub const PAYMENT_CONFIG_PDA_SEED: &[u8] = b"DOGE_COIN.PAYMENT_CONFIG";
pub const ORDER_TS_PDA: &[u8] = b"DOGE_COIN.ORDER_TS";
pub const CONTRACT_ATA: &[u8] = b"DOGE_COIN.CONTRACT_ATA.V2";

pub const BPS: u16 = 100_00;