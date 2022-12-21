use crate::collections::{Map, Vec};
use alloc::boxed::Box;
use crate::low::uart::Uart;
use core::fmt::Write;
use crate::state::State;

type Stack = Vec<u32>;
type Data = Vec<u8>;

pub trait Word {
    fn word(&self) -> &str;
    fn execute(&self, stack: &mut Stack, data: &mut Data) -> ();
}

enum BuiltinWord {
    Store,
    Fetch,
    Comma,
    Align,
    Cells,
    CStore,
    CComma,
    CFetch,
    Chars,
    Plus,
    Minus,
    Star,
    Slash,
    Dot
}

impl Word for BuiltinWord {
    fn word(&self) -> &str {
        use BuiltinWord::*;
        match self {
            Store => "!",
            Fetch => "@",
            Comma => ",",
            Align => "ALIGN",
            Cells => "CELLS",
            CStore => "C!",
            CComma => "C,",
            CFetch => "C@",
            Chars => "CHARS",
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Dot => "."
        }
    }

    fn execute(&self, stack: &mut Stack, data: &mut Data) {
        use BuiltinWord::*;
        match self {
            Store => {
                // TODO: Handle errors
                let x = stack.pop().unwrap();
                let addr = stack.pop().unwrap() as *mut u32;
                unsafe { *addr = x };
            },
            Fetch => {
                let addr = stack.pop().unwrap() as *mut u32;
                stack.push( unsafe { *addr });
            },
            Comma => {
                let x = stack.pop().unwrap();
                let x: [u8;4] = unsafe { core::mem::transmute(x) };
                for &v in x.iter() {
                    data.push(v);
                }
            },
            Align => {
                let out = data.len() % 4;
                if out != 0 {
                    data.push(0);
                }
            },
            Cells => {
                let x = stack.pop().unwrap();
                stack.push(x * 4);
            },
            CStore => {
                let x = stack.pop().unwrap() as u8;
                let addr = stack.pop().unwrap() as *mut u8;
                unsafe { *addr = x };
            }
            CComma => {
                let x = stack.pop().unwrap() as u8;
                data.push(x);
            }
            CFetch => {
                let addr = stack.pop().unwrap() as *const u8;
                stack.push( unsafe { *addr as u32 });
            }
            Chars => {
                // No OP (is the same value)
            }
            Plus => {
                let op1 = stack.pop().unwrap();
                let op2 = stack.pop().unwrap();
                stack.push(op1 + op2);
            }
            Minus => {
                let op1 = stack.pop().unwrap();
                let op2 = stack.pop().unwrap();
                stack.push(op1 - op2);
            }
            Star => {
                let op1 = stack.pop().unwrap();
                let op2 = stack.pop().unwrap();
                stack.push(op1 * op2);
            }
            Slash => {
                let op1 = stack.pop().unwrap();
                let op2 = stack.pop().unwrap();
                stack.push(op1 / op2);
            }
            Dot  => {
                let mut uart0 = Uart::new(0x1001_3000 as *mut usize);
                let v = stack.pop().unwrap();
                write!(uart0, "{}\n", v).unwrap();
            }
        }
    }
}

pub fn init(dic: &mut Map<&str, Box<dyn Word>>) {
    use BuiltinWord::*;
    dic.insert(Store.word(), Box::new(Store));
    dic.insert(Fetch.word(), Box::new(Fetch));
    dic.insert(Comma.word(), Box::new(Comma));
    dic.insert(Align.word(), Box::new(Align));
    dic.insert(Cells.word(), Box::new(Cells));
    dic.insert(CStore.word(), Box::new(CStore));
    dic.insert(CFetch.word(), Box::new(CFetch));
    dic.insert(CComma.word(), Box::new(CComma));
    dic.insert(Chars.word(), Box::new(Chars));
    dic.insert(Plus.word(), Box::new(Plus));
    dic.insert(Minus.word(), Box::new(Minus));
    dic.insert(Star.word(), Box::new(Star));
    dic.insert(Slash.word(), Box::new(Slash));
    dic.insert(Dot.word(), Box::new(Dot));
}

pub fn eval(state: &mut State) {
    let input = core::str::from_utf8(&state.input_buffer).unwrap();
    if let Some(v) = state.forth.dictionary.get(input) {
        v.execute(&mut state.forth.stack, &mut state.forth.data_space);
    }
    else if let Ok(n) = input.parse::<u32>() {
        state.forth.stack.push(n);
    } else {
        write!(state.mpu.uart0, "unrecognised input\n").unwrap();
    }
    state.input_buffer.purge();
}
