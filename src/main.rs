#![allow(unused, dead_code)]

mod defines;
mod oasm;
mod opcode;
mod error;

use std::{env, fs::File, io::{Read, Write}, ops::Index, process::exit};
use std::io::stdin;
use serde::{Serialize, Deserialize};

use defines::*;
use oasm::*;
use opcode::*;
use error::*;

#[derive(Serialize, Deserialize)]
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
    
    pc: Word,

    // Stack
    stack: Vec<Word>,
    
    // Other
    program: Vec<Opcode>,
    
    halt: bool,
}

impl OSVM {
    fn init() -> OSVM {
        OSVM {
            // Registers
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            r16: 0,
            
            pc: 0,
            
            // Stack
            stack: Vec::new(),
            
            // Other
            program: Vec::new(),
            
            halt: false,
        }
    }
    
    fn assign_register(self: &mut Self, opcode: &Opcode, index: usize, new_value: Word) {
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
    
    fn find_register(self: &mut Self, opcode: &Opcode, index: usize) -> Option<&mut Word> {
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
    
    fn execute_opcode(self: &mut Self) -> Error {
        if self.pc as usize >= self.program.len() {
            return Error::InvalidOpcodeAccess;
        }
        let opcode = self.program[self.pc as usize].clone();
        
        match opcode.op_type {
            OpcodeType::Mov => {
                if opcode.op_operand == None {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterOverflow;
                    } else if opcode.op_regs.len() > 2 {
                        return Error::RegisterUnderflow;
                    }
                    
                    let reg = *self.find_register(&opcode, 1).unwrap();
                    self.assign_register(&opcode, 0, reg);
                } else {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterOverflow;
                    } else if opcode.op_regs.len() > 1 {
                        return Error::RegisterUnderflow;
                    }
                    
                    self.assign_register(&opcode, 0, opcode.op_operand.unwrap());
                }
                self.pc += 1;
            }
            
            OpcodeType::Srg => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterOverflow;
                } else if opcode.op_regs.len() > 2 {
                    return Error::RegisterUnderflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                let reg2 = *self.find_register(&opcode, 1).unwrap();
                self.assign_register(&opcode, 0, reg2);
                self.assign_register(&opcode, 1, reg1);
                self.pc += 1;
            }
            
            OpcodeType::Clr => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterOverflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterUnderflow;
                }
                
