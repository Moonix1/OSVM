#![allow(unused, dead_code)]

use crate::preprocessor;

use crate::utils::defines;
use crate::utils::error;
use crate::utils::file;

use crate::oasm;
use crate::opcode;
use crate::utils::sys_functions::SysFunction;
use crate::utils::sys_functions::SystemFunctions;

use std::{
    alloc::alloc,
    env,
    ffi::{c_void, CString},
    fs::File,
    io::{Read, Write},
    ops::{Add, Deref, Index},
    process::exit
};

use libc::{free, malloc};

use std::io::stdin;

use preprocessor::*;

use defines::*;
use oasm::*;
use opcode::*;
use error::*;
use file::*;

pub struct OSVM {
    // Registers
    r0: Word,
    r1: Word,
    r2: Word,
    r3: Word,
    r4: Word,
    r5: Word,
    r6: Word,
    r7: Word,
    r8: Word,
    r9: Word,
    r10: Word,
    r11: Word,
    r12: Word,
    r13: Word,
    r14: Word,
    r15: Word,
    r16: Word,
    
    pub tsr: usize,
    rspc: usize,
    pc: usize,

    // Stack
    pub stack: Vec<Word>,
    
    // Other
    pub program: Vec<Opcode>,
    pub sys_functions: Vec<SysFunction>,
    
    halt: bool,
}

impl OSVM {
    pub fn init() -> OSVM {
        OSVM {
            // Registers
            r0: Word { as_u64: 0 },
            r1: Word { as_u64: 0 },
            r2: Word { as_u64: 0 },
            r3: Word { as_u64: 0 },
            r4: Word { as_u64: 0 },
            r5: Word { as_u64: 0 },
            r6: Word { as_u64: 0 },
            r7: Word { as_u64: 0 },
            r8: Word { as_u64: 0 },
            r9: Word { as_u64: 0 },
            r10: Word { as_u64: 0 },
            r11: Word { as_u64: 0 },
            r12: Word { as_u64: 0 },
            r13: Word { as_u64: 0 },
            r14: Word { as_u64: 0 },
            r15: Word { as_u64: 0 },
            r16: Word { as_u64: 0 },
            
            tsr: 0,
            rspc: 0,
            pc: 0,
            
            // Stack
            stack: Vec::new(),
            
            // Other
            program: Vec::new(),
            
            sys_functions: Vec::new(),
            
            halt: false,
        }
    }
    
    pub fn init_default_sysf(self: &mut Self) {
        self.sys_functions = vec![
            SystemFunctions::print_ptr,
            SystemFunctions::print_f64,
            SystemFunctions::print_i64,
            SystemFunctions::print_u64,
            SystemFunctions::free,
            SystemFunctions::alloc,
        ]
    }
    
    pub fn assign_register(self: &mut Self, opcode: &Opcode, index: usize, new_value: Word) {
        let reg: String = opcode.op_regs[index].clone();
        match reg.as_str() {
            R0 => {
                self.r0 = new_value;
            }
            R1 => {
                self.r1 = new_value;
            }
            R2 => {
                self.r2 = new_value;
            }
            R3 => {
                self.r3 = new_value;
            }
            R4 => {
                self.r4 = new_value;
            }
            R5 => {
                self.r5 = new_value;
            }
            R6 => {
                self.r6 = new_value;
            }
            R7 => {
                self.r7 = new_value;
            }
            R8 => {
                self.r8 = new_value;
            }
            R9 => {
                self.r9 = new_value;
            }
            R10 => {
                self.r10 = new_value;
            }
            R11 => {
                self.r11 = new_value;
            }
            R12 => {
                self.r12 = new_value;
            }
            R13 => {
                self.r13 = new_value;
            }
            R14 => {
                self.r14 = new_value;
            }
            R15 => {
                self.r15 = new_value;
            }
            R16 => {
                self.r16 = new_value;
            }
            
            _ => {
                
            }
        }
    }
    
    pub fn find_register(self: &mut Self, opcode: &Opcode, index: usize) -> Option<&mut Word> {
        let reg: String = opcode.op_regs[index].clone();
        match reg.as_str() {
            R0 => {
                return Some(&mut self.r0);
            }
            R1 => {
                return Some(&mut self.r1);
            }
            R2 => {
                return Some(&mut self.r2);
            }
            R3 => {
                return Some(&mut self.r3);
            }
            R4 => {
                return Some(&mut self.r4);
            }
            R5 => {
                return Some(&mut self.r5);
            }
            R6 => {
                return Some(&mut self.r6);
            }
            R7 => {
                return Some(&mut self.r7);
            }
            R8 => {
                return Some(&mut self.r8);
            }
            R9 => {
                return Some(&mut self.r9);
            }
            R10 => {
                return Some(&mut self.r10);
            }
            R11 => {
                return Some(&mut self.r11);
            }
            R12 => {
                return Some(&mut self.r12);
            }
            R13 => {
                return Some(&mut self.r13);
            }
            R14 => {
                return Some(&mut self.r14);
            }
            R15 => {
                return Some(&mut self.r15);
            }
            R16 => {
                return Some(&mut self.r16);
            }
            
            _  => {
                None
            }
        }
    }
    
    pub fn set_tsr(self: &mut Self, value: Word) {
        match value {
            Word { as_u64: _ } => self.tsr = 0,
            Word { as_i64: _ } => self.tsr = 1,
            Word { as_f64: _ } => self.tsr = 2,
            
            Word { as_ptr: _ } => self.tsr = 3,
        }
    }
    
