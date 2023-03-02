#![no_std]
#![no_main]
#![allow(unused_imports)]

use firmware::acc;
use firmware::hal;

use hal::pac::TWIM0;
use hal::prelude::*;
use hal::twim::{Frequency as TwimFrequency, Pins as TwimPins};
use hal::Twim;
use hal::{
    gpio::{Output, Pin, PushPull},
    gpiote::Gpiote,
};

use embedded_hal::timer::CountDown;
use hal::{
    gpio::{p0::Parts, Level},
    pac::TIMER0,
    timer::Periodic,
    Timer,
};
use lis3dh::accelerometer::Accelerometer;
use lis3dh::Configuration;
use lis3dh::DataRate;
use lis3dh::Lis3dh;
use lis3dh::Lis3dhI2C;
use lis3dh::SlaveAddr;

type Lis3dhInstance = Lis3dh<Lis3dhI2C<Twim<TWIM0>>>;

// ANCHOR: app_attr
#[rtic::app(
    device=firmware::hal::pac,
    peripherals=true,
    dispatchers = [SWI0_EGU0, SWI1_EGU1, SWI2_EGU2],
)]
mod app {
    // ANCHOR_END: app_attr

    //ANCHOR: resources
    #[local]
    struct LocalResources {
        gpiote: Gpiote,
        timer0: Timer<TIMER0, Periodic>,
        led1: Pin<Output<PushPull>>,
    }

    #[shared]
    struct SharedResources {
        led2: Pin<Output<PushPull>>,
    }
    //ANCHOR_END: resources

    // ANCHOR: init
    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<1000>; // 1000 Hz / 1 ms granularity

    #[init]
    fn init(ctx: init::Context) -> (SharedResources, LocalResources, innit::Monotonics) {
        let port0 = Parts::new(ctx.device.P0);

        // Enable systick counter for task scheduling
        let mono = Systick::new(ctx.core.SYST, 64_000_000);

        // Init pins
        let led1 = port0.p0_13.into_push_pull_output(Level::High).degrade();
        let led2 = port0.p0_14.into_push_pull_output(Level::High).degrade();
        let btn1 = port0.p0_11.into_pullup_input().degrade();

        // Configure GPIOTE
        let gpiote = Gpiote::new(ctx.device.GPIOTE);
        gpiote
            .channel0()
            .input_pin(&btn1)
            .hi_to_lo()
            .enable_interrupt();

        // Initialize TIMER0
        let mut timer0 = Timer::periodic(ctx.device.TIMER0);
        timer0.enable_interrupt();
        timer0.start(1_000_000u32); // 1000 ticks = 1 ms

        // Return the resources
        (
            SharedResources { led2 },
            LocalResources {
                led1,
                gpiote,
                timer0,
            },
            innit::Monotonics(mono),
        )
    }
    // ANCHOR_END: init

    // ANCHOR: idle
    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            // Go to sleep, waiting for an interrupt
            cortex_m::asm::wfi();
        }
    }
    // ANCHOR_END: idle

    // ANCHOR: sw_task
    #[task(
        capacity = 5,
        priority = 1, // Very low priority
        local = [led1]
    )]
    fn set_led1_state(ctx: set_led1_state::Context, enabled: bool) {
        if enabled {
            ctx.local.led1.set_low().unwrap();
        } else {
            ctx.local.led1.set_high().unwrap();
        }
    }
    // ANCHOR_END: sw_task

    // ANCHOR: hw_task
    #[task(
        binds = TIMER0,
        priority = 7, // Very high priority
        local = [timer0],
    )]
    fn on_timer0(ctx: on_timer0::Context) {
        let timer0 = ctx.resources.timer0;
        if timer0.event_compare_cc0().read().bits() != 0x00u32 {
            timer0.event_compare_cc0().write(|w| unsafe { w.bits(0) });
            // Try to spawn set_led1_state. If its queue is full, we do nothing.
            let _ = set_led1_state::spawn(false);
        }
    }
    // ANCHOR_END: hw_task

    // ANCHOR: lock_bad
    #[task(capacity = 5, priority = 1, shared = [led2])]
    fn low_prio_task(ctx: low_prio_task::Context) {
        let led2 = ctx.shared.led2;

        led2.set_high();
    }
    // ANCHOR_END: lock_bad

    // ANCHOR: lock_ok
    #[task(capacity = 5, priority = 1, shared = [led2])]
    fn low_prio_task(ctx: low_prio_task::Context) {
        // Locking mutates
        let mut led2 = ctx.shared.led2;

        led2.lock(|led2_lock| {
            led2_lock.set_low().unwrap();
        });
    }
    // ANCHOR_END: lock_ok
}
