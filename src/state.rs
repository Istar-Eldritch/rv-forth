use crate::collections::{Vec, Map};
use crate::forth::Word;
use alloc::boxed::Box;
use crate::low::uart::Uart;
use crate::low::plic::Plic;

#[link_section = ".bbs"]
static mut STATE: Option<State> = None;

pub struct MPU {
    pub uart0: Uart,
    pub plic: Plic
}

pub struct ForthState {
    pub dictionary: Map<&'static str, Box<dyn Word>>,
    pub data_space: Vec<u8>,
    pub stack: Vec<u32>,
}

pub struct State {
    pub input_buffer: Vec<u8>,
    pub mpu: MPU,
    pub forth: ForthState,
}

impl State {
    pub fn get() -> &'static mut Self {
        unsafe {
            if let Some(state) = &mut STATE {
                state
            } else {
                let state = State {
                    input_buffer: Vec::new(),
                    forth: ForthState {
                        dictionary: Map::new(),
                        data_space: Vec::new(),
                        stack: Vec::new(),
                    },
                    mpu: MPU {
                        uart0: Uart::new(0x1001_3000 as *mut usize),
                        plic: Plic::new(0x0C00_0000 as *mut usize),
                    }
                };
                STATE = Some(state);
                STATE.as_mut().unwrap()
            }
        }
    }
}
