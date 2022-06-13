#![windows_subsystem = "console"]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(overflowing_literals)]
#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {

    loop {}
}


#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}