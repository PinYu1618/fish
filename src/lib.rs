#![cfg_attr(not(test), no_std)]

pub mod block_cache;
pub mod buffer;
pub mod dev;
pub mod dirty;
pub mod vfs;

#[macro_use]
extern crate lazy_static;
extern crate alloc;
