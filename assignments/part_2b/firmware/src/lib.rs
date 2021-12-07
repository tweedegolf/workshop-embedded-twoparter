#![no_std]

use panic_probe as _;

pub mod uarte;
mod hal_import;
pub use hal_import::hal;
