#![no_std]

extern crate maestro;

// TODO rm
use maestro::print;

#[no_mangle]
pub extern "C" fn init() {
	maestro::println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn fini() {
	maestro::println!("Goodbye!");
}
