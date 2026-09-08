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
            Token{ type: TokenType.FUNCTION, literal: "fct" },
            Token{ type: TokenType.IDENT, literal: "main" },
            Token{ type: TokenType.LPAREN, literal: "(" },
            Token{ type: TokenType.RPAREN, literal: ")" },
            Token{ type: TokenType.LBRACE, literal: "{" },
            Token{ type: TokenType.DEF, literal: "def" },
            Token{ type: TokenType.IDENT, literal: "a" },
            Token{ type: TokenType.COLON, literal: ":" },
            Token{ type: TokenType.I32, literal: "i32" },
            Token{ type: TokenType.ASSIGN, literal: "=" },
            Token{ type: TokenType.NUMBER, literal: "7" },
            Token{ type: TokenType.SEMICOLON, literal: ";" },
            Token{ type: TokenType.IDENT, literal: "fibo" },
            Token{ type: TokenType.LPAREN, literal: "(" },
            Token{ type: TokenType.IDENT, literal: "a" },
            Token{ type: TokenType.RPAREN, literal: ")" },
            Token{ type: TokenType.SEMICOLON, literal: ";" },
            Token{ type: TokenType.RBRACE, literal: "}" },
            Token{ type: TokenType.FUNCTION, literal: "fct" },
            Token{ type: TokenType.IDENT, literal: "fibo" },
            Token{ type: TokenType.LPAREN, literal: "(" },
            Token{ type: TokenType.IDENT, literal: "n" },
            Token{ type: TokenType.COLON, literal: ":" },
            Token{ type: TokenType.U32, literal: "u32" },
            Token{ type: TokenType.RPAREN, literal: ")" },
            Token{ type: TokenType.COLON, literal: ":" },
            Token{ type: TokenType.U32, literal: "u32" },
            Token{ type: TokenType.LBRACE, literal: "{" },
            Token{ type: TokenType.IF, literal: "if" },
            Token{ type: TokenType.IDENT, literal: "n" },
            Token{ type: TokenType.EQ, literal: "==" },
            Token{ type: TokenType.NUMBER, literal: "0" },
            Token{ type: TokenType.OR, literal: "or" },
            Token{ type: TokenType.IDENT, literal: "n" },
            Token{ type: TokenType.EQ, literal: "==" },
            Token{ type: TokenType.NUMBER, literal: "1" },
            Token{ type: TokenType.LBRACE, literal: "{" },
            Token{ type: TokenType.IDENT, literal: "n" },
            Token{ type: TokenType.RBRACE, literal: "}" },
            Token{ type: TokenType.ELSE, literal: "else" },
            Token{ type: TokenType.LBRACE, literal: "{" },
            Token{ type: TokenType.IDENT, literal: "n" },
            Token{ type: TokenType.PLUS, literal: "+" },
            Token{ type: TokenType.IDENT, literal: "fibo" },
            Token{ type: TokenType.LPAREN, literal: "(" },
            Token{ type: TokenType.IDENT, literal: "n" },
            Token{ type: TokenType.MINUS, literal: "-" },
            Token{ type: TokenType.NUMBER, literal: "1" },
            Token{ type: TokenType.RPAREN, literal: ")" },
            Token{ type: TokenType.RBRACE, literal: "}" },
        ];

        let lexer = Lexer::default();
        assert!(lexer.feed(program));

        for token in tokens.iter() {
            let current = lexer.next();

            assert!(!lexer.is_eof());
            assert_eq!(token.type, current.type);
            assert_eq!(token.literal, current.literal);
        }
    }
}
