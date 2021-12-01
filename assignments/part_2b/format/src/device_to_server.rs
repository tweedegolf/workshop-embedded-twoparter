#[cfg(feature = "defmt")]
use defmt::Format;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct DeviceToServer {
    pub led_status: Option<(u8, bool)>,
    pub said_hello: bool,
    // TODO add you own fields here for the CLI to handle
}
