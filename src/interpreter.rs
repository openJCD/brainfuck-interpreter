use crate::tokenizer::*;
use std::{collections::HashMap, fs::OpenOptions, io::Write};

pub fn execute_instructions(tokens:Vec<Tokens>, loop_map:HashMap<usize, usize>, debug_enable:bool) -> Result<(), String> {
    
    let map = &loop_map;

    let mut index = 0;
    let debug_output_buf;
    debug_output_buf = Vec::<String>::new();
    std::fs::File::create(".dump").expect("error creating the dumpfile");
    let mut debug_dumpfile = OpenOptions::new().append(true).open(".dump").expect("could not open .dump file");
    let mut memory_map:Vec<u8> = vec![0;100_000];
    let mut memory_pointer:usize = 10_000;
    let mut loopcount = 0; 
    while index < tokens.len() {
        match tokens[index] {
            Tokens::IncrementCell => {
                memory_map[memory_pointer] +=1;
            },
            Tokens::DecrementCell => {
                memory_map[memory_pointer] -=1;  
            },
            Tokens::NextCell => {
                memory_pointer +=1;
            },
            Tokens::PreviousCell => {
                memory_pointer -=1;
            },
            Tokens::OpenLoop(_open) => {
                if memory_map[memory_pointer] == 0 {
                    let jmp = match map.get_key_value(&_open) {
                        Some(n) => n,
                        None => {
                            return Err{
                                0: String::from ("Error in finding the endpoint of loop")
                            }
                        }
                    };
                    if memory_map[memory_pointer] == 0 {
                        // might not work
                        index = *jmp.1 ;
                    }
                }  
            },
            Tokens::CloseLoop(_close) => {
                let val = match map.get_key_value(&_close) {
                    Some(v) => v,
                    None => {println!("Error in interpreting loop mapping.");  return Err{0:String::from("Could not find corresponding loop,")}}
                };
                if memory_map[memory_pointer] > 0 {
                    index = *val.1;
                    loopcount+=1;
                }
            },
            Tokens::OutputCell => {
                print!{"{}", memory_map[memory_pointer] as char};
            },
            Tokens::InputToCell => {
                let mut input = String::new();
                println!("Input character: ");
                std::io::stdin().read_line(&mut input).expect("input error");
                let c:Vec<char> = input.chars().collect();
                memory_map[memory_pointer] = c[0] as u8;
            },
            Tokens::Comment => {}
        }
        //if debug_enable {
        //    let buf = format!{"mem index: {memory_pointer} | tape index: {} | OPCODE: {:?} | ascii char no: {}\r\n", index, tokens[index], memory_map[memory_pointer]};
        //    debug_dumpfile.write_all(buf.as_bytes()).expect("error writing dumpfile");
        //}
        index +=1;
        if loopcount > 1_000_000_000 {
            return Err(String::from("Stack overflow - loop count exceeded 1 000 000 000, aborting..."))
        }
    }
    Ok(())
}