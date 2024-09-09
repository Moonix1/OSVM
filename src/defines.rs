use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Word {
    U64(u64),
    I64(i64),
    F64(f64),
}

impl Word {
    pub fn to_usize(&self) -> usize {
        match self {
            Word::U64(value) => *value as usize,
            Word::I64(value) => Word::U64(*value as u64).to_usize(),
            Word::F64(value) => Word::U64(*value as u64).to_usize(),
        }
    }
    
    pub fn to_u64(&self) -> u64 {
        match self {
            Word::U64(value) => *value,
            Word::I64(value) => Word::U64(*value as u64).to_u64(),
            Word::F64(value) => Word::U64(*value as u64).to_u64(),
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Word::I64(value) => *value,
            Word::U64(value) => Word::I64(*value as i64).to_i64(),
            Word::F64(value) => Word::I64(*value as i64).to_i64(),
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Word::F64(value) => *value,
            Word::I64(value) => Word::F64(*value as f64).to_f64(),
            Word::U64(value) => Word::F64(*value as f64).to_f64(),
        }
    }
}

// Register Names
pub const R0: &str = "r0";
pub const R1: &str = "r1";
pub const R2: &str = "r2";
pub const R3: &str = "r3";
pub const R4: &str = "r4";
pub const R5: &str = "r5";
pub const R6: &str = "r6";
pub const R7: &str = "r7";
pub const R8: &str = "r8";
pub const R9: &str = "r9";
pub const R10: &str = "r10";
pub const R11: &str = "r11";
pub const R12: &str = "r12";
pub const R13: &str = "r13";
pub const R14: &str = "r14";
pub const R15: &str = "r15";
pub const R16: &str = "r16";

// Special Characters
pub const CONST: &str = "#";
pub const GSI: &str = "$";

// Opcode Names

// Register opcodes
pub const MOV: &str = "mov";
pub const SRG: &str = "srg";
pub const CLR: &str = "clr";
pub const ADD: &str = "add";
pub const SUB: &str = "sub";
pub const MUL: &str = "mul";
pub const DIV: &str = "div";
pub const DEC: &str = "dec";
pub const INC: &str = "inc";
pub const EQUAL: &str = "eq";
pub const JT: &str = "jt";
pub const JZ: &str = "jz";
pub const JNZ: &str = "jnz";

// Stack opcodes
pub const PUSH: &str = "push";
pub const POP: &str = "pop";
pub const ADDS: &str = "adds";
pub const SUBS: &str = "subs";
pub const MULS: &str = "muls";
pub const DIVS: &str = "divs";
pub const EQUALS: &str = "eqs";
pub const DUPL: &str = "dupl";
pub const JTS: &str = "jts";
pub const JZS: &str = "jzs";
pub const JNZS: &str = "jzs";

pub const SWC: &str = "swc";

// Universal opcode
pub const JMP: &str = "jmp";
pub const HLT: &str = "hlt";

// Deprecated
pub const PHSR: &str = "phsr";