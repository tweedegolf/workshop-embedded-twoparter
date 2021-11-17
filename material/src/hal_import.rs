#[cfg(not(any(feature = "nrf52840dk", feature = "nrf52dk")))]
compile_error!("Please select exactly one of the features [nrf52840dk, nrf52dk] using --features=nrf52840dk or --features=nrf52dk");
#[cfg(all(feature = "nrf52840dk", feature = "nrf52dk"))]
compile_error!("Please select exactly one of the features [nrf52840dk, nrf52dk] using --features=nrf52840dk or --features=nrf52dk");

#[cfg(feature = "nrf52840dk")]
pub use nrf52840_hal as hal;

#[cfg(feature = "nrf52dk")]
pub use nrf52832_hal as hal;
