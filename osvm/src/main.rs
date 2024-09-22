use std::{env, fs::File, io::Read, process::exit};
use osvm_lib::prelude::*;

pub fn get_file_contents(file_path: &str) -> String {
    let mut file: File = File::open(file_path).unwrap();
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    
    contents
}

fn usage(program_file: &String) {
    println!("[Usage]: {program_file} <SUBCOMMAND> <ARGS>");
    println!("[Subcommands]:");
    println!("  -   build <INPUT.OSV> <OUTPUT.VBIN>  ->  Compiles the program");
    println!("  -   run   <INPUT.OSV> <OUTPUT.VBIN>  ->  Runs the program");
    println!("  -   debug <INPUT.OSV> <OUTPUT.VBIN>  ->  Compiles the program");
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
    osvm.init_default_sysf();
    
    let mut osvm_file: OSVMFile = OSVMFile {}; 
    let oasm: OASM = OASM::init();
    
    let subcommand = shift(&mut index, &args);
    
    match subcommand.as_str() {
        "build" | "run" | "debug" => {
            println!("----------- Compiling -----------");
            let input_path = shift(&mut index, &args);
            let output_path = shift(&mut index, &args);
            let source = get_file_contents(input_path.clone().as_str());
            osvm.translate_source(oasm, input_path.clone(), source.clone());
            println!("[Converting File] => {} => {}", input_path, output_path);
            osvm_file.save_program_to_file(&mut osvm, &output_path);
            
            if subcommand == "run" {
                osvm_file.load_program_from_file(&mut osvm, &output_path);
                println!("------------ Running ------------");
                osvm.execute_program();
            } else if subcommand == "debug" {
                osvm_file.load_program_from_file(&mut osvm, &output_path);
                println!("------ Running (Debugging) ------");
                osvm.execute_program_debug();
            }
        }
        
        _ => {
            usage(&program_file);
            eprintln!("[Error]: Invalid Subcommand");
            exit(1);
        }
    }
}