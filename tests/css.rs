
// mod regelexer;
use regelexer_derive::LexerMacro;
use compiler::regelexer::lexer_enum::LexerEnumTrait;
// use super::regelexer::lexer_enum::LexerMacro;


#[derive(LexerMacro,Debug, PartialEq)]
enum Pancakes {
    #[regex("aaba",skip)]
    Error,
    #[token("baa")]
    Test
}

#[test]
pub fn sample_test() {
    assert_eq!(Pancakes::get_skips(), vec![&Pancakes::Error]);
    assert_eq!(Pancakes::get_regexes(), vec!["aaba"]);
    assert_eq!(Pancakes::get_literals(), vec!["baa"]);
    assert_eq!(Pancakes::get_enum_from_pattern("aaba"), &Pancakes::Error);
    assert_eq!(Pancakes::get_enum_from_pattern("baa"), &Pancakes::Test);
}