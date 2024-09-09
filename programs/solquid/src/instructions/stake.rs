use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::{config::Config, staking_pool::StakingPool, user_stake::UserStake};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"pool".as_ref()],
        bump = staking_pool.bump,
    )]
    pub staking_pool: Account<'info, StakingPool>,
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = user,
        seeds = [b"user".as_ref()],
        bump,
        space = UserStake::INIT_SPACE
    )]
    pub user_stake: Account<'info, UserStake>,
    #[account(
        mut,
        seeds = [b"qsol".as_ref()],
        bump = config.q_sol_bump
    )]
    pub q_sol_mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = q_sol_mint,
        associated_token::authority = user
    )]
    pub q_sol_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"vault".as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    // We set the get the amount a user staked, mint the equivalent amount of qSOL
    // and update the UserStake account.
    pub fn stake(&mut self, amount: u64, bumps: &StakeBumps) -> Result<()> {
        // Deposit x amount of native SOL.
        self.deposit_sol(amount)?;
        // Mint equivalent amount of qSOL.
        self.mint_q_sol(amount)?;
        // Update UserStake.
        self.user_stake.set_inner(UserStake {
            address: self.user.key(),
            amount_staked: amount,
            q_sol_received: amount,
            stake_time_stamp: Clock::get()?.unix_timestamp,
            bump: bumps.user_stake,
        });
        // Update total_stake and q_total_supply in StakingPool.
        self.staking_pool.total_staked += amount;
        self.staking_pool.total_q_sol_minted += amount;

        Ok(())
    }

    fn deposit_sol(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(ctx, amount)?;

        Ok(())
    }

    fn mint_q_sol(&mut self, amount: u64) -> Result<()> {
        let accounts = MintTo {
            mint: self.q_sol_mint.to_account_info(),
            to: self.user.to_account_info(),
            authority: self.config.to_account_info(),
        };
        // seeds = [b"config".as_ref()],
        // bump,
        // create valid signer seeds
        let signer_seeds = [];
        let ctx = CpiContext::new_with_signer(
          self.token_program.to_account_info(),
          accounts,
          &signer_seeds
        );
        mint_to(ctx, amount)?;

        todo!("Validate the signer seeds for the CpiContext, Figure out how to add some metadata to this mint.")
    }
}
