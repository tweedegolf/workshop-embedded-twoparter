use std::{io::Write, time::Duration, thread};

use clap::{App, Arg};
use format::DeviceToServer;
use serialport::{SerialPortType, UsbPortInfo};

use crate::{cmd::CommandParser, serial::TxPort};

mod cmd;
mod serial;

fn handle_message(msg: DeviceToServer) {
    println!("Got message: {:?}", msg);
    let DeviceToServer { led_status, said_hello } = msg;
    if said_hello {
        println!("Device said hello!");
    }

    if let Some((led_id, enabled)) = led_status {
        let status = match enabled {
            true => "on",
            false => "off",
        };
        println!("Led {} status: {}", led_id, status);
    }
    // TODO, do cool stuff with the message that just came in.
}

fn run<const N: usize>(mut tx_port: TxPort<N>) {
    use crate::cmd::ParseError::*;
    use std::io::BufRead;

    let stdin = std::io::stdin();
    println!("Welcome to the device Commander! Please enter your command and press Enter");
    let mut lines = stdin.lock().lines();
    loop {
        if let Some(line) = lines.next() {
            match CommandParser::parse(&line.unwrap()) {
                Ok(cmd) => {
                    let msg = cmd.build_message();

                    tx_port.write_message(&msg).unwrap();
                    println!("Command sent!");
                }
                Err(CommandNotFound) => eprintln!("Error: Command not found"),
                Err(InvalidArgs) => eprintln!("Error: Command arguments invalid"),
            }
        } else {
            break;
        }
    }
}

fn main() {
    let matches = App::new("Device commander")
        .version("0.1")
        .arg(
            Arg::with_name("PORT")
                .index(1)
                .takes_value(true)
                .help("The path to the serial port to listen to"),
        )
        .get_matches();

    if let Some(port_name) = matches.value_of("PORT") {
        listen(port_name)
    } else {
        eprintln!("Please specify port as the first argument. For help, run with --help");
        eprintln!();
        print_available_ports();
    }
}

fn listen(port_name: &str) {
    let port = serial::SerialPort::new(port_name.to_owned());

    match port {
        Ok(port) => {
            let (tx_port, mut rx_port): (TxPort<32>, _) = port.split();

            let rx_thread =
                thread::spawn(move || rx_port.run_read_task::<_, 32>(handle_message));

            run(tx_port);

            rx_thread.join().unwrap();
        }
        Err(e) => {
            eprintln!("Error opening serial port {}: {}", port_name, e);
            eprintln!();
            print_available_ports();
        }
    }
}

fn print_available_ports() {
    println!("Available ports (listing USB only):");
    for port in serialport::available_ports().unwrap() {
        match (port.port_name, port.port_type) {
            (
                port_name,
                SerialPortType::UsbPort(UsbPortInfo {
                    vid,
                    pid,

                    manufacturer,
                    ..
                }),
            ) => {
                let manufacturer = manufacturer.unwrap_or_default();
                eprintln!(
                    "\t - {} (Vendor ID: {:#x}; Product ID: {:#x}; Manufacturer: {})",
                    port_name, vid, pid, manufacturer,
                );
            }
            _ => {} // Ignore other types
        }
    }
}
