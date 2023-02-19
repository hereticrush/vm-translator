use std::collections::HashMap;


enum Command {
    Add, // 0000 0000
    Sub,     // 0000 0001
    Push, // 0000 0010
    Pop, // 0000 0011
    Label, // 0000 0100
    Func, // 0000 0101
}

enum Segment {
    StackPointee, // 0000
    Local, // 0001
    Argument, // 0010
    Static, // 0011
    This,  // 0100
    That, // 0101
}

enum Value {
    Val(u16),
}

pub enum VmLine {
    X(Command, Segment, Value),
    Y(Command),
}

#[derive(Debug)]
pub struct Parser {
    map: HashMap<usize, String>,
}

impl Parser {
    pub fn new() -> Option<Parser> {
        Some(Parser { map: HashMap::new() })
    }
    fn is_valid_line(token: &str) -> bool {
        if token.is_empty() || token.starts_with("//") {
            return false; 
        }
        true
    }

    pub fn parse_valid_tokenlines(&mut self, lines: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        
        // Flatten the vector of vector of valid vm strs
        let v: Vec<&str> = lines.iter()
            .filter_map(|&line| {
                if Parser::is_valid_line(line) {
                    Some(line) 
                } else {
                    None
                }
            }).flat_map(|token| token.split_ascii_whitespace()).collect();
        v.iter().for_each(|&token| {
            self.map_to_enums(token);
        });
        Ok(println!("parsing done!"))
    }

    // [push, static, 10, push, argument, 2, pop, static, 10, add]
    fn map_to_enums(&mut self, token: &str) -> Option<VmLine> {
        if token.parse::<u16>().is_ok() {
            let num = token.parse::<u16>().unwrap();
            let value = Value::Val(num); 
        } 
        None

    } 

}
