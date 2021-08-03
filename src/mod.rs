#![no_std]

// TODO rm
use abi::print;

#[no_mangle]
pub extern "C" fn init() {
	abi::println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn fini() {
	abi::println!("Goodbye!");
}
