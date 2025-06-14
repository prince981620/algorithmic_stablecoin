pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7YYzjV1PiTy7X2yCMQqUuAoFeyr2yFVAHCSEsZmHjdEp");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitialzeConfig>) -> Result<()> {
        ctx.accounts.initialize_config(ctx.bumps)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        ctx.accounts.update_config(min_health_factor)
    }


}
