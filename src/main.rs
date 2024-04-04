use std::fs;
use std::env;
fn main() -> Result<(), String> {
   
    println!("----- BFI v1.0.0 release -----");
   
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    
    println!("loading from file: {}", args[1]);
    
    let filedata = fs::read_to_string(path).expect("Error reading the file given as first argument.");
    
    execute_tokens(filedata)?;

    //-----END-----     
    Ok(())

}

fn execute_tokens(filedata:String) -> Result<(), String>{

    let token_array: &Vec<char> = &filedata.chars().collect(); 
    
    let mut mem_arr:Vec<u8> = vec![0; 30_000];
     
    // start halfway through the array
    let mut mem_position:usize = 14_999;
    
    let mut debug_data: Vec<String> = Vec::new();
    
    let mut file_index = 0;
    //limit the depth of nested loops to 255
    let mut current_loop_scope_start: usize = 0;
    while filedata.capacity() > file_index {
        let token = &token_array[file_index];
        //pythoners have to use copious amounts of nesting to do this (losers)
        match token {
            '+' => {mem_arr[mem_position] += 1}, 
            '-' => {mem_arr[mem_position] -= 1}, 
            '>' => {mem_position += 1},
            '<' => {mem_position -= 1},
            '[' => {/*loop_nest += 1;*/ current_loop_scope_start = file_index},
            ']' => {if mem_arr[mem_position] > 0 {file_index = current_loop_scope_start}},
            ',' => { 
                let mut input:String = String::new();
                println!("expecting char input:");                        
                std::io::stdin().read_line(&mut input).expect("issue reading input");
                let characters:Vec<char> = input.chars().collect();
                mem_arr[mem_position] = characters[0] as u8;
            }, 
            '.' => {print!("{}", mem_arr[mem_position] as char)},
            _ => {},
        } ;
        file_index +=1;
        debug_data.push(create_debug_string(&token, mem_position, &mem_arr, file_index));
    }
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