use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Collateral {
    pub depositor: Pubkey,
    pub sol_account: Pubkey, // pda account to hold reserve token i.e SOL
    pub token_account: Pubkey, // ata to store stable coin
    pub lamports_balance: u64, // to calculate health factor and liquidate on the basis of this
    pub amount_minted: u64,  // amount of stable coin minted
    pub is_initialized: bool,
    pub bump: u8,
    pub bump_sol_account: u8,
}