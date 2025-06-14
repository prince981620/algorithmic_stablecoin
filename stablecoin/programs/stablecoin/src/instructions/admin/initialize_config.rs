use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{ Config, MINT_DECIMALS, SEED_CONFIG_ACCOUNT, SEED_MINT_ACCOUNT};

#[derive(Accounts)]
pub struct InitialzeConfig <'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program,
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,

    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

impl <'info> InitialzeConfig <'info> {
    pub fn initialize (&mut self) -> Result<()> {
        Ok(())
    }
}