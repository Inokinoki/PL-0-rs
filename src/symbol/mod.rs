
pub const SYMBOL_NUMBER: u8 = 32;
/* legal symbols */
pub enum symbol {
    nul,
    ident,
    number,
    plus,
    minus,
    times,
    slash,
    oddsym,
    eql,
    neq,
    lss,
    leq,
    gtr,
    geq,
    lparen,
    rparen,
    comma,
    semicolon,
    period,
    becomes,
    beginsym,
    endsym,
    ifsym,
    thensyn,
    whilesym,
    writesym,
    readsym,
    dosym,
    callsym,
    constsym,
    varsym,
    procsym,
}

pub mod io;
