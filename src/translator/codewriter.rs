use std::{fs::{File, OpenOptions}, path::Path, io::{self, Write}};

pub trait IOWrite {
   fn write_asm(&mut self, result_line: &str) -> io::Result<()>;
}

#[derive(Debug)]
pub struct CodeWriter {
    file_name: String,
    output_file: File,
}

impl IOWrite for CodeWriter {
    fn write_asm(&mut self, result_line: &str) -> io::Result<()> {
        self.write_asm_code(result_line) 
    }
}

impl CodeWriter {
    pub fn with_filepath(fname: &str) -> Option<CodeWriter> {
        let path = Path::new(fname);
        let display = path.display();
        let asm_path = path.with_extension("asm");
        let file = OpenOptions::new().write(true).create(true).open(asm_path.to_str()?);
        Some(CodeWriter { 
            file_name: asm_path.to_str()?.to_string(),
            output_file: match file {
                Err(why) => panic!("could not create {display:?}: {why:?}"),
                Ok(file) => file,
            },
        })
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    fn write_asm_code(&mut self, result_line: &str) -> io::Result<()> {
        match self.output_file.write(result_line.as_bytes()) {
            Err(why) => panic!("cannot write to file {}: {why:?}", self.file_name),
            Ok(_) => Ok(println!("Successfully written in file: {}", self.file_name)),
        }
    }

}


