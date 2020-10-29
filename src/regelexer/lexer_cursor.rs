use std::cell::Cell;

pub struct LexerCursor<'a> {
    data: &'a String,
    index: Cell<usize>,
}

impl<'a> AsRef<String> for LexerCursor<'a> {
    fn as_ref(&self) -> &String {
        unimplemented!();
    }
}
