<div class="read">

# Cortex-m PAC & cortex-m-rt

Most of embedded Rust is built around the Arm Cortex M microcontollers. These chips have gotten very popular the last decade, but now there's increasing interest in and support for Risc-V.

Assuming we're using a Cortex M mcu, two of the crates you'll pretty much always see are the [`cortex-m`](https://crates.io/crates/cortex-m) and [`cortex-m-rt`](https://crates.io/crates/cortex-m-rt) crates.

#### Cortex-m
The first one is a so called PAC, `Peripheral Access Crate`.

A PAC takes on the some of the same role as CMSIS does in C projects.
It defines all registers of the Cortex M core and has some convenience functions for managing things like the NVIC and ITM.

#### Cortex-m-rt
This is a runtime crate because Rust, just like C, has a runtime.

This crates takes care of:
- The memory layout of the program. In particular, it populates the vector table so the device can boot correctly, and properly dispatch exceptions and interrupts.
- Initializing static variables before the program entry point.
- Enabling the FPU before the program entry point if the target is thumbv7em-none-eabihf.

It also gives some tools to set up the program entry and interrupt functions.

All C projects have to do this as well, so often there's a generated file that may have the name `startup` or something similar.

</div>