    pub fn execute_opcode(self: &mut Self) -> Error {
        if self.pc >= self.program.len() {
            return Error::InvalidOpcodeAccess;
        }
        
        let opcode = self.program[self.pc].clone();
        
        match opcode.op_type {
            OpcodeType::Mov => {
                match opcode.op_operand {
                    None => {
                        if opcode.op_regs.len() < 1 {
                            return Error::RegisterOverflow;
                        } else if opcode.op_regs.len() > 2 {
                            return Error::RegisterUnderflow;
                        }
                        
                        let reg = *self.find_register(&opcode, 1).unwrap();
                        self.set_tsr(reg);
                        self.assign_register(&opcode, 0, reg);
                    }
                    _ => {
                        if opcode.op_regs.len() < 1 {
                            return Error::RegisterOverflow;
                        } else if opcode.op_regs.len() > 1 {
                            return Error::RegisterUnderflow;
                        }
                    
                        self.set_tsr(opcode.op_operand.unwrap());
                        self.assign_register(&opcode, 0, opcode.op_operand.unwrap());
                    }
                }
                self.pc += 1
            }
            OpcodeType::Movfs => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterOverflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterUnderflow;
                }
                
                unsafe {
                    self.set_tsr(self.stack[self.stack.len() - 1 - opcode.op_operand.unwrap().as_usize]);
                    self.assign_register(&opcode, 0, self.stack[self.stack.len() - 1 - opcode.op_operand.unwrap().as_usize]);
                }
                
                self.pc += 1
            }
            
