use std::fs;
use std::env;
fn main() -> Result<(), std::io::Error> {
   
    println!("----- BFI v0.0.1B.Debug -----");
   
    let args: Vec<String> = env::args().collect();
    let path: &String = &args[1];
    
    println!("loading from file: {}", args[1]);
    
    let filedata = fs::read_to_string(path).expect("Error reading the file given as first argument.");
    let token_array: &Vec<char> = &filedata.chars().collect(); 
    
    // whole bf mem array
    let mut mem_arr:Vec<u8> = vec![0; 10_000];
     
    // start halfway through the array
    let mut mem_position:usize = 4_999;
    
    let mut debug_data: Vec<String> = Vec::new();
    
    let mut file_index = 0;

    while filedata.capacity() > file_index {
        let token = &token_array[file_index];
        //pythoners have to use copious amounts of nesting to do this (losers)
        match token {
            '+' => {mem_arr[mem_position] += 1}, 
            '-' => {mem_arr[mem_position] -= 1},
            '<' => {mem_position -= 1},
            '>' => {mem_position += 1},
            '[' | ']'=> {println!("loops not implemented yet")},
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
        debug_data.push(create_debug_string(&token, mem_position, &mem_arr));
    }
    
    std::fs::write("dump.txt", debug_data.concat())?;
    
    //-----END-----     
    Ok(())

}

fn create_debug_string (tok:&char, arr_pos:usize, arr:&Vec<u8>) -> String {
    let token = *tok;
    if token == '\r' || token == '\n' {
        return String::from("\nDetected newline")
    }
    let message = format!("\r\n Operation {token} at memory cell {} resulting in character {} (ASCII reference {})",
         arr_pos, arr[arr_pos] as char,
         arr[arr_pos] as u8);
    
    String::from(message)
}
