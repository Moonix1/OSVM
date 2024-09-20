use crate::{OSVM, opcode::Opcode};

use std::{ffi::{c_void, CString}, process::exit};
use libc::{
    fclose, ferror, fopen, fread,
    free, fseek, ftell, fwrite, malloc,
    SEEK_END, SEEK_SET
};

use std::mem;

pub struct OSVMFile {}

impl OSVMFile {
    pub fn load_program_from_file(self: &mut Self, osvm: &mut OSVM, file_path: &str) {
        unsafe {
            let file_name = CString::new(file_path).unwrap();
            let file_mode = CString::new("rb").unwrap();
            let file = fopen(file_name.as_ptr(), file_mode.as_ptr());
            if file.is_null() {
                eprintln!("[Error]: Could not open file `{}`", file_path);
                exit(1);
            }
            
            if fseek(file, 0, SEEK_END) < 0 {
                eprintln!("[Error]: Could not read file `{}`", file_path);
                exit(1);
            }
            
            let m = ftell(file);
            if m < 0 {
                eprintln!("[Error]: Could not read file `{}`", file_path);
                exit(1);
            }
            
            assert!(m as usize % mem::size_of::<Opcode>() == 0);
            
            if fseek(file, 0, SEEK_SET) < 0 {
                eprintln!("[Error]: Could not read file `{}`", file_path);
                exit(1);
            }
            
            osvm.program.set_len(fread(osvm.program.as_ptr() as *mut c_void, mem::size_of::<Opcode>(),
                m as usize / mem::size_of::<Opcode>(), file));
            
            if ferror(file) != 0 {
                eprintln!("[Error]: Could not read file `{}`", file_path);
                exit(1);
            }
            
            println!("[Loading File] => {} => OSVM", file_path);
            
            fclose(file);
        }
    }

    pub fn save_program_to_file(self: &Self, osvm: &mut OSVM, file_path: &str) {
        unsafe {
            let file_name = CString::new(file_path).unwrap();
            let file_mode = CString::new("wb").unwrap();
            let file = fopen(file_name.as_ptr(), file_mode.as_ptr());
            if file.is_null() {
                eprintln!("[Error]: Could not open file `{}`", file_path);
                exit(1);
            }
            
            fwrite(osvm.program.as_ptr() as *const c_void, mem::size_of::<Opcode>(), osvm.program.len(), file);
        
            if ferror(file) != 0 {
                eprintln!("[Error]: Could not write to file `{}`", file_path);
                exit(1);
            }
            
            println!("[Created File] => {}", file_path);
            
            fclose(file);
        }
    }
}