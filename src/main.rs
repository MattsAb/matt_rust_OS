#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mattos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use mattos::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    mattos::init();


    let ptr = 0xdeadbeaf as *mut u8;
    unsafe { *ptr = 42; }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    mattos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mattos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mattos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}