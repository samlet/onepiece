pub mod protos;
pub mod hex_util;

extern crate protobuf;

use protos::simple::MyObj;
//use protos::addressbook::{AddressBook, Person, Person_PhoneNumber as PhoneNumber,
//                          Person_PhoneType as PhoneType};
use hex_util::encode_hex;

use protobuf::*;

fn main() {
    let rust = "Rust";
    println!("Hello, {}!", rust);

    let mut po=MyObj::new();
    let name=String::from("tom");
    po.set_name(name);
    po.set_number(18);
    println!("{}, {}", po.name, po.number);

    let serialized = po.write_to_bytes().unwrap();
    println!("{}: {}", serialized.len(), encode_hex(&serialized));
    let mut is = CodedInputStream::from_bytes(&serialized);
    let parsed = parse_from_bytes::<MyObj>(&serialized).unwrap();
    println!("{}, {}", parsed.name, parsed.number);
    println!("{}", text_format::print_to_string(&parsed));
}

