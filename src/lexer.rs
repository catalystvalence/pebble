use std::borrow::Cow;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Literal(Literal<'a>),
    Keyword(Keyword),
    Symbol(Symbol),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
enum Literal<'a> {
    Identifier(&'a str),
    Integer(i128),
    Floating(f64),
    Boolean(bool),
    String(Cow<'a, str>),
    Character(char),
    Type(Type<'a>),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum Keyword {
    Fn,     // fn
    Let,    // let
    Return, // return
    Break,  // break
    While,  // while
    For,    // for
    If,     // if
    Match,  // match
}

#[derive(Debug, PartialEq)]
enum Type<'a> {
    Void,                               // void
    Byte,                               // byte
    Sbyte,                              // sbyte
    Short,                              // short
    Ushort,                             // ushort
    Int,                                // int
    Uint,                               // uint
    Long,                               // long
    Ulong,                              // ulong
    Float,                              // float
    Double,                             // double
    String,                             // string
    Char,                               // char
    Bool,                               // bool
    Array(&'a Type<'a>, Option<usize>), // [type; count] OR [type]
    Optional(&'a Type<'a>),             // ?type
    Reference(&'a Type<'a>),            // &type
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
    use super::*;

    #[test]
    fn let_with_integers() {
        let lexer = Lexer::new(
            "let unix_ownership: byte = 0o644;\n\
                let almost_useless: sbyte = -0xAB;\n\
                let hacker: short = -1337;\n\
                let blazer: ushort = 420;\n\
                let dead: int = -0xDEADBEEF;\n\
                let undead: uint = 0xFEEDBEEF;\n\
                let weird: long = -0o7777777777777;\n\
                let normal: ulong = 12345678901234;\n\
                ",
        );

        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(
            tokens,
            &[
                Token::Keyword(Keyword::Let),
                Token::Literal(Literal::Identifier("unix_ownership")),
                Token::Symbol(Symbol::TypeAssign),
                Token::Literal(Literal::Type(Type::Byte)),
                Token::Symbol(Symbol::Assign),
                Token::Literal(Literal::Integer(0o644)),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Literal(Literal::Identifier("almost_useless")),
                Token::Symbol(Symbol::TypeAssign),
                Token::Literal(Literal::Type(Type::Sbyte)),
                Token::Symbol(Symbol::Assign),
                Token::Literal(Literal::Integer(-0xAB)),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Literal(Literal::Identifier("hacker")),
                Token::Symbol(Symbol::TypeAssign),
                Token::Literal(Literal::Type(Type::Short)),
                Token::Symbol(Symbol::Assign),
                Token::Literal(Literal::Integer(-1337)),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Literal(Literal::Identifier("blazer")),
                Token::Symbol(Symbol::TypeAssign),
                Token::Literal(Literal::Type(Type::Ushort)),
                Token::Symbol(Symbol::Assign),
                Token::Literal(Literal::Integer(420)),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Literal(Literal::Identifier("dead")),
                Token::Symbol(Symbol::TypeAssign),
                Token::Literal(Literal::Type(Type::Int)),
                Token::Symbol(Symbol::Assign),
                Token::Literal(Literal::Integer(-0xDEADBEEF)),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Literal(Literal::Identifier("undead")),
                Token::Symbol(Symbol::TypeAssign),
                Token::Literal(Literal::Type(Type::Uint)),
                Token::Symbol(Symbol::Assign),
                Token::Literal(Literal::Integer(0xFEEDBEEF)),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Literal(Literal::Identifier("weird")),
                Token::Symbol(Symbol::TypeAssign),
                Token::Literal(Literal::Type(Type::Long)),
                Token::Symbol(Symbol::Assign),
                Token::Literal(Literal::Integer(-0o7777777777777)),
                Token::Symbol(Symbol::Semicolon),
                Token::Keyword(Keyword::Let),
                Token::Literal(Literal::Identifier("normal")),
                Token::Symbol(Symbol::TypeAssign),
                Token::Literal(Literal::Type(Type::Ulong)),
                Token::Symbol(Symbol::Assign),
                Token::Literal(Literal::Integer(12345678901234)),
                Token::Symbol(Symbol::Semicolon),
            ]
        );
    }
}
