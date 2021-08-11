#![no_std]

// TODO rm
use kernel::print;

#[no_mangle]
pub extern "C" fn init() {
	kernel::println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn fini() {
	kernel::println!("Goodbye!");
}
