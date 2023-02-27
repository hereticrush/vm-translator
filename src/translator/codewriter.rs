use std::{fs::{self, File}, path::Path, io::{self, Write}};

pub trait IOWrite {
   fn write_asm(&mut self, result_line: &str) -> io::Result<()>;
}

pub struct CodeWriter {
    file_name: String,
    output_file: File,
}

impl IOWrite for CodeWriter {
    fn write_asm(&mut self, result_line: &str) -> io::Result<()> {
        self.write_asm_code(result_line) 
    }
}

/*
 * TRY TO PASS A PARSER REF TO CODEWRITER FUNCTION AND USE ITS
 * CURRENT COMMAND
 * */
impl CodeWriter {
    pub fn with_filepath(fname: &str) -> Option<CodeWriter> {
        let path = Path::new(fname);
        let display = path.display();
        let asm_path = path.with_extension("asm");
        Some(CodeWriter { 
            file_name: asm_path.to_string_lossy().to_string(),
            output_file: match File::create(asm_path) {
                Err(why) => panic!("could not create {display:?}: {why:?}"),
                Ok(file) => file,
            },
        })
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }

    fn write_asm_code(&mut self, result_line: &str) -> io::Result<()> {
        match self.output_file.write_all(result_line.as_bytes()) {
            Err(why) => panic!("cannot write to file {}: {why:?}", self.file_name),
            Ok(_) => Ok(println!("Successfully written in file: {}", self.file_name)),
        }
    }

}


