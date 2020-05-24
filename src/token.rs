use std::fmt;
#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    StringLiteral,
    IntegerLiteral,
    RealLiteral,
    Eof,
    Error,
    Var,
    And,
    Or,
    Not,
    If,
    Then,
    Else,
    Of,
    While,
    Do,
    Begin,
    End,
    Array,
    Procedure,
    Function,
    Program,
    Assert,
    Return,
    Plus,
    Minus,
    Multi,
    Division,
    Modulo,
    Equal,
    NotEqual,
    SmallerThan,
    LargerThan,
    ESmallerThan,
    ELargerThan,
    OpenBracket,
    CloseBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    Dot,
    Colon,
    Comma,
    SemiColon,
    Assign,
}

impl PartialEq for TokenKind {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Copy for TokenKind {}

impl Clone for TokenKind {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Token {
    pub row: usize,
    pub column: usize,
    pub lexeme: String,
    pub token_kind: TokenKind,
}

impl Clone for Token {
    fn clone(&self) -> Token {
        Token {
            row: self.row,
            column: self.column,
            lexeme: self.lexeme.clone(),
            token_kind: self.token_kind,
        }
    }
}

