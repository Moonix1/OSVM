use crate::defines::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OpcodeType {
    Nop,
    
    // Register opcodes
    Mov,
    
    Clr,
    
    Add,
    Sub,
    Mul,
    Div,
    
    Equal,
    
    Jt,
    Jz,
    
    // Stack opcodes
    Push,
    Pop,
    
    Dupl,
    
    Adds,
    Subs,
    Muls,
    Divs,
    
    Equals,
    
    Jts,
    Jzs,
    
    // Universal opcode
    Jmp,
    
    Hlt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opcode {
    pub op_type: OpcodeType,
    pub op_operand: Option<Word>,
    
    pub op_regs: Vec<String>,
}

impl Opcode {
    pub fn init() -> Opcode {
        Opcode {
            op_type: OpcodeType::Nop,
            op_operand: Some(0),
            op_regs: Vec::new(),
        }
    }
}