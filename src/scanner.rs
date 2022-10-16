use crate::token::Token;
use crate::token_type::TokenType;
use crate::Regg;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: Some("".to_string()),
            line: self.line,
        });

        return &self.tokens;
    }

    fn scan_token(&mut self) {
        // Current character being scanned
        let c = self.advance();
        let mut regg = Regg::new();

        match c {
            '{' => {
                if self.match_char('{') {
                    self.add_token(TokenType::StartExprToken, None);
                }
                self.advance();
            }
            '}' => {
                if self.match_char('}') {
                    self.add_token(TokenType::EndExprToken, None);
                }
                self.advance();
            }
            _ => regg.error(self.line, "Unexpected character"),
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn is_at_end(&mut self) -> bool {
        // TODO: Handle Errors Better
        return self.current >= self.source.len().try_into().unwrap();
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

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];

        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        })
    }
}
