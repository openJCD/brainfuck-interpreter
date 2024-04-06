use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Tokens {
    IncrementCell,
    DecrementCell,
    NextCell,
    PreviousCell,
    OpenLoop(usize),
    CloseLoop(usize),
    OutputCell,
    InputToCell,
}

pub fn do_all_token_ops(data:String) ->(Vec<Tokens>, HashMap<usize, usize>) {
    let char_data = data.chars().collect();
    let tokens = tokenize(data);
    let hash = map_loops(char_data);
    //println!("{:?}", tokens);
    (tokens, hash)
}

/// Parse the characters into abstract tokens. Largely useful for 
pub fn tokenize(bf_data:String) -> Vec<Tokens> {
    let chars:Vec<char> = bf_data.chars().collect();
    let mut tokens:Vec<Tokens> = Vec::new();
    let mut parser_index:usize = 0;
    while parser_index < chars.len(){
        match chars[parser_index] {
            '+' => {tokens.push(Tokens::IncrementCell)},
            '-' => {tokens.push(Tokens::DecrementCell)},
            '>' => {tokens.push(Tokens::NextCell)},
            '<' => {tokens.push(Tokens::PreviousCell)},
            '[' => {tokens.push(Tokens::OpenLoop(parser_index))},
            ']' => {tokens.push(Tokens::CloseLoop(parser_index))},
            ',' => {tokens.push(Tokens::InputToCell)},
            '.' => {tokens.push(Tokens::OutputCell)},
            '\r' => {},
            '\n' => {},
            _ => {}
        };
        parser_index += 1;
    }
    tokens        
}

/// Pair up opening and closing loop tokens into a HashMap of corresponding file positions
pub fn map_loops (char_data:Vec<char>) -> HashMap<usize, usize> { 
    
    let mut map:HashMap<usize, usize> = HashMap::new();
    let mut index = 0;
    let mut stack = Vec::new();
    while index < char_data.len() {
        match char_data[index] {
            '[' => {
                stack.push(index);
            }
            ']' => {
                let n = match stack.pop() {
                    Some(x) => x,
                    None => {println!("Mismatched brackets at {index}!"); std::process::exit(1); }
                };
                map.insert(index, n);
            }
            _ => {}
        }
        //println!{"{:?}", stack}
        index +=1;
    }
    //println!("{:?}", map);
    map
}