use std::{fs::File, path::Path, io::{LineWriter, BufRead, BufReader, Write}};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use mac_address::{self, get_mac_address};

#[derive(Serialize, Deserialize)]
struct Header{
    total: usize,
    creation: String,
    unique_id: String,
    version: f32,
}

#[derive(Serialize, Deserialize)]
struct Keyboard{
    count: u8,
    bus_number: u8,
    address: u8,
    vendor_id: u16,
    product_id: u16,
    class_code: u8,
    protocol_code: u8,
    speed: String,
    port_number: u8
}

struct Test{
    bus_number: bool,
    address: bool,
    vendor_id: bool,
    product_id: bool,

}

const REPORT_ON_PASS: bool = false;
const REPORT_ON_PORT_CHANGE: bool = false;

const PATH: &str = "./code.json";

fn main() { 

    //Check to see if the file containing keyboard data exists, if not, create it

    if Path::new(PATH).exists() == false{
        print!("Creating path...");
        File::create(PATH).expect("Failed to create json file");

        write_hid();

    }

    let devices = hid_devices();

    let file = File::open(PATH).expect("Coulnt open file to verify data");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines(){
        lines.push(line.expect("Failed to push to vector."));
    }

    let header_data: Header = serde_json::from_str(&lines[0]).unwrap();

    for line in lines{
        let keyboard_data: Keyboard = serde_json::from_str(&line).unwrap();

        

        for vendor_id in &devices{
            let keyboard_current = hid_data(&vendor_id); 
            
            
            

        }
    }
}

    

    
    



//function retirns a vector of all HID devices' Vendor ID's

fn hid_devices() -> Vec<u16>{
    let mut hid_vendor_ids: Vec<u16> = Vec::new();

    let api = hidapi_rusb::HidApi::new().unwrap(); 

    for hid_device in api.device_list().into_iter(){
        hid_vendor_ids.push(hid_device.vendor_id());
    }

    hid_vendor_ids.sort();

    return hid_vendor_ids;
}

fn hid_data(vendor_id: &u16) -> Keyboard {

    let mut keyboard = Keyboard {count: (0), bus_number: (0), address: (0), vendor_id: (0), product_id: (0), class_code: (0), protocol_code: (0), speed: "low".to_string(), port_number: (0) };

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if  device_desc.vendor_id() == *vendor_id {
        
            keyboard.bus_number = device.bus_number();
            keyboard.address = device.address();
            keyboard.vendor_id = device_desc.vendor_id();
            keyboard.product_id = device_desc.product_id();
            keyboard.class_code = device_desc.class_code();
            keyboard.protocol_code = device_desc.protocol_code();
            keyboard.speed = format!("{:?}", device.speed());
            keyboard.port_number = device.port_number();
                    
      }
    };
    return keyboard;
}


fn write_hid() {

    let mut count:u8 = 0;
    let devices = hid_devices();

    let total: usize = devices.len();
    let creation = Local::now().to_string();
    let unique_id = get_unique_id();
    let version = 1.0;

    
    let header = Header { total, creation, unique_id, version };
    let json_header: String = serde_json::to_string(&header).unwrap();

    

    let mut file = File::create(PATH).expect("Failed to open json file");
    
    file.write_all(json_header.as_bytes()).expect("Failed to write header data to json file.");
    file.write_all("\n".as_bytes()).expect("Failed to write newline to json file.");

    for vendor_id in devices {
        let mut keyboard = hid_data(&vendor_id);
        keyboard.count = count;

        let json_keyboard = serde_json::to_string(&keyboard).unwrap();

        file.write_all(json_keyboard.as_bytes()).expect("Failed to write keyboard data to json file.");
        file.write_all("\n".as_bytes()).expect("Failed to write newline to json file.");

        print!("writing to file");

        count = count +1;
    }

}

fn get_unique_id() -> String{

    //This is a placeholder function for now, designed to be adapted to work with whatever unique ID works best for you

   return get_mac_address().expect("failed to get mac address").unwrap().to_string();


}