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
use lis3dh::Configuration;
use lis3dh::DataRate;
use lis3dh::Lis3dh;
use lis3dh::Lis3dhI2C;
use lis3dh::SlaveAddr;
use lis3dh::accelerometer::Accelerometer;

type Accelerometer = Lis3dh<Lis3dhI2C<Twim<TWIM0>>>;

#[rtic::app(
    device=firmware::hal::pac,
    peripherals=true,
    monotonic=rtic::cyccnt::CYCCNT
)]
const APP: () = {
    struct Resources {
        gpiote: Gpiote,
        timer0: Timer<TIMER0, Periodic>,
        led1: Pin<Output<PushPull>>,
        // TODO accelerometer: Accelerometer,
    }

    // Initialize peripherals, before interrupts are unmasked
    // Returns all resources that need to be dynamically instantiated
    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        // Split port P0 up into a set of different pins
        // to allow them to be owned by different modules
        let port0 = Parts::new(ctx.device.P0);

        // Initialize pin p0.13 as push-pull output with high output level
        // We degrade the pin, removing the pin number from type information so
        // it can be used in generic modules
        let led1 = port0.p0_13.into_push_pull_output(Level::High).degrade();

        // Initialize pin p0.11 as a pull up input pin
        let btn1 = port0.p0_11.into_pullup_input().degrade();

        // Configure GPIOTE
        let gpiote = Gpiote::new(ctx.device.GPIOTE);
        // Connect btn1 (p0.11) to GPIOTE channel 0, and listen for high-to-low transitions
        gpiote
            .channel0()
            .input_pin(&btn1)
            .hi_to_lo()
            .enable_interrupt();

        // Initialize TIMER0 as periodic timer with a frequency of 1 second (1M ticks)
        let mut timer0 = Timer::periodic(ctx.device.TIMER0);
        timer0.enable_interrupt();
        timer0.start(1_000_000u32); // 1000 ticks = 1 ms

        // TODO:
        // ===== 1. Initialize i2c SCL,SDA as floating inputs and degrade. ======
        // Initialize pin connected to the acceleromers INT pin as pull down input.
        // let scl = port0...;
        // let sda = port0...;
        // let int = port0...;
        // let pins = TwimPins { scl, sda };

        // ===== 2. Initialize TWIM0 peripheral using HAL with 400kHz clock frequency =====
        // let freq = TwimFrequency...;
        // let twim0 = hal::twim::Twim::new...;

        // ===== 3. Connect INT pin to GPIOTE channel 1, listening for high-to-low transition =====
        // TODO

        // ===== 4. Initialize lis3dh driver with, add them to the resources =====
        // Just uncomment the line below for this step
        // let accelerometer = acc::config_acc(twim0).unwrap();
        // ===== 5. Update the existing `on_gpiote` task to spawn a new task on channel 1 event,
        //    which fetches accelerometer data and prints it =====
        //    *To read a sample:*
        //      let sample = accelerometer.accel_norm().unwrap();
        //      defmt::info!("Sample: x: {}, y: {}, z: {}", sample.x, sample.y, sample.z);


        init::LateResources {
            led1,
            gpiote,
            timer0,

        }
    }

    // Defines what happens when there's nothing left to do
    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            // Go to sleep, waiting for an interrupt
            cortex_m::asm::wfi();
        }
    }

    #[task(
        capacity = 5,
        priority = 1, // Very low priority
        resources = [led1]
    )]
    fn set_led1_state(ctx: set_led1_state::Context, enabled: bool) {
        defmt::debug!("Running set_led1_state: {}", enabled);
        // Leds on the DK are active low
        if enabled {
            ctx.resources.led1.set_low().unwrap(); // Can't actually fail
        } else {
            ctx.resources.led1.set_high().unwrap();
        }
    }

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

    #[task(
        binds = TIMER0,
        priority = 99, // Very high priority
        resources = [timer0],
        spawn = [set_led1_state]
    )]
    fn on_timer0(ctx: on_timer0::Context) {
        let timer0 = ctx.resources.timer0;
        if timer0.event_compare_cc0().read().bits() != 0x00u32 {
            defmt::debug!("Timer 0 fired");
            timer0.event_compare_cc0().write(|w| unsafe { w.bits(0) });
            // Try to spawn set_led1_state. If it's queue is full, we do nothing.
            let _ = ctx.spawn.set_led1_state(false);
        }
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these interrupts will be used to dispatch the
    // software tasks.
    // See https://rtic.rs/0.5/book/en/by-example/tasks.html;
    extern "C" {
        // Software interrupt 0 / Event generator unit 0
        fn SWI0_EGU0();
        // Software interrupt 1 / Event generator unit 1
        fn SWI1_EGU1();
        // Software interrupt 2 / Event generator unit 2
        fn SWI2_EGU2();
    }
};
