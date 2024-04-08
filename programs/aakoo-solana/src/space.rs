use anchor_lang::prelude::*;

pub const DISCRIMINATOR: usize = std::mem::size_of::<u64>();
pub const PUBKEY: usize = std::mem::size_of::<Pubkey>();
pub const U8: usize = std::mem::size_of::<u8>();
pub const U64: usize = std::mem::size_of::<u64>();
pub const STRING: usize = std::mem::size_of::<String>();