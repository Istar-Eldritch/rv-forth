use core::arch::asm;
use register_interface::*;

#[link_section = ".bss"]
static mut MEPC: usize = 0;
#[link_section = ".bss"]
static mut MTVAL: usize = 0;

//#[field(mie, 3, 3)]
//#[field(mpie, 7, 7)]
//#[field(mpp, 11, 12)]
pub struct MStatus(usize);

impl MStatus {
    pub fn new() -> Self {
        let mut mstatus = MStatus(0);
        mstatus.reload();
        mstatus
    }

    pub fn reload(&mut self) {
        unsafe {
            asm!("csrr {s}, mstatus", s = out(reg) self.0);
        };
    }

    pub fn apply(&mut self) {
        unsafe {
            asm!("csrrw x0, mstatus, {s}", s = in(reg) self.0);
        };
    }

    pub fn set_mie(&mut self, v: bool) {
        if v {
            self.0 = self.0 | (1 << 3);
        } else {
            self.0 = self.0 & !(1 << 3);
        }
    }

    pub fn mie(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    pub fn set_mpie(&mut self, v: bool) {
        if v {
            self.0 = self.0 | (1 << 7);
        } else {
            self.0 = self.0 & !(1 << 7);
        }
    }

    pub fn mpie(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    pub fn set_mpp(&mut self, v: usize) {
        let v = v & 0b11;
        self.0 = (self.0 & !(11 << 11)) | (v << 11);
    }

    pub fn mpp(&self) -> usize {
        (self.0 >> 11) & 0b11
    }
}

// #[field(msie, 3, 3)]
// #[field(mtie, 7, 7)]
// #[field(meie, 11, 11)]
pub struct Mie(usize);
impl Mie {
    pub fn new() -> Self {
        unsafe {
            let mut mie = Mie(0);
            mie.reload();
            mie
        }
    }

    pub fn reload(&mut self) {
        unsafe { asm!("csrr {m}, mie", m = out(reg) self.0) };
    }

    pub fn apply(&mut self) {
        unsafe {
            asm!("csrrw x0, mie, {m}", m = in(reg) self.0);
        }
    }

    pub fn set_msie(&mut self, v: bool) {
        if v {
            self.0 = self.0 | (1 << 3);
        } else {
            self.0 = self.0 & !(1 << 3);
        }
    }

    pub fn msie(&self) -> bool {
        self.0 & (1 << 3) != 0
    }

    pub fn set_mtie(&mut self, v: bool) {
        if v {
            self.0 = self.0 | (1 << 7);
        } else {
            self.0 = self.0 & !(1 << 7);
        }
    }

    pub fn mtie(&self) -> bool {
        self.0 & (1 << 7) != 0
    }

    pub fn set_meie(&mut self, v: bool) {
        if v {
            self.0 = self.0 | (1 << 11);
        } else {
            self.0 = self.0 & !(1 << 11);
        }
    }

    pub fn meie(&self) -> bool {
        self.0 & (1 << 11) != 0
    }
}

// #[field(code, 0, 9)]
// #[field(interrupt, 31, 31)]
pub struct MCause(usize);

impl MCause {
    pub fn new() -> Self {
        let mut mcause = MCause(0);
        mcause.reload();
        mcause
    }

    pub fn reload(&mut self) {
        unsafe { asm!("csrr {m}, mcause", m = out(reg) self.0) };
    }

    pub fn code(&self) -> usize {
        self.0 & 0x3ff
    }

    pub fn is_interrupt(&self) -> bool {
        self.0 & 0x8000_0000 != 0
    }
}
