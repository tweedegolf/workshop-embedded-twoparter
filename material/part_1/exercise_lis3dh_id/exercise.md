<div class="read">

# Exercise: LIS3DH ID

We've covered a lot of topics already, so let's go back and try to make something ourselves.

To get started we'll setup the i2c on our development kit and read out the ID register of the LIS3DH accelerometer.
The starting point can be found in `assignments/part_1` of this repository.

Depending on your development kit, there are some steps to take.
If you have the nRF52840-DK, then you're good to go.

If you have the nRF52832-DK, then we have some things to change:
- Go to `Cargo.toml` and swap out `nrf52840-hal` to `nrf52832-hal`
- Go to `.cargo/config.toml` and swap out `nRF52840_xxAA` with `nRF52832_xxAA`
- Go to `src/main.rs` and swap out `nrf52840_hal` with `nrf52832_hal`

Try to run the existing project and then fill in the functionality as instructed by the comments.

To use that project, you can use the following commands from inside that folder using the terminal:
- `cargo build`: Builds the project
- `cargo run`: Builds the project, flashes it to the device and listens for any logs which it will display in the terminal. (This uses the `probe-run` tool)

In both cases you can add the `--release` flag to turn on optimizations.

*Note: There is a module called `lis3dh` in the assignment project. This is meant to be used in the second assignment, so can be ignored for now.*

</div>
