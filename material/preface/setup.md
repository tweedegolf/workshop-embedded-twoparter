<div class="read">

# Setting up your environment
Before we begin, you'll need to check your parts, install some software, and verify that everything works the way it should.
## Hardware
You should have received the following parts:

- nRF52840-DK or nRF52-DK
- Breadboard
- LIS3DH Breakout board
- Male-to-male breadboard wires

You'll also need a Micro-USB cable, but we're sure you've got one to spare.

Please check that everything is complete. If not, please contact us.

## Software

### Rust

Next, we'll need the software needed to build and flash the firmware we're going to create.

**Please install Rustup, using [the instructions here](https://rustup.rs/)**. This will install the rustc compiler, as well as [Cargo](https://doc.rust-lang.org/cargo/), Rust's package manager. We'll be using cargo extensively in this workshop

Make sure you've got the latest stable rust version:
```bash
rustup default stable
rustup update
```

Install the `thumbv7em-none-eabi` toolchain with the following command:
```bash
rustup target add thumbv7em-none-eabihf
```

### Repository

For the rest of the steps, you'll need the [source code of this workshop](https://github.com/tweedegolf/workshop-embedded-twoparter).

```bash
git clone git@github.com:tweedegolf/workshop-embedded-twoparter.git embedded-workshop
cd 
```
Or, if you like to use HTTPS instead:
```bash
git clone https://github.com/tweedegolf/workshop-embedded-twoparter.git embedded-workshop
cd embedded-workshop
```

### Flashing

Then, we'll install some tools needed to flash the mcu and inspect the code.

On `Linux` you need to install the "dev" libraries for udev, usb, and ftdi libudev-dev. If you're on Ubuntu:
```bash
# ubuntu
sudo apt install -y libusb-1.0-0-dev libftdi1-dev libudev-dev
```

On `all platforms`:
```bash
rustup component add llvm-tools-preview rustfmt clippy
cargo install cargo-binutils
cargo install cargo-flash
cargo install probe-run
```

If you're on `linux`, you'll need to update your udev rules.
On ubuntu, run the following inside the workshop folder you just cloned;

```bash
sudo cp 99-jlink-nrf.rules /etc/udev/rules.d
sudo udevadm control --reload-rules
```

If you're on `windows`, we need to install a generic WinUSB driver. You can use [Zadig](https://zadig.akeo.ie/) to select the usb device that uses the jlink driver and install WinUSB on it. 
*This will uninstall the official driver, which means that the official Segger tools will not work anymore after this.* To revert, go to `device manager` and uninstall the usb device. The jlink driver will then be used again for that usb connection.

Then, switch the DK off and on or remove the cable and plug it in again.

### Other tools

Of course, you're free to use your editor of choice. To improve the Rust development experience, we use Rust Analyzer, which can be found [here](https://github.com/rust-analyzer/rust-analyzer). It's available for many different editors. As for ourselves, we will be using Visual Studio Code along with a couple of extensions. To install them, please use the instructions for [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) and [Cortex Debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug).

Debugging is also possible using the normal GDB tools.

## Testing
Before we begin, we need to test our hardware. We'll be testing the LIS3DH accelerometer, as well as the nRF52840-DK or nRF52-DK board. Make sure you have checked out the latest version of the workshop source.

### LIS3DH accelerometer connection
First, let's wire up the LIS3DH accelerometer for I2C usage. 
**Please turn off your DK**. Then, wire up the accelerometer, referring to the table below.

| LIS3DH Pin | nRF52 pin 	  |
|------------|----------------|
| VIN (+)    | VDD            |
| 3vo        | -              |
| GND (-)    | GND            |
| SCL        | P0.27          |
| SDA        | P0.26          |
| SDO        | -              |
| CS'        | -              |
| INT        | -              |
| A1         | -              |
| A2         | -              |
| A3         | -              |

*We'll be using other pins later on, but they're not needed to test the hardware*

### Running the test

The tests are located in `assignments/test`. Follow the next instructions using that folder.

#### Specific for nRF52-DK

If you're using the nRF52-DK, you need to update the configuration a bit. Edit the first lines of `.cargo/config.toml` from

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# Uncomment the line below to use probe-rs for nRF52840
runner = "probe-run --chip nRF52840 --erase-all"
# Uncomment the line below to use probe-rs for nRF52832
#runner = "probe-run --chip nRF52832 --erase-all"
<rest of file...>
```

to

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# Uncomment the line below to use probe-rs for nRF52840
#runner = "probe-run --chip nRF52840 --erase-all"
# Uncomment the line below to use probe-rs for nRF52832
runner = "probe-run --chip nRF52832 --erase-all"
<rest of file...>
```

To test the hardware, please connect the nRF52-DK to your pc, switch it on, and run
```bash
cd ./assignments/test
cargo run --release -p workshop-examples --features=nrf52dk --bin test
```

If everything works correctly, you should now see the accelerometer samples being printed on the display. If not, don't worry and contact us.
#### Specific for nRF52840-DK
To test the hardware, please connect the nrf52840-DK to your pc, switch it on, and run
```bash
cd ./assignments/test
cargo run --release -p workshop-examples --features=nrf52840dk --bin test
```

If everything works correctly, you should now see the accelerometer samples being printed on the display. If not, don't worry and contact us.


## Docs
Datasheets, manuals, and schematics of the parts we are using in this workshop.
### nRF52840
- [nRF52840DK documentation](https://infocenter.nordicsemi.com/topic/ug_nrf52840_dk/UG/dk/intro.html)
- [nRF52840 product specification](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.2.pdf)

### nRF52832
- [nRF52-DK documentation](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fug_nrf52832_dk%2FUG%2Fnrf52_DK%2Fintro.html)
- [nRF52832 product specification](https://infocenter.nordicsemi.com/pdf/nRF52832_PS_v1.8.pdf)
### LIS3DH
- [Datsheet](https://cdn-learn.adafruit.com/assets/assets/000/085/846/original/lis3dh.pdf?1576396666)
- [Schematic](https://cdn-learn.adafruit.com/assets/assets/000/028/587/original/sensors_sch.png?1447888851)
</div>
