#![windows_subsystem = "windows"]

use config::Config;
use std::time::Duration;
use std::thread::sleep;
use ddc_winapi::Monitor;

mod config;


fn main() {

    let f = std::fs::File::open(format!("C:\\Users\\{}\\AppData\\Roaming\\display-monitor\\config.yaml", whoami::username())).expect("Could not open the config file.");
    let config: config::Config = serde_yaml::from_reader(f).expect("Could not read the config file.");


    let usb_info = extract_usb_info(&config);
    let mut was_connected = is_connected(&usb_info);

    loop {
        sleep(Duration::from_millis(200));

        let is_connected_now = is_connected(&usb_info);

        if !was_connected && is_connected_now {
            println!("Device Connected!");
            connect(&config);
        } else if was_connected && !is_connected_now {
            println!("Device Disconnected!");
            disconnect(&config);
        }

        was_connected = is_connected_now;
    }
}

fn is_connected(usb_info: &USBInfo) -> bool {
    !usb_enumeration::enumerate(Some(usb_info.vendor_id), Some(usb_info.product_id)).is_empty()
}

fn connect(config: &Config) {
    let mut i = 0;
    for ddc in Monitor::enumerate().unwrap() {
        if config.on_connect.len() > i {
            let res = ddc.winapi_set_vcp_feature(0x60, config.on_connect[i]);

            match res {
                Ok(_) => {
                    //Nothing
                }
                Err(e) => {
                    println!("There was an error setting the monitor input source. {e:?}")
                }
            }
        }
        i += 1;
    }
}

fn disconnect(config: &Config) {
    let mut i = 0;
    for ddc in Monitor::enumerate().unwrap() {
        if config.on_disconnect.len() > i {
            let to_be = config.on_disconnect[i];

            let res = ddc.winapi_set_vcp_feature(0x60, to_be);

            match res {
                Ok(_) => {
                    //Nothing
                }
                Err(e) => {
                    println!("There was an error setting the monitor input source. {e:?}")
                }
            }
        }
        i += 1;
    }
}

struct USBInfo {
    vendor_id: u16,
    product_id: u16
}

fn extract_usb_info(config: &Config) -> USBInfo {
    let split: Vec<&str> = config.usb_id.split(":").collect();

    USBInfo {
        vendor_id: u16::from_str_radix(&split[0].to_string(), 16).expect("Invalid Vendor Id"),
        product_id: u16::from_str_radix(&split[1].to_string(), 16).expect("Invalid Vendor Id"),
    }
}