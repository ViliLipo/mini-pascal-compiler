use crate::constants;
use crate::token::*;
use crate::source::Source;

type ScanFunction = fn(&mut Source) -> Option<Token>;

pub struct Scanner {
    src: Source,
    scanfunctions: Vec<ScanFunction>,
}

impl Scanner {
    pub fn get_next_token(&mut self) -> Token {
        self.screening();
        if self.src.end_of_file() {
            return Token {
                row: self.src.get_row(),
                column: self.src.get_column(),
                lexeme: String::from("eof"),
                token_kind: TokenKind::Eof,
            };
        }
        for func in &self.scanfunctions {
            match func(&mut self.src) {
                None => continue,
                Some(t) => return t,
            }
        }
        self.src.get_next_char();
        Token {
            row: self.src.get_row(),
            column: self.src.get_column(),
            lexeme: String::from(""),
            token_kind: TokenKind::Error,
        }
    }

    fn screen_white_space(&mut self) {
        loop {
            match self.src.peek() {
                '\n' | ' ' | '\t' => {
                    self.src.get_next_char();
                }
                _ => break,
            }
        }
    }

    fn screening(&mut self) {
        let mut result = true;
        while result {
            self.screen_white_space();
            result = self.screen_multiline_comment();
            self.screen_white_space();
            result = result || self.screen_single_line_comment();
        }
    }

    fn screen_multiline_comment(&mut self) -> bool {
        if self.src.peek() == '{' {
            self.src.get_next_char();
            if self.src.peek() == '*' {
                self.src.get_next_char();
                let mut open_comment_count = 1;
                loop {
                    match self.src.get_next_char() {
                        '*' => {
                            if self.src.peek() == '}' {
                                self.src.get_next_char();
                                open_comment_count = open_comment_count - 1;
                                if open_comment_count == 0 {
                                    break;
                                }
                            }
                        }
                        '\0' => break, // TODO: Runaway comment error
                        '{' => {
                            if self.src.peek() == '*' {
                                self.src.get_next_char();
                                open_comment_count = open_comment_count + 1;
                            }
                        }
                        _ => (),
                    }
                }
                true
            } else {
                self.src.reverse();
                false
            }
        } else {
            false
        }
    }

    fn screen_single_line_comment(&mut self) -> bool {
        match self.src.peek() {
            '/' => {
                self.src.get_next_char();
                match self.src.peek() {
                    '/' => {
                        self.src.get_next_char();
                        loop {
                            match self.src.get_next_char() {
                                '\n' | '\0' => break,
                                _ => (),
                            }
                        }
                        true
                    }
                    _ => {
                        self.src.reverse();
                        false
                    }
                }
            }
            _ => false,
        }
    }
}

fn scan_digit_part(src: &mut Source) -> Option<Token> {
    match src.peek().is_digit(10) {
        false => None,
        true => {
            let mut lexeme = String::from("");
            while src.peek().is_digit(10) {
                lexeme.push(src.get_next_char());
            }
            Some(Token {
                lexeme: lexeme,
                row: src.get_row(),
                column: src.get_column(),
                token_kind: TokenKind::IntegerLiteral,
            })
        }
    }
}

fn scan_exponent(src: &mut Source) -> Option<Token> {
    match src.peek() == 'e' {
        false => None,
        true => {
            let mut lexeme = String::from("");
            lexeme.push(src.get_next_char());
            if src.peek() == '+' || src.peek() == '-' {
                lexeme.push(src.get_next_char());
            }
            match scan_digit_part(src) {
                None => None, // TODO: Lexical error
                Some(token2) => {
                    lexeme = lexeme + token2.lexeme.as_str();
                    Some(Token {
                        lexeme: lexeme,
                        token_kind: TokenKind::Error,
                        row: src.get_row(),
                        column: src.get_column(),
                    })
                }
            }
        }
    }
}

fn scan_number(src: &mut Source) -> Option<Token> {
    match scan_digit_part(src) {
        None => None,
        Some(token) => match src.peek() == '.' {
            true => {
                let mut lexeme = String::from(&token.lexeme);
                lexeme.push(src.get_next_char());
                match scan_digit_part(src) {
                    None => {
                        src.reverse();
                        Some(token)
                    }
                    Some(token2) => {
                        lexeme = lexeme + token2.lexeme.as_str();
                        match scan_exponent(src) {
                            None => (),
                            Some(token3) => lexeme = lexeme + token3.lexeme.as_str(),
                        };
                        let column = src.get_column();
                        let row = src.get_row();
                        Some(Token {
                            lexeme: lexeme,
                            token_kind: TokenKind::RealLiteral,
                            column: column,
                            row: row,
                        })
                    }
                }
            }
            false => Some(token),
        },
    }
}

