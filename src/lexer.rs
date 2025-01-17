use std::borrow::Cow;
use std::iter::Peekable;
use std::str::Chars;

enum Token<'a> {
    Literal(Literal<'a>),
    Keyword(Keyword),
    Symbol(Symbol),
    Operator(Operator),
}

enum Literal<'a> {
    Identifier(&'a str),
    Integer(i128),
    Floating(f64),
    Boolean(bool),
    String(Cow<'a, str>),
    Character(char),
    Type(Type<'a>),
}

enum Operator {
    Plus,              // +
    Minus,             // -
    Multiply,          // *
    Divide,            // /
    Modulo,            // %
    And,               // &&
    Or,                // ||
    Not,               // !
    Equals,            // ==
    NotEquals,         // !=
    LessThan,          // <
    GreaterThan,       // >
    LessThanEquals,    // <=
    GreaterThanEquals, // >=
    BitwiseAnd,        // &
    BitwiseOr,         // |
    BitwiseXor,        // ^
    BitwiseNot,        // ~
    BitwiseLeftShift,  // <<
    BitwiseRightShift, // >>
}

enum Symbol {
    Semicolon,          // ;
    Assign,             // =
    TypeAssign,         // :
    FunctionTypeAssign, // ->
    Scope,              // ::
    Comma,              // ,
    LeftParenthesis,    // (
    RightParenthesis,   // )
    LeftBrace,          // {
    RightBrace,         // }
    LeftBracket,        // [
    RightBracket,       // ]
}

enum Keyword {
    Fn,     // fn
    Let,    // let
    Return, // return
    Break,  // break
    While,  // while
    For,    // for
    If,     // if
}

enum Type<'a> {
    Void,                       // void
    Byte,                       // byte
    Short,                      // short
    Int,                        // int
    Long,                       // long
    Float,                      // float
    Double,                     // double
    String,                     // string
    Char,                       // char
    Array(&'a Type<'a>, usize), // [type; count] OR [type]
    Optional(&'a Type<'a>),     // ?type
    Reference(&'a Type<'a>),    // &type
}

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn one() {
        todo!();
    }
}
