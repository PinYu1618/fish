#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate lazy_static;
extern crate alloc;

mod bitmap;
mod block_cache;
mod block_dev;
mod dirty;
mod fish;
mod layout;
mod vfs;

pub const BLOCK_SIZE: usize = 512;

pub use bitmap::*;
pub use block_dev::BlockDevice;
pub use block_cache::*;
pub use dirty::*;
pub use fish::*;
pub use layout::*;
pub use vfs::*;