                self.assign_register(&opcode, 0, 0);
                self.pc += 1;
            }

            OpcodeType::Add => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                let sum = reg1 + reg2;
                
                let dest = self.find_register(&opcode, 0).unwrap();
                *dest = sum;
                self.pc += 1;
            }
            OpcodeType::Sub => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                let sum = reg1 - reg2;
                
                let dest = self.find_register(&opcode, 0).unwrap();
                *dest = sum;
                self.pc += 1;
            }
            OpcodeType::Mul => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                let sum = reg1 * reg2;
                
                let dest = self.find_register(&opcode, 0).unwrap();
                *dest = sum;
                self.pc += 1;
            }
            OpcodeType::Div => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                if reg1 == 0 || reg2 == 0 {
                    return Error::DivByZero;
                }
                
                let sum = reg1 / reg2;
                
                let dest = self.find_register(&opcode, 0).unwrap();
                *dest = sum;
                self.pc += 1;
            }
            
            OpcodeType::Dec => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = self.find_register(&opcode, 0).unwrap();
                *reg1 = *reg1 - 1;
                self.pc += 1;
            }
            
            OpcodeType::Equal => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 3 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 1).unwrap();
                let reg2 = *self.find_register(&opcode, 2).unwrap();
                let sum = (reg1 == reg2) as Word;
                
                let dest = self.find_register(&opcode, 0).unwrap();
                *dest = sum;
                self.pc += 1;
            }
            
            OpcodeType::Jt => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                if reg1 == 1 {
                    self.pc = opcode.op_operand.unwrap();
                } else {
                    self.pc += 1;
                }
            }
            OpcodeType::Jz => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                if reg1 == 0 {
                    self.pc = opcode.op_operand.unwrap();
                } else {
                    self.pc += 1;
                }
            }
            OpcodeType::Jnz => {
                if opcode.op_regs.len() < 1 {
                    return Error::RegisterUnderflow;
                } else if opcode.op_regs.len() > 1 {
                    return Error::RegisterOverflow;
                }
                
                let reg1 = *self.find_register(&opcode, 0).unwrap();
                if reg1 != 0 {
                    self.pc = opcode.op_operand.unwrap();
                } else {
                    self.pc += 1;
                }
            }
            
            // Stack opcodes
            OpcodeType::Push => {
                if opcode.op_operand == None {
                    if opcode.op_regs.len() < 1 {
                        return Error::RegisterUnderflow;
                    } else if opcode.op_regs.len() > 1 {
                        return Error::RegisterOverflow;
                    }
                    
                    let reg = *self.find_register(&opcode, 0).unwrap();
                    self.stack.push(reg);
                } else {
                    self.stack.push(opcode.op_operand.unwrap());
                }
                self.pc += 1;
            }
            
            OpcodeType::Pop => {
                if self.stack.len() < 1 {
                    return Error::StackUnderflow;
                }
                
                self.stack.pop();
                self.pc += 1;
            }
            
            OpcodeType::Adds => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(b + a);
                self.pc += 1;
            }
            OpcodeType::Subs => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(b - a);
                self.pc += 1;
            }
            OpcodeType::Muls => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(b * a);
                self.pc += 1;
            }
            OpcodeType::Divs => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push(b / a);
                self.pc += 1;
            }
            
            OpcodeType::Equals => {
                if self.stack.len() < 2 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack[self.stack.len() - 1];
                let b = self.stack[self.stack.len() - 2];
                self.stack.push((b == a) as i64);
                self.pc += 1;
            }
            
            OpcodeType::Dupl => {
                if self.stack.len() - opcode.op_operand.unwrap() as usize <= 0 {
                    return Error::StackUnderflow;
                }
                
                if opcode.op_operand < Some(0) {
                    return Error::InvalidOperand;
                }
                
                self.stack.push(self.stack[self.stack.len() - 1 - opcode.op_operand.unwrap() as usize]);
                self.pc += 1;
            }
            
            OpcodeType::Jts => {
                if self.stack.len() < 1 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                if a == 1 {
                    self.pc = opcode.op_operand.unwrap();
                } else {
                    self.pc += 1;
                }
            }
            OpcodeType::Jzs => {
                if self.stack.len() < 1 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack.pop().unwrap();
                if a == 0 {
                    self.pc = opcode.op_operand.unwrap();
                } else {
                    self.pc += 1;
                }
            }
            OpcodeType::Jnzs => {
                if self.stack.len() < 1 {
                    return Error::StackUnderflow;
                }
                
                let a = self.stack[self.stack.len() - 1];
                if a != 0 {
                    self.pc = opcode.op_operand.unwrap();
                } else {
                    self.pc += 1;
                }
            }
            
            // Universal opcodes
            OpcodeType::Jmp => {
                self.pc = opcode.op_operand.unwrap();
            }
            
            OpcodeType::Hlt => {
                self.halt = true;
            }
            
            _ => {
                return Error::InvalidOperand;
            }
        }
        
        Error::None
    }
    
    fn get_operands<'a>(self: &Self, tokens: Vec<&'a str>, len1: usize, len2: usize, line_num: &u64) -> Vec<&'a str> {
        let mut operands: Vec<&str> = tokens[0].split(", ").collect();
        if operands.len() < len1 || operands.len() > len2 {
            eprintln!("[Error]: Invalid number of opcode arguments at line: {}", line_num);
            exit(1);
        }
        
        operands
    }
    
    fn translate_source(self: &mut Self, mut oasm: OASM, source: String) {
        let lines: Vec<&str> = source.lines().collect();
        let mut line_num = 0;
        for line in lines {
            line_num += 1;
            if line.is_empty() || line.trim().starts_with(';') {
                continue;
            }
            
            let mut tokens: Vec<&str> = line.trim().splitn(2, char::is_whitespace).collect();
            if !tokens.is_empty() && !tokens[0].is_empty() {
                let inst_name = tokens[0];
                tokens.remove(0);
                
                if inst_name.ends_with(':') {
                    let label = inst_name.replace(":", "");
                    oasm.labels_push(&label, self.program.len());
                    continue;
                }
                
                match inst_name {
                    // Register opcodes
                    MOV => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 2, 2, &line_num);
                        
                        if operands[1].starts_with("r") {
                            self.program.push(Opcode { op_type: OpcodeType::Mov, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string()] });
                        } else if operands[1].starts_with("#") {
                            self.program.push(Opcode { op_type: OpcodeType::Mov, op_operand: Some(operands[1].replace("#", "").parse().unwrap()), op_regs: vec![operands[0].to_string()] });
                        } else {
                            eprintln!("[Error]: Invalid operand `{}` at line: {}", operands[1], line_num);
                        }
                    }
                    SRG => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 2, 2, &line_num);
                        
                        self.program.push(Opcode { op_type: OpcodeType::Srg, op_operand: None, op_regs: vec![operands[0].to_string(), operands[1].to_string()] });
                    }
                    
                    CLR => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 1, 1, &line_num);
                        
                        self.program.push(Opcode { op_type: OpcodeType::Clr, op_operand: Some(0), op_regs: vec![tokens[0].to_string()] });
                    }
                    
                    ADD | SUB | MUL | DIV => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 3, 3, &line_num);
                        
                        match inst_name {
                            ADD => {
                                self.program.push(Opcode { op_type: OpcodeType::Add, op_operand: Some(0), op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                            }
                            SUB => {
                                self.program.push(Opcode { op_type: OpcodeType::Sub, op_operand: Some(0), op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                            }
                            MUL => {
                                self.program.push(Opcode { op_type: OpcodeType::Mul, op_operand: Some(0), op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                            }
                            DIV => {
                                self.program.push(Opcode { op_type: OpcodeType::Div, op_operand: Some(0), op_regs: vec![operands[0].to_string(), operands[1].to_string(), operands[2].to_string()] });
                            }
                    
                            _ => {}
                        }
                    }
                    
                    DEC => {
                        self.program.push(Opcode { op_type: OpcodeType::Dec, op_operand: Some(0), op_regs: vec![tokens[0].to_string()] });
                    }
                    
                    EQUAL => {
                        self.program.push(Opcode { op_type: OpcodeType::Equal, op_operand: Some(0), op_regs: Vec::new() });
                    }
                    
                    JT | JZ | JNZ => {
                        let mut operands: Vec<&str> = self.get_operands(tokens.clone(), 2, 2, &line_num);
                        
                        oasm.jumps_push(operands[0], self.program.len());
                        match inst_name {
                            JT => {
                                self.program.push(Opcode { op_type: OpcodeType::Jt, op_operand: Some(0), op_regs: vec![operands[1].to_string()] });
                            }
                            JZ => {
                                self.program.push(Opcode { op_type: OpcodeType::Jz, op_operand: Some(0), op_regs: vec![operands[1].to_string()] });
                            }
                            JNZ => {
                                self.program.push(Opcode { op_type: OpcodeType::Jnz, op_operand: Some(0), op_regs: vec![operands[1].to_string()] });
                            }
                            
                            _ => {}
                        }
                    }
                    
                    // Stack opcodes
                    PUSH => {
                        if tokens[0].starts_with('r') {
                            self.program.push(Opcode { op_type: OpcodeType::Push, op_operand: None, op_regs: vec![tokens[0].to_string()] });
                        } else if tokens[0].starts_with('#') {
                            self.program.push(Opcode { op_type: OpcodeType::Push, op_operand: Some(tokens[0].replace("#", "").parse().unwrap()), op_regs: Vec::new() });
                        } else {
                            eprintln!("[Error]: Invalid operand `{}` at line: {}", tokens[0], line_num);
                        }
                    }
                    POP => {
                        self.program.push(Opcode { op_type: OpcodeType::Pop, op_operand: Some(0), op_regs: Vec::new() });
                    }
                    
                    ADDS | SUBS | MULS | DIVS => {
                        match inst_name {
                            ADDS => {
                                self.program.push(Opcode { op_type: OpcodeType::Adds, op_operand: Some(0), op_regs: Vec::new() });
                            }
                            SUBS => {
                                self.program.push(Opcode { op_type: OpcodeType::Subs, op_operand: Some(0), op_regs: Vec::new() });
                            }
                            MULS => {
                                self.program.push(Opcode { op_type: OpcodeType::Muls, op_operand: Some(0), op_regs: Vec::new() });
                            }
                            DIVS => {
                                self.program.push(Opcode { op_type: OpcodeType::Divs, op_operand: Some(0), op_regs: Vec::new() });
                            }
                            
                            _ => {}
                        }
                    }
                    
                    DUPL => {
                        let op: i64 = tokens[0].parse().unwrap();
                        self.program.push(Opcode { op_type: OpcodeType::Dupl, op_operand: Some(op), op_regs: Vec::new() });
                    }
                    
                    EQUALS => {
                        self.program.push(Opcode { op_type: OpcodeType::Equals, op_operand: Some(0), op_regs: Vec::new() });
                    }
                    
                    JTS | JZS | JNZS => {
                        oasm.jumps_push(tokens[0], self.program.len());
                        match inst_name {
                            JTS => {
                                self.program.push(Opcode { op_type: OpcodeType::Jts, op_operand: Some(0), op_regs: Vec::new() });
                            }
                            JZS => {
                                self.program.push(Opcode { op_type: OpcodeType::Jzs, op_operand: Some(0), op_regs: Vec::new() });
                            }
                            JNZS => {
                                self.program.push(Opcode { op_type: OpcodeType::Jnzs, op_operand: Some(0), op_regs: Vec::new() });
                            }
                            
                            _ => {}
                        }
                    }
                    
                    // Universal opcodes
                    JMP => {
                        oasm.jumps_push(tokens[0], self.program.len());
                        self.program.push(Opcode { op_type: OpcodeType::Jmp, op_operand: Some(0), op_regs: Vec::new() });
                    }
                    
                    HLT => {
                        self.program.push(Opcode { op_type: OpcodeType::Hlt, op_operand: Some(0), op_regs: Vec::new() });
                    }
                    
                    _ => {
                        eprintln!("[Error]: Invalid instruction `{}` at line: {}", inst_name, line_num);
                    }
                }
            }
            
        }
        
        for i in 0..oasm.jumps.len() {
            let label_addr = oasm.labels_contains(oasm.jumps[i].label.as_str());
            self.program[oasm.jumps[i].addr].op_operand = label_addr;
        }
    }
    
    fn load_program_from_memory(self: &mut Self, program: Vec<Opcode>) {
        self.program.extend_from_slice(&program);
    }
    
    fn load_program_from_file(self: &mut Self, file_path: &str) {
        let mut file: File = File::open(file_path).unwrap();
        let mut encoded: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut encoded);

        let data: Vec<Opcode> = bincode::deserialize(&encoded).unwrap();
        self.program = data;
    }

    fn save_program_to_file(self: &Self, file_path: &str) {
        let mut file: File = File::create(file_path).unwrap();
        let encoded: Vec<u8> = bincode::serialize(&self.program).unwrap();
        let _ = file.write_all(&encoded);
    }
    
    
    fn dump(self: &Self) {
        println!("\n[Registers]:");
        println!("    r0:  {}", self.r0);
        println!("    r1:  {}", self.r1);
        println!("    r2:  {}", self.r2);
        println!("    r3:  {}", self.r3);
        println!("    r4:  {}", self.r4);
        println!("    r5:  {}", self.r5);
        println!("    r6:  {}", self.r6);
        println!("    r7:  {}", self.r7);
        println!("    r8:  {}", self.r8);
        println!("    r9:  {}", self.r9);
        println!("    r10: {}", self.r10);
        println!("    r11: {}", self.r11);
        println!("    r12: {}", self.r12);
        println!("    r13: {}", self.r13);
        println!("    r14: {}", self.r14);
        println!("    r15: {}", self.r15);
        println!("    r16: {}", self.r16);
        println!("    pc:  {}", self.pc);
        
        if self.stack.len() > 0 {
            println!("[Stack]:");
            for i in self.stack.clone() {
                println!("    {}", i);
            }
        }
    }
    
    fn execute_program(self: &mut Self, debug: bool) {
        while !self.halt {
            let err: Error = self.execute_opcode();
            if debug {
                let mut buffer = [0; 1];
                let _ = stdin().read_exact(&mut buffer);
                self.dump();
            }
            if err != Error::None {
                println!("[Error]: {}", err.as_string());
                exit(1);
            }
        }
    }
}

