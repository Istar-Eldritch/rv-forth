#![allow(dead_code)]
use core::arch::asm;

pub fn halt() -> ! {
    unsafe {
        asm!(
            "
                li x10, 255
                ecall
            ",
            options(noreturn)
            );
    }
}

pub fn memdump(from: u32, to: u32) {
    unsafe {
        asm!(
            "
                mv t0, {}
                mv t1, {}
                mv x11, t0
                mv x12, t1
                li x10, 254
                ecall
            ",
            in(reg) from,
            in(reg) to
            );
    }
}

pub fn wfi() {
    unsafe {
        asm!("wfi");
    }
}

pub fn mret() -> ! {
    unsafe {
        asm!("mret", options(noreturn))
    }
}

#[naked]
#[no_mangle]
#[link_section = ".init"]
pub extern "C" fn _init() -> ! {
    unsafe {
        asm!(
            "
                csrw mie, 0
                csrw mip, 0

                li  x1, 0
                li  x2, 0
                li  x3, 0
                li  x4, 0
                li  x5, 0
                li  x6, 0
                li  x7, 0
                li  x8, 0
                li  x9, 0
                li  x10,0
                li  x11,0
                li  x12,0
                li  x13,0
                li  x14,0
                li  x15,0
                li  x16,0
                li  x17,0
                li  x18,0
                li  x19,0
                li  x20,0
                li  x21,0
                li  x22,0
                li  x23,0
                li  x24,0
                li  x25,0
                li  x26,0
                li  x27,0
                li  x28,0
                li  x29,0
                li  x30,0
                li  x31,0

                la sp, _stack_start
                lui t0, %hi(_hart_stack_size)
                add t0, t0, %lo(_hart_stack_size)

                la t0, m_trap_handler
                csrw mtvec, t0

                j _start
    ",
    options(noreturn)
        )
    }
}

