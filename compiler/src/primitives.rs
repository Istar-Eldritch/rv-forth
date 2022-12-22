use riscv_isa_types::format::*;
use riscv_isa_types::rv32i::RV32i;

pub enum Primitive {
    Load,
    Fetch,
    Push(u32),
    LShift,
    RShift,
    Add,
    Sub,
    And,
    Xor,
    Or,
    Eq,
    Gt,
    Lt,
    Branch,
    RFrom,
    RTo,
}

impl Primitive {
    pub fn get_instructions(&self) -> (usize, [u32; 8]) {
        use Primitive::*;
        match self {
            Load => (
                4,
                [
                    0x03254100, // lw a0, 4(sp)   # load addr
                    0x83258100, // lw a1, 8(sp)   # load value
                    0x13018100, // addi sp, sp, 8 # move stack pt 2 cells up
                    0x2320b500, // sw a1, 0(a0)   # write to memory(addr) the value
                    0, 0, 0, 0,
                ],
            ),
            Fetch => (
                3,
                [
                    0x03254100, // lw a0, 4(sp) # Load addr
                    0x03250500, // lw a0, 0(a0) # Load value at memory(addr)
                    0x2322a100, // sw a0, 4(sp) # Push value on stack
                    0, 0, 0, 0, 0,
                ],
            ),
            LShift => (
                5,
                [
                    0x03254100, // lw a0, 4(sp)    # load amount
                    0x83258100, // lw a1, 8(sp)    # load value
                    0x3395a500, // sll a0, a1, a0  # left logical shift value << amount
                    0x13014100, // addi sp, sp, 4  # reduce stack size by one cell
                    0x2322a100, // sw a0, 4(sp)    # store shifted value to stack
                    0, 0, 0,
                ],
            ),
            RShift => (
                5,
                [
                    0x03254100, // lw a0, 4(sp)    # load amount
                    0x83258100, // lw a1, 8(sp)    # load value
                    0x33d5a500, // srl a0, a1, a0  # right logical shift value >> amount
                    0x13014100, // addi sp, sp, 4  # reduce stack size by one cell
                    0x2322a100, // sw a0, 4(sp)    # store shifted value to stack
                    0, 0, 0,
                ],
            ),
            Add => (
                5,
                [
                    0x03254100, // lw a0, 4(sp)   # load operands
                    0x83258100, // lw a1, 8(sp)
                    0x3385a500, // add a0, a1, a0 # perform operation
                    0x13014100, // addi sp, sp, 4 # reduce stack size by one cell
                    0x2322a100, // sw a0, 4(sp)   # store result to stack
                    0, 0, 0,
                ],
            ),
            Sub => (
                5,
                [
                    0x03254100, // lw a0, 4(sp)   # load operands
                    0x83258100, // lw a1, 8(sp)
                    0x3385a540, // sub a0, a1, a0 # perform operation
                    0x13014100, // addi sp, sp, 4 # reduce stack size by one cell
                    0x2322a100, // sw a0, 4(sp)   # store result to stack
                    0, 0, 0,
                ],
            ),
            Xor => (
                5,
                [
                    0x03254100, // lw a0, 4(sp)   # load operands
                    0x83258100, // lw a1, 8(sp)
                    0x33c5a500, // xor a0, a1, a0 # perform operation
                    0x13014100, // addi sp, sp, 4 # reduce stack size by one cell
                    0x2322a100, // sw a0, 4(sp)   # store result to stack
                    0, 0, 0,
                ],
            ),
            Or => (
                5,
                [
                    0x03254100, // lw a0, 4(sp)   # load operands
                    0x83258100, // lw a1, 8(sp)
                    0x33e5a500, // or a0, a1, a0  # perform operation
                    0x13014100, // addi sp, sp, 4 # reduce stack size by one cell
                    0x2322a100, // sw a0, 4(sp)   # store result to stack
                    0, 0, 0,
                ],
            ),
            And => (
                5,
                [
                    0x03254100, // lw a0, 4(sp)   # load operands
                    0x83258100, // lw a1, 8(sp)
                    0x33f5a500, // and a0, a1, a0 # perform operation
                    0x13014100, // addi sp, sp, 4 # reduce stack size by one cell
                    0x2322a100, // sw a0, 4(sp)   # store result to stack
                    0, 0, 0,
                ],
            ),
            Eq => (
                8,
                [
                    0x03254100, // lw a0, 4(sp)    # load left
                    0x83258100, // lw a1, 8(sp)    # load right
                    0x13014100, // addi sp, sp, 4  # reduce stack size by one cell
                    0x3345b500, // xor a0, a0, a1  # perform eq checks
                    0x13351500, // seqz a0, a0
                    0x1315f501, // slli a0, a0, 31 # sext
                    0x1355f541, // srai a0, a0, 31
                    0x2322a100, // sw a0, 4(sp)    # store result to stack
                ],
            ),
            Lt => (
                7,
                [
                    0x03254100, // lw a0, 4(sp)     # load right
                    0x83258100, // lw a1, 8(sp)     # load left
                    0x13014100, // addi sp, sp, 4   # reduce stack size by one cell
                    0x33a5a500, // slt a0, a1, a0   # check less than
                    0x1315f501, // slli a0, a0, 31  # sext
                    0x1355f541, // srai a0, a0, 31
                    0x2320a100, // sw a0, 0(sp)     # store result to stack
                    0,
                ],
            ),
            Gt => (
                7,
                [
                    0x03254100, // lw a0, 4(sp)     # load right
                    0x83258100, // lw a1, 8(sp)     # load left
                    0x13014100, // addi sp, sp, 4   # reduce stack size by one cell
                    0x3325b500, // slt a0, a0, a1   # check greater than
                    0x1315f501, // slli a0, a0, 31  # sext
                    0x1355f541, // srai a0, a0, 31
                    0x2320a100, // sw a0, 0(sp)     # store result to stack
                    0,
                ],
            ),
            Branch => (
                7,
                [
                    0x13014100, // addi sp, sp, 4   # load address
                    0x03250100, // lw a0, 0(sp)
                    0x23201400, // sw x1, 0(fp)     # add return pt to Rstack
                    0x1304c4ff, // addi fp, fp, -4
                    0xe7000500, // jalr x1, 0(a0)   # jump and link
                    0x13044400, // addi fp, fp, 4   # recover return pt from Rstack
                    0x83200400, // lw x1, 0(fp)
                    0,
                ],
            ),
            RTo => (
                4,
                [
                    0x03254100, // lw a0, 4(sp)     # load value from stack
                    0x13014100, // addi sp, sp, 4   # reduce stack size by one cell
                    0x2320a400, // sw a0, 0(fp)     # add value to return stack
                    0x1304c4ff, // addi fp, fp, -4  # increase return stack size by one cell
                    0, 0, 0, 0,
                ],
            ),
            RFrom => (
                4,
                [
                    0x03254400, // lw a0, 4(fp)     # load value from return stack
                    0x13044400, // addi fp, fp, 4   # reduce Rstack by one cell
                    0x2320a100, // sw a0, 0(sp)     # add value to data stack
                    0x1301c1ff, // addi sp, sp, -4  # inscrease data stack size by one cell
                    0, 0, 0, 0,
                ],
            ),
            Push(v) => {
                if *v < 0xfff {
                    let addi_format = IFormat {
                        funct3: 0b000,
                        imm: *v,
                        op: 0b0010011,
                        rd: 10, // x10|a0
                        rs1: 0, // x0|zero
                    };
                    let addi = RV32i::ADDI(addi_format);
                    let addi: u32 = addi.into();
                    (
                        3,
                        [
                            addi,
                            0x2320a100, // sw a0, 0(sp)     # store the result in the stack
                            0x1301c1ff, // addi sp, sp, -4  # inscrease data stack size by one cell
                            0, 0, 0, 0, 0,
                        ],
                    )
                } else {
                    // lui + addi
                    let vu = v >> 12;
                    let vl = (v << 20) >> 20;
                    let lui_format = UFormat {
                        op: 0b0110111,
                        imm: vu,
                        rd: 10,
                    };
                    let lui = RV32i::LUI(lui_format);
                    let addi_format = IFormat {
                        funct3: 0b000,
                        imm: vl,
                        op: 0b0010011,
                        rd: 10, // x10|a0
                        rs1: 0, // x0|zero
                    };
                    let addi = RV32i::ADDI(addi_format);
                    (
                        4,
                        [
                            lui.into(),
                            addi.into(),
                            0x2320a100, // sw a0, 0(sp)     # store the result in the stack
                            0x1301c1ff, // addi sp, sp, -4  # inscrease data stack size by one cell
                            0,
                            0,
                            0,
                            0,
                        ],
                    )
                }
            }
        }
    }
}

impl TryFrom<&str> for Primitive {
    type Error = ();
    fn try_from(s: &str) -> Result<Primitive, ()> {
        use Primitive::*;
        match s {
            "!" => Ok(Load),
            "@" => Ok(Fetch),
            "<<" => Ok(LShift),
            ">>" => Ok(RShift),
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "XOR" => Ok(Xor),
            "OR" => Ok(Or),
            "AND" => Ok(And),
            "=" => Ok(Eq),
            ">" => Ok(Gt),
            "<" => Ok(Lt),
            "BRANCH" => Ok(Branch),
            "R<" => Ok(RTo),
            "R>" => Ok(RFrom),
            _ => Err(()),
        }
    }
}
