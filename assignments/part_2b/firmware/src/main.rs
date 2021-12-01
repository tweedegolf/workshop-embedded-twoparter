#![no_std]
#![no_main]

use firmware::hal;

use firmware::uarte::{Baudrate, Parity, Pins as UartePins, TimeoutUarte};

#[allow(unused_imports)]
use hal::prelude::*;

use embedded_hal::timer::CountDown;
use format::{DeviceToServer, ServerToDevice};
use hal::{
    gpio::{p0, Level},
    pac::{TIMER0, UARTE0},
    ppi::{self, Ppi0},
    Timer,
};
use postcard::CobsAccumulator;

#[rtic::app(
    device=firmware::hal::pac,
    peripherals=true,
    monotonic=rtic::cyccnt::CYCCNT
)]
const APP: () = {
    // The resources that are to be shared between tasks
    struct Resources {
        accumulator: CobsAccumulator<32>,
        uarte0: TimeoutUarte<UARTE0, TIMER0, Ppi0>,
    }

    // Initialize peripherals, before interrupts are unmasked
    // Initializes and returns all resources that need to be dynamically instantiated
    #[init(spawn = [read_uarte0])]
    fn init(ctx: init::Context) -> init::LateResources {
        // Initialize UARTE0
        // Initialize port0
        let port0 = p0::Parts::new(ctx.device.P0);
        // Initialize PPI
        let ppi = ppi::Parts::new(ctx.device.PPI);

        // UART Receiving pin, initialize as input
        let rxd = port0.p0_08.into_floating_input().degrade();

        // UART Transmitting pin, initialize as output
        let txd = port0.p0_06.into_push_pull_output(Level::Low).degrade(); // Erase the type, creating a generic pin

        // Create Pins struct to pass to Uarte
        let uarte_pins = UartePins {
            rxd,
            txd,
            // We don't use cts/rts
            cts: None, // Clear to send pin
            rts: None, // Request to send pin
        };

        // A timer that is used to time-out UARTE0 read transactions,
        // so the device can react to commands even if the
        // UARTE0 RX FIFO is not yet full
        let mut timer0 = Timer::periodic(ctx.device.TIMER0);
        timer0.start(200_000u32); // 200 ms period

        // Initialize UARTE0 peripheral with standard configuration
        let uarte0 = TimeoutUarte::init(
            ctx.device.UARTE0, // Take peripheral handle by value
            uarte_pins,        // Take pins by value
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
            timer0,   // Take TIMER0 by value
            ppi.ppi0, // Take PPI channel 0 by value
        );

        // An accumulator for postcard-COBS messages
        let accumulator = CobsAccumulator::new();

        init::LateResources {
            uarte0,
            accumulator,
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

    // Do something with a message that just came in
    #[task(capacity = 5, priority = 10, spawn = [send_message])]
    fn handle_message(ctx: handle_message::Context, msg: ServerToDevice) {
        defmt::println!("Received message: {:?}. What do I need to do now?", msg);
        let ServerToDevice { say_hello, set_led_status, .. } = msg;

        if say_hello {
            ctx.spawn
            .send_message(DeviceToServer {
                said_hello: true,
                ..DeviceToServer::default()
            })
            .ok();
        }

        if let Some((led_id, enabled)) = set_led_status {
            // TODO react to an incoming message, possibly by spawning a newly defined task
            // Don't forget to declare it!
        }
    }

    // Send a message over UARTE0
    // This task waits until the last TX transaction is completed
    #[task(capacity = 10, resources = [uarte0], priority  = 1)]
    fn send_message(mut ctx: send_message::Context, msg: DeviceToServer) {
        defmt::info!("Sending message: {:?}", &msg);
        let mut buf = [0; 32];
        // Serialize message
        if let Ok(bytes) = postcard::to_slice_cobs(&msg, &mut buf) {
            // Repeatedly try to write the message. We need to lock uarte0 here,
            // as this task might be pre-empted by another task that uses uarte0.
            // uarte0.try_start_tx returns an Err variant if there is already a
            // TX transaction going on.
            while let Err(_) = ctx
                .resources
                .uarte0
                .lock(|uarte0| uarte0.try_start_tx(&bytes))
            {
                defmt::trace!("Waiting for currently running tx task to finish");
                // Go to sleep to avoid busy waiting
                cortex_m::asm::wfi();
            }
        } else {
            defmt::error!(
                "Could not serialize message {}. Please increase buffer size.",
                msg
            )
        }
        defmt::debug!("Done sending message");
    }

    // React to an interrupt from UARTE0
    #[task(
        binds = UARTE0_UART0,
        priority = 100,
        resources = [uarte0],
        spawn = [read_uarte0],
    )]
    fn on_uarte0(mut ctx: on_uarte0::Context) {
        use firmware::uarte::UarteEvent::*;
        defmt::trace!("Running task on_uarte0");

        ctx.resources
            .uarte0
            // We need to lock here, as this task might be pre-empted by
            // higher-priority tasks that use uarte0.
            .lock(|uarte0| match uarte0.get_clear_event() {
                Some(EndRx) => {
                    // Read transaction ended, spawn read task
                    ctx.spawn.read_uarte0().ok();
                }
                Some(EndTx) => {
                    // This event causes the running
                    // send_message task to try sending once more.
                    // No need to handle it here.
                }
                _ => (),
            });
    }

    // Software task that reads the UARTE0 DMA buffer,
    // deserializes the data, and spawns the `handle_message`
    // task.
    #[task(
        priority = 101,
        resources = [uarte0, accumulator],
        spawn = [handle_message],
    )]
    fn read_uarte0(ctx: read_uarte0::Context) {
        use postcard::FeedResult::*;

        // No need to lock here, as from all tasks that use
        // uarte0 or the accumulator, this task has the highest priority
        // and will therefore not be pre-empted.
        let chunk = ctx.resources.uarte0.get_rx_chunk();
        match ctx.resources.accumulator.feed(chunk) {
            Consumed => {}
            OverFull(_) => defmt::warn!("Accumulator full, dropping contents"),
            DeserError(_) => defmt::error!("Deserialize error, throwing away message"),
            Success { data, .. } => ctx
                .spawn
                .handle_message(data)
                .expect("Could not start handle_message task, please increase its capacity."),
        }
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these interrupts will be used to dispatch the
    // software tasks. For every software task, one interrupt must be sacraficed.
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
