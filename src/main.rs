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

use low::asm::{halt, wfi};
use low::csr::{MCause, MStatus, Mie};
use low::plic::Plic;
use low::uart::Uart;
use core::panic::PanicInfo;
use state::State;

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
    loop {
        wfi();
    }
}

#[panic_handler]
fn panic_handler(_arg: &PanicInfo) -> ! {
    halt()
}

#[no_mangle]
fn m_trap_handler() {
    let interrupt = MCause::new().is_interrupt();
    if interrupt {
        let uart0 = Uart::new(0x1001_3000 as *mut usize);
        let plic = Plic::new(0x0C00_0000 as *mut usize);

        let uart_interrupt = plic.pending1().bit1() == 1;
        if uart_interrupt {
            let state = State::get();
            let data = uart0.rxdata().data();
            if data == 10 {
                while let Some(data) = state.input_buffer.pop() {
                    uart0.txdata().set_data(data as usize);
                }
                uart0.txdata().set_data(10);
            } else {
                let data = if data >= 97 && data <= 122 {
                    data - 32
                } else {
                    data
                };
                state.input_buffer.push(data as u8);
                uart0.txdata().set_data(data as usize);
            }
        }
    } else {
        halt();
    }
}
