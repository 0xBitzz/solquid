use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata as Metaplex,
    },
    token::{Mint, Token},
};

use crate::state::{config::Config, staking_pool::StakingPool};
use crate::constants::{TOKEN_DECIMALS, TOKEN_NAME, TOKEN_SYMBOL, TOKEN_URI};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub metadata: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"config".as_ref()],
        bump,
        space = Config::INIT_SPACE
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        seeds = [b"qsol".as_ref()],
        bump,
        mint::decimals = TOKEN_DECIMALS,
        mint::authority = config,
    )]
    pub q_sol_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        seeds = [b"pool".as_ref()],
        bump,
        space = StakingPool::INIT_SPACE
    )]
    pub staking_pool: Account<'info, StakingPool>,
    pub token_metadata_program: Program<'info, Metaplex>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> InitializeConfig<'info> {
    pub fn init_config(&mut self, fee: u64, bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner(Config {
            admin: self.admin.key(),
            fee,
            q_sol_bump: bumps.q_sol_mint,
            bump: bumps.config,
        });

        self.staking_pool.set_inner(StakingPool {
            total_staked: 0,
            total_q_sol_minted: 0,
            bump: bumps.staking_pool,
        });

        Ok(())
    }

    pub fn set_q_sol_metadata(&mut self) -> Result<()> {
        let signer_seeds = &[];

        let q_sol_data = DataV2 {
            name: TOKEN_NAME.to_string(),
            symbol: TOKEN_SYMBOL.to_string(),
            uri: TOKEN_URI.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let metaplex_ctx = CpiContext::new_with_signer(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: self.metadata.to_account_info(),
                mint: self.q_sol_mint.to_account_info(),
                mint_authority: self.q_sol_mint.to_account_info(),
                payer: self.admin.to_account_info(),
                update_authority: self.q_sol_mint.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
            signer_seeds,
        );
        create_metadata_accounts_v3(metaplex_ctx, q_sol_data, false, true, None)?;

        Ok(())
    }
}
