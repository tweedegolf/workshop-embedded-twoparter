//! This module imports the HAL for either the nRF52840 or the nRF52832, depending on which feature is enabled.
//! From outside, we can just refer to the hal as `firmware::hal`.

#[cfg(not(any(feature = "nrf52840dk", feature = "nrf52dk")))]
compile_error!("Please select exactly one of the features [nrf52840dk, nrf52dk] using --features=nrf52840dk or --features=nrf52dk");
#[cfg(all(feature = "nrf52840dk", feature = "nrf52dk"))]
compile_error!("Please select exactly one of the features [nrf52840dk, nrf52dk] using --features=nrf52840dk or --features=nrf52dk");

#[cfg(feature = "nrf52840dk")]
pub use nrf52840_hal as hal;

#[cfg(feature = "nrf52dk")]
pub use nrf52832_hal as hal;
