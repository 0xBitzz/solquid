use anchor_lang::prelude::*;

#[account]
pub struct StakingPool {
    pub total_staked: u64,
    pub total_q_sol_minted: u64,
    pub bump: u8,
}

impl Space for StakingPool {
    const INIT_SPACE: usize = 8 + 8 + 1;
}
