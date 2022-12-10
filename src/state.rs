use crate::collections::Vec;

#[link_section = ".bbs"]
static mut STATE: Option<State> = None;

pub struct State {
    pub input_buffer: Vec<u8>,
}

impl State {
    pub fn get() -> &'static mut Self {
        unsafe {
            if let Some(state) = &mut STATE {
                state
            } else {
                let state = State {
                    input_buffer: Vec::new(),
                };
                STATE = Some(state);
                STATE.as_mut().unwrap()
            }
        }
    }
}
