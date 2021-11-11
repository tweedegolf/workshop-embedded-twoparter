# Typestate

Rust has a very rich typesystem which can be used build in a lot of guarantees into API's.

One of the things that can be done is so called typestate programming.
The principle is that a part of the state of an object is encoded into the type of the object.

In HALs it's often used in gpio pins. Let's give an example:

```rust
use nrf52840_hal::gpio::p0::P0_04;

fn do_something(pin: P) // TODO
```
