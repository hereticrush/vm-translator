use std::{collections::HashMap, fs, io, cell::RefCell, rc::{Rc, Weak}};

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
    match tr.read_vm_code(fname) {
        Ok(_) => tr.notify(&Event::Reading),
        Err(why) => eprintln!("{why:?}"),
    };
    match tr.write_wm_code(fname) {
        Ok(_) => tr.notify(&Event::Writing),
        Err(why) => eprintln!("{why:?}"),
    };
    Ok(())
}

#[derive(Debug)]
enum Event {
   Reading,
   Writing,
}

trait IOListener {
    fn notify(&mut self, event: &Event);
}

trait CodeReader {
    fn read_vm_code(&mut self, fname: &str) -> io::Result<()>;
}

pub struct Translator {
    map: HashMap<u32, String>,
    state: Option<Event>,
    pptr: Rc<RefCell<Parser>>,
}

impl Translator {
    fn new() -> Option<Translator> {
        let map = HashMap::new();
        let state = None;
        let pptr = Rc::new(RefCell::new(Parser::new().unwrap()));
        Some(Translator { map, state, pptr })
    }

    // Entry point to output
    fn write_wm_code(&mut self, fname: &str) -> io::Result<()> {
        let mut w = CodeWriter::with_filepath(fname).unwrap();
        let current_cmd = &self
            .pptr
            .borrow_mut().convert_to_asm().unwrap(); 
        let wres = match w.write_asm(current_cmd) {
            Err(why) => panic!("cannot write to file {}: {why:?}", w.file_name()),
            Ok(_) => {},
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
               Ok(_) => {},
           };
           
       }
       Ok(())
   } 
}

impl IOListener for Translator {
    fn notify(&mut self, event: &Event) {
        if let Event::Reading = event {
            println!("Reading file..");
        } else if let Event::Writing = event {
            println!("Writing to file...");
        }
    }
}
