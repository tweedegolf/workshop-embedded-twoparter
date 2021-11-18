use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let matches = App::new("My awesome CLI")
        .version("0.1")
        .subcommand(
            SubCommand::with_name("listen")
                .about("Listen to firmware output over serial")
                .arg(
                    Arg::with_name("PORT")
                        .long("port")
                        .short("p")
                        .takes_value(true)
                        .required(true)
                        .help("The serial port to listen to"),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("listen") {
        listen(matches)
    } else {
        eprintln!("Command not found. Try --help");
    }
}

fn listen(matches: &ArgMatches) {
    use std::{
        io::{BufRead, BufReader, ErrorKind::TimedOut},
        time::Duration,
    };

    dbg!(matches);
    let port_name = matches.value_of("PORT").unwrap(); // required argument

    let port = serialport::new(port_name, 115200)
        .timeout(Duration::from_millis(5000))
        .open()
        .expect(&format!("Failed to open port {}", port_name));

    println!("Listening on `{}`. Press CTRL+C to quit", port_name);

    let mut full_buf: Vec<u8> = Vec::with_capacity(1024);
    let mut serial_buf: Vec<u8> = vec![0; 32];
    // TODO pipe bytes to postcard
}
