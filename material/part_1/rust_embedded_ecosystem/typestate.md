<div class="read">

# Typestate

Rust has a very rich typesystem which can be used build in a lot of guarantees into API's.

One of the things that can be done is so called typestate programming.
The principle is that a part of the state of an object is encoded into the type of the object.

In HALs it's often used in gpio pins. Let's give an example:

```rust
use nrf52840_hal::gpio::{Pin, p0::P0_04, Input, PullDown, Output, PushPull};

/// Take an nrf pin.
/// It must be:
/// - Port 0 pin 4 (Compile time known)
/// - Configured as input
/// - Pulldown enabled
fn do_something_1(pin: P0_04<Input<PullDown>>) {}

/// Take an nrf pin.
/// It must be:
/// - Any port and pin (Runtime known)
/// - Configured as output
/// - Configured as push-pull
fn do_something_2(pin: Pin<Output<PushPull>>) {}
```

With this you can build modules for your applications that demand very specific configuration.
This is especially nice with chips that have a lot of subtle configuration options like the STM32 series.

This concept can also be taken to device drivers. As an example, take a look at the [dw1000 docs](https://docs.rs/dw1000/0.5.0/dw1000/hl/struct.DW1000.html).

When you construct a new instance, it will be in the `Uninitialized` state. You can then call the initialize function which will consume the instance and give you back a new one in the `Ready` state.
The reason to do this is for correctness. The initialize function is only implemented for instances that are uninitialized and if you try to initialize in any other state, then you'll get a compile error.

</div>
