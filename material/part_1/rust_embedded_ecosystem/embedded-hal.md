<div class="read">

# Embedded-hal

[Embedded-hal](https://crates.io/crates/embedded-hal) is a crate that glues the entire Rust embedded ecosystem together.
It contains abstractions for many common operations that you do in embedded projects.

Let's take a look at one of the abstractions for SPI: ([docs](https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/spi/trait.Transfer.html#tymethod.transfer))
```rust
pub trait Transfer<W> {
    type Error;
    fn transfer<'w>(
        &mut self, 
        words: &'w mut [W]
    ) -> Result<&'w [W], Self::Error>;
}
```

There are a couple of things going on here.

- We're looking at a trait. This is similar to an `interface` or `abstract class` (without fields) in other languages
- A trait can be implemented by a concrete type
- The implementor of the trait must define its error type
- The implementor must implement the transfer function
- The transfer function receives a slice (a language feature for an array pointer with accompanying size integer) that it needs to send over the SPI bus
- The transfer function returns a slice that contains the data that was read from the bus if everything went ok or the error type if something went wrong

If we look at the [documentation of the SPI implementation of the nRF HAL](https://docs.rs/nrf52840-hal/0.14.0/nrf52840_hal/struct.Spim.html#trait-implementations), then we can see that it implements this trait.

So now we can use this SPI implementation in any place that uses the embedded-hal SPI abstraction.
This is used everywhere even to the point that if you see a device driver or a HAL that doesn't use embedded-hal,
that you would be surprise to see that.

Pretty much every major HAL implements the traits and pretty much every device driver accepts these traits.

</div>