fn get_file_contents(file_path: &str) -> String {
    let mut file: File = File::open(file_path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    
    contents
}

fn usage(program_file: &String) {
    println!("[Usage]: {program_file} <SUBCOMMAND> <ARGS>");
    println!("[Subcommands]:");
    println!("  -   build <INPUT.OS> <OUTPUT.VBIN>  ->  Compiles the program");
    println!("  -   run   <INPUT.OS> <OUTPUT.VBIN>  ->  Runs the program");
    println!("  -   debug <INPUT.BIN>               ->  Compiles the program");
}

fn shift(index: &mut usize, args: &Vec<String>) -> String {
    let last_index = *index;
    if last_index >= args.len() {
        usage(&args[0]);
        exit(1);
    }
    
    *index += 1;
    return args[last_index].clone();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut index = 0;
    let program_file = shift(&mut index, &args);
    
    let mut osvm: OSVM = OSVM::init();
    let oasm: OASM = OASM::init();
    let opcode: Opcode = Opcode::init();
    
    let subcommand = shift(&mut index, &args);
    
    match subcommand.as_str() {
        "build" | "run" => {
            let input_path = shift(&mut index, &args);
            let output_path = shift(&mut index, &args);
            let source = get_file_contents(&input_path);
            osvm.translate_source(oasm, source);
            osvm.save_program_to_file(&output_path);
            
            if subcommand == "run" {
                osvm.load_program_from_file(&output_path);
                osvm.execute_program(false);
            }
        }
        
        "debug" => {
            let output_path = shift(&mut index, &args);
            osvm.load_program_from_file(&output_path);
            osvm.execute_program(true);
        }
        
        _ => {
            usage(&program_file);
            eprintln!("[Error]: Invalid Subcommand");
            exit(1);
        }
    }
}