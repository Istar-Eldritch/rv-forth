#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(default_alloc_error_handler)]

extern crate alloc;
mod allocator;
mod collections;
mod low;
mod hash;
mod state;
mod forth;

use low::asm::{halt, wfi, memdump};
use low::csr::{MCause, MStatus, Mie};
use low::plic::Plic;
use low::uart::Uart;
use core::panic::PanicInfo;
use state::State;
use core::fmt::Write;

#[no_mangle]
pub fn _start() {
    let uart0 = Uart::new(0x1001_3000 as *mut usize);
    uart0.rxctrl().set_rxen(1); // enable uart0 rx channel
    uart0.txctrl().set_txen(1); // enable uart0 tx channel
    uart0.ie().set_rxwm(1); // enable uart0 rx interrupts
    let plic = Plic::new(0x0C00_0000 as *mut usize);
    plic.enabled1().set_bit1(1); // enable uart0 interrupts

    let mut mie = Mie::new();
    mie.set_meie(true); // enable external interrupts
    mie.apply();
    let mut mstatus = MStatus::new();
    mstatus.set_mie(true); // enable interrupts
    mstatus.apply();
    let state = State::get();
    forth::init(&mut state.forth.dictionary);
    write!(state.mpu.uart0, "ready\n").unwrap();
    loop {
        wfi();
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    let state = State::get();
    write!(state.mpu.uart0, "Panic: {:?}", info).unwrap_or_else(|_|());
    halt()
}

#[no_mangle]
fn m_trap_handler() -> ! {
    let mcause = MCause::new();
    let interrupt = mcause.is_interrupt();
    let state = State::get();

    if interrupt {

        let uart_interrupt = state.mpu.plic.pending1().bit1() == 1;
        if uart_interrupt {
            let data = state.mpu.uart0.rxdata().data();
            if data == 10 {
                // TODO: Handle errors?
                forth::eval(state);
            } else {
                state.input_buffer.push(data as u8);
            };
            low::asm::mret()
        } else {
            state.mpu.uart0.write_str("Unrecognised interrupt").unwrap();
            halt()
        }
    } else {
        let code = mcause.code();
        if code <= 2 { // instruction error
            memdump(0, 200 * 1024);
        }
        state.mpu.uart0.write_str("Not an interrupt").unwrap();
        halt()
    }
}
