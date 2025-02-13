use anchor_lang::{prelude::*, solana_program::system_program};

use crate:: {
    constants::{BPS, ORDER_TS_PDA}, error::DogeFoodContractError,  state::OrderTs
};

#[derive(Accounts)]
pub struct CloseAccounts<'info> {
    /// CHECK: not required
    #[account(mut)]
    executor_reward_wallet: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn close_order_ts(
    ctx: Context<CloseAccounts>,
    order_ids: Vec<Vec<u8>>,
) -> Result<()> {
    let remaining_accts = ctx.remaining_accounts;
    let mut remaining_accts_iter = remaining_accts.iter();
    let executor_acc = &mut ctx.accounts.executor_reward_wallet;

    require!(order_ids.len() * 2 == remaining_accts.len(), DogeFoodContractError::InputLengthMismatch);

    let clock = Clock::get()?;

    for order_id in order_ids {
        let (order_ts_pda, _) = Pubkey::find_program_address(&[ORDER_TS_PDA, &order_id], &crate::id());
        let order_ts = remaining_accts_iter.next().unwrap();
        let user = remaining_accts_iter.next().unwrap();

        require!(order_ts_pda == *order_ts.key, DogeFoodContractError::AccountIsNotPda);
        require!(*order_ts.owner == crate::id(), DogeFoodContractError::AccountIsClosed);
        let balance = **(order_ts.lamports.borrow());
        require!(balance > 0, DogeFoodContractError::AccountIsClosed);

        let order_ts_data = OrderTs::try_from_slice(&order_ts.data.borrow())?;

        require!(order_ts_data.ts < clock.unix_timestamp as u64, DogeFoodContractError::AccountNotExpired);
        require!(order_ts_data.user == *user.key, DogeFoodContractError::UserNotBelongToClosingAccount);

        let user_portion = 90_00 * balance / BPS as u64;

        **order_ts.lamports.borrow_mut() -= balance;
        **user.lamports.borrow_mut() += user_portion;
        **executor_acc.lamports.borrow_mut() += balance - user_portion;
        
        order_ts.assign(&system_program::ID);
    }

    Ok(())
}