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

For the rest of the steps, you'll need the [source code of this workshop](https://github.com/tweedegolf/workshop-december-2021/).

```bash
git clone git@github.com:tweedegolf/workshop-december-2021.git
cd embedded-workshop
```
Or, if you like to use HTTPS instead:
```bash
git clone https://github.com/tweedegolf/workshop-december-2021.git
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
cargo install flip-link --version 0.1.5
cargo install cargo-binutils --version 0.3.3
cargo install cargo-flash --version 0.11.0
cargo install cargo-embed --version 0.11.0
cargo install probe-run --version 0.3.0
rustup component add llvm-tools-preview rustfmt clippy
```

If you're on `linux`, you'll need to update your udev rules.
On ubuntu, run the following inside the workshop folder you just cloned;

```bash
sudo cp 99-jlink-nrf52840dk.rules /etc/udev/rules.d
sudo udevadm control --reload-rules
```

If you're on `windows`, we need to install a generic WinUSB driver. You can use [Zadig](https://zadig.akeo.ie/) to select the usb device that uses the jlink driver and install WinUSB on it. 
*This will uninstall the official driver, which means that the official Segger tools will not work anymore after this.* To revert, go to `device manager` and uninstall the usb device. The jlink driver will then be used again for that usb connection.

Then, switch the DK off and on or remove the cable and plug it in again.

### Debugging

For debugging, we will be using GDB and OpenOCD.

#### Linux
On Ubuntu, GDB can be installed with:
```bash
sudo apt update
sudo apt install gdb-multiarch
```

In order to get logging working correctly, we'll use OpenOCD **version 0.11.x**. You can download it using xPack package manager or `xpm`. To install `xpm`, please follow the instructions [on this page](https://xpack.github.io/xpm/install/#). Once xpm is working correctly, you can install OpenOCD with:

```bash
xpm install --global @xpack-dev-tools/openocd@latest --verbose
```
That command will output the location OpenOCD is installed in.
For more details, take a look at the [install page](https://xpack.github.io/openocd/install/).

Alternatively, you can get OpenOCD from [GitHub Releases](https://github.com/xpack-dev-tools/openocd-xpack/releases).

*You need to update your PATH variable to make everything a bit more ergonomic. You may need to restart applications for this change to take effect. Alternatively, you can use a symbolic link.*

To check that you've got the correct version working:
```bash
openocd --version
```

The output should be something like this:
```bash
xPack OpenOCD x86_64 Open On-Chip Debugger 0.11.0+dev (2021-10-16-21:15)
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
```

#### Windows

On `Windows`, make sure openocd and arm-none-eabi-gdb can be found on the path.
- [Arm embedded toolchain download](https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads)
- [OpenOCD download](https://github.com/xpack-dev-tools/openocd-xpack/releases)
- Wherever you installed these, make sure to add them to the path. *Note: You may have to restart applications for this change to take effect* ([How to](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/))


### Other tools
Of course, you're free to use your editor of choice. To improve the Rust development experience, we use Rust Analyzer, which can be found [here](https://github.com/rust-analyzer/rust-analyzer). It's available for many different editors. As for ourselves, we will be using Visual Studio Code along with a couple of extensions. To install them, please use the instructions for [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) and [Cortex Debug](https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug).


## Testing
Before we begin, we need to test our hardware. We'll be testing the LIS3DH accelerometer, as well as the nRF52840DK board. Make sure you have checked out the latest version of the workshop source.

### LIS3DH accelerometer connection
First, let's wire up the LIS3DH accelerometer for I2C usage. 
**Please turn off your nRF52840DK**. Then, wire up the accelerometer, referring to the table below.

| LIS3DH Pin | nRF52840DK pin |
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

#### Specific for nRF52-DK

If you're using the nRF52-DK, you need to update the configuration a bit. Edit the first lines of `.cargo/config.toml` from

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# Uncomment the line below to use probe-rs for nRF52840
runner = "probe-run --chip nRF52840"
# Uncomment the line below to use probe-rs for nRF52832
#runner = "probe-run --chip nRF52832"
# Uncomment the line below to use GDB with OpenOCD
# runner = "gdb-multiarch -q -x openocd.gdb"
<rest of file...>
```

to

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# Uncomment the line below to use probe-rs for nRF52840
#runner = "probe-run --chip nRF52840"
# Uncomment the line below to use probe-rs for nRF52832
runner = "probe-run --chip nRF52832"
# Uncomment the line below to use GDB with OpenOCD
# runner = "gdb-multiarch -q -x openocd.gdb"
<rest of file...>
```

To test the hardware, please connect the nRF52-DK to your pc, switch it on, and run
```bash
cargo run --release -p workshop-examples --features=nrf52dk --bin test
```

If everything works correctly, you should now see the accelerometer samples being printed on the display. If not, don't worry and contact us.
#### Specific for nRF52840-DK
To test the hardware, please connect the nrf52840-DK to your pc, switch it on, and run
```bash
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
