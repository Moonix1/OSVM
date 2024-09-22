use std::ffi::c_void;

use libc::{exit, free, malloc};

use crate::{utils::defines::*, opcode::Opcode, osvm::OSVM};

pub struct SystemFunctions {}

pub type SysFunction = fn(osvm: &mut OSVM, opcode: &Opcode, reg: Vec<String>);

impl SystemFunctions {
    pub fn alloc(osvm: &mut OSVM, opcode: &Opcode, reg: Vec<String>) {
        unsafe {
            if reg.is_empty() {
                if osvm.stack.len() < 1 {
                    eprintln!("[Error]: Stack Underflow");
                    exit(1);
                }
                
                let a = osvm.stack.len() - 1;
                osvm.stack[a].as_ptr = malloc(osvm.stack[a].as_usize);
            } else {
                let reg1 = &osvm.find_register(opcode, 0).unwrap();
                osvm.find_register(opcode, 0).unwrap().as_ptr = malloc(reg1.as_usize);
            }
        }
    }
    
    pub fn free(osvm: &mut OSVM, opcode: &Opcode, reg: Vec<String>) {
        unsafe {
            if reg.is_empty() {
                if osvm.stack.len() < 1 {
                    eprintln!("[Error]: Stack Underflow");
                    exit(1);
                }
                
                let a = osvm.stack.len() - 1;
                free(osvm.stack.pop().unwrap().as_ptr as *mut c_void);
            } else {
                free(osvm.find_register(opcode, 0).unwrap().as_ptr as *mut c_void);
                osvm.find_register(opcode, 0).unwrap().as_usize = 0;
            }
        }
    }
    
    pub fn print_num(osvm: &mut OSVM, opcode: &Opcode, reg: Vec<String>) {
        unsafe {
            if !reg.is_empty() {
                match osvm.find_register(opcode, 0).unwrap() {
                    Word { as_u64: _ } => println!("{}", osvm.find_register(opcode, 0).unwrap().as_u64),
                    Word { as_i64: _ } => println!("{}", osvm.find_register(opcode, 0).unwrap().as_i64),
                    Word { as_f64: _ } => println!("{}", osvm.find_register(opcode, 0).unwrap().as_f64),
                    
                    _ => {}
                }
            } else {
                match osvm.stack[osvm.stack.len() - 1] {
                    Word { as_u64: _ } => println!("{}", osvm.stack[osvm.stack.len() - 1].as_u64),
                    Word { as_i64: _ } => println!("{}", osvm.stack[osvm.stack.len() - 1].as_i64),
                    Word { as_f64: _ } => println!("{}", osvm.stack[osvm.stack.len() - 1].as_f64),
                    
                    _ => {}
                }
            }
        }
    }
    
    pub fn print_ptr(osvm: &mut OSVM, opcode: &Opcode, reg: Vec<String>) {
        unsafe {
            if !reg.is_empty() {
                match osvm.find_register(opcode, 0).unwrap() {
                    Word { as_ptr: _ } => println!("{:?}", osvm.find_register(opcode, 0).unwrap().as_ptr),
                    
                    _ => {}
                }
            } else {
                match osvm.stack[osvm.stack.len() - 1] {
                    Word { as_ptr: _ } => println!("{:?}", osvm.stack[osvm.stack.len() - 1].as_ptr),
                    
                    _ => {}
                }
            }
        }
    }
}