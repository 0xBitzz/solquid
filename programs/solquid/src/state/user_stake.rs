use anchor_lang::prelude::*;

#[account]
pub struct UserStake {
    pub address: Pubkey,
    pub amount_staked: u64,
    pub q_sol_received: u64,
    pub stake_time_stamp: i64,
    pub bump: u8,
}

impl Space for UserStake {
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 8 + 1;
}
