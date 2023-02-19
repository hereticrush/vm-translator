use std::{fs::{self, File}, path::Path, io::{self, Write}};

pub trait IOWrite {
   fn write_asm(&mut self, line: &str) -> io::Result<()>;
}

struct CodeWriter {
    cursor: usize,
    output_file: File,
}

impl IOWrite for CodeWriter {
    fn write_asm(&mut self, line: &str) -> io::Result<()> {
        match self.output_file.write_all(line.as_bytes()) {
            Err(why) => panic!("cannot write into asm file: {why:?}"),
            Ok(_) => Ok(())
        } 
    }
}

impl CodeWriter {
    pub fn with_filepath(path: &Path) -> Option<CodeWriter> {
        let display = path.display();
        let asm_path = path.with_extension("asm");
        Some(CodeWriter { 
            cursor: 0, 
            output_file: match File::create(asm_path) {
                Err(why) => panic!("could not create {display:?}: {why:?}"),
                Ok(file) => file,
            },
        })
    }
}
