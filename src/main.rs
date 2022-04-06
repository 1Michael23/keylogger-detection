use chrono::prelude::*;
use mac_address::{self, get_mac_address};
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

#[macro_use]
extern crate json;

#[derive(Serialize, Deserialize)]
struct Header {
    total: usize,
    creation: String,
    unique_id: String,
    version: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Keyboard {
    count: u8,
    bus_number: u8,
    address: u8,
    vendor_id: u16,
    product_id: u16,
    class_code: u8,
    protocol_code: u8,
    speed: String,
    port_number: u8,
}

#[derive(Serialize, Deserialize)]
struct Request {
    username: String,
    avatar_url: String,
    embeds: String,
}

struct Tests {
    bus_number_passed: bool,
    address_passed: bool,
    vendor_id_passed: bool,
    product_id_passed: bool,
    class_code_passed: bool,
    protocol_code_passed: bool,
    speed_passed: bool,
    port_number_passed: bool,
}

const REPORT_ON_PASS: bool = false;
const REPORT_ON_PORT_CHANGE: bool = false;
const REPORT_ON_ADDRESS_CHANGE: bool = false;

const WEBHOOK_URL: &str = "WEBHOOK URL GOES HERE";
const AVATAR_URL: &str = "AVATAR URL GOES HERE";

const PATH: &str = "./code.json";
fn main() {
    //Check to see if the file containing keyboard data exists, if not, create it

    if !Path::new(PATH).exists() {
        print!("Creating path...");
        File::create(PATH).expect("Failed to create json file");

        write_hid();
    }

    let mut current_devices: Vec<Keyboard> = Vec::new();

    for vendor_id in hid_devices() {
        current_devices.push(hid_data(&vendor_id));
    }

    let (saved_devices, header_data) = load_hid();

    let mut all_passed: bool = true;
    let mut device_passed: Vec<bool> = Vec::new();

    let mut tests: Vec<Tests> = Vec::new();

    if header_data.total != current_devices.len() {
        all_passed = false;
    } else {
        for count in 0..current_devices.len() {
            let current_keyboard: Keyboard = current_devices[count].clone();
            let saved_keyboard: Keyboard = saved_devices[count].clone();

            device_passed.push(true);

            let mut test: Tests = Tests {
                bus_number_passed: true,
                address_passed: true,
                vendor_id_passed: true,
                product_id_passed: true,
                class_code_passed: true,
                protocol_code_passed: true,
                speed_passed: true,
                port_number_passed: true,
            };

            if current_keyboard.bus_number != saved_keyboard.bus_number {
                test.bus_number_passed = false;
                device_passed[count] = false;
            }
            if current_keyboard.address != saved_keyboard.address {
                test.address_passed = false;
                if REPORT_ON_ADDRESS_CHANGE {
                    device_passed[count] = false;
                }
            }
            if current_keyboard.vendor_id != saved_keyboard.vendor_id {
                test.vendor_id_passed = false;
                device_passed[count] = false;
            }
            if current_keyboard.product_id != saved_keyboard.product_id {
                test.product_id_passed = false;
                device_passed[count] = false;
            }
            if current_keyboard.class_code != saved_keyboard.class_code {
                test.class_code_passed = false;
                device_passed[count] = false;
            }
            if current_keyboard.protocol_code != saved_keyboard.protocol_code {
                test.protocol_code_passed = false;
                device_passed[count] = false;
            }
            if current_keyboard.speed != saved_keyboard.speed {
                test.speed_passed = false;
                device_passed[count] = false;
            }
            if current_keyboard.port_number != saved_keyboard.port_number {
                test.port_number_passed = false;
                if REPORT_ON_PORT_CHANGE {
                    device_passed[count] = false;
                }
            }

            if !device_passed[count] {
                all_passed = false;
            }

            tests.push(test);
        }
    }

    if !all_passed || REPORT_ON_PASS {
        report(header_data, current_devices, saved_devices);
    }
}

fn send_report(input_string: String) {
    let json_request = object! {
        username: "Keylogger-Detection",
        avatar_url: AVATAR_URL,
        contents: "beans",
        embeds: [
            {
                title: "Keyboard Discrepancy Detected",
                description: input_string.as_str(),
                color: 16711680,
            }
        ]
    };

    let json = json_request.dump();
    let url = WEBHOOK_URL;

    ureq::post(url)
        .set("Content-Type", "application/json")
        .send(json.as_bytes())
        .expect("Failed to post to webhook");
}

//function to generate report upon device descrepancy

fn report(header_data: Header, current_devices: Vec<Keyboard>, saved_devices: Vec<Keyboard>) {
    let header_string = format!(
        "Header Data: Total: {}, Time Created: {}, Unique_ID: {}, Version: {}\n",
        header_data.total, header_data.creation, header_data.unique_id, header_data.version
    );

    let mut current_device_strings: Vec<String> = Vec::new();
    let mut saved_device_strings: Vec<String> = Vec::new();

    for current_device in current_devices {

        current_device_strings.push(format!("**({})**, Bus_number: {:03}, Address: {:03}, Device_ID: {:04x}:{:04x}, Class_code: {}, Protocol_code: {}, Speed: {}, Port_number: {}\n",current_device.count, current_device.bus_number, current_device.address, current_device.vendor_id, current_device.product_id,current_device.class_code, current_device.protocol_code, current_device.speed, current_device.port_number))
    }
    for saved_device in saved_devices {

        saved_device_strings.push(format!("**({})**, Bus_number: {:03}, Address: {:03}, Device_ID: {:04x}:{:04x}, Class_code: {}, Protocol_code: {}, Speed: {}, Port_number: {}\n",saved_device.count, saved_device.bus_number, saved_device.address, saved_device.vendor_id, saved_device.product_id,saved_device.class_code, saved_device.protocol_code, saved_device.speed, saved_device.port_number))
    }

    let mut total_string: String = String::new();

    total_string.push_str(&header_string);

    total_string.push_str("\n__**Current Devices:**__\n");

    if current_device_strings.is_empty() {
        total_string.push_str("No Devices Found...")
    } else {
        for line in current_device_strings {
            total_string.push_str(&line);
        }
    }
    total_string.push_str("\n__**Devices loaded from file:**__\n");

    for line in saved_device_strings {
        total_string.push_str(&line);
    }

    send_report(total_string);
}

//function to generate report upon program error

fn report_error(error_code: String) {
    send_report(error_code);
}

//function retirns a vector of the vendor ids of all current hid devices

fn hid_devices() -> Vec<u16> {
    let mut hid_vendor_ids: Vec<u16> = Vec::new();

    let api = hidapi_rusb::HidApi::new().unwrap();

    for hid_device in api.device_list() {
        hid_vendor_ids.push(hid_device.vendor_id());
    }

    hid_vendor_ids.sort_unstable();

    hid_vendor_ids
}

//tales a vendor id and returns full information on the device

fn hid_data(vendor_id: &u16) -> Keyboard {
    let mut keyboard = Keyboard {
        count: (0),
        bus_number: (0),
        address: (0),
        vendor_id: (0),
        product_id: (0),
        class_code: (0),
        protocol_code: (0),
        speed: "low".to_string(),
        port_number: (0),
    };

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == *vendor_id {
            keyboard.bus_number = device.bus_number();
            keyboard.address = device.address();
            keyboard.vendor_id = device_desc.vendor_id();
            keyboard.product_id = device_desc.product_id();
            keyboard.class_code = device_desc.class_code();
            keyboard.protocol_code = device_desc.protocol_code();
            keyboard.speed = format!("{:?}", device.speed());
            keyboard.port_number = device.port_number();
        }
    }
    keyboard
}

