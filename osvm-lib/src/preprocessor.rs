use std::{env, fs::File, io::Read, path::PathBuf, process::exit};
use log::*;

pub struct Preprocessor {}

impl Preprocessor {
    fn remove_line_by_sstr(self: &Self, starts_with: &str,  source: String) -> String {
        let mut new_source = String::new();
        for line in source.lines() {
            let line = line.trim();
            if line.starts_with(starts_with) {
                continue;
            }
            
            new_source.push_str(line);
            new_source.push('\n');
        }
        
        new_source
    }
    
    fn get_string(self: &Self, line: &str) -> String {
        let mut string = String::new();
        let mut found = false;
        for c in line.chars() {
            if c == '"' {
                if found != true {
                    found = true;
                } else {
                    found = false;
                }
                continue;
            }
            
            if found == true {
                string.push(c);
            }
        }
        
        string
    }
    
    fn get_cindex(self: &Self, index: usize, source: &str) -> usize {
        let mut lindex = 0;
        let mut cindex = 0;
        for c in source.chars() {
            if lindex == index {
                break;
            }
            
            if c != '\n' {
                cindex += 1;
            } else {
                cindex += 1;
                lindex += 1;
            }
        }
        
        cindex
    }
    
    pub fn process_includes(self: &Self, file_path: String, mut source: String) -> String {
        info!("[Preprocessor] => includes => {}", file_path);
        let mut index = 0;
        for line in source.clone().lines() {
            let line = line.trim();
            if line.starts_with("%") {
                if line.replace("%", "").starts_with("include") {
                    let path = self.get_string(line);
                    let mut file;
                    match env::var("OSVM_LIBS_DIR") {
                        Ok(_) => {}
                        Err(_) => {
                            eprintln!("[Error]: OSVM_LIBS_DIR NOT SET!");
                            exit(1);
                        }
                    }
                    
                    match File::open(PathBuf::from(env::var("OSVM_LIBS_DIR").unwrap()).join(path.clone())) {
                        Ok(_) => file = File::open(PathBuf::from(env::var("OSVM_LIBS_DIR").unwrap()).join(path)).unwrap(),
                        Err(_) => file = File::open(PathBuf::from(file_path.clone()).parent().unwrap().join(path)).unwrap(),
                    }
                    
                    let mut include_source = String::new();
                    let _ = file.read_to_string(&mut include_source);
                    
                    let isource = include_source + "\n";
                    source.insert_str(self.get_cindex(index + 1, source.as_str()), isource.as_str());
                }
            }
            
            index += 1;
        }
        
        source = self.remove_line_by_sstr("%include", source);
        source
    }
    
    pub fn process_source(self: &Self, file_path: String, source: String) -> String {
        info!("[Preprocessor] => all => {}", file_path);
        let mut _index = 0;
        
        let mut macros = Vec::<(&str, &str)>::new();
        for line in &mut source.lines() {
            let line = line.trim();
            if line.starts_with("%") {
                if line.replace("%", "").starts_with("define") {
                    let splitted: Vec<&str> = line.trim().splitn(3, |c: char| c.is_whitespace()).collect();
                    macros.push((splitted[1], splitted[2]));
                }
            }
            
            _index += 1;
        }
        
        let mut source = self.remove_line_by_sstr("%", source.clone());
        source = self.remove_line_by_sstr(";", source);
        for _macro in macros.clone() {
            source = source.replace(_macro.0, _macro.1);
        }
        
        source
    }
}