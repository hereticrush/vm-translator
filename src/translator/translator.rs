use std::{collections::HashMap, fs};

use super::parser::Parser;

fn init_translator() -> Translator {
    let tr = Translator::new().unwrap(); 
    tr
}
pub fn translate(fname: &str) -> std::io::Result<()> {
    let mut tr = init_translator();
    let res = match tr.read_vm_code(fname) {
        Ok(opt) => println!("done: {opt:?}"),
        Err(e) => eprintln!("{e:?}"),
    };
    Ok(res)
}

trait CodeReader {
    fn read_vm_code(&mut self, fname: &str) -> std::io::Result<()>;
}

trait CodeWriter {
    fn insert_tokens_to_table(&mut self);
    fn write_hack_asm(&mut self);
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
       let tokens: Vec<&str> = contents_raw.split_whitespace().collect();
       let mut p = Parser::new().unwrap();
       let opt_res = p.parse_tokens(tokens);
       Ok(())
   } 
}

impl CodeWriter for Translator {
    fn insert_tokens_to_table(&mut self) {
        let m = &self.map;

        let _pairs = m.iter().map(|(k, v)| (k, v))
            .for_each(|pair| println!("{pair:?}"));
    }
    fn write_hack_asm(&mut self) {
        
    }
}

fn handle_translation(contents: &str, mut w: Box<dyn CodeWriter>) -> std::io::Result<()> {
    
    w.write_hack_asm(); 
    Ok(())
}
