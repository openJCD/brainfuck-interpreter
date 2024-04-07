pub mod tokenizer;
pub mod interpreter;
use tokenizer::*;
use interpreter::*;
use std::collections::HashMap;
use std::fs;
use std::env;

fn main() -> Result<(), String> {
   
    println!("----- BFOxide v1.0.0 release -----");
   
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    
    println!("loading from file: {}", args[1]);
    
    let filedata = fs::read_to_string(path).expect("Error reading the file given as first argument.");

    let data:(Vec<Tokens>, HashMap<usize, usize>) = do_all_token_ops(filedata); 
    execute_instructions(data.0, data.1)?;
    Ok(())
}