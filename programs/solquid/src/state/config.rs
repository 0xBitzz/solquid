use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub admin: Pubkey,
    pub fee: u64,
    pub q_sol_bump: u8,
    pub bump: u8,
}

impl Space for Config {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1 + 1;
}
