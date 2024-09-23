use std::ffi::c_void;
use std::fmt;

#[derive(Clone, Copy)]
pub union Word {
    pub as_usize: usize,
    pub as_u64: u64,
    pub as_i64: i64,
    pub as_f64: f64,
    pub as_ptr: *const c_void,
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(f, "u: {}, i: {}, f: {:.10}, ptr: {:p}", self.as_u64, self.as_i64, self.as_f64, self.as_ptr)
        }
    }
}

pub const MEMORY_CAPACITY: usize = 640 * 1000;

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
pub const SYSF: &str = "sysf";

// Stack opcodes
pub const PUSH: &str = "push";
pub const ADDS: &str = "adds";
pub const SUBS: &str = "subs";
pub const MULS: &str = "muls";
pub const DIVS: &str = "divs";
pub const EQUALS: &str = "eqs";
pub const DUPL: &str = "dupl";
pub const JTS: &str = "jts";
pub const JZS: &str = "jzs";
pub const JNZS: &str = "jnzs";

pub const SWC: &str = "swc";


// Universal opcode
pub const JMP: &str = "jmp";
pub const CALL: &str = "call";
pub const READ: &str = "rd";
pub const WRITE: &str = "wrt";
pub const AND: &str = "and";
pub const OR: &str = "or";
pub const XOR: &str = "xor";
pub const SHR: &str = "shr";
pub const SHL: &str = "shl";
pub const NOT: &str = "not";
pub const POP: &str = "pop";
pub const RET: &str = "ret";
pub const HLT: &str = "hlt";

// Deprecated
pub const PHSR: &str = "phsr";