//gets currently attatched HID devices and writes them to a json file called {PATH}

fn write_hid() {
    let mut count: u8 = 0;
    let devices = hid_devices();

    let total: usize = devices.len();
    let creation = Local::now().to_string();
    let unique_id = get_unique_id();
    let version = 1.0;

    let header = Header {
        total,
        creation,
        unique_id,
        version,
    };
    let json_header: String = serde_json::to_string(&header).unwrap();

    let mut file = File::create(PATH).expect("Failed to open json file");

    file.write_all(json_header.as_bytes())
        .expect("Failed to write header data to json file.");
    file.write_all("\n".as_bytes())
        .expect("Failed to write newline to json file.");

    for vendor_id in devices {
        let mut keyboard = hid_data(&vendor_id);
        keyboard.count = count;

        let json_keyboard = serde_json::to_string(&keyboard).unwrap();

        file.write_all(json_keyboard.as_bytes())
            .expect("Failed to write keyboard data to json file.");
        file.write_all("\n".as_bytes())
            .expect("Failed to write newline to json file.");

        print!("writing to file");

        count += 1;
    }
}

//loads the saved json file and returns a vector of keyboard structs and a header struct

fn load_hid() -> (Vec<Keyboard>, Header) {
    let file = File::open(PATH).expect("Coulnt open file to verify data");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.expect("Failed to push to vector."));
    }

    let mut saved_devices: Vec<Keyboard> = Vec::new();

    let header_data: Header = serde_json::from_str(&lines[0]).unwrap();

    if header_data.total != 0 {
        for line in lines.iter().skip(0) {
            let keyboard_data: Keyboard =
                serde_json::from_str(line).expect("Failed to decode keyboard data");
            saved_devices.push(keyboard_data);
            
        }
    } else {
        report_error("No Devices found in file.".to_string());

        exit(0);
    }

    (saved_devices, header_data)
}

//This is a placeholder function for now, designed to be adapted to work with whatever unique ID works best for you

fn get_unique_id() -> String {
    get_mac_address()
        .expect("failed to get mac address")
        .unwrap()
        .to_string()
}
