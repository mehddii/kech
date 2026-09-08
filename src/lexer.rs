#[derive(Default)]
struct Lexer {
    program: &'static str,
    current: usize,
    next: usize,
}

struct Token {
    token_type: TokenType,
    literal: &'static str,
}

enum TokenType {
    Function,
    IDENT,
    LPAREN,
    RPAREN,
    ASSIGN,
    EQ,
    LBRACE,
    RBRACE,
    DEF,
    FUNCTION,
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
    EOF,
}

impl Lexer {
    fn feed(&mut self, program: &'static str) {
        self.program = program;
    }

    fn current(&mut self) -> Token {
        while self.next < self.program.len() {
            match self.current {}

            self.advance();
        }

        return Token {
            token_type: TokenType::EOF,
            literal: "",
        };
    }

    fn advance(&mut self) {
        if self.next >= self.program.len() {
            return;
        }

        self.current = self.next;
        self.next += 1;
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
                n + fibo(n - 1)
            }
        }
        ";

        let tokens = [
            Token {
                token_type: TokenType::Function,
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
                literal: "n",
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
                literal: "1",
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}",
            },
        ];

        let lexer = Lexer::default();
        lexer.feed(program);

        for token in tokens.iter() {
            let current = lexer.current();
            lexer.advance();

            assert!(!lexer.is_eof());
            assert_eq!(token.type, current.type);
            assert_eq!(token.literal, current.literal);
        }
    }
}
