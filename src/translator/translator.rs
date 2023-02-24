use std::{collections::HashMap, fs, io};

use super::{parser::Parser, codewriter::{IOWrite, CodeWriter}};

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
        Ok(opt) => println!("parsing done: {opt:?}"),
        Err(why) => eprintln!("{why:?}"),
    };
    /*let write_resp = match tr.write_wm_code(fname) {
        Ok(wopt) => println!("Writing done: {wopt:?}"),
        Err(why) => eprintln!("{why:?}"),
    };*/
    Ok(res)
}

trait CodeReader {
    fn read_vm_code(&mut self, fname: &str) -> io::Result<()>;
}

pub struct Translator {
    map: HashMap<u32, String>,
}

impl Translator {
    fn new() -> Option<Translator> {
        Some(Translator { map: HashMap::new() })
    }

    fn write_wm_code(&mut self, fname: &str) -> io::Result<()> {
        let mut w = CodeWriter::with_filepath(fname).unwrap();
        let wres = match w.write_asm("dd") {
            Err(why) => panic!("cannot write to file: {why:?}"),
            Ok(_) => println!("write was successful"),
        };
        Ok(wres) 
    }

    fn display_map(&self) {
        self.map.iter().for_each(|(&k, v)| println!("key = {}, vm_code = {}", k, v));
    }

    fn store_pair(&mut self, pair: (u32, &str)) -> Result<(), Box<dyn std::error::Error>> {
        self.map.insert(pair.0, pair.1.to_string());
        Ok(())
    }
}

impl CodeReader for Translator {
    
    fn read_vm_code(&mut self, fname: &str) -> std::io::Result<()> {
       let contents_raw = fs::read_to_string(fname)?;
       contents_raw.lines().enumerate().filter_map(|(i, line)| {
           if !(line.is_empty() || line.starts_with("//")) {
               Some((i, line))
           } else {
               None
           }
       }).for_each(|(i, line)| {
               let res = self.store_pair((i as u32, line));
               match res {
                   Err(why) => eprintln!("cannot insert pair: {why:?}"),
                   Ok(_) => {},
               }
        });
       let m = &self.map;
       let mut p = Parser::new().unwrap();
       for (_, v) in m.iter() {
           let mut token_vector = v.split_ascii_whitespace().collect::<Vec<&str>>();
           match p.parse(&mut token_vector) {
               Err(why) => eprintln!("cannot parse token {token_vector:?}: {why:?}"),
               Ok(opt) => println!("{opt:?} done"),
           };
           
       }
       Ok(())
   } 
}

