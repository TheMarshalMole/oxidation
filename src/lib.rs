// disable std & high level libraries
#![no_std]
pub use core::panic::PanicInfo;

// This will define the scheleton of the starting driver.
#[path = "./iron/iron.rs"]
pub mod iron;

// macros
pub use iron::entries;