fn scan_identifier_or_keyword(src: &mut Source) -> Option<Token> {
    match src.peek().is_alphanumeric() {
        false => None,
        true => {
            let mut lexeme = String::from("");
            let mut t_type = TokenKind::Identifier;
            let row = src.get_row();
            let column = src.get_column();
            while src.peek().is_alphanumeric() || src.peek() == '_' {
                lexeme.push(src.get_next_char());
            }
            let keywords = constants::get_keywords();
            if keywords.contains(&lexeme.as_str()) {
                t_type = kw_str_to_tokenkind(lexeme.as_str());
            }
            Some(Token {
                lexeme: lexeme.to_lowercase(),
                token_kind: t_type,
                row: row,
                column: column,
            })
        }
    }
}

fn scan_colon_or_assign(src: &mut Source) -> Option<Token> {
    match src.peek() {
        ':' => {
            let mut lexeme = String::from("");
            lexeme.push(src.get_next_char());
            match src.peek() {
                '=' => {
                    lexeme.push(src.get_next_char());
                }
                _ => (),
            }
            let token_kind = special_symbol_to_tokenkind(lexeme.as_str());
            Some(Token {
                token_kind,
                lexeme: lexeme,
                column: src.get_column(),
                row: src.get_row(),
            })
        }
        _ => None,
    }
}

fn scan_special_symbols(src: &mut Source) -> Option<Token> {
    let operators = constants::get_special_symbols();
    let mut charastring = String::from("");
    charastring.push(src.peek());
    match operators.contains(&charastring.as_str()) {
        false => None,
        true => {
            let mut lexeme = String::from("");
            match src.peek() {
                '<' => {
                    lexeme.push(src.get_next_char());
                    match src.peek() {
                        '>' | '=' => {
                            lexeme.push(src.get_next_char());
                        }
                        _ => (),
                    }
                }
                '>' => {
                    lexeme.push(src.get_next_char());
                    match src.peek() {
                        '=' => lexeme.push(src.get_next_char()),
                        _ => (),
                    }
                }
                _ => {
                    lexeme.push(src.get_next_char());
                }
            }
            Some(Token {
                token_kind: special_symbol_to_tokenkind(lexeme.as_str()),
                lexeme: lexeme,
                row: src.get_row(),
                column: src.get_column(),
            })
        }
    }
}

fn handle_escape_characters(src: &mut Source, lexeme: &mut String) {
    src.get_next_char();
    match src.peek() {
        '"' | '\\' => lexeme.push(src.get_next_char()),
        'n' => {
            src.get_next_char();
            lexeme.push_str("\\n");
        }
        't' => {
            src.get_next_char();
            lexeme.push_str("\\t");
        }
        _ => (),
    }
}

fn scan_string_literal(src: &mut Source) -> Option<Token> {
    match src.peek() {
        '"' => {
            let mut lexeme = String::from("");
            lexeme.push(src.get_next_char());
            loop {
                match src.peek() {
                    '"' => {
                        lexeme.push(src.get_next_char());
                        break;
                    }
                    '\\' => handle_escape_characters(src, &mut lexeme),
                    _ => lexeme.push(src.get_next_char()),
                }
            }
            Some(Token {
                lexeme: lexeme,
                token_kind: TokenKind::StringLiteral,
                column: src.get_column(),
                row: src.get_row(),
            })
        }
        _ => None,
    }
}

fn kw_str_to_tokenkind<'a>(kw_str: &'a str) -> TokenKind {
    match kw_str {
        "var" => TokenKind::Var,
        "and" => TokenKind::And,
        "or" => TokenKind::Or,
        "if" => TokenKind::If,
        "then" => TokenKind::Then,
        "else" => TokenKind::Else,
        "of" => TokenKind::Of,
        "while" => TokenKind::While,
        "do" => TokenKind::Do,
        "begin" => TokenKind::Begin,
        "end" => TokenKind::End,
        "array" => TokenKind::Array,
        "procedure" => TokenKind::Procedure,
        "function" => TokenKind::Function,
        "program" => TokenKind::Program,
        "assert" => TokenKind::Assert,
        "return" => TokenKind::Return,
        "not" => TokenKind::Not,
        _ => TokenKind::Error,
    }
}

