use anchor_lang::prelude::*;

#[error_code]
pub enum DogeFoodContractError {
    
    #[msg("Bps value either exceed 100.00% or to small then 0.00%")]
    InvalidBps,

    #[msg("Global config already initialized")]
    AlreadyInitialized,

    #[msg("Global config must be initialized first")]
    NotInitialized,

    #[msg("This token configuration is not enabled")]
    PaymentNotEnabled,

    #[msg("Order experied")]
    OrderExpired,

    #[msg("Amount of tokens is different")]
    InvalidAmount,
    
    #[msg("Variable account length should be 2 times bigger then lnegth of orders")]
    InputLengthMismatch,
    
    #[msg("Submitted order TS account must be PDA")]
    AccountIsNotPda,
    
    #[msg("Submitted order TS account must be alive PDA")]
    AccountIsClosed,
    
    #[msg("Order cannot be refunded because it is not yet expired")]
    AccountNotExpired,
    
    #[msg("Order TS does not match given user")]
    UserNotBelongToClosingAccount,

}