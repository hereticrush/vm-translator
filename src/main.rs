use std::{path::Path, env, io};

use translator::translator::translate;
mod translator;
const PROGRAM_NAME: &str = "vm-translator";
fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {PROGRAM_NAME} <infile>");
        return Ok(());
    }
   
    let path = Path::new(&args[1]);
    assert_eq!("vm", path.extension().unwrap()); 
    if path.exists() {
        let fname = path.clone().to_str();
        match fname {
            Some(name) => {
               return translate(name); 
            },
            None => eprintln!("error: file name does not exist"),
        }
    } 
    Ok(())
}
