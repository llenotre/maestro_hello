#![no_std]

extern crate maestro;

#[no_mangle]
pub extern "C" fn init() {
	maestro::println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn fini() {
	maestro::println!("Goodbye!");
}
