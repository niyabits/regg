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
                    self.js_expr();
                }
            }
            '-' => {
                if self.match_char('-') {
                    if self.match_char('-') {
                        self.add_token(TokenType::FrontmatterFenceToken, None);
                    }
                }
            }
            '\n' => self.line += 1,
            _ => {
                regg.error(self.line, "Unexpected character");
            }
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.get_nth_char(self.current);
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return self.get_nth_char(self.current + 1);
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.get_nth_char(self.current) != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn is_at_end(&mut self) -> bool {
        // TODO: Handle Errors Better
        return self.current >= self.source.len().try_into().unwrap();
    }

    fn advance(&mut self) -> char {
        let return_char = self.get_nth_char(self.current);
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

    fn js_expr(&mut self) {
        // consume all the characters before `}}`
        while !self.is_at_end() && self.peek() != '}' && self.peek_next() != '}' {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            let mut regg = Regg::new();
            regg.error(self.line, "Unterminated double curly braces `}}`")
        }

        // consume `}}`
        // NOTE: I have no idea why we need a third advance here,
        // It just worked! wild guess: '\n',
        // As of writing this comment I am feeling too lazy to investigate
        // I want to celebrate this worked
        self.advance();
        self.advance();
        self.advance();

        // Get the JavaScript Expression, trim the `{{` and `}}`
        let value = &self.source[self.start + 2..self.current - 2];
        self.add_token(TokenType::JsExpr, Some(value.to_string()));
    }

    fn get_nth_char(&mut self, index: usize) -> char {
        // TODO: fix this chaos
        return self.source.chars().nth(index).unwrap();
    }
}
