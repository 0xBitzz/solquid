use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5Yd1B8yd8achvY54WLZ49HMSyXYiUicjoVQZwGkeD9C3");

#[program]
pub mod solquid {
    use super::*;

    pub fn initialize(ctx: Context<InitializeConfig>, fee: u64) -> Result<()> {
        ctx.accounts.init_config(fee, &ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        ctx.accounts.stake(amount, &ctx.bumps)
    }
}
