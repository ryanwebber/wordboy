#![feature(naked_functions)]
#![no_std]
#![no_main]

use ab_os::{
    mmio::{BACKDROP, DISPCNT, KEYINPUT},
    video::Color,
};

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text._start"]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm! {
      "b 1f",
      ".space 0xE0",
      "1:",
      "ldr r12, =main",
      "bx r12",
      options(noreturn)
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    DISPCNT.write(0);
    loop {
        let k = KEYINPUT.read();
        BACKDROP.write(if k.a() { Color::RED } else { Color::GREEN })
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
