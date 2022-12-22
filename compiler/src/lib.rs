#![no_std]

mod hash;
mod primitives;

use core::hash::{Hash, Hasher};
use hash::DJB2;
use primitives::Primitive;

pub struct CompiledWord<'a> {
    pub len: usize,
    pub pos: usize,
    pub name: &'a str,
    pub original: &'a str,
}

impl<'a> CompiledWord<'a> {
    pub fn new(name: &'a str, original: &'a str, len: usize) -> Self {
        CompiledWord {
            name,
            original,
            pos: 0,
            len,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

pub struct ForthDictionary<'a> {
    len: usize,
    keys: &'a mut [CompiledWord<'a>],
    mem_len: usize,
    memory: &'a mut [u32],
}

impl<'a> ForthDictionary<'a> {
    pub fn new(
        len: usize,
        keys: &'a mut [CompiledWord<'a>],
        mem_len: usize,
        memory: &'a mut [u32],
    ) -> Self {
        ForthDictionary {
            len,
            keys,
            mem_len,
            memory,
        }
    }

    fn get(&self, key: u32) -> Option<(&CompiledWord<'a>, &[u32])> {
        // TODO: Perform binary search
        for word in self.keys.iter() {
            if key == get_hash(word.name) {
                return Some((word, &self.memory[word.pos..word.pos + word.len]));
            }
        }
        None
    }

    fn insert(&mut self, mut word: CompiledWord<'a>, instructions: &[u32]) {
        // TODO: Use binary search to insert compiledword.
        word.pos = self.mem_len;
        for i in 0..instructions.len() {
            self.memory[word.pos + i] = instructions[i];
        }
        self.mem_len += word.len;
        self.keys[self.len] = word;
        self.len += 1;
    }
}

fn get_hash(s: &str) -> u32 {
    let mut hasher = DJB2::new();
    s.hash(&mut hasher);
    hasher.finish() as u32
}

pub struct ForthCompiler<'a> {
    dictionary: ForthDictionary<'a>,
}

pub enum CompilerError {
    WordOutOfBounds,
    MalformedCompilation,
    UnrecognizedToken,
}

impl<'a> ForthCompiler<'a> {
    const COMPILE_BUFFER_SIZE: usize = 255;
    pub fn compile(
        &mut self,
        code: &'a str,
        output: &'a mut [u32],
    ) -> Result<usize, CompilerError> {
        let mut split = code.split_ascii_whitespace().peekable();
        let mut alloc_ptr = 0;
        let mut code_idx = 0;

        let mut compiling = false;
        let mut compiling_from = 0;
        let mut compiling_instruction_idx = 0;
        let mut compiling_instructions = [0; Self::COMPILE_BUFFER_SIZE];
        let mut name: &str = "";
        while let Some(token) = split.next() {
            if token == ":" {
                if let Some(_name) = split.peek() {
                    name = _name;
                    compiling = true;
                    compiling_from = code_idx;
                    compiling_instruction_idx = 0;
                } else {
                    return Err(CompilerError::MalformedCompilation);
                }
            } else if token == ";" {
                let compiled_word = CompiledWord::new(
                    name,
                    &code[compiling_from..code_idx],
                    compiling_instruction_idx,
                );

                self.dictionary
                    .insert(compiled_word, &compiling_instructions);
            } else if let Ok(primitive) = Primitive::try_from(token) {
                let (len, instructions) = primitive.get_instructions();
                for idx in 0..len {
                    let instruction = instructions[idx];
                    if compiling {
                        if compiling_instruction_idx >= Self::COMPILE_BUFFER_SIZE {
                            return Err(CompilerError::WordOutOfBounds);
                        }
                        compiling_instructions[compiling_instruction_idx] = instruction;
                        compiling_instruction_idx += 1;
                    } else {
                        output[alloc_ptr] = instruction;
                        alloc_ptr += 1;
                    }
                }
            } else if let Some((_word, compiled)) = self.dictionary.get(get_hash(token)) {
                // XXX: We need to add some logic to decide when to branch vs when to append, For now we always clone, which will likely produce masive binaries
                for w in compiled.iter() {
                    if compiling {
                        if compiling_instruction_idx >= Self::COMPILE_BUFFER_SIZE {
                            return Err(CompilerError::WordOutOfBounds);
                        }
                        compiling_instructions[compiling_instruction_idx] = *w;
                        compiling_instruction_idx += 1;
                    } else {
                        output[alloc_ptr] = *w;
                        alloc_ptr += 1;
                    }
                }
            } else if let Ok(n) = token.parse::<u32>() {
                let (len, instructions) = Primitive::Push(n).get_instructions();
                for idx in 0..len {
                    let instruction = instructions[idx];
                    if compiling {
                        if compiling_instruction_idx >= Self::COMPILE_BUFFER_SIZE {
                            return Err(CompilerError::WordOutOfBounds);
                        }
                        compiling_instructions[compiling_instruction_idx] = instruction;
                        compiling_instruction_idx += 1;
                    } else {
                        output[alloc_ptr] = instruction;
                        alloc_ptr += 1;
                    }
                }
            } else {
                return Err(CompilerError::UnrecognizedToken);
            }
            code_idx += 1;
        }
        Ok(alloc_ptr)
    }
}
