#![feature(async_await)]

pub mod protos;
pub mod hex_util;
pub mod add_person;
pub mod list_people;

//extern crate futures;
//extern crate futures_cpupool;
//extern crate grpc;
extern crate protobuf;
//extern crate tls_api;

pub use protos::addressbook::{AddressBook, Person, Person_PhoneNumber as PhoneNumber,
                              Person_PhoneType as PhoneType};

