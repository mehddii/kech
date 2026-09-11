use std::collections::HashMap;

#[derive(Default)]
pub struct Lexer {
    program: &'static str,
    current: usize,
    next: usize,
    keywords: HashMap<&'static str, TokenType>,
}

#[derive(Default, Debug)]
pub struct Token {
    token_type: TokenType,
    literal: &'static str,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum TokenType {
    FUNCTION,
    IDENT,
    LPAREN,
    RPAREN,
    ASSIGN,
    EQ,
    LBRACE,
    RBRACE,
    DEF,
    NUMBER,
    I32,
    U32,
    MINUS,
    PLUS,
    COLON,
    SEMICOLON,
    IF,
    ELSE,
    OR,
    #[default]
    EOF,
}

fn is_char(c: u8) -> bool {
    b'a' <= c && c <= b'z' || b'A' <= c && c <= b'Z' || c == b'_'
}

fn is_num(c: u8) -> bool {
    b'0' <= c && c <= b'9'
}

fn is_space(c: u8) -> bool {
    c == b'\t' || c == b' ' || c == b'\n'
}

impl Lexer {
    pub fn new() -> Lexer {
        let mut lexer = Lexer::default();

        lexer.keywords = HashMap::from([
            ("def", TokenType::DEF),
            ("fct", TokenType::FUNCTION),
            ("if", TokenType::IF),
            ("else", TokenType::ELSE),
            ("or", TokenType::OR),
            ("i32", TokenType::I32),
            ("u32", TokenType::U32),
        ]);
        lexer
    }

    pub fn feed(&mut self, program: &'static str) {
        self.program = program;
    }

    pub fn current(&mut self) -> Token {
        let mut token = Token::default();

        self.consume_space();
        if self.next <= self.program.len() {
            match self.program[self.current..=self.current]
                .chars()
                .nth(0)
                .unwrap()
            {
                // Single char
                '{' => {
                    token.token_type = TokenType::LPAREN;
                    token.literal = "{";
                }
                '}' => {
                    token.token_type = TokenType::RPAREN;
                    token.literal = "}";
                }
                '(' => {
                    token.token_type = TokenType::LBRACE;
                    token.literal = "(";
                }
                ')' => {
                    token.token_type = TokenType::RBRACE;
                    token.literal = ")";
                }
                '-' => {
                    token.token_type = TokenType::MINUS;
                    token.literal = "-";
                }
                '+' => {
                    token.token_type = TokenType::PLUS;
                    token.literal = "+";
                }
                ':' => {
                    token.token_type = TokenType::COLON;
                    token.literal = ":";
                }
                ';' => {
                    token.token_type = TokenType::SEMICOLON;
                    token.literal = ";";
                }

                // Double char
                '=' => {
                    let next = self.peek();

                    if next == '=' {
                        token.token_type = TokenType::EQ;
                        token.literal = "==";
                    } else if is_char(next as u8) {
                        token.token_type = TokenType::ASSIGN;
                        token.literal = "=";
                    }
                }

                // Multi chars
                c => {
                    if is_char(c as u8) {
                        let word = self.get_word();

                        token.literal = word;
                        if self.keywords.contains_key(word) {
                            token.token_type = self.keywords.get(word).unwrap().clone();
                        } else {
                            token.token_type = TokenType::IDENT;
                        }
                    } else if is_num(c as u8) {
                        token.token_type = TokenType::NUMBER;
                        token.literal = self.get_num();
                    }
                }
            }
            self.consume_space();
        }

        return token;
    }

    fn consume_space(&mut self) {
        while is_space(self.next() as u8) {
            self.advance();
        }
    }

    fn get_num(&mut self) -> &'static str {
        let start = self.current;
        while is_num(self.peek() as u8) {
            self.advance();
        }

        return &self.program[start..self.next];
    }

    fn get_word(&mut self) -> &'static str {
        let start = self.current;
        while is_char(self.peek() as u8) {
            self.advance();
        }

        return &self.program[start..self.next];
    }

    fn next(&self) -> char {
        if self.next >= self.program.len() {
            return char::default();
        }

        self.program[self.current..=self.current]
            .chars()
            .nth(0)
            .unwrap()
    }

    fn peek(&self) -> char {
        if self.next >= self.program.len() {
            return char::default();
        }

        self.program[self.next..=self.next].chars().nth(0).unwrap()
    }

    pub fn advance(&mut self) {
        if self.next > self.program.len() {
            return;
        }

        self.current = self.next;
        self.next += 1;
    }

    pub fn is_eof(&self) -> bool {
        self.next > self.program.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        let program = "
        fct main() {
            def a: i32 = 7;
            fibo(a);
        }

        fct fibo(n: u32): u32 {
            if n == 0 or n == 1 {
                n
            } else {
                fibo(n - 1) + fibo(n - 2)
            }
        }
        ";

        let tokens = [
            Token {
                token_type: TokenType::FUNCTION,
                literal: "fct",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "main",
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(",
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{",
            },
            Token {
                token_type: TokenType::DEF,
                literal: "def",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "a",
            },
            Token {
                token_type: TokenType::COLON,
                literal: ":",
            },
            Token {
                token_type: TokenType::I32,
                literal: "i32",
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=",
            },
            Token {
                token_type: TokenType::NUMBER,
                literal: "7",
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "fibo",
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "a",
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";",
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}",
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: "fct",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "fibo",
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "n",
            },
            Token {
                token_type: TokenType::COLON,
                literal: ":",
            },
            Token {
                token_type: TokenType::U32,
                literal: "u32",
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            Token {
                token_type: TokenType::COLON,
                literal: ":",
            },
            Token {
                token_type: TokenType::U32,
                literal: "u32",
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{",
            },
            Token {
                token_type: TokenType::IF,
                literal: "if",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "n",
            },
            Token {
                token_type: TokenType::EQ,
                literal: "==",
            },
            Token {
                token_type: TokenType::NUMBER,
                literal: "0",
            },
            Token {
                token_type: TokenType::OR,
                literal: "or",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "n",
            },
            Token {
                token_type: TokenType::EQ,
                literal: "==",
            },
            Token {
                token_type: TokenType::NUMBER,
                literal: "1",
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "n",
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}",
            },
            Token {
                token_type: TokenType::ELSE,
                literal: "else",
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "fibo",
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "n",
            },
            Token {
                token_type: TokenType::MINUS,
                literal: "-",
            },
            Token {
                token_type: TokenType::NUMBER,
                literal: "1",
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            Token {
                token_type: TokenType::PLUS,
                literal: "+",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "fibo",
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(",
            },
            Token {
                token_type: TokenType::IDENT,
                literal: "n",
            },
            Token {
                token_type: TokenType::MINUS,
                literal: "-",
            },
            Token {
                token_type: TokenType::NUMBER,
                literal: "2",
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}",
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}",
            },
        ];

        let mut lexer = Lexer::new();
        lexer.feed(program);

        for token in tokens.iter() {
            let current = lexer.current();
            lexer.advance();

            assert!(!lexer.is_eof());
            assert_eq!(token.token_type, current.token_type);
            assert_eq!(token.literal, current.literal);
        }
    }
}
