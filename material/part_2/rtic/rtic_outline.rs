/* ANCHOR: all */
#![no_std]
#![no_main]

use embedded_hal::timer;
use examples as _;
use nrf52840_hal as hal;

use hal::{
    gpio::{p0::Parts, Input, Level, Output, Pin, PullUp, PushPull},
    gpiote::Gpiote,
    pac::DWT,
    pac::TIMER0,
    prelude::*,
    timer::Periodic,
    Timer,
};

#[rtic::app(
    device=nrf52840_hal::pac,
    peripherals=true,
    monotonic=rtic::cyccnt::CYCCNT
)]
const APP: () = {

    // Shared state
    struct Resources {
        // Define your shared resources here
        led_1_pin: ...
        timer0: ...
    }
    #[init]
    fn init(mut ctx: init::Context) -> init::LateResources {
        let peripherals = ctx.device;
        // Initialize your peripherals and state here
        // - snip -
        init::LateResources {
            led_1_pin,
            timer0
        }
    }

    // Defines what happens when there's nothing left to do
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            // Go to sleep, waiting for an interrupt
            cortex_m::asm::wfi();
        }
    }

    // Software task
    #[task(resources = [led_1_pin], priority = 1)]
    fn toggle_led_1(ctx: toggle_led_1::Context) {
        // - snip -
    }
   
    // Hardware task
    #[task(
        binds = TIMER0,
        priority = 4,
        resources = [timer0],
        spawn = [toggle_led_1],
    )]
    fn on_timer0(ctx: on_timer0::Context) {
        // - snip -
    }

    // 'Sacraficed' interrupts
    extern "C" {
        // Software interrupt 0 / Event generator unit 0
        fn SWI0_EGU0();
        // Software interrupt 1 / Event generator unit 1
        fn SWI1_EGU1();
        // Software interrupt 2 / Event generator unit 2
        fn SWI2_EGU2();
    }
};
