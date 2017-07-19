use combine;
use combine::primitives::IteratorStream;
// use std::fs;
use std::io;
use std::vec;
use token::Token;

#[derive(Debug, error_chain)]
pub enum ErrorKind {
    Msg(String),

    // #[error_chain(custom)]
    // #[error_chain(description = r#"|_| "undefined symbol""#)]
    // #[error_chain(display = r#"|t| write!(f, "undefined symbol {}", t)"#)]
    // UndefinedSymbol(Symbol),

    #[error_chain(foreign)]
    Io(io::Error),

    // #[error_chain(foreign)]
    // File(combine::ParseError<LineStream<io::BufReader<fs::File>>>),

    // #[error_chain(foreign)]
    // Lex(combine::ParseError<LineStream<Readline>>),

    #[error_chain(foreign)]
    Parse(combine::ParseError<combine::State<IteratorStream<vec::IntoIter<Token>>>>),

    #[error_chain(custom)]
    Eof,

    #[error_chain(custom)]
    Exit(i32),

    // #[error_chain(custom)]
    // #[error_chain(description = r#"|_, _| "type error""#)]
    // #[error_chain(display = r#"|f, value, type| write!(f, "type error: received {}, expected {}", value, type)"#)]
    // Type(Expr, String),
}
