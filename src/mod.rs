#![no_std]

use kernel::module::version::Version;
use kernel::print;

kernel::module!("hello", Version::new(1, 0, 0));

#[no_mangle]
pub extern "C" fn init() {
	kernel::println!("Hello world!");
}

#[no_mangle]
pub extern "C" fn fini() {
	kernel::println!("Goodbye!");
}