fn special_symbol_to_tokenkind<'a>(kw_str: &'a str) -> TokenKind {
    match kw_str {
        "+" => TokenKind::Plus,
        "-" => TokenKind::Minus,
        "*" => TokenKind::Multi,
        "/" => TokenKind::Division,
        "%" => TokenKind::Modulo,
        "=" => TokenKind::Equal,
        "<>" => TokenKind::NotEqual,
        "<" => TokenKind::SmallerThan,
        "<=" => TokenKind::ESmallerThan,
        ">" => TokenKind::LargerThan,
        ">=" => TokenKind::ELargerThan,
        "(" => TokenKind::OpenBracket,
        ")" => TokenKind::CloseBracket,
        "[" => TokenKind::OpenSquareBracket,
        "]" => TokenKind::CloseSquareBracket,
        "." => TokenKind::Dot,
        "," => TokenKind::Comma,
        ":" => TokenKind::Colon,
        ";" => TokenKind::SemiColon,
        ":=" => TokenKind::Assign,
        _ => TokenKind::Error,
    }
}

pub fn build_scanner(source: Source) -> Scanner {
    let mut scanfunctions: Vec<ScanFunction> = Vec::new();
    scanfunctions.push(scan_number);
    scanfunctions.push(scan_identifier_or_keyword);
    scanfunctions.push(scan_colon_or_assign);
    scanfunctions.push(scan_special_symbols);
    scanfunctions.push(scan_string_literal);
    Scanner {
        scanfunctions: scanfunctions,
        src: source,
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::source;

    fn variant_eq<T>(a: &T, b: &T) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    fn get_scanner(text: String) -> Scanner {
        let s = source::create_source(text);
        build_scanner(s)
    }

    #[test]
    fn test_scan_identifier() {
        let text = String::from("variable_name_12345;\n");
        let mut scanner = get_scanner(text);
        let t = scanner.get_next_token();
        let ok_lexeme = String::from("variable_name_12345");
        let ok_t_type = TokenKind::Identifier;
        assert_eq!(ok_lexeme, t.lexeme);
        assert!(variant_eq(&ok_t_type, &t.token_kind));
    }

    #[test]
    fn test_scan_real_literal() {
        let text = String::from("3.14159");
        let mut scanner = get_scanner(text);
        let t = scanner.get_next_token();
        assert!(variant_eq(&TokenKind::RealLiteral, &t.token_kind));
        assert_eq!("3.14159", t.lexeme.as_str());
    }

    #[test]
    fn test_scan_real_literal_with_exponent() {
        let text = String::from("3.14159e+10");
        let mut scanner = get_scanner(text);
        let t = scanner.get_next_token();
        assert!(variant_eq(&TokenKind::RealLiteral, &t.token_kind));
        assert_eq!("3.14159e+10", t.lexeme.as_str());
    }

    #[test]
    fn test_scan_string_literal() {
        let text = String::from("\"lit string\"");
        let mut scanner = get_scanner(text);
        let t = scanner.get_next_token();
        assert!(variant_eq(&TokenKind::StringLiteral, &t.token_kind));
        assert_eq!("\"lit string\"", t.lexeme.as_str());
    }

    #[test]
    fn test_scan_declaration_stmnt() {
        let text = String::from("var MAXIMIUM_POWER_999: integer := 99999;\n");
        let mut scanner = get_scanner(text);
        let ok_tokens = vec![
            (TokenKind::Var, "var"),
            (TokenKind::Identifier, "maximium_power_999"),
            (TokenKind::Colon, ":"),
            (TokenKind::Identifier, "integer"),
            (TokenKind::Assign, ":="),
            (TokenKind::IntegerLiteral, "99999"),
            (TokenKind::SemiColon, ";"),
        ];
        for ok in ok_tokens {
            let token = scanner.get_next_token();
            assert!(variant_eq(&ok.0, &token.token_kind));
            assert_eq!(ok.1, token.lexeme.as_str());
        }
    }

    #[test]
    fn test_screen_multiline_comments() {
        let text = String::from(
            "{* This is a minipascal comment \n {* nested comment *} *} \n var i: int;",
        );
        let mut scanner = get_scanner(text);
        let token = scanner.get_next_token();
        assert_eq!("var", token.lexeme.as_str());
    }

    #[test]
    fn test_screen_singleline_comments() {
        let text = String::from("// This is a minipascal comment\nvar i: int;");
        let mut scanner = get_scanner(text);
        let token = scanner.get_next_token();
        assert_eq!("var", token.lexeme.as_str());
    }

    #[test]
    fn test_scan_not_equal() {
        let text = String::from("<>");
        let mut scanner = get_scanner(text);
        let t = scanner.get_next_token();
        assert!(variant_eq(&TokenKind::NotEqual, &t.token_kind));
        assert_eq!("<>", t.lexeme.as_str());
    }

}
