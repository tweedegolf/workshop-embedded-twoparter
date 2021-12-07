<div class="read">

# Exercise 2B
**Device-host communication**

The goal of this exercise is to get an idea of how code can be shared between a device and a host, in order to set up a robust communication system.

### Instructions
1. For this exercice, we'll be working in part 2B. There are a couple of packages in there:
    - `firmware` contains all code that is run on the device. Apart from what you saw in the last exercise, it contains a `uarte` module, which uses the PAC to enable advanced functionality not implemented by the HAL. Take a peek at the functions in there. Try not to get distracted by the implementation details, but focus on the overall functionality that is available. The main application already implements tasks to control the `TimeoutUarte`.
    - `cli` defines a simple CLI application that listens for incoming messages, and opens a rudimentary repl with which you can send commands. You'll be implementing a couple of commands yourself, so have a peek at `cmd.rs`, to get an idea of how to do that.
    - `format` contains definitions of whatever is being send from the device to the server and vice-versa. To send new commands, you'll need to update the `ServerToDevice` and `DeviceToServer` structs. If you do, don't forget to compile both the firmware and the CLI in order for getting them to communicate nicely.

1. Flash the firmware onto the device using this command:
```bash
cargo run -p firmware --release
```
5. Run the CLI app with one of these commands. For `<PORT>` substitute the device's serial port path. If you omit the argument, the app will print any serial ports detected.
```bash
# Linux
cargo run-linux -p cli -- <PORT>
# Windows
cargo run-windows -p cli -- <PORT>
```

6. Test the setup. In the CLI repl, type
```
hello
```

Another command you can try (although it will only print stuff over RTT for now):
```
led 1 on
```

7. Open `main.rs`, and look up the `handle_message` task. In there, incoming messages are processed. You'll find the first instructions there.

1. Your next objective is to implement your own command. Add a command to `cmd.rs` in the `cli` package. Register it with the `CommandParser::parse` method, referring to the other  commands in that module. Update the `ServerToDevice` and `DeviceToServer` structs in `format`, and handle the messages in `firmware`. If you need ideas, you can have the device send over accelerometer measurements whenever they're ready. 

### Resources
- [The RTIC book](https://rtic.rs)
- [Lis3dh driver documentation](https://docs.rs/lis3dh/latest/lis3dh/) *Note that we are using an unreleased version here, so the docs on docs.rs are not complete*
- [Lis3dh driver repository](https://github.com/BenBergman/lis3dh-rs)
- [nRF-HAL examples](https://github.com/nrf-rs/nrf-hal/tree/master/examples)

</div>