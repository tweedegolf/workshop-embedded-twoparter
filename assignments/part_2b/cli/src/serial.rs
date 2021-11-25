use std::io;

use format::{DeviceToServer, ServerToDevice};
use postcard::CobsAccumulator;
use serialport::SerialPort;

pub struct RxPort {
    port: Box<dyn SerialPort>,
}

impl RxPort {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self { port }
    }

    pub fn run_read_task<F: Fn(DeviceToServer) -> (), const N: usize>(&mut self, on_msg: F) {
        let mut accumulator = CobsAccumulator::<32>::new();
        let mut serial_buf = [0u8; N];
        use postcard::FeedResult::*;
        loop {
            let chunk_len = self
                .port
                .read(&mut serial_buf)
                .or_else(|e| {
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        Ok(0) // Just a time out
                    } else {
                        Err(e)
                    }
                })
                .expect("Serial read error");

            let chunk = &serial_buf[..chunk_len];
            match accumulator.feed(chunk) {
                Consumed => {} // Do nothing
                OverFull(_) => eprintln!("Accumulator full, dropping contents"),
                DeserError(_) => eprintln!("Deserialize error, throwing away message"),
                Success { data, .. } => on_msg(data),
            }
        }
    }
}

pub struct TxPort<const N: usize> {
    port: Box<dyn SerialPort>,
    buf: [u8; N],
}

impl<const N: usize> TxPort<N> {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Self {
            port,
            buf: [0u8; N],
        }
    }

    pub fn write_message(&mut self, msg: &ServerToDevice) -> Result<(), io::Error> {
        while let 1.. = self.port.bytes_to_write().unwrap() {
            // There are still bytes awaiting transmission
            // Wait for current write task to finish
        }
        let msg = postcard::to_slice_cobs(msg, &mut self.buf).unwrap();
        self.port.write(&msg).map(|_| {})
    }
}
