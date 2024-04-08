use anchor_lang::prelude::*;
pub mod space;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("FjT98dJySPLHjAMGfAfa1dLioFuHsdPg6ScrJ3NfAEZQ");

#[program]
pub mod aakoo_solana {
    use super::*;

    pub fn setup_auth(ctx: Context<Setup>, seed: u64, secret: String, mobile: String) -> Result<()> {
        ctx.accounts.setup(seed, secret, mobile, &ctx.bumps)
    }
}