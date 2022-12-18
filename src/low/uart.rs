#![allow(dead_code)]
use register_interface::*;

#[register(txdata, TxData, 0x0)]
#[register(rxdata, RxData, 0x4)]
#[register(txctrl, TxCtrl, 0x08)]
#[register(rxctrl, RxCtrl, 0x0C)]
#[register(ie, InterruptRegister, 0x10)]
#[register(ip, InterruptRegister, 0x14)]
#[register(div, Div, 0x18)]
#[derive(Clone)]
pub struct Uart {
    addr: *mut usize,
}

impl Uart {
    pub fn new(addr: *mut usize) -> Self {
        Uart { addr }
    }
}

#[field(data, 0, 7)]
#[field(full, 31, 31)]
pub struct TxData {
    addr: *mut usize,
}

#[field(data, 0, 7)]
#[field[empty, 31, 31]]
#[field(all, 0, 31)]
pub struct RxData {
    addr: *mut usize,
}

#[field(txen, 0, 0)]
#[field(nxtop, 1, 1)]
#[field(txcnt, 16, 18)]
pub struct TxCtrl {
    addr: *mut usize,
}

#[field(rxen, 0, 0)]
#[field(rxcnt, 16, 18)]
pub struct RxCtrl {
    addr: *mut usize,
}

#[field(txwm, 0, 0)]
#[field(rxwm, 1, 1)]
pub struct InterruptRegister {
    addr: *mut usize,
}

#[field(div, 0, 15)]
pub struct Div {
    addr: *mut usize,
}
