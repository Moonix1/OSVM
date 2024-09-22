use crate::utils::defines::*;

#[derive(Debug, Clone, Copy)]
pub enum OpcodeType {
    Nop,
    
    // Register opcodes
    Mov,
    Movfs,
    
    Srg,
    
    Clr,
    
    Add,
    Sub,
    Mul,
    Div,
    
    Dec,
    Inc,
    
    Equal,
    
    Jt,
    Jz,
    Jnz,
    
    Sysf,
    
    // Stack opcodes
    Push,
    
    Dupl,
    
    Adds,
    Subs,
    Muls,
    Divs,
    
    Equals,
    
    Jts,
    Jzs,
    Jnzs,
    
    Swc,
    
    // Universal opcode
    Jmp,
    Call,
    
    Pop,
    
    Ret,
    Hlt,
    
    // Deprecated
    Phsr,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Opcode {
    pub op_type: OpcodeType,
    pub op_operand: Option<Word>,
    
    pub op_regs: Vec<String>,
}

impl Opcode {
    pub fn init() -> Opcode {
        Opcode {
            op_type: OpcodeType::Nop,
            op_operand: Some(Word { as_u64: 0 }),
            op_regs: Vec::new(),
        }
    }
}