use std::{collections::VecDeque, cell::{Cell, RefCell, RefMut}, rc::{Rc, Weak}};

use super::{handler::handler::{Handler, CommandHandler, MultipleCommandHandler}, error::error::VmError, expr::expr::VirtualMachineExpr};

pub type Result<T> = std::result::Result<T, VmError>;

pub struct Parser {
    pub commands: VecDeque<Option<VirtualMachineExpr>>,
}

impl<'a> Parser {
    pub fn new() -> Option<Parser> {
        let commands = VecDeque::new();
        Some(Parser { commands })
    }

    fn handle_command<T: Handler<'a>>(&self, handler: &T, tokens: &mut Vec<&str>) -> Result<String> {
        let s = match handler {
            CommandHandler => handler.handle(tokens),
            MultipleCommandHandler => handler.handle(tokens),
        };
        if let Err(e) = s {
            panic!("something cannot be handled {e:?}")
        } else {
            let a = s.unwrap();
            Ok(a)
        }
    }
   
    pub fn parse(&self, tokens:&mut Vec<&str>) -> Result<String> {
        let length = &tokens.len();
        match length {
            1 => {
                let chandler = CommandHandler::new();
                self.handle_command(&chandler, tokens)
            },
            2|3 => {
                let mhandler = MultipleCommandHandler::new();
                self.handle_command(&mhandler, tokens)
            },
            _ => {
                let e = VmError::ParsingError(std::fmt::Error::default());
                Err(e)
            },
        }
    }

    pub fn convert_to_asm(&mut self) -> Result<Box<String>> {
        let mut sptr = Box::new(String::from(""));
        while let Some(vml) = self.commands.pop_front() {
            let vml = vml.unwrap();
            /*match self.handle_vmline(&vml) {
                Ok(s) => sptr.push_str(s.as_str()),
                Err(why) => eprintln!("an error occurred: {why:?}"),
            };*/
        };
        Ok(sptr)
    }

}
