<div class="read">

# Abstraction in Rust

Just like C++, Rust allows us to use higher level constructs to solve our problems.
But where C++ was born as a classic OOP language, Rust [has been influenced by](https://doc.rust-lang.org/reference/influences.html) a lot of modern functional languages as well.
We can put Rust's strong typesystem and generics to good use and integrate `embedded-hal` into our driver.

## Traits and generics

Embedded-hal has defined a standard set of traits for us to use. We'll use them as well so we can be portable.

We need two traits:

- [`spi::Transfer`](https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/spi/trait.Transfer.html):
  We can give an array which will be put onto the bus. The response is written back into the array we gave and is returned.
- [`OutputPin`](https://docs.rs/embedded-hal/0.2.6/embedded_hal/digital/v2/trait.OutputPin.html):
  A pin that can be set high and low.

To make this all work together nicely we'll need to use some objects. We'll turn this driver into an object called `Device`
and implement the same example function as a method for the `Device`.

```rust
use embedded_hal::blocking::spi;
use embedded_hal::digital::v2::OutputPin;

pub struct Device<SPI, CS>
where
    SPI: spi::Transfer<u8>,
    CS: OutputPin,
{
    bus: SPI,
    chipselect: CS,
}

impl<SPI, CS> Device<SPI, CS>
where
    SPI: spi::Transfer<u8>,
    CS: OutputPin,
{
    pub fn new(bus: SPI, chipselect: CS) -> Self {
        Self { bus, chipselect }
    }

    pub fn example(&mut self) -> u8 {
        self.chipselect.set_low().ok();
        self.bus.transfer(&mut [0xDE]).ok();
        let result = self.bus.transfer(&mut [0xAD]).ok().unwrap()[0];
        self.chipselect.set_high().ok();

        result
    }
}
```

To test the performance of this code, we'll need to create mock implementations for the traits that do the same thing as
the tests for the C implementations.

And because generic functions are not generated unless they are used, we need to create a function that uses everything
so we can look at the assembly.

```rust
struct Spi;
impl spi::Transfer<u8> for Spi {
    type Error = core::convert::Infallible;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        unsafe {
            core::ptr::write_volatile(0x2000001 as *mut u8, words[0]);
            words[0] = core::ptr::read_volatile(0x2000001 as *mut u8);
        }
        Ok(words)
    }
}

struct Pin;
impl OutputPin for Pin {
    type Error = core::convert::Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { core::ptr::write_volatile(0x2000000 as *mut u8, 0) };
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { core::ptr::write_volatile(0x2000000 as *mut u8, 1) };
        Ok(())
    }
}

pub fn make_assembly_show_up() {
    let mut device = Device::new(Spi, Pin);
    device.example();
}
```

Now we can look at the assembly using the [cargo asm]() tool:

```bash
$ cargo asm workshop_code::make_assembly_show_up
workshop_code::make_assembly_show_up:
 mov     byte, ptr, [33554432], 0
 mov     byte, ptr, [33554433], -34
 mov     al, byte, ptr, [33554433]
 mov     byte, ptr, [33554433], -83
 mov     al, byte, ptr, [33554433]
 mov     byte, ptr, [33554432], 1
 ret
```

There are only `mov` instructions just as in the best C solutions, so the performance is excellent. Everything gets nicely
inlined and all unused abstraction is optimized away and the nice thing is that this device we created is fully
decoupled from the hardware layer.

</div>
