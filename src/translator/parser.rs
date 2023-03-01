use std::collections::VecDeque;


#[derive(Debug)]
pub enum Segment {
    StackPointer, // 0000
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
    PointerCommand { cmd_bit: bool, val: i16 }, // 0 -> this, 1 -> that
}

#[derive(Debug)]
pub struct Parser {
    pub commands: VecDeque<Option<VmLine>>,
}

impl Parser {
    pub fn new() -> Option<Parser> {
        Some(Parser { commands: VecDeque::new() })
    }
   
    pub fn parse(&mut self, token_vector:&mut Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let current = &mut *token_vector;
        match current.len() {
            1 => {
                let t = *current.get(0).unwrap();
                let cmd_bit = Parser::set_cmdbit(t).unwrap();
                let cmd = Some(VmLine::ArithmeticCommand { cmd_bit });
                Ok(self.commands.push_back(cmd))
            },
            3 => {
                let t0 = *current.get(0).unwrap();
                let t1 = *current.get(1).unwrap();
                let t2 = *current.get(2).unwrap();
                let val = t2.parse::<i16>().unwrap_or_else(|_| 0);
                let cmd_bit = Parser::set_cmdbit(t0).unwrap();
                let seg = Parser::set_segment(t1).unwrap();

                let cmd = Some(VmLine::PushPopCommand { cmd_bit, seg, val }); 
                Ok(self.commands.push_back(cmd))
            },
            _ => panic!("not possible"),
        }
    }

    /*
     *          Some(VmLine::PushPopCommand { cmd_bit, seg, val }) => format!("This is push_pop line {} {:?} {}", cmd_bit, seg, val),      
                Some(VmLine::ArithmeticCommand { cmd_bit }) => format!("This is add sub line {}", cmd_bit),
                Some(VmLine::PointerCommand { cmd_bit, val }) => format!("This or that line {} {}", cmd_bit, val),
                None => format!(""),
     * */
    // FIX THIS ASAP
    pub fn convert_to_asm(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(vml) = self.commands.pop_front() {
            println!("{vml:?}");
        };
        Ok(())
    }

    fn set_cmdbit(token: &str) -> Option<bool> {
        match token {
            "add" => Some(false),
            "sub" => Some(true),
            "push" => Some(false),
            "pop" => Some(true),
            _ => None,
        } 
    }

    fn set_segment(token: &str) -> Option<Segment> {
        match token {
            "constant" => Some(Segment::StackPointer),
            "local" => Some(Segment::Local),
            "argument" => Some(Segment::Argument),
            "static" => Some(Segment::Static),
            "this" => Some(Segment::This),
            "that" => Some(Segment::That),
            "temp" => Some(Segment::Temp),
            _ => None,
        } 
    }
}