            OpcodeType::Srg => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterOverflow;
                } else if opcode.op_regs.len() > 2 {
                    return Error::RegisterUnderflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                let reg2 = *self.find_register(&opcode, 1).unwrap();
                self.set_tsr(reg1);
                self.assign_register(&opcode, 0, reg2);
                self.assign_register(&opcode, 1, reg1);
                self.pc += 1
            }

            OpcodeType::Add => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                self.set_tsr(reg1);
                unsafe {
                    match self.tsr {
                        0 => {
                            let sum = reg1.as_u64 + reg2.as_u64;
                    
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_u64: sum };
                        }
                        1 => {
                            let sum = reg1.as_i64 + reg2.as_i64;
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_i64: sum };
                        }
                        2 => {
                            let sum = reg1.as_f64 + reg2.as_f64;
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_f64: sum };
                        }
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            OpcodeType::Sub => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                self.set_tsr(reg1);
                unsafe {
                    match self.tsr {
                        0 => {
                            let sum = reg1.as_u64 - reg2.as_u64;
                    
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_u64: sum };
                        }
                        1 => {
                            let sum = reg1.as_i64 - reg2.as_i64;
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_i64: sum };
                        }
                        2 => {
                            let sum = reg1.as_f64 - reg2.as_f64;
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_f64: sum };
                        }
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            OpcodeType::Mul => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                self.set_tsr(reg1);
                unsafe {
                    match self.tsr {
                        0 => {
                            let sum = reg1.as_u64 * reg2.as_u64;
                    
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_u64: sum };
                        }
                        1 => {
                            let sum = reg1.as_i64 * reg2.as_i64;
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_i64: sum };
                        }
                        2 => {
                            let sum = reg1.as_f64 * reg2.as_f64;
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_f64: sum };
                        }
        
                        _ => {}
                    }
                }
                self.pc += 1
            }
            OpcodeType::Div => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                self.set_tsr(reg1);
                unsafe {
                    match self.tsr {
                        0 => {
                            if reg1.as_u64 == 0 || reg2.as_u64 == 0 {
                                return Error::DivByZero;
                            }
    
                            let sum = reg1.as_u64 + reg2.as_u64;
                    
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_u64: sum };
                        }
                        1 => {
                            if reg1.as_i64 == 0 || reg2.as_i64 == 0 {
                                return Error::DivByZero;
                            }
    
                            let sum = reg1.as_i64 + reg2.as_i64;
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_i64: sum };
                        }
                        2 => {
                            if reg1.as_f64 == 0.0 || reg2.as_f64 == 0.0 {
                                return Error::DivByZero;
                            }
    
                            let sum = reg1.as_f64 + reg2.as_f64;
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = Word { as_f64: sum };
                        }
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Dec => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                self.set_tsr(reg1);
                unsafe {
                    match self.tsr {
                        0 => {
                            let reg1 = self.find_register(&opcode, 0).unwrap();
                            *reg1 = Word { as_u64: reg1.as_u64 - 1 };
                        }
                        1 => {
                            let reg1 = self.find_register(&opcode, 0).unwrap();
                            *reg1 = Word { as_i64: reg1.as_i64 - 1 };
                        }
                        2 => {
                            let reg1 = self.find_register(&opcode, 0).unwrap();
                            *reg1 = Word { as_f64: reg1.as_f64 - 1.0 };
                        }
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            OpcodeType::Inc => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                self.set_tsr(reg1);
                unsafe {
                    match self.tsr {
                        0 => {
                            let reg1 = self.find_register(&opcode, 0).unwrap();
                            *reg1 = Word { as_u64: reg1.as_u64 + 1 };
                        }
                        1 => {
                            let reg1 = self.find_register(&opcode, 0).unwrap();
                            *reg1 = Word { as_i64: reg1.as_i64 + 1 };
                        }
                        2 => {
                            let reg1 = self.find_register(&opcode, 0).unwrap();
                            *reg1 = Word { as_f64: reg1.as_f64 + 1.0 };
                        }
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Equal => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                self.set_tsr(reg1);
                unsafe {
                    match self.tsr {
                        0 => {
                            let sum = Word { as_u64: (reg1.as_u64 == reg2.as_u64) as u64 };
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = sum;
                        }
                        1 => {
                            let sum = Word { as_u64: (reg1.as_i64 == reg2.as_i64) as u64 };
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = sum;
                        }
                        2 => {
                            let sum = Word { as_u64: (reg1.as_f64 == reg2.as_f64) as u64 };
                            
                            let dest = self.find_register(&opcode, 0).unwrap();
                            *dest = sum;
                        }
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Jt => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                unsafe {
                    if reg1.as_u64 == 1 {
                        self.pc = opcode.op_operand.unwrap().as_usize;
                    } else {
                        self.pc += 1
                    }
                }
            }
            OpcodeType::Jz => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                unsafe {
                    if reg1.as_u64 == 0 {
                        self.pc = opcode.op_operand.unwrap().as_usize;
                    } else {
                        self.pc += 1
                    }
                }
            }
            OpcodeType::Jnz => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                unsafe {
                    if reg1.as_u64 != 0 {
                        self.pc = opcode.op_operand.unwrap().as_usize;
                    } else {
                        self.pc += 1
                    }
                }
            }
            
            OpcodeType::Sysf => {
                unsafe {
                    if self.r7.as_u64 == 0 {
                        return Error::InvalidSysFunction;
                    }
                    
                    if opcode.op_regs.is_empty() {
                        self.sys_functions[self.sys_functions.len() - self.r7.as_usize](self, &opcode.clone(), Vec::new());
                    } else {
                        self.sys_functions[self.sys_functions.len() - self.r7.as_usize](self, &opcode.clone(), opcode.op_regs);
                    }
                }
                self.pc += 1;
            }
            
            // Stack opcodes
            OpcodeType::Push => {
                match opcode.op_operand {
                    None => {
                        if opcode.op_regs.len() < 1 {
                            return Error::RegisterUnderflow;
                        } else if opcode.op_regs.len() > 1 {
                            return Error::RegisterOverflow;
                        }
                        
                        let reg = *self.find_register(&opcode, 0).unwrap();
                        self.stack.push(reg);
                    }
                    
                    _ => {
                        self.stack.push(opcode.op_operand.unwrap());
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Adds => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.set_tsr(b);
                unsafe {
                    match self.tsr {
                        0 => self.stack.push(Word { as_u64: b.as_u64 + a.as_u64 }),
                        1 => self.stack.push(Word { as_i64: b.as_i64 + a.as_i64 }),
                        2 => self.stack.push(Word { as_f64: b.as_f64 + a.as_f64 }),
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            OpcodeType::Subs => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.set_tsr(b);
                unsafe {
                    match self.tsr {
                        0 => self.stack.push(Word { as_u64: b.as_u64 - a.as_u64 }),
                        1 => self.stack.push(Word { as_i64: b.as_i64 - a.as_i64 }),
                        2 => self.stack.push(Word { as_f64: b.as_f64 - a.as_f64 }),
    
                        _ => {}
                    }
                }
                self.pc += 1;
            }
            
            OpcodeType::Muls => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.set_tsr(b);
                unsafe {
                    match self.tsr {
                        0 => self.stack.push(Word { as_u64: b.as_u64 * a.as_u64 }),
                        1 => self.stack.push(Word { as_i64: b.as_i64 * a.as_i64 }),
                        2 => self.stack.push(Word { as_f64: b.as_f64 * a.as_f64 }),
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            OpcodeType::Divs => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.set_tsr(b);
                unsafe {
                    match self.tsr {
                        0 => self.stack.push(Word { as_u64: b.as_u64 / a.as_u64 }),
                        1 => self.stack.push(Word { as_i64: b.as_i64 / a.as_i64 }),
                        2 => self.stack.push(Word { as_f64: b.as_f64 / a.as_f64 }),
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Equals => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack[self.stack.len() - 1];
                let b = self.stack[self.stack.len() - 2];
                self.set_tsr(b);
                unsafe {
                    match self.tsr {
                        0 => self.stack.push(Word { as_u64: (b.as_u64 == a.as_u64) as u64 }),
                        1 => self.stack.push(Word { as_u64: (b.as_i64 == a.as_i64) as u64 }),
                        2 => self.stack.push(Word { as_u64: (b.as_f64 == a.as_f64) as u64 }),
    
                        _ => {}
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Dupl => {
                unsafe {
                    if self.stack.len() - opcode.op_operand.unwrap().as_usize <= 0 {
                        return Error::StackUnderflow;
                    }
                    
                    self.stack.push(self.stack[self.stack.len() - 1 - opcode.op_operand.unwrap().as_usize]);
                    self.pc += 1
                }
            }
            
            OpcodeType::Jts => {
                if self.stack.len() < 1 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                unsafe {
                    if a.as_u64 == 1 {
                        self.pc = opcode.op_operand.unwrap().as_usize;
                    } else {
                        self.pc += 1
                    }
                }
            }
            OpcodeType::Jzs => {
                if self.stack.len() < 1 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                unsafe {
                    if a.as_u64 == 0 {
                        self.pc = opcode.op_operand.unwrap().as_usize;
                    } else {
                        self.pc += 1;
                    }
                }
            }
            OpcodeType::Jnzs => {
                if self.stack.len() < 1 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                unsafe {
                    if a.as_u64 != 0 {
                        self.pc = opcode.op_operand.unwrap().as_usize;
                    } else {
                        self.pc += 1
                    }
                }
            }
            
            OpcodeType::Swc => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.len() - 1;
                unsafe {
                    let b = self.stack.len() - 1 - opcode.op_operand.unwrap().as_usize;
                    let old = self.stack[a];
                    self.stack[a] = self.stack[b];
                    self.stack[b] = old;
                }
                self.pc += 1
            }
            
            // Universal opcodes
            OpcodeType::Jmp => {
                unsafe {
                    self.pc = opcode.op_operand.unwrap().as_usize;
                }
            }
            OpcodeType::Call => {
                self.rspc = self.pc + 1;
                unsafe {
                    self.pc = opcode.op_operand.unwrap().as_usize;
                }
            }
            
            
            OpcodeType::And => {
                if opcode.op_regs.is_empty() {
                    if self.stack.len() < 2 {
                        return Error::StackUnderflow;
                    }
                    
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.set_tsr(b);
                    unsafe {
                        match self.tsr {
                            0 => self.stack.push(Word { as_u64: b.as_u64 & a.as_u64 }),
                            1 => self.stack.push(Word { as_i64: b.as_i64 & a.as_i64 }),
                            
                            _ => {}
                        }
                    }
                } else {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterUnderflow;
                    } else if opcode.op_regs.len() > 3 {
                        return Error::RegisterOverflow;
                    }
                    
                    let reg1 = *self.find_register(&opcode, 1).unwrap();
                    let reg2 = *self.find_register(&opcode, 2).unwrap();
                    self.set_tsr(reg1);
                    unsafe {
                        match self.tsr {
                            0 => {
                                let sum = reg1.as_u64 & reg2.as_u64;
                        
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_u64: sum };
                            }
                            1 => {
                                let sum = reg1.as_i64 ^ reg2.as_i64;
                                
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_i64: sum };
                            }
        
                            _ => {}
                        }
                    }
                }
                self.pc += 1
            }
            OpcodeType::Or => {
                if opcode.op_regs.is_empty() {
                    if self.stack.len() < 2 {
                        return Error::StackUnderflow;
                    }
                    
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.set_tsr(b);
                    unsafe {
                        match self.tsr {
                            0 => self.stack.push(Word { as_u64: b.as_u64 | a.as_u64 }),
                            1 => self.stack.push(Word { as_i64: b.as_i64 | a.as_i64 }),
                            
                            _ => {}
                        }
                    }
                } else {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterUnderflow;
                    } else if opcode.op_regs.len() > 3 {
                        return Error::RegisterOverflow;
                    }
                    
                    let reg1 = *self.find_register(&opcode, 1).unwrap();
                    let reg2 = *self.find_register(&opcode, 2).unwrap();
                    self.set_tsr(reg1);
                    unsafe {
                        match self.tsr {
                            0 => {
                                let sum = reg1.as_u64 | reg2.as_u64;
                        
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_u64: sum };
                            }
                            1 => {
                                let sum = reg1.as_i64 | reg2.as_i64;
                                
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_i64: sum };
                            }
        
                            _ => {}
                        }
                    }
                }
                self.pc += 1
            }
            OpcodeType::Xor => {
                if opcode.op_regs.is_empty() {
                    if self.stack.len() < 2 {
                        return Error::StackUnderflow;
                    }
                    
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.set_tsr(b);
                    unsafe {
                        match self.tsr {
                            0 => self.stack.push(Word { as_u64: b.as_u64 ^ a.as_u64 }),
                            1 => self.stack.push(Word { as_i64: b.as_i64 ^ a.as_i64 }),
                            
                            _ => {}
                        }
                    }
                } else {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterUnderflow;
                    } else if opcode.op_regs.len() > 3 {
                        return Error::RegisterOverflow;
                    }
                    
                    let reg1 = *self.find_register(&opcode, 1).unwrap();
                    let reg2 = *self.find_register(&opcode, 2).unwrap();
                    self.set_tsr(reg1);
                    unsafe {
                        match self.tsr {
                            0 => {
                                let sum = reg1.as_u64 ^ reg2.as_u64;
                        
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_u64: sum };
                            }
                            1 => {
                                let sum = reg1.as_i64 ^ reg2.as_i64;
                                
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_i64: sum };
                            }
        
                            _ => {}
                        }
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Shr => {
                if opcode.op_regs.is_empty() {
                    if self.stack.len() < 2 {
                        return Error::StackUnderflow;
                    }
                    
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.set_tsr(b);
                    unsafe {
                        match self.tsr {
                            0 => self.stack.push(Word { as_u64: b.as_u64 >> a.as_u64 }),
                            1 => self.stack.push(Word { as_i64: b.as_i64 >> a.as_i64 }),
                            
                            _ => {}
                        }
                    }
                } else {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterUnderflow;
                    } else if opcode.op_regs.len() > 3 {
                        return Error::RegisterOverflow;
                    }
                    
                    let reg1 = *self.find_register(&opcode, 1).unwrap();
                    let reg2 = *self.find_register(&opcode, 2).unwrap();
                    self.set_tsr(reg1);
                    unsafe {
                        match self.tsr {
                            0 => {
                                let sum = reg1.as_u64 >> reg2.as_u64;
                        
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_u64: sum };
                            }
                            1 => {
                                let sum = reg1.as_i64 >> reg2.as_i64;
                                
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_i64: sum };
                            }
        
                            _ => {}
                        }
                    }
                }
                self.pc += 1
            }
            OpcodeType::Shl => {
                if opcode.op_regs.is_empty() {
                    if self.stack.len() < 2 {
                        return Error::StackUnderflow;
                    }
                    
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.set_tsr(b);
                    unsafe {
                        match self.tsr {
                            0 => self.stack.push(Word { as_u64: b.as_u64 << a.as_u64 }),
                            1 => self.stack.push(Word { as_i64: b.as_i64 << a.as_i64 }),
                            
                            _ => {}
                        }
                    }
                } else {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterUnderflow;
                    } else if opcode.op_regs.len() > 3 {
                        return Error::RegisterOverflow;
                    }
                    
                    let reg1 = *self.find_register(&opcode, 1).unwrap();
                    let reg2 = *self.find_register(&opcode, 2).unwrap();
                    self.set_tsr(reg1);
                    unsafe {
                        match self.tsr {
                            0 => {
                                let sum = reg1.as_u64 << reg2.as_u64;
                        
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_u64: sum };
                            }
                            1 => {
                                let sum = reg1.as_i64 << reg2.as_i64;
                                
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_i64: sum };
                            }
        
                            _ => {}
                        }
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Not => {
                if opcode.op_regs.is_empty() {
                    if self.stack.len() < 1 {
                        return Error::StackUnderflow;
                    }
                    
                    let a = self.stack.pop().unwrap();
                    self.set_tsr(a);
                    unsafe {
                        match self.tsr {
                            0 => self.stack.push(Word { as_u64: !a.as_u64 }),
                            1 => self.stack.push(Word { as_i64: !a.as_i64 }),
                            
                            _ => {}
                        }
                    }
                } else {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterUnderflow;
                    } else if opcode.op_regs.len() > 3 {
                        return Error::RegisterOverflow;
                    }
                    
                    let reg1 = *self.find_register(&opcode, 1).unwrap();
                    self.set_tsr(reg1);
                    unsafe {
                        match self.tsr {
                            0 => {
                                let sum = !reg1.as_u64;
                        
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_u64: sum };
                            }
                            1 => {
                                let sum = !reg1.as_i64;
                                
                                let dest = self.find_register(&opcode, 0).unwrap();
                                *dest = Word { as_i64: sum };
                            }
        
                            _ => {}
                        }
                    }
                }
                self.pc += 1
            }
            
            OpcodeType::Pop => {
                if opcode.op_regs.is_empty() {
                    if self.stack.len() < 1 {
                        return Error::StackUnderflow;
                    }
                    
                    self.stack.pop();
                } else {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterOverflow;
                    } else if opcode.op_regs.len() > 1 {
                        return Error::RegisterUnderflow;
                    }
                    
                    self.set_tsr(Word { as_u64: 0 });
                    self.assign_register(&opcode, 0, Word { as_u64: 0 });
                }
                self.pc += 1
            }
            
            OpcodeType::Ret => {
                self.pc = self.rspc;
                self.rspc = 0;
            }
            OpcodeType::Hlt => {
                self.halt = true;
            }
            
            // Deprecated
            OpcodeType::Phsr => {
                if self.stack.len() < 1 {
                    return Error::StackUnderflow;
                }
                
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterOverflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterUnderflow;
                }
                
                self.set_tsr(self.stack[self.stack.len() - 1]);
                self.assign_register(&opcode, 0, self.stack[self.stack.len() - 1]);
                self.pc += 1
            }
            
            _ => {
                return Error::InvalidOperand;
            }
        }
        
        Error::None
    }
    
    fn get_operands<'a>(self: &Self, tokens: Vec<&'a str>, len1: usize, len2: usize, line_num: &usize) -> Vec<&'a str> {
        let mut operands: Vec<&str> = tokens[0].trim().split(", ").collect();
        if operands.len() < len1 || operands.len() > len2 {
            eprintln!("[Error]: Invalid number of opcode arguments at line: {}", line_num);
            exit(1);
        }

        operands
    }
    
    pub fn translate_source(self: &mut Self, mut oasm: OASM, input_path: String, source: String) {
        let preprocessor = Preprocessor {};
        let mut source = preprocessor.process_includes(input_path.clone(), source);
        source = preprocessor.process_source(input_path.clone(), source);
        
        let lines: Vec<&str> = source.lines().collect();
        let mut line_num = 0;
        for line in lines {
            let ic = 0;
            line_num += 1;
            
            let mut tokens: Vec<&str> = line.trim().splitn(2, char::is_whitespace).collect();
            if !tokens.is_empty() && !tokens[0].is_empty() {
                let mut inst_name = tokens[0];
                tokens.remove(0);
                
                if inst_name.ends_with(':') {
                    let label = inst_name.replace(":", "");
                    oasm.labels_push(&label, self.program.len());
                    
                    if tokens.len() > 0 {
                        tokens = tokens[0].trim().splitn(2, char::is_whitespace).collect();
                        inst_name = tokens[0];
                        tokens.remove(0);
                    } else {
                        continue;
                    }
                }
                
                match inst_name {
                    // Register opcodes
                    MOV => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 2, 2, &line_num);
                        
                        if operands[1].starts_with("r") {
                            self.program.push(Opcode { op_type: OpcodeType::Mov, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string()] });
                        } else if operands[1].starts_with(CONST) {
                            if operands[1].replace(CONST, "").parse::<u64>().is_ok() {
                                self.program.push(Opcode { op_type: OpcodeType::Mov, op_operand: Some(Word { as_u64: (operands[1].replace(CONST, "").parse().unwrap()) }), op_regs: vec![operands[0].to_string()] });
                            } else if operands[1].replace(CONST, "").parse::<i64>().is_ok() {
                                self.program.push(Opcode { op_type: OpcodeType::Mov, op_operand: Some(Word { as_i64: (operands[1].replace(CONST, "").parse().unwrap()) }), op_regs: vec![operands[0].to_string()] });
                            } else if operands[1].replace(CONST, "").parse::<f64>().is_ok() {
                                self.program.push(Opcode { op_type: OpcodeType::Mov, op_operand: Some(Word { as_f64: (operands[1].replace(CONST, "").parse().unwrap()) }), op_regs: vec![operands[0].to_string()] });
                            }
                        } else if operands[1].starts_with(GSI) {
                            self.program.push(Opcode { op_type: OpcodeType::Movfs, op_operand: Some(Word { as_u64: (operands[1].replace(GSI, "").parse().unwrap()) }), op_regs: vec![operands[0].to_string()] });
                        } else {
                            eprintln!("[Error]: Invalid operand `{}` at line: {}", operands[1], line_num);
                        }
                    }
                    PHSR => {
                        eprintln!("[Warning]: `phsr` is deprecated use `mov [reg], $[index]` instead.");
                        self.program.push(Opcode { op_type: OpcodeType::Phsr, op_operand: None, op_regs: vec![tokens[0].to_string()] });
                    }
                    
                    SRG => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 2, 2, &line_num);
                        
                        self.program.push(Opcode { op_type: OpcodeType::Srg, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string()] });
                    }
                    
                    CLR => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 1, 1, &line_num);
                        
                        self.program.push(Opcode { op_type: OpcodeType::Clr, op_operand: None, op_regs: vec![tokens[0].to_string()] });
                    }
                    
                    ADD | SUB | MUL | DIV => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 3, 3, &line_num);
                        
                        match inst_name {
                            ADD => {
                                self.program.push(Opcode { op_type: OpcodeType::Add, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                            }
                            SUB => {
                                self.program.push(Opcode { op_type: OpcodeType::Sub, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                            }
                            MUL => {
                                self.program.push(Opcode { op_type: OpcodeType::Mul, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                            }
                            DIV => {
                                self.program.push(Opcode { op_type: OpcodeType::Div, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                            }
                    
                            _ => {}
                        }
                    }
                    
                    DEC => {
                        self.program.push(Opcode { op_type: OpcodeType::Dec, op_operand: None, op_regs: vec![tokens[0].to_string()] });
                    }
                    INC => {
                        self.program.push(Opcode { op_type: OpcodeType::Inc, op_operand: None, op_regs: vec![tokens[0].to_string()] });
                    }
                    
                    EQUAL => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 3, 3, &line_num);
                        
                        self.program.push(Opcode { op_type: OpcodeType::Equal, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                    }
                    
                    JT | JZ | JNZ => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 2, 2, &line_num);
                        
                        if operands[0].starts_with(CONST) {
                            match inst_name {
                                JT => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jt, op_operand: Some(Word { as_u64: (operands[1].replace(CONST, "").parse().unwrap()) }), op_regs: vec![operands[1].to_string()] });
                                }
                                JZ => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jz, op_operand: Some(Word { as_u64: (operands[1].replace(CONST, "").parse().unwrap()) }), op_regs: vec![operands[1].to_string()] });
                                }
                                JNZ => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jnz, op_operand: Some(Word { as_u64: (operands[1].replace(CONST, "").parse().unwrap()) }), op_regs: vec![operands[1].to_string()] });
                                }
                                
                                _ => {}
                            }
                        } else {
                            oasm.deferred_operands_push(operands[0], self.program.len());
                            match inst_name {
                                JT => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jt, op_operand: None, op_regs: vec![operands[1].to_string()] });
                                }
                                JZ => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jz, op_operand: None, op_regs: vec![operands[1].to_string()] });
                                }
                                JNZ => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jnz, op_operand: None, op_regs: vec![operands[1].to_string()] });
                                }
                                
                                _ => {}
                            }
                        }
                    }
                    
                    SYSF => {
                        if tokens.is_empty() {
                            self.program.push(Opcode { op_type: OpcodeType::Sysf, op_operand: None, op_regs: Vec::new() });
                        } else {
                            let operands = self.get_operands(tokens, 1, 1, &line_num);
                            self.program.push(Opcode { op_type: OpcodeType::Sysf, op_operand: None, op_regs: vec![operands[0].to_string()] });
                        }
                    }
                    
                    // Stack opcodes
                    PUSH => {
                        if tokens[0].starts_with('r') {
                            self.program.push(Opcode { op_type: OpcodeType::Push, op_operand: None, op_regs: vec![tokens[0].to_string()] });
                        } else if tokens[0].starts_with(CONST) {
                            if tokens[0].replace(CONST, "").parse::<u64>().is_ok() {
                                self.program.push(Opcode { op_type: OpcodeType::Push, op_operand: Some(Word { as_u64: (tokens[0].replace(CONST, "").parse().unwrap()) }), op_regs: Vec::new() });
                            } else if tokens[0].replace(CONST, "").parse::<i64>().is_ok() {
                                self.program.push(Opcode { op_type: OpcodeType::Push, op_operand: Some(Word { as_i64: (tokens[0].replace(CONST, "").parse().unwrap()) }), op_regs: Vec::new() });
                            } else if tokens[0].replace(CONST, "").parse::<f64>().is_ok() {
                                self.program.push(Opcode { op_type: OpcodeType::Push, op_operand: Some(Word { as_f64: (tokens[0].replace(CONST, "").parse().unwrap()) }), op_regs: Vec::new() });
                            }
                        } else {
                            eprintln!("[Error]: Invalid operand `{}` at line: {}", tokens[0], line_num);
                        }
                    }
                    
                    ADDS | SUBS | MULS | DIVS => {
                        match inst_name {
                            ADDS => {
                                self.program.push(Opcode { op_type: OpcodeType::Adds, op_operand: None, op_regs: Vec::new() });
                            }
                            SUBS => {
                                self.program.push(Opcode { op_type: OpcodeType::Subs, op_operand: None, op_regs: Vec::new() });
                            }
                            MULS => {
                                self.program.push(Opcode { op_type: OpcodeType::Muls, op_operand: None, op_regs: Vec::new() });
                            }
                            DIVS => {
                                self.program.push(Opcode { op_type: OpcodeType::Divs, op_operand: None, op_regs: Vec::new() });
                            }
                            
                            _ => {}
                        }
                    }
                    
                    DUPL => {
                        let op: i64 = tokens[0].parse().unwrap();
                        self.program.push(Opcode { op_type: OpcodeType::Dupl, op_operand: Some(Word { as_u64: op as u64 }), op_regs: Vec::new() });
                    }
                    
                    EQUALS => {
                        self.program.push(Opcode { op_type: OpcodeType::Equals, op_operand: None, op_regs: Vec::new() });
                    }
                    
                    JTS | JZS | JNZS => {
                        if tokens[0].starts_with(CONST) {
                            match inst_name {
                                JTS => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jts, op_operand: Some(Word { as_u64: (tokens[0].replace(CONST, "").parse().unwrap()) }), op_regs: Vec::new() });
                                }
                                JZS => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jzs, op_operand: Some(Word { as_u64: (tokens[0].replace(CONST, "").parse().unwrap()) }), op_regs: Vec::new() });
                                }
                                JNZS => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jnzs, op_operand: Some(Word { as_u64: (tokens[0].replace(CONST, "").parse().unwrap()) }), op_regs: Vec::new() });
                                }
                                
                                _ => {}
                            }
                        } else {
                            oasm.deferred_operands_push(tokens[0], self.program.len());
                            match inst_name {
                                JTS => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jts, op_operand: None, op_regs: Vec::new() });
                                }
                                JZS => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jzs, op_operand: None, op_regs: Vec::new() });
                                }
                                JNZS => {
                                    self.program.push(Opcode { op_type: OpcodeType::Jnzs, op_operand: None, op_regs: Vec::new() });
                                }
                                
                                _ => {}
                            }
                        }
                    }
                    
                    SWC => {
                        let op: i64 = tokens[0].parse().unwrap();
                        self.program.push(Opcode { op_type: OpcodeType::Swc, op_operand: Some(Word { as_u64: op as u64 }), op_regs: Vec::new() });
                    }
                    
                    // Universal opcodes
                    JMP => {
                        if tokens[0].starts_with(CONST) {
                            self.program.push(Opcode { op_type: OpcodeType::Jmp, op_operand: Some(Word { as_u64: (tokens[0].replace(CONST, "").parse().unwrap()) }), op_regs: Vec::new() });
                        } else {
                            oasm.deferred_operands_push(tokens[0], self.program.len());
                            self.program.push(Opcode { op_type: OpcodeType::Jmp, op_operand: None, op_regs: Vec::new() });
                        }
                    }
                    
                    CALL => {
                        oasm.deferred_operands_push(tokens[0], self.program.len());
                        self.program.push(Opcode { op_type: OpcodeType::Call, op_operand: None, op_regs: Vec::new() });
                    }
                    
                    AND => {
                        println!("{:?}", tokens);
                        if tokens.len() > 0 && tokens[0].starts_with('r') && !tokens.is_empty() {
                            let operand = self.get_operands(tokens.clone(), 3, 3, &line_num);
                            self.program.push(Opcode { op_type: OpcodeType::And, op_operand: None, op_regs: vec![operand[0].to_string(), operand[1].to_string(), operand[2].to_string()] });
                        } else {
                            self.program.push(Opcode { op_type: OpcodeType::And, op_operand: None, op_regs: Vec::new() });
                        }
                    }
                    
                    OR => {
                        if tokens.len() > 0 && tokens[0].starts_with('r') && !tokens.is_empty() {
                            let operand = self.get_operands(tokens.clone(), 3, 3, &line_num);
                            self.program.push(Opcode { op_type: OpcodeType::Or, op_operand: None, op_regs: vec![operand[0].to_string(), operand[1].to_string(), operand[2].to_string()] });
                        } else {
                            self.program.push(Opcode { op_type: OpcodeType::Or, op_operand: None, op_regs: Vec::new() });
                        }
                    }
                    XOR => {
                        if tokens.len() > 0 && tokens[0].starts_with('r') && !tokens.is_empty() {
                            let operand = self.get_operands(tokens.clone(), 3, 3, &line_num);
                            self.program.push(Opcode { op_type: OpcodeType::Xor, op_operand: None, op_regs: vec![operand[0].to_string(), operand[1].to_string(), operand[2].to_string()] });
                        } else {
                            self.program.push(Opcode { op_type: OpcodeType::Xor, op_operand: None, op_regs: Vec::new() });
                        }
                    }
                    
                    SHL => {
                        if tokens.len() > 0 && tokens[0].starts_with('r') && !tokens.is_empty() {
                            let operand = self.get_operands(tokens.clone(), 3, 3, &line_num);
                            self.program.push(Opcode { op_type: OpcodeType::Shl, op_operand: None, op_regs: vec![operand[0].to_string(), operand[1].to_string(), operand[2].to_string()] });
                        } else {
                            self.program.push(Opcode { op_type: OpcodeType::Shl, op_operand: None, op_regs: Vec::new() });
                        }
                    }
                    SHR => {
                        if tokens.len() > 0 && tokens[0].starts_with('r') && !tokens.is_empty() {
                            let operand = self.get_operands(tokens.clone(), 3, 3, &line_num);
                            self.program.push(Opcode { op_type: OpcodeType::Shr, op_operand: None, op_regs: vec![operand[0].to_string(), operand[1].to_string(), operand[2].to_string()] });
                        } else {
                            self.program.push(Opcode { op_type: OpcodeType::Shr, op_operand: None, op_regs: Vec::new() });
                        }
                    }
                    
                    NOT => {
                        if tokens.len() > 0 && tokens[0].starts_with('r') && !tokens.is_empty() {
                            let operand = self.get_operands(tokens.clone(), 2, 2, &line_num);
                            self.program.push(Opcode { op_type: OpcodeType::Not, op_operand: None, op_regs: vec![operand[0].to_string(), operand[1].to_string()] });
                        } else {
                            self.program.push(Opcode { op_type: OpcodeType::Not, op_operand: None, op_regs: Vec::new() });
                        }
                    }
                    
                    POP => {
                        if tokens.len() > 0 && tokens[0].starts_with('r') && !tokens.is_empty() {
                            self.program.push(Opcode { op_type: OpcodeType::Pop, op_operand: None, op_regs: vec![tokens[0].to_string()] });
                        } else {
                            self.program.push(Opcode { op_type: OpcodeType::Pop, op_operand: None, op_regs: Vec::new() });
                        }
                    }
                    
                    RET => {
                        self.program.push(Opcode { op_type: OpcodeType::Ret, op_operand: None, op_regs: Vec::new() });
                    }
                    
                    HLT => {
                        self.program.push(Opcode { op_type: OpcodeType::Hlt, op_operand: None, op_regs: Vec::new() });
                    }
                    
                    _ => {
                        eprintln!("[Error]: Invalid instruction `{}` at line: {}", inst_name, line_num);
                    }
                }
            }
            
        }
        
        for label in oasm.labels.clone() {
            if label.name == "_start" {
                self.pc = label.addr;
            }
        }
        
        for i in 0..oasm.deferred_operands.len() {
            let label_addr = oasm.labels_contains(oasm.deferred_operands[i].label.as_str());
            self.program[oasm.deferred_operands[i].addr].op_operand = Some(Word { as_u64: label_addr.unwrap() as u64 });
        }
        
    }
    
    pub fn load_program_from_memory(self: &mut Self, program: Vec<Opcode>) {
        self.program.extend_from_slice(&program);
    }
    
    pub fn dump(self: &Self) {
        println!("\n[Registers]:");
        println!("    r0:  {:?}", self.r0);
        println!("    r1:  {:?}", self.r1);
        println!("    r2:  {:?}", self.r2);
        println!("    r3:  {:?}", self.r3);
        println!("    r4:  {:?}", self.r4);
        println!("    r5:  {:?}", self.r5);
        println!("    r6:  {:?}", self.r6);
        println!("    r7:  {:?}", self.r7);
        println!("    r8:  {:?}", self.r8);
        println!("    r9:  {:?}", self.r9);
        println!("    r10: {:?}", self.r10);
        println!("    r11: {:?}", self.r11);
        println!("    r12: {:?}", self.r12);
        println!("    r13: {:?}", self.r13);
        println!("    r14: {:?}", self.r14);
        println!("    r15: {:?}", self.r15);
        println!("    r16: {:?}", self.r16);
        println!("    tsr: {}", self.tsr);
        println!("    rspc: {}", self.rspc);
        println!("    pc:  {}", self.pc);
        
        if self.stack.len() > 0 {
            println!("[Stack]:");
            for i in self.stack.clone() {
                println!("    {:?}", i);
            }
        }
    }
    
    pub fn execute_program(self: &mut Self) {
        while !self.halt {
            let err: Error = self.execute_opcode();
            if err != Error::None {
                println!("[Error]: {}", err.as_string());
                exit(1);
            }
        }
    }
    
    pub fn execute_program_debug(self: &mut Self) {
        while !self.halt {
            let err: Error = self.execute_opcode();
            let mut buffer = String::new();
            
            let _ = stdin().read_line(&mut buffer);
            self.dump();
            println!("[Instruction] => {:?}", self.program[self.pc]);
            
            if err != Error::None {
                println!("[Error]: {}", err.as_string());
                exit(1);
            }
        }
    }
}