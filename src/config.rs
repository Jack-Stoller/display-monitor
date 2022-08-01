use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub usb_id: String,
    pub on_connect: Vec::<u32>,
    pub on_disconnect: Vec::<u32>
}