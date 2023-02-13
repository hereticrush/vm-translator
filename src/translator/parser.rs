
#[derive(Debug)]
pub struct Parser {
}

impl Parser {
    pub fn new() -> Option<Parser> {
        Some(Parser {})
    }
    fn is_valid_line(tokens: Vec<&str>) {
        for token in tokens {
           if token.is_empty() || token.contains("//") {
              continue; 
           }
        }
    }

    pub fn parse_tokens(&mut self, tokens: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
        
        Ok(())
    }
}
