use std::collections::VecDeque;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Segment {
    Constant, // 0000
    Local, // 0001
    Argument, // 0010
    Static, // 0011
    This,  // 0100
    That, // 0101
    Temp, 
    Pointer, // supplies pointer -> 0 This and pointer -> 1 That
}

#[derive(Debug)]
pub enum VmLine {
    PushPopCommand { cmd_bit: bool, seg: Segment, val: i16 }, // push -> 0, pop -> 1
    ArithmeticCommand { cmd_bit: bool }, // 0 -> add, 1 -> sub
}

const INIT_CMD: &str = "@256\nD=A\n@SP\nM=D\n0; JMP\n";
// add sub push pop commands are in this structure
const ARR: [&str; 4] = [ 
"@SP\nAM=M-1\nD=M\nA=A-1\nM=D+M\n",
"@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n",
"@SP\nA=M\nM=D\n@SP\nM=M+1\n", 
"@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n"];

#[derive(Debug)]
pub struct Parser {
    pub commands: VecDeque<Option<VmLine>>,
    pub commands_imp: VecDeque<VmLine>, 
}

impl Parser {
    pub fn new() -> Option<Parser> {
        let commands_imp = VecDeque::new();
        Some(Parser { commands: VecDeque::new(), commands_imp })
    }
   
    pub fn parse(&mut self, token_vector:&mut Vec<&str>) -> Result<()> {
        let current = &mut *token_vector;
        match current.len() {
            1 => {
                let t = *current.get(0).unwrap();
                let opt = self.handle_arithmetic_command(t).unwrap();
                Ok(opt)
            },
            3 => {
                let t0 = *current.get(0).unwrap();
                let t1 = *current.get(1).unwrap();
                let t2 = *current.get(2).unwrap();
                let opt = self.handle_pushpop_command(t0, t1, t2).unwrap();
                Ok(opt)
            },
            _ => panic!("not possible"),
        }
    }

    pub fn parse_better(&mut self, tokens:&mut Vec<&str>) -> Result<()> {
        let mut current = tokens.iter();
        while let Some(&t) = current.next() {
            println!("{t:?}");
        };
        Ok(())
    }

    pub fn convert_to_asm(&mut self) -> Result<Box<String>> {
        let mut sptr = Box::new(String::from(""));
        while let Some(vml) = self.commands.pop_front() {
            let vml = vml.unwrap();
            match Parser::decode_vmline(&vml) {
                Ok(s) => sptr.push_str(s.as_str()),
                Err(why) => eprintln!("an error occurred: {why:?}"),
            };
        };
        Ok(sptr)
    }

    fn decode_vmline(vml: &VmLine) -> Result<String> {
        // maybe static array to define const register strings
        match vml {
            VmLine::PushPopCommand { cmd_bit, seg, val } => {
                let c = match cmd_bit {
                    false => format!("{}", ARR[2]),
                    true => format!("{}", ARR[3]),
                };
                let d = match seg {
                    Segment::Constant => format!("@{val}\n"), // @val\n
                    Segment::Local => format!(""), // 
                    Segment::Argument => format!(""),
                    Segment::This => format!(""),
                    Segment::That => format!(""),
                    Segment::Static => format!(""),
                    Segment::Temp => format!(""),
                    Segment::Pointer => format!(""),
                };
                let mut sptr = Box::new(String::from("")); 
                sptr.push_str(d.as_str());
                sptr.push_str(c.as_str());
                Ok(*sptr)
            },
            VmLine::ArithmeticCommand { cmd_bit } => {
                let c = match cmd_bit {
                    false => format!("{}", ARR[0]),
                    true => format!("{}", ARR[1]),
                };
                Ok(c)
            },
        }
    }

    fn handle_arithmetic_command(&mut self, token: &str) -> Result<()> {
        let o = match token {
            "add" => Some(VmLine::ArithmeticCommand { cmd_bit: false }),
            "sub" => Some(VmLine::ArithmeticCommand { cmd_bit: true }),
            _ => None,
        };
        Ok(self.commands.push_back(o))
    }

    fn handle_pushpop_command(&mut self, t0: &str, t1: &str, t2: &str) -> Result<()> {
        let seg = match t1 {
            "constant" => Some(Segment::Constant),
            "local" => Some(Segment::Local),
            "argument" => Some(Segment::Argument),
            "static" => Some(Segment::Static),
            "this" => Some(Segment::This),
            "that" => Some(Segment::That),
            "temp" => Some(Segment::Temp),
            _ => None,
        }.unwrap();
        let val = t2.parse::<i16>().unwrap_or_else(|_| 0);
        let o = match t0 {
            "push" => Some(VmLine::PushPopCommand { cmd_bit: false, seg, val }),
            "pop" => Some(VmLine::PushPopCommand { cmd_bit: true, seg, val }),
            _ => None,
        };
        Ok(self.commands.push_back(o))
    }

}
