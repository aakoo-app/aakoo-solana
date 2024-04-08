use anchor_lang::prelude::*;

use crate::state::Vault;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Setup<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
    init,
    payer = user,
    seeds = [b"datavault", user.key().as_ref(), seed.to_le_bytes().as_ref()],
    space = Vault::INIT_SPACE,
    bump
)]
  pub vault: Account<'info, Vault>,
  pub system_program: Program<'info, System>
}

impl <'info> Setup <'info> {
  pub fn setup(&mut self, seed: u64, secret: String, mobile: String, bumps: &SetupBumps) -> Result<()> {
    self.vault.set_inner(Vault { 
      user: self.user.to_account_info().key(), 
      bump: bumps.vault, 
      mobile,
      seed, 
      secret,
    });
    Ok(())
  }
}