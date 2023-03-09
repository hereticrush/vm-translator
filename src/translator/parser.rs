use std::collections::VecDeque;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Segment {
    Constant,
    Local,
    Argument,
    Static,
    This,
    That,
    Temp,
    Pointer, // supplies pointer -> 0 This and pointer -> 1 That
}

#[derive(Debug)]
pub enum VmLine {
    PushPop { cmd_bit: bool, seg: Segment, val: i16 }, // push -> 0, pop -> 1
    Arithmetic { cmd_bit: bool }, // 0 -> add, 1 -> sub
    Label { func_name: String },
    LogicalOperation { flag: bool, val: u8 },  // if flag == 0 then eq = false else eq = true for lt gt; 
                                               // neg -> 0, and -> 1, or -> 2, not -> 3, lt -> 4,
                                               // gt -> 5 
}

const INIT_CMD: &str = "@256\nD=A\n@SP\nM=D\n0; JMP\n";
const CMD_ARR: [&str; 4] = [ 
    "@SP\nAM=M-1\nD=M\nA=A-1\nM=D+M\n", // ADD
    "@SP\nAM=M-1\nD=M\nA=A-1\nM=M-D\n", // SUB
    "@SP\nA=M\nM=D\n@SP\nM=M+1\n", // PUSH
    "@R13\nM=D\n@SP\nAM=M-1\nD=M\n@R13\nA=M\nM=D\n" // POP
];

const LOGICAL_ARR: [&str; 4] = [
    "@SP\nA=M-1\nM=!M\n", // NOT
    "@SP\nA=M-1\nM=-M\n", // NEG
    "@SP\nAM=M-1\nD=M\nA=A-1\nM=D&M\n", // AND
    "@SP\nAM=M-1\nD=M\nA=A-1\nM=D|M\n", // OR
];

#[derive(Debug)]
pub struct Parser {
    pub commands: VecDeque<Option<VmLine>>,
}

impl Parser {
    pub fn new() -> Option<Parser> {
        let commands = VecDeque::new();
        Some(Parser { commands })
    }
   
