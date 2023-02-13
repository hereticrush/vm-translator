use std::collections::HashMap;

#[derive(Debug)]
pub struct Parser {
    map: HashMap<usize, String>,
}

impl Parser {
    pub fn new() -> Option<Parser> {
        Some(Parser { map: HashMap::new() })
    }
    pub fn parse_tokens(&mut self, tokens: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
