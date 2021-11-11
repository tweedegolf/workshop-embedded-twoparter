# Device PACs

Most devices have a PAC for their peripherals. These PACs are not handmade, but generated from SVD files.
This is an Arm XML file standard that describes the peripherals and the bits of every device.
To go to market with a cortex-m design, a manufacturer is required to make the SVD files public.

Many manufacturers give also CMSIS headers for their devices' peripherals.
Nordic seemingly stopped with this, so we'll take a MicroChip SAMD21 as example.

In CMSIS the peripherals and their registers are defined with `#define` for the pointers and unions to contain the fields.
To read and write to/from a peripheral it gives you a bitfield struct and also gives you bit position and mask definitions.

A PAC however uses a read and write struct that have functions for setting and getting the bits. 

What does it look like to, say, read and modify the timeout period of the WDT?

In C: ([source](https://github.com/avrxml/asf/blob/master/sam0/utils/cmsis/samd21/include/samd21e17l.h))
```c
#include "samd21e17l.h"

// Raw
bool is_8_cycles = ((WDT->CONFIG.reg & WDT_CONFIG_PER_Msk) << WDT_CONFIG_PER_Pos) == WDT_CONFIG_PER_8_val;
WDT->CONFIG.reg = (WDT->CONFIG.reg & ~WDT_CONFIG_PER_Msk) | WDT_CONFIG_PER_16;

// Bitfield
bool is_8_cycles = WDT->CONFIG.bit.PER == WDT_CONFIG_PER_8_val;
WDT->CONFIG.bit.PER = WDT_CONFIG_PER_16;
```

In Rust: ([docs](https://docs.rs/atsamd21e/0.11.0/atsamd21e/index.html))
```rust
// Take ownership of the peripherals
let dp = atsamd21e::Peripherals::take().unwrap();

let is_8_cycles = dp.WDT.CONFIG.read().per().is_8();
dp.WDT.CONFIG.modify(|_, w| w.per()._8());
```

All of these examples should produce the same instructions.
