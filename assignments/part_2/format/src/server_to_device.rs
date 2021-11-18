#[cfg(feature = "defmt")]
use defmt::Format;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct ServerToDevice {
    // TODO
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::is_false")]
    pub send_acc_data: bool,
}
