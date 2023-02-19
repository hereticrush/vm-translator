use std::{collections::HashMap, fs, io};

use super::parser::Parser;

fn init_translator() -> Translator {
    let tr = Translator::new(); 
    match tr {
        Some(tr) => tr,
        None => panic!("error: cannot initialize translator."),
    }
}
pub fn translate(fname: &str) -> std::io::Result<()> {
    let mut tr = init_translator();
    let res = match tr.read_vm_code(fname) {
        Ok(opt) => println!("Translation done: {opt:?}"),
        Err(e) => eprintln!("{e:?}"),
    };
    Ok(res)
}

trait CodeReader {
    fn read_vm_code(&mut self, fname: &str) -> std::io::Result<()>;
}

trait CodeWriter {
    fn write_hack_asm(&mut self) -> io::Result<()>;
}

pub struct Translator {
    map: HashMap<u32, String>,
}

impl Translator {
    fn new() -> Option<Translator> {
        Some(Translator { map: HashMap::new() })
    }
}

impl CodeReader for Translator {
    
    fn read_vm_code(&mut self, fname: &str) -> std::io::Result<()> {
       let contents_raw = fs::read_to_string(fname)?;
       let tokens: Vec<&str> = contents_raw.lines().collect();
       let mut p = Parser::new().unwrap();
       let opt = p.parse_valid_tokenlines(tokens).unwrap();
       Ok(opt)
   } 
}

impl CodeWriter for Translator {
    
    fn write_hack_asm(&mut self) -> io::Result<()> {
        
        Ok(()) 
    }
}

fn handle_write(contents: &str, mut w: Box<dyn CodeWriter>) -> std::io::Result<()> {
    
    w.write_hack_asm(); 
    Ok(())
}
