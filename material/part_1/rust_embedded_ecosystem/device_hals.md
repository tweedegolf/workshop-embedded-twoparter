<div class="read">

# Device HALs

Programming using registers forms the basis of everything, but usually we want to work on more higher level components.
Most mcu manufacturers therefore have a Hardware Abstraction Layer and there are a lot of 3rd party HALs as well.

So far only Espressive has started investing in their own Rust HAL, so for the other chips we need to look at opensource projects.
For the exercises we're going to use the nRF52840 and luckily for us, there's a really good and complete HAl for us to use.

The [nrf52840-hal](https://crates.io/crates/nrf52840-hal) crate provides all the functionality we need from the chip.

For example, if we want to have a timer running for 1 million cycles after which it generates an interrupt, we can use the following code:
```rust
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use nrf52840_hal::pac::{Peripherals, Interrupt, interrupt, TIMER0};
use nrf52840_hal::Timer;
use embedded_hal::timer::CountDown;

#[entry]
fn main() -> ! {
    // Take the device's peripherals
    let dp = Peripherals::take().unwrap();

    // Create the timer and give it access to the peripheral
    let mut timer = Timer::periodic(dp.TIMER0);
    timer.enable_interrupt();
    timer.start(1000000u32); // Timer runs at 1 Mhz, so it will interrupt every second
    drop(timer);

    // Unmask the timer interrupt in the NVIC, this can be unsafe in some situations,
    // so we have to put it in an unsafe block
    unsafe { NVIC::unmask(Interrupt::TIMER0); }

    loop {}
}

#[interrupt]
fn TIMER0() {
    // Get a reference to the peripheral.
    // This is unsafe because only one instance may exist at a time or we'll trigger UB.
    // In this case it's fine because we dropped the timer in main.
    // Normally we wouldn't do this.
    // We'd have to use a mutex to share the timer peripheral between contexts.
    let timer = unsafe { &*TIMER0::ptr() };
    // Stop the interrupt
    timer.events_compare[0].write(|w| w);
}
```

If you're paying attention, you'll see that we have an extra dependency called `embedded_hal` and this is what the next chapter is about.

</div>
