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
    monotonic=rtic::cyccnt::CYCCNT
)]
const APP: () = {
    // ANCHOR_END: app_attr

    //ANCHOR: resources
    struct Resources {
        gpiote: Gpiote,
        timer0: Timer<TIMER0, Periodic>,
        led1: Pin<Output<PushPull>>,
    }
    //ANCHOR_END: resources

    // ANCHOR: init
    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        let peripherals = ctx.device;
        let port0 = Parts::new(peripherals.P0);

        // Init pins
        let led1 = port0.p0_13.into_push_pull_output(Level::High).degrade();
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
        init::LateResources {
            led1,
            gpiote,
            timer0,
        }
    }
    // ANCHOR_END: init

    // ANCHOR: schedule_init
    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        // Enable cycle counter
        ctx.core.DWT.enable_cycle_counter();

        // Init peripherals...

        let now = ctx.start;
        // Schedule toggle_led_2 task
        ctx.schedule.toggle_led_2(now, true).unwrap();

        init::LateResources {
            // The resources
        }
    }
    // ANCHOR_END: schedule_init

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
        resources = [led1]
    )]
    fn set_led1_state(ctx: set_led1_state::Context, enabled: bool) {
        if enabled {
            ctx.resources.led1.set_low().unwrap();
        } else {
            ctx.resources.led1.set_high().unwrap();
        }
    }
    // ANCHOR_END: sw_task

    // ANCHOR: schedule_task
    #[task(
        capacity = 5,
        priority = 2,
        resources = [led2],
        schedule = [toggle_led_2]
    )]
    fn toggle_led_2(ctx: toggle_led_2::Context, enabled: bool) {
        let led2 = ctx.resources.led2;
        if enabled {
            led2.set_high().unwrap(); // Disable
        } else {
            led2.set_low().unwrap(); // Enable
        }

        // Use ctx.start in HW task and init
        let task_scheduled_at = ctx.scheduled;
        ctx.schedule
            .toggle_led_2(task_scheduled_at + 10_000_000u32.cycles(), !enabled)
            .ok();
    }
    // ANCHOR_END: schedule_task

    #[task(
        binds = GPIOTE,
        priority = 99,
        resources = [gpiote],
        spawn = [set_led1_state]
    )]
    fn on_gpiote(ctx: on_gpiote::Context) {
        let gpiote = ctx.resources.gpiote;

        // Check if an event was triggered on channel 0
        if gpiote.channel0().is_event_triggered() {
            defmt::debug!("Button 1 pressed");
            // Clear events
            gpiote.channel0().reset_events();
            // Try to spawn set_led1_state. If it's queue is full, we do nothing.
            ctx.spawn.set_led1_state(true).ok();
        }
        // TODO check if LIS3DH caused the interrupt. If so, spawn read task
    }

    // ANCHOR: hw_task
    #[task(
        binds = TIMER0,
        priority = 99, // Very high priority
        resources = [timer0],
        spawn = [set_led1_state]
    )]
    fn on_timer0(ctx: on_timer0::Context) {
        let timer0 = ctx.resources.timer0;
        if timer0.event_compare_cc0().read().bits() != 0x00u32 {
            timer0.event_compare_cc0().write(|w| unsafe { w.bits(0) });
            // Try to spawn set_led1_state. If it's queue is full, we do nothing.
            let _ = ctx.spawn.set_led1_state(false);
        }
    }
    // ANCHOR_END: hw_task

    // ANCHOR: lock_bad
    #[task(capacity = 5, priority = 1, resources = [led2])]
    fn low_prio_task(ctx: low_prio_task::Context) {
        let led2 = ctx.resources.led2;

        led2.set_high();
    }
    // ANCHOR_END: lock_bad

    // ANCHOR: lock_ok
    #[task(capacity = 5, priority = 1, resources = [led2])]
    fn low_prio_task(ctx: low_prio_task::Context) {
        // Locking mutates
        let mut led2 = ctx.resources.led2;

        led2.lock(|led2_lock| {
            led2_lock.set_low().unwrap();
        });
    }
    // ANCHOR_END: lock_ok

    // ANCHOR: interrupts
    extern "C" {
        // Software interrupt 0 / Event generator unit 0
        fn SWI0_EGU0();
        // Software interrupt 1 / Event generator unit 1
        fn SWI1_EGU1();
        // Software interrupt 2 / Event generator unit 2
        fn SWI2_EGU2();
    }
    // ANCHOR_END: interrupts
};
