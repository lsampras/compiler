use std::cell::Cell;

pub struct LexerCursor<'a> {
    data: &'a String,
    index: Cell<usize>,
}

impl AsRef<String> for LexerCursor {
    fn as_ref(&self) -> &String {
        unimplemented!();
    }
}