    pub fn parse(&mut self, tokens:&mut Vec<&str>) -> Result<()> {
        if let Some((&first, args)) = tokens.split_first() {
            let opt = match first {
                "push"|"pop" => { 
                    let seg = *args.get(0).unwrap();
                    let val = *args.get(1).unwrap();
                    self.handle_pushpop_command(first, seg, val)
                },
                _ => Ok(()),
            };
        } else {
            let opt = match tokens.iter().next() {
                Some(&t) => match t {
                    "add"|"sub" => self.handle_arithmetic_command(t),
                    "label" => self.handle_label(t),
                    "not"|"neg"|"and"|"or"|"eq"|"lt"|"gt" => self.handle_logical_ops(t),
                    _ => Ok(()),
                },
                None => Ok(()),
            };
        }
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
        let mut sptr = Box::new(String::from(""));
        match vml {
            VmLine::PushPop { cmd_bit, seg, val } => {
                // push actions
                if cmd_bit.eq(&false) {
                    let s = match seg {
                        Segment::Constant => format!("@{val}\nD=A\n"), // @val\n
                        Segment::Local => format!("@LCL\nD=M\n@{val}\nA=D+A\nD=M\n"), 
                        Segment::Argument => format!("@ARG\nD=M\n@{val}\nA=D+A\nD=M\n"),
                        Segment::This => format!("@THIS\nD=M\n@{val}\nA=D+A\nD=M\n"),
                        Segment::That => format!("@THAT\nD=M\n@{val}\nA=D+A\nD=M\n"),
                        Segment::Static => unimplemented!(),
                        Segment::Temp => format!("@R5\nD=A\n@{val}\nA=D+A\nD=M\n"),
                        Segment::Pointer => match val {
                            0 => format!("@THIS\nD=M\n"),
                            1 => format!("@THAT\nD=M\n"),
                            _ => panic!("impossible value for pointer segment"),
                        },
                    };
                    sptr.push_str(s.as_str());
                    sptr.push_str(CMD_ARR[2]);
                } else { // pop actions
                    let s = match seg {
                        Segment::Constant => format!("@{val}\nD=M\n"), // @val\n
                        Segment::Local => format!("@LCL\nD=M\n@{val}\nD=D+A\n"), 
                        Segment::Argument => format!("@ARG\nD=M\n@{val}\nD=D+A\n"),
                        Segment::This => format!("@THIS\nD=M\n@{val}\nD=D+A\n"),
                        Segment::That => format!("@THAT\nD=M\n@{val}\nD=D+A\n"),
                        Segment::Static => unimplemented!(),
                        Segment::Temp => format!("@R5\nD=A\n@{val}\nD=D+A\n"),
                        Segment::Pointer => match val {
                            0 => format!("@THIS\nD=A\n"),
                            1 => format!("@THAT\nD=A\n"),
                            _ => panic!("impossible value for pointer segment"),
                        },
                    };
                    sptr.push_str(s.as_str());
                    sptr.push_str(CMD_ARR[3]);
                }
                Ok(*sptr)
            },
            VmLine::Arithmetic { cmd_bit } => {
                let c = match cmd_bit {
                    false => format!("{}", CMD_ARR[0]),
                    true => format!("{}", CMD_ARR[1]),
                };
                sptr.push_str(c.as_str());
                Ok(*sptr)
            },
            VmLine::LogicalOperation { flag, val } => {
                let opr = match flag {
                    false => match val {
                        0 => format!("{}", LOGICAL_ARR[0]),
                        1 => format!("{}", LOGICAL_ARR[1]),
                        2 => format!("{}", LOGICAL_ARR[2]),
                        3 => format!("{}", LOGICAL_ARR[3]),
                        _ => panic!("impossible value"),
                    },
                    true => unimplemented!(),
                };
                sptr.push_str(opr.as_str());
                Ok(*sptr)
            },
            VmLine::Label { func_name } => {
               let s = format!("({func_name}$)\n");
               sptr.push_str(s.as_str());
               Ok(*sptr)
            },
        }
    }

    fn handle_arithmetic_command(&mut self, token: &str) -> Result<()> {
        let o = match token {
            "add" => Some(VmLine::Arithmetic { cmd_bit: false }),
            "sub" => Some(VmLine::Arithmetic { cmd_bit: true }),
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
            "pointer" => Some(Segment::Pointer),
            _ => None,
        }.unwrap();
        let val = t2.parse::<i16>().unwrap_or_else(|_| 0);
        let o = match t0 {
            "push" => Some(VmLine::PushPop { cmd_bit: false, seg, val }),
            "pop" => Some(VmLine::PushPop { cmd_bit: true, seg, val }),
            _ => None,
        };
        Ok(self.commands.push_back(o))
    }

    fn handle_logical_ops(&mut self, token: &str) -> Result<()> {
        let opr = match token {
            "not" => Some(VmLine::LogicalOperation { flag: false, val: 0 }),
            "neg" => Some(VmLine::LogicalOperation { flag: false, val: 1 }),
            "and" => Some(VmLine::LogicalOperation { flag: false, val: 2 }),
            "or" => Some(VmLine::LogicalOperation { flag: false, val: 3 }),
            "lt" => Some(VmLine::LogicalOperation { flag: false, val: 4 }),
            "gt" => Some(VmLine::LogicalOperation { flag: false, val: 5 }),
            "eq" => Some(VmLine::LogicalOperation { flag: true, val: 0 }),
            _ => None,
        };
        Ok(self.commands.push_back(opr))
    }

    fn handle_label(&mut self, token: &str) -> Result<()> {
        let func_name = String::from(token).clone();
        let o = Some(VmLine::Label { func_name });
        Ok(self.commands.push_back(o))
    }

}
