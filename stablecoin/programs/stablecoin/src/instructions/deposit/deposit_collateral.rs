use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, Token2022, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{check_health_factor, deposit_sol, mint_token, Collateral, Config, SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT, SEED_SOL_ACCOUNT};

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account
    )]
    pub config_account: Box<Account<'info, Config>>,

    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed, // dont use for better security better create it on client side
        payer = depositor,
        space = 8 + Collateral::INIT_SPACE,
        seeds = [SEED_COLLATERAL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(
        mut,
        seeds = [SEED_SOL_ACCOUNT, depositor.key().as_ref()],
        bump,
    )]
    pub sol_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = depositor,
        // mut,
        associated_token::mint = mint_account,
        associated_token::authority = depositor,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub price_update: Account<'info, PriceUpdateV2>,
}

impl<'info> DepositCollateralAndMintTokens<'info> {
    pub fn deposit_collateral_and_mint_tokens(
        &mut self,
        amount_collateral: u64,
        amount_to_mint: u64,
        bump: DepositCollateralAndMintTokensBumps
    ) -> Result<()> {
        // self.collateral_account.lamports_balance = self.sol_account.lamports() + amount_collateral; // update to checked_add
        // self.collateral_account.amount_minted += amount_to_mint; // update to checked_add

        if !self.collateral_account.is_initialized {
            self.collateral_account.set_inner(Collateral {
                depositor: self.depositor.key(),
                sol_account: self.sol_account.key(),
                token_account: self.token_account.key(),
                lamports_balance: self.sol_account.lamports() + amount_collateral,
                amount_minted: self.collateral_account.amount_minted + amount_to_mint,
                is_initialized: true,
                bump: bump.collateral_account,
                bump_sol_account: bump.sol_account,
            });
        } else {
            self.collateral_account.lamports_balance = self.sol_account.lamports() + amount_collateral; // update to checked_add
            self.collateral_account.amount_minted += amount_to_mint; // update to checked_add
            
        }

        check_health_factor(
            &self.collateral_account, 
            &self.config_account, 
            &self.price_update
        )?;


        deposit_sol(
            &self.depositor, 
            &self.sol_account, 
            &self.system_program, 
            amount_collateral
        )?;

        mint_token(
            &self.mint_account,
            &self.token_account,
            &self.token_program,
            amount_to_mint,
            self.config_account.bump_mint_account
        )?;

        Ok(())
    }
}
