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
        match c {
            '{' => {
                if self.match_char('{') {
                    self.js_expr();
                }
            }
            '-' => {
                if self.match_char('-') {
                    if self.match_char('-') {
                        self.js_block();
                    }
                }
            }
            '\n' => self.line += 1,
            _ => {
                self.markup();
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_at_end() {
            return Some('\0');
        }

        return self.get_nth_char(self.current);
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.current + 1 >= self.source.len() {
            // Contemplate: Isn't it better to have an error that next character cannot be peeked?
            return Some('\0');
        }

        return self.get_nth_char(self.current + 1);
    }

    fn peek_third(&mut self) -> Option<char> {
        if self.current + 2 >= self.source.len() {
            // Contemplate: Isn't it better to have an error that next character cannot be peeked?
            return Some('\0');
        }

        return self.get_nth_char(self.current + 2);
    }

    fn match_char(&mut self, expected: char) -> bool {
        let mut regg = Regg::new();
        if self.is_at_end() {
            return false;
        }

        match self.get_nth_char(self.current) {
            Some(current_char) => {
                if current_char != expected {
                    return false;
                }

                self.current += 1;
                return true;
            }
            None => {
                regg.error(self.line, "Scanner went out of bound");
                return false;
            }
        }
    }

    fn is_at_end(&mut self) -> bool {
        // TODO: Handle Errors Better
        return self.current >= self.source.len().try_into().unwrap();
    }

    fn advance(&mut self) -> char {
        let mut regg = Regg::new();
        let return_char = self.get_nth_char(self.current);
        self.current = self.current + 1;

        match return_char {
            Some(char) => return char,
            None => {
                regg.error(self.line, "Character does not exist");
                return '\0';
            }
        }
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
        // TODO: refactor, this is not a good way to handle match enums
        // consume all the characters before `}}`
        while !self.is_at_end() && self.peek().unwrap() != '}' && self.peek_next().unwrap() != '}' {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            let mut regg = Regg::new();
            regg.error(self.line, "Unterminated double curly braces `}}`");
        }

        // consumes `}}`
        self.advance(); // consumes a whitespace, figure this out
        self.advance();
        self.advance();

        // Get the JavaScript Expression, trim the `{{` and `}}`
        let value = &self.source[self.start + 2..self.current - 2];
        self.add_token(TokenType::JsExpr, Some(value.to_string()));
    }

    fn js_block(&mut self) {
        // TODO: refactor, this is not a good way to handle match enums
        // consume all the characters before `}}`
        while !self.is_at_end()
            && self.peek().unwrap() != '-'
            && self.peek_next().unwrap() != '-'
            && self.peek_third().unwrap() != '-'
        {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            let mut regg = Regg::new();
            regg.error(self.line, "Unterminated frontmatter fence token `---`");
        }

        self.advance(); // consumes white space

        // consume `---`
        self.advance();
        self.advance();
        self.advance();

        self.advance(); // consumes white space

        // Get the JavaScript Expression, trim the `{{` and `}}`
        let value = &self.source[self.start + 3..self.current - 3];
        self.add_token(TokenType::JsBlock, Some(value.to_string()));
    }

    fn markup(&mut self) {
        if self.current + 2 <= self.source.len().try_into().unwrap() {
            while !self.is_at_end() {
                if self.peek().unwrap() == '\n' {
                    self.line += 1;
                }

                self.advance();

                // Break if JavaScript Expression syntax `{{` is encountered
                if self.peek().unwrap() == '{' && self.peek_next().unwrap() == '{' {
                    break;
                }
            }
        }

        let value = &self.source[self.start..self.current];
        self.add_token(TokenType::Markup, Some(value.to_string()));
    }

    fn get_nth_char(&mut self, index: usize) -> Option<char> {
        return self.source.chars().nth(index);
    }
}
