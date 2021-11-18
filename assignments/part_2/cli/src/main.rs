use std::{thread, time::Duration};

use clap::{App, Arg};
use format::{DeviceToServer, ServerToDevice};
use serialport::{SerialPortType, UsbPortInfo};

use crate::serial::TxPort;

mod serial;

fn handle_message(msg: DeviceToServer) {
    println!("Got message: {:?}", msg);
    // TODO, do cool stuff with the message that just came in.
}

fn run<const N: usize>(mut tx_port: TxPort<N>) {
    let mut send_acc_data = true;
    loop {
        let msg = ServerToDevice { send_acc_data };

        println!("Sending message: {:?}", &msg);
        tx_port.write_message(&msg);
        send_acc_data = !send_acc_data;
        thread::sleep(Duration::from_millis(200));
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
    let mut port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(1000))
        .open();

    match port {
        Ok(port) => {
            let tx_port: TxPort<32> = TxPort::new(port.try_clone().unwrap());

            let rx_thread = std::thread::spawn(|| {
                serial::RxPort::new(port).run_read_task::<_, 32>(handle_message)
            });

            run(tx_port);

            rx_thread.join();
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
