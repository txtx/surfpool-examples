#![no_std]

extern crate alloc;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pub mod error;
pub mod instruction;
pub mod state;
pub mod dex;

pinocchio_pubkey::declare_id!("D7Nv2Yt9i7r1xSGgTZo9zGHgZ8wwiAX13nFodBXdpox4");
