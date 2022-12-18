#![allow(dead_code)]
use core::arch::asm;

pub fn halt() -> ! {
    unsafe {
        asm!(
            "
                li x15, 255
                ecall
            ",
            options(noreturn)
        );
    }
}

pub fn wfi() {
    unsafe {
        asm!("wfi");
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
            
                la t0, _m_trap
                csrw mtvec, t0

                j _start
    ",
            options(noreturn)
        )
    }
}

#[naked]
#[no_mangle]
#[link_section = ".trap"]
pub extern "C" fn _save_registers() -> ! {
    unsafe {
        asm!(
            "
                sw t0, 1*(1 << 2)(sp)
                sw t1, 2*(1 << 2)(sp)
                sw t2, 3*(1 << 2)(sp)
                sw t3, 4*(1 << 2)(sp)
                sw t4, 5*(1 << 2)(sp)
                sw t5, 6*(1 << 2)(sp)
                sw t6, 7*(1 << 2)(sp)
                sw a0, 8*(1 << 2)(sp)
                sw a1, 9*(1 << 2)(sp)
                sw a2, 10*(1 << 2)(sp)
                sw a3, 11*(1 << 2)(sp)
                sw a4, 12*(1 << 2)(sp)
                sw a5, 13*(1 << 2)(sp)
                sw a6, 14*(1 << 2)(sp)
                sw a7, 15*(1 << 2)(sp)
                ret
            ",
            options(noreturn)
        )
    }
}

#[naked]
#[no_mangle]
#[link_section = ".trap"]
pub extern "C" fn _recover_registers() -> ! {
    unsafe {
        asm!(
            "
                lw t0, 1*(1 << 2)(sp)
                lw t1, 2*(1 << 2)(sp)
                lw t2, 3*(1 << 2)(sp)
                lw t3, 4*(1 << 2)(sp)
                lw t4, 5*(1 << 2)(sp)
                lw t5, 6*(1 << 2)(sp)
                lw t6, 7*(1 << 2)(sp)
                lw a0, 8*(1 << 2)(sp)
                lw a1, 9*(1 << 2)(sp)
                lw a2, 10*(1 << 2)(sp)
                lw a3, 11*(1 << 2)(sp)
                lw a4, 12*(1 << 2)(sp)
                lw a5, 13*(1 << 2)(sp)
                lw a6, 14*(1 << 2)(sp)
                lw a7, 15*(1 << 2)(sp)
            
                ret
            ",
            options(noreturn)
        )
    }
}

#[naked]
#[no_mangle]
#[link_section = ".trap"]
pub extern "C" fn _m_trap() -> ! {
    unsafe {
        asm!(
            "

                //addi sp, sp, -16*(1<<2)
                //sw ra, 0*(1 << 2)(sp)

                //j _save_registers
                //add a0, sp, zero

                jal ra, m_trap_handler
                // j _recover_registers

                // lw ra, 0*(1 << 2)(sp)
                // addi sp, sp, 16*(1<<2)

                mret
            ",
            options(noreturn)
        )
    }
}
