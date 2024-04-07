pub mod tokenizer;
pub mod interpreter;
use tokenizer::*;
use interpreter::*;
use std::collections::HashMap;
use std::fs;
use std::env;
use std::time::Instant;

fn main() -> Result<(), String> {
    
    let timer2 = Instant::now();
    println!("----- BFOxide v1.0.0 release -----");
   
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    let mut mode = "--default";
    if args.len() >2 {
        let arg2: &String = &args[2];
        println!("mode? {:?}", arg2);
        mode = arg2;
    }
    println!("loading from file: {}", args[1]);
    let filedata = fs::read_to_string(path).expect("Error reading the file given as first argument.");
    let mut data;
    if mode == "--debug" {
        data = do_all_token_ops(filedata, true); 
        execute_instructions(data.0, data.1, true)?;
    }
    else {
        data = do_all_token_ops(filedata, false);
        execute_instructions(data.0, data.1, false)?;
    }
    println!("\r\nFinished everything in {:.2?}", timer2.elapsed());
    Ok(())
}