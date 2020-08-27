use std::ops::Range;

use crate::symbol;

use logos::{ Lexer, Logos };

pub struct PL0Lexer<'a> {
    lexer: Lexer<'a, symbol::Symbol>,
    current_symbol: symbol::Symbol,
    current_symbol_content: String,
    previous_symbol: symbol::Symbol,
    keep_current_once: bool,
}

impl PL0Lexer<'_> {
    pub fn create_from_content(content: &str) -> PL0Lexer {
        let lexer = PL0Lexer {
            lexer: symbol::Symbol::lexer(content),
            current_symbol: symbol::Symbol::Nul,
            current_symbol_content: String::new(),
            previous_symbol: symbol::Symbol::Nul,
            keep_current_once: false,
        };
        lexer
    }

    pub fn next(&mut self) -> &symbol::Symbol {
        if !self.keep_current_once {
            self.previous_symbol = self.current_symbol;
            self.current_symbol = self.lexer.next().unwrap_or(symbol::Symbol::EOF);
            self.current_symbol_content = self.lexer.slice().to_string();
        } else {
            self.keep_current_once = false;
        }
        &self.current_symbol
    }

    pub fn current(&self) -> &symbol::Symbol {
        &self.current_symbol
    }

    pub fn current_content(&self) -> &str {
        &self.current_symbol_content
    }

    pub fn current_index(&self) -> Range<usize> {
        self.lexer.span()
    }

    pub fn previous(&self) -> symbol::Symbol {
        self.previous_symbol
    }

    pub fn keep_once(&mut self) {
        self.keep_current_once = true;
    }
}
