<div class="read">

# Low level & high level drivers

So far we've only covered one half of a low level driver, namely the hardware interaction.
But there is more to a good driver.

## Low level

A low level driver does two things:

1. Uses the hardware abstractions and implements functions that allow higher up code to talk to the hardware.
   For many devices this means functions to read and write to registers.
2. Defines what the hardware is and can do. This can be register definitions, buffer definitions or command definitions.

A low level driver is the minimum that is required to call something a driver.
But using a low level driver is a lot of work. None of the functions of the hardware are abstracted.
This means that all people using it have to have a relatively deep understanding of the hardware they are using.
All code written for it has to be very specific and will only work with that specific device or its family.

## High level

To solve the specificness of the low level, a high level driver can do a couple of things:

- Implement often used routines
- Assert that certain hardware requirements are met or make a design where the requirements are guaranteed
- Export a common abstract interface

The rust opensource community has defined a lot of abstract interface that drivers can opt into exporting.
Here are some examples:

- [`embedded-nal`](https://crates.io/crates/embedded-nal): A common set of traits for networks. (IP, TCP, UDP, DNS)
- [`accelerometer`](https://crates.io/crates/accelerometer): High level definition for an accelerometer
- [`usb-device`](https://crates.io/crates/usb-device/0.2.8): A usb stack with a bus trait that can be implemented by usb hardware drivers
- [`embedded-graphics`](https://crates.io/crates/embedded-graphics): A graphics stack for drawing to lcd and oled screens

</div>
