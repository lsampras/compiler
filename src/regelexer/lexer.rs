use super::lexer_cursor::LexerCursor;
use super::lexer_enum::LexerEnum;
use super::lexer_error::Error;
use super::tokenizer::Tokenizer;

pub struct Lexer<'a> {
    file_path: String,
    file_contents: &'a String,
    tokenizer: Tokenizer,
    cursor: LexerCursor<'a>,
    tokens: Vec<LexerToken>,
}

/// Contains lexer enum extracted from input and
/// meta data like line number, col number etc
pub struct LexerToken {
    lex_enum: LexerEnum,
}

impl<'a> Lexer<'a> {
    /// Create new Lexer
    pub fn new(filepath: &'a String) -> Result<Self, Error> {
        unimplemented!();
    }

    /// Open and read input file into memory
    fn read_file(file_path: &String) -> String {
        unimplemented!();
    }

    /// Parse and tokenize input. Store tokens in a vector
    fn lex_input() {
        unimplemented!();
    }
}

impl<'a> IntoIterator for Lexer<'a> {
    type Item = LexerToken;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!();
    }
}
