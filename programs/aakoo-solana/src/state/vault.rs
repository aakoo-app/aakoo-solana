use anchor_lang::prelude::*;

use crate::space::{DISCRIMINATOR, PUBKEY, U64, U8, STRING};

#[account]
pub struct Vault {
    pub user: Pubkey,
    pub secret: String,
    pub mobile: String,
    pub bump: u8,
    pub seed: u64,
}

impl Space for Vault {
  const INIT_SPACE: usize = DISCRIMINATOR
  + PUBKEY
  + STRING
  + STRING
  + U8
  + U64;
}