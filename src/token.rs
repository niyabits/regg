use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&mut self) -> String {
        match &self.literal {
            Some(literal) => {
                return self.token_type.to_string() + " " + &self.lexeme[..] + " " + &literal[..]
            }
            None => return self.token_type.to_string() + " " + &self.lexeme[..] + " " + "NULL",
        }
    }
}
