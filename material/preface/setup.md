<div class="read">

# Setting up your environment
Before we begin, you'll need to check your parts, install some software, and verify that everything works the way it should.
## Hardware
You should have received the following parts:

- nRF52840-DK
- Breadboard
- LIS3DH Breakout board
- Male-to-male breadboard wires

You'll also need a Micro-USB cable, but we're sure you've got one to spare.

Please check that everything is complete. If not, please contact us.

## Software
Next, we'll need the software needed to build and flash the firware we're going to create.

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

Then, we'll install some tools needed to flash the mcu and inspect the code.

On Linux you need to install libudev and libusb. If you're on Ubuntu:
```bash
# ubuntu
sudo apt install -y libusb-1.0-0-dev libftdi1-dev
```

On all platforms:
```bash
cargo install flip-link --version 0.1.5
cargo install cargo-binutils --version 0.3.3
cargo install cargo-flash --version 0.11.0
cargo install cargo-embed --version 0.11.0
cargo install probe-run --version 0.3.0
rustup component add llvm-tools-preview
```

If you're on linux, you'll need to update your udev rules.
On ubuntu, run the following inside the workshop folder you just cloned;

```bash
sudo cp 99-jlink-nrf52840dk.rules /etc/udev/rules.d
sudo udevadm control --reload-rules
```

If you're on windows, we need to install a generic WinUSB driver. You can use [Zadig](https://zadig.akeo.ie/). 
*This will uninstall the official driver, which means that the official Segger tools will not work anymore after this.* To revert, you can use the same tool.

Then, switch the nRF52840DK off and on or remove the cable and plug it in again.


For debugging, we will be using GDB and OpenOCD. On Ubuntu, GDB can be installed with:
```bash
sudo apt update
sudo apt install gdb-multiarch
```

On Windows: *TODO*

In order to get logging working correctly, we'll use OpenOCD **version 0.11.x**. You can download it using xPack package manager or `xpm`. To install `xpm`, please follow the instructions [on this page](https://xpack.github.io/xpm/install/#). Once xpm is working correctly, you can install OpenOCD with:

```bash
xpm install --global @xpack-dev-tools/openocd@latest --verbose
```
That command will output the location OpenOCD is installed in. You might need to update your PATH variable to make everything a bit more ergonomic.
For more details, take a look at the [install page](https://xpack.github.io/openocd/install/).

To check that you've got the correct version:
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

**Please verify that you've got version 0.11**

### Other tools
Of course, you're free to use your editor of choice. We will be using Visual Studio Code. To improve the Rust development experience, we use Rust Analyzer, which can be found [here](https://github.com/rust-analyzer/rust-analyzer).


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

To test the hardware, please connect the nrf board to your pc, switch it on, and run
```bash
DEFMT_LOG=info cargo run --release -p workshop-examples --bin test
```

If everything works correctly, you should now see the accelerometer samples being printed on the display. If not, don't worry and contact us.

## Docs
Datasheets, manuals, and schematics of the parts we are using in this workshop.
### nRF52840
- [nRF52840DK documentation](https://infocenter.nordicsemi.com/topic/ug_nrf52840_dk/UG/dk/intro.html)
- [nRF52840 product specification](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.2.pdf)

### LIS3DH
- [Datsheet](https://cdn-learn.adafruit.com/assets/assets/000/085/846/original/lis3dh.pdf?1576396666)
- [Schematic](https://cdn-learn.adafruit.com/assets/assets/000/028/587/original/sensors_sch.png?1447888851)
</div>
