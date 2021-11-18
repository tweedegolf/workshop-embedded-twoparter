<div class="read">

# Runtimes

It is very usual when starting an embedded project to choose what your runtime will be. With Rust that is not different.

However, with C & C++, people usually choose to use an RTOS (Real Time Operating System) not because they need an RTOS per se but because the closed and open source ecosystem for that chip has standardized around it.

But because the Rust ecosystem is built around `embedded-hal` and all runtimes make sure to use it, the ecosystem is not a big consideration. Even the build system is standardized for all with Rust's `Cargo`. This means that we can choose our runtime purely on our technical needs and the quality of the solution it provides.

In practice this means that few people are actually using an RTOS. If you think about it, does your battery powered sensor reading IoT device really need real time scheduled threads? Probably not.

So what is out there for Rust?

## Bare metal with interrupts

This is the most straightforward way to program a device. Normally this is pretty difficult to get right.
Sharing data between the normal operation and the interrupts is not trivial. At best you're very strict with critical sections and have documented every global variable with where it can be used. Probably you've got a data race somewhere. At worst there's Undefined Behavior in your application ([video](https://www.youtube.com/watch?v=-W5zEtqJJIo)).

Rust makes this easier because it's very fussy about how you use your data. How and why is for a different time, but Rust forces us to use mutexes or other solutions if we want to share data between different contexts.

```rust
// Same as a non-const C global. Requires unsafe to access
static mut MY_VAR1: i32 = 42;
// Same as a const C global. No unsafe, but cannot be changed
static MY_VAR2: i32 = 42;
// Can be changed because it is safe to do so
static MY_VAR3: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(42);
// A mutex for the cortex m that can only be accessed in a critical section
static MY_VAR4: cortex_m::interrupt::Mutex<RefCell<i32>> = cortex_m::interrupt::Mutex::new(RefCell::new(42));

fn main() {
    unsafe { MY_VAR1 += 1 };

    // MY_VAR2 += 1; // (Compile error)

    MY_VAR3.fetch_add(1, core::sync::atomic::Ordering::AcqRel);

    cortex_m::interrupt::free(|cs| {
        *MY_VAR4.borrow(cs).borrow_mut() += 1;
    });
}
```

There are also tools to make this more ergonomic and performant in some cases. One example is [CMIM](https://crates.io/crates/cmim/0.2.1): see the [example](https://github.com/jamesmunns/cmim/blob/master/app-examples/timer-uart/src/main.rs).

## RTIC (Real-Time Interrupt-driven Concurrency)

Even though Rust protects us from doing anything wrong with sharing data between contexts, it's still very ceremonial and bothersome to do. To help us, there's framework called RTIC that does all the interrupt and data management for us with barely any overhead and maximum performance.

They've written a book as well: [link](https://rtic.rs/dev/book/en/).

There's also a talk done by the creator on youtube: [link](https://youtu.be/saNdh0m_qHc)

The principle is that if you know which data will be shared where that you can be very efficient with your mutexes and critical sections. Because of that, this framework is a real recommendation for any project.

## RTOS

There are real time operating systems as well. Of course you could use a binding to e.g. freertos or use one of the ones written in Rust like [Tock](https://github.com/tock/tock).

## Async

Since about the start of 2021, it's possible to use Rust's async machinery on embedded.

Instead of doing thread context switching or writing big statemachines, we can make the compiler write our statemachines.

Let's take a look at what this looks like. We'll use [embassy](https://github.com/embassy-rs/embassy) which is the most mature embedded async runtime out there.

```rust
#[embassy::task(pool_size = 4)]
async fn button_task(n: usize, mut pin: PortInput<'static, AnyPin>) {
    loop {
        pin.wait_for_low().await;
        info!("Button {:?} pressed!", n);
        pin.wait_for_high().await;
        info!("Button {:?} released!", n);
    }
}

#[embassy::main]
async fn main(spawner: Spawner, p: Peripherals) {
    info!("Starting!");

    let btn1 = PortInput::new(Input::new(p.P0_11.degrade(), Pull::Up));
    let btn2 = PortInput::new(Input::new(p.P0_12.degrade(), Pull::Up));
    let btn3 = PortInput::new(Input::new(p.P0_24.degrade(), Pull::Up));
    let btn4 = PortInput::new(Input::new(p.P0_25.degrade(), Pull::Up));

    unwrap!(spawner.spawn(button_task(1, btn1)));
    unwrap!(spawner.spawn(button_task(2, btn2)));
    unwrap!(spawner.spawn(button_task(3, btn3)));
    unwrap!(spawner.spawn(button_task(4, btn4)));
}
```

Here we have four tasks. They wait for their pin to go low, then print a message, then wait for the pin to go high and print a message again in a loop.
Every task has its own state (no heap required) and you never have to deal with interrupts.

We've done a couple of [blogposts](https://tweedegolf.nl/blog/63/async-on-embedded-present-and-future) on this as well that are worth checking out.

While perfectly usable today, Rust still lacks some features which makes it so that `embedded-hal` can't support async yet. But work is being done to make this possible and once it is, `embedded-hal` will be updated to support it.

</div>
