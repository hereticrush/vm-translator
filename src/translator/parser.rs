
enum Command {
    Arithmetic,
    Push,
    Pop,
    Null,
}

enum Segment {
    StackPointee,
    Local,
    Argument,
    Static,
    This,
    That,
}

#[derive(Debug)]
pub struct Parser {

}

impl Parser {
    pub fn new() -> Option<Parser> {
        Some(Parser {})
    }
    fn is_valid_line(token: &str) -> bool {
        if token.is_empty() || token.starts_with("//") {
            return false; 
        }
        true
    }

    pub fn parse_valid_tokens(&mut self, tokens: Vec<&str>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut valid: Vec<&str> = tokens.iter().map(|&token| token)
            .filter(|&token| Parser::is_valid_line(token).eq(&true))
            .map(|token| token).collect();
        let s_valid = valid.iter_mut().map(|tokens| tokens.to_string()).collect();
        Ok(s_valid)
    }
}
