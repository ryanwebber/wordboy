#![feature(naked_functions)]
#![no_std]

pub mod input;
pub mod mmio;
pub mod rand;
pub mod video;

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

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
