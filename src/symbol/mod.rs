
use logos::Logos;

/* legal symbols */
#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Symbol {
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    #[regex("[0-9]+")]
    Number,
    
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Times,

    #[token("/")]
    Slash,

    #[token("odd")]
    Oddsym,

    #[token("=")]
    Eql,

    #[token("!=")]
    Neq,

    #[token("<")]
    Lss,

    #[token("<=")]
    Leq,

    #[token(">")]
    Gtr,

    #[token(">=")]
    Geq,

    #[token("(")]
    Lparen,

    #[token(")")]
    Rparen,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token(".")]
    Period,

    #[token(":=")]
    Becomes,

    #[token("begin")]
    Beginsym,

    #[token("end")]
    Endsym,

    #[token("if")]
    Ifsym,

    #[token("then")]
    Thensyn,

    #[token("while")]
    Whilesym,

    #[token("write")]
    Writesym,

    #[token("read")]
    Readsym,

    #[token("do")]
    Dosym,

    #[token("call")]
    Callsym,

    #[token("const")]
    Constsym,

    #[token("var")]
    Varsym,

    #[token("procedure")]
    Procsym,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Nul,

    EOF,
}

pub mod io;
