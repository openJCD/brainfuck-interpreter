pub mod tokenizer;
pub mod interpreter;
use tokenizer::*;
use interpreter::*;
use std::collections::HashMap;
use std::fs;
use std::env;

fn main() -> Result<(), String> {
   
    println!("----- BFI v0.1.0 release -----");
   
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    
    println!("loading from file: {}", args[1]);
    
    let filedata = fs::read_to_string(path).expect("Error reading the file given as first argument.");

    let data:(Vec<Tokens>, HashMap<usize, usize>) = do_all_token_ops(filedata); 
    execute_instructions(data.0, data.1)?;
    //println!{"-----------------------TOKENS:------------------------------\r\n{:?}", tokens}

    //-----END-----     
    Ok(())

}


fn create_debug_string (tok:&char, arr_pos:usize, arr:&Vec<u8>, file_pos:usize) -> String {
    let token = *tok;
    if token == '\r' || token == '\n' {
        return String::from("\nDetected newline")
    }

    let message = format!("\r\n| {} |:  Operation | {token} | @ | {} => {} (ASCII {})", 
        file_pos,
        arr_pos, arr[arr_pos] as char,
        arr[arr_pos] as u8);
    
    String::from(message)
}



#[test]
fn nested_loop() {

}