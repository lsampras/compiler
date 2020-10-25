use std::cell::Cell;

struct LexerCursor {
    data: &String,
    index: Cell<usize>,
}

impl AsRef<&String> for LexerCursor {
    fn as_ref(&self) -> &String {}
}
