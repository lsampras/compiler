use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;
use std::path::Path;

use crate::regelexer::lexer_cursor::LexerCursor;
use crate::regelexer::lexer_enum::LexerEnum;
use crate::regelexer::tokenizer::Tokenizer;

pub struct Lexer {
    file_path: String,
    file_contents: String,
    tokenizer: Tokenizer,
    cursor: LexerCursor,
    tokens: Vec<LexerToken>,
}

pub struct LexerToken {
    lex_enum: LexerEnum,
}

impl Lexer {
    /// Create new Lexer
    pub fn new() -> Self {}

    /// Open and read input file into memory
    fn read_file(file_path: &String) -> String {}

    /// Parse and tokenize input. Store tokes in a vector
    pub fn lex_input() {}
}
