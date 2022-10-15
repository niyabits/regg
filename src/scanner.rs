use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        });

        return self.tokens;
    }

    fn is_at_end(&mut self) -> bool {
        // TODO: Handle Errors Better
        return self.current >= self.source.len().try_into().unwrap();
    }

    fn scan_token(self) {
        let c = self.advance();

        match c {}
    }

    fn advance(&mut self) -> char {
        let return_char = self
            .source
            .chars()
            .nth(self.current.try_into().unwrap())
            .unwrap();
        self.current = self.current + 1;

        return return_char;
    }

    fn add_token(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        })
    }
}
