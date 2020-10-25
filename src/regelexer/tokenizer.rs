use crate::regelexer::lexer_cursor::LexerCursor;
use crate::regelexer::lexer_enum::LexerEnum;
use regex::{Regex, RegexSet};

pub struct Tokenizer {
    regex_set: RegSet,
    regexes: Vector<Regex>,
}

impl Tokenizer {
    /// Creates a new tokenizer
    pub fn new() -> Self {}

    /// Extracts and returns a LexerEnum from the given cursor
    /// It also updates the cursor the point ahead of the extracted
    /// string.
    pub fn next_enum(&self, cur: &LexerCursor) -> LexerEnum {}

    /// Returns the index of the regex in regexes that matched
    /// the regex set. It assumes that the given regex set will
    /// match on one and exactly one regex value.
    fn match_regex_set(&self, cur: &LexerCursor) -> usize {}

    /// Extracts and returns the string that matches
    /// the given regex in the cursor.
    fn match_regex(&self, index: usize, cur: &LexerCursor) -> String {}
}
