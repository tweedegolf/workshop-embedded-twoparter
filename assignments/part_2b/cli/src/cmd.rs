use std::{fmt::Debug, iter::Peekable, str::Split};

use format::ServerToDevice;

type ChunkIter<'a> = Peekable<Split<'a, char>>;
use commands::*;

pub trait Command: Debug {
    fn parse(chunks: ChunkIter) -> Result<Box<dyn Command>, ParseError>
    where
        Self: Sized;

    fn boxed(self) -> Box<dyn Command>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }

    fn build_message(&self) -> ServerToDevice;
}

#[derive(Debug)]
pub enum ParseError {
    CommandNotFound,
    InvalidArgs,
}

pub struct CommandParser;

impl CommandParser {
    pub fn parse(cmd: &str) -> Result<Box<dyn Command>, ParseError> {
        let chunks = cmd.split(' ').peekable().clone();

        fn parse_next<'c, C: Command>(
            chunks: ChunkIter<'c>,
        ) -> impl FnOnce(ParseError) -> Result<Box<dyn Command>, ParseError> + 'c {
            move |e: ParseError| match e {
                ParseError::CommandNotFound => C::parse(chunks),
                r => Err(r),
            }
        }

        LedStatus::parse(chunks.clone()).or_else(parse_next::<SayHello>(chunks.clone()))
    }
}

mod commands {
    use format::ServerToDevice;

    use super::{
        ChunkIter, Command,
        ParseError::{self, *},
    };

    #[derive(Debug)]
    pub struct LedStatus {
        led_no: u8,
        on: bool,
    }

    impl Command for LedStatus {
        fn parse(mut chunks: ChunkIter) -> Result<Box<dyn Command>, ParseError>
        where
            Self: Sized,
        {
            let cmd = chunks.next();
            let arg1: Option<u8> = chunks.next().map(|a| a.parse().ok()).flatten();
            let arg2: Option<bool> = chunks
                .next()
                .map(|a| match a {
                    "on" => Some(true),
                    "off" => Some(false),
                    _ => None,
                })
                .flatten();

            match (cmd, arg1, arg2) {
                (Some("led"), Some(led_no @ 1..=4), Some(on)) => Ok(Self { led_no, on }.boxed()),
                (Some("led"), _, _) => Err(InvalidArgs),
                _ => Err(CommandNotFound),
            }
        }

        fn build_message(&self) -> ServerToDevice {
            ServerToDevice {
                set_led_status: Some((self.led_no, self.on)),
                ..ServerToDevice::default()
            }
        }
    }

    #[derive(Debug)]
    pub struct SayHello;

    impl Command for SayHello {
        fn parse(mut chunks: ChunkIter) -> Result<Box<dyn Command>, ParseError>
        where
            Self: Sized,
        {
            let cmd = chunks.next();

            match cmd {
                Some("hello") => Ok(Self.boxed()),
                _ => Err(CommandNotFound),
            }
        }

        fn build_message(&self) -> ServerToDevice {
            ServerToDevice {
                say_hello: true,
                ..ServerToDevice::default()
            }
        }
    }

    // TODO add your own commands here
}
