use std::{rc::Rc, cell::RefCell};




pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Handler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a>;
    fn handle(&self, tokens:&mut Vec<&str>) -> Result<String>;
}

pub struct CommandHandler<'a> {
    commands: Option<Vec<&'a str>>,
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> CommandHandler<'a> {
    pub fn new() -> CommandHandler<'a> {
        CommandHandler { commands: None, next: None } 
    }
}

impl<'a> Handler<'a> for CommandHandler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
    fn handle(&self, tokens:&mut Vec<&str>) -> Result<String> {
        if let Some(token) = tokens.pop() {
            let result_string = match token {
                "add" => "A",
                "sub" => "S",
                "not" => "NOT",
                _ => panic!("no such command"),
            };
            Ok(result_string.to_string())
        } else {
            panic!("error: tokens vector length cannot be zero.")
        } 
    }
}

pub struct MultipleCommandHandler<'a> {
    commands: Option<Vec<&'a str>>,
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> MultipleCommandHandler<'a> {
    pub fn new() -> MultipleCommandHandler<'a> {
        MultipleCommandHandler { commands: None, next: None }
    }
}

impl<'a> Handler<'a> for MultipleCommandHandler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }
    fn handle(&self, tokens:&mut Vec<&str>) -> Result<String> {
        if let Some(&token) = tokens.first() {
            let result_string = match token {
                "push" => "PUSH",
                "pop" => "POP",
                _ => "NULL",
            };
            Ok(result_string.to_string())
        } else {
            panic!("error: no such command.")
        }
    }
}
