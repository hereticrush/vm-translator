use std::{fs, io, collections::HashMap, sync::Mutex, rc::{Rc, Weak}, cell::RefCell};

use lazy_static::lazy_static;

use super::{parser::Parser, codewriter::{IOWrite, CodeWriter}};

// TO BE IMPLEMENTED
const INIT_CMD: &str = "@256\nD=A\n@SP\nM=D\n0; JMP\n";

lazy_static! {
    static ref G_TABLE: Mutex<HashMap<usize, String>> = Mutex::new({
        let mut h = HashMap::new();
        h.insert(1, INIT_CMD.to_string());
        h
    });
}

fn init_translator() -> Translator {
    let tr = Translator::new(); 
    match tr {
        Some(tr) => tr,
        None => panic!("error: cannot initialize translator."),
    }
}

fn init_run_to_create_gtable(fname: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents_raw = fs::read_to_string(fname)?;
    contents_raw.lines().enumerate().filter_map(|(i, line)| {
       if !(line.is_empty() || line.starts_with("//")) {
           let pair = (i, line);
           Some(pair)
       } else {
           None
       }
    }).for_each(|pair| {
        G_TABLE.lock().unwrap().insert(pair.0.into(), pair.1.to_string().into());
    });

    Ok(())
}

pub fn translate(fname: &str) -> std::io::Result<()> {
    let listener = Rc::new(Listener {});
    let mut tr = init_translator().attach(listener);
    let pptr = Rc::new(RefCell::new(Parser::new().unwrap()));
    match tr.read_vm_tokens_from_table(pptr.clone(), fname) {
        Ok(_) => tr.notify(),
        Err(why) => eprintln!("{why:?}"),
    };
    match tr.write_asm_to_file(pptr, fname) {
        Ok(_) => tr.notify(),
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
    fn notify(&mut self);
    fn attach(self, listener: Rc<Listener>) -> Translator;
}

trait Translate {
    fn read_vm_tokens_from_table(&mut self, pptr: Rc<RefCell<Parser>>, fname: &str) -> io::Result<()>;
    fn write_asm_to_file(&mut self, pptr: Rc<RefCell<Parser>>, fname: &str) -> io::Result<()>;
}

#[derive(Debug)]
struct Listener {
    //pub subscribers: Option<Vec<u32>>,
}

pub struct Translator {
    state: Option<Rc<RefCell<Event>>>,
    listener: Option<Rc<Listener>>,
    translated_parts: Rc<RefCell<Vec<String>>>,
}

impl Translator {
    fn new() -> Option<Translator> {
        let translated_parts: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
        Some(Translator { state: None, listener: None, translated_parts })
    }
    fn state(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        let wrapper = Rc::new(RefCell::new(event));
        self.state = Some(wrapper);
        Ok(())
    }
}

impl Translate for Translator {
    fn write_asm_to_file(&mut self, pptr: Rc<RefCell<Parser>>, fname: &str) -> io::Result<()> {
        let _sres = self.state(Event::Writing);
        let mut wptr = Box::new(CodeWriter::with_filepath(fname.clone()).unwrap());
        let mut sptr = String::new();
        while let Some(value) = self.translated_parts.borrow_mut().pop() {
            let strlen = value.len();
            sptr.insert_str(strlen - 1, &value); 
        }
        match wptr.write_asm(sptr.as_str()) {
            Err(why) => panic!("cannot write to file {}: {why:?}", wptr.file_name()),
            Ok(_) => Ok(()),
        }
    }
    fn read_vm_tokens_from_table(&mut self, pptr: Rc<RefCell<Parser>>, fname: &str) -> io::Result<()> {
        let res = init_run_to_create_gtable(fname);
        let _sres = self.state(Event::Reading);
        if let Err(why) = res {
            panic!("ERROR: G_TABLE cannot be initialized: {why:?}");
        } 
        for (_, val) in G_TABLE.lock().unwrap().iter() {
            let mut tokens = val.split_ascii_whitespace().collect::<Vec<&str>>(); 
            if let Err(e) = pptr.borrow_mut().parse(&mut tokens) {
                 eprintln!("{}", e)
            } else {
                 let s = pptr.borrow().parse(&mut tokens).unwrap();
                 self.translated_parts.borrow_mut().push(s);
            }
        }
        Ok(())
    }
}

impl IOListener for Translator {
    fn notify(&mut self) {
        let s = self.state.clone().unwrap();
        let e = &*s.borrow_mut();
        match e {
            Event::Reading => println!("reading to file"),
            Event::Writing => println!("writing into file"),
        }
    }
    fn attach(mut self, listener: Rc<Listener>) -> Translator {
        self.listener = Some(listener);
        self
    }
}

#[cfg(test)]
mod translator_tests {
    use super::*;
    #[test]
    fn test_add_line_to_gtable() {
        let mut m = G_TABLE.lock().unwrap();
        let test_string = "teststr";
        m.insert(32, test_string.to_string());
    }

    #[test]
    fn test_gtable_first_item_is_initcmd() {
        let m = G_TABLE.lock().unwrap();
        assert_eq!(INIT_CMD, m.get(&1).unwrap());
    }
}
