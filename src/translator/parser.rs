
#[derive(Debug)]
pub enum Segment {
    StackPointee, // 0000
    Local, // 0001
    Argument, // 0010
    Static, // 0011
    This,  // 0100
    That, // 0101
}


#[derive(Debug)]
pub enum VmLine {
    PushPopCommand { cmd_bit: bool, seg: Segment, val: i16 },
    ArithmeticCommand { cmd_bit: bool }, // 0 -> add, 1 -> sub
}

#[derive(Debug)]
pub struct Parser {
    current_command: Option<VmLine>,
}

impl Parser {
    pub fn new() -> Option<Parser> {
        Some(Parser { current_command: None })
    }
   
    // FIX THE BUG THAT RESULTS IN INF LOOP
    pub fn parse(&mut self, token_vector:&mut Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        let current = &mut *token_vector;
        match current.len() {
            1 => {
                let t = *current.get(0).unwrap();
                match t {
                    "add" => { 
                        let cmd = Some(VmLine::ArithmeticCommand { cmd_bit: false });
                        return self.set_current_command(cmd);
                    },
                    "sub" => { 
                        let cmd = Some(VmLine::ArithmeticCommand { cmd_bit: true });
                        return self.set_current_command(cmd);
                    },
                    _ => {},
                }
            },
            3 => {
                let t0 = *current.get(0).unwrap();
                let t1 = *current.get(1).unwrap();
                let t2 = *current.get(2).unwrap();
                let val = t2.parse::<i16>().unwrap();

                let cmd_bit = Parser::set_cmdbit(t0).unwrap();
                let seg = Parser::set_segment(t1).unwrap();

                let cmd = Some(VmLine::PushPopCommand { cmd_bit, seg, val }); 
                return self.set_current_command(cmd);
            },
            _ => { panic!("not possible"); },
        } 
        
        Ok(())
    }
     
    fn set_current_command(&mut self, vm_line: Option<VmLine>) -> Result<(), Box<dyn std::error::Error>> {
        self.current_command = vm_line;
        Ok(()) 
    }

    fn set_cmdbit(token: &str) -> Option<bool> {
        match token {
            "push" => Some(false),
            "pop" => Some(true),
            _ => None,
        } 
    }

    fn set_segment(token: &str) -> Option<Segment> {
        match token {
            "local" => Some(Segment::Local),
            "argument" => Some(Segment::Argument),
            "static" => Some(Segment::Static),
            "this" => Some(Segment::This),
            "that" => Some(Segment::That),
            _ => None,
        } 
    }
}
