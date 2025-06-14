use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DepositCollateralAndMintTokens <'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    
}