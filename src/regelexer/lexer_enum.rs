pub enum LexerEnum {
    /// Keywords
    /// Skip
    /// Literals
    /// Error
    Error,
}

/// List of regex patterns
///
/// Macro return enum instance with string from regex pattern
///
/// Return boolean whether current token is skip
impl LexerEnum {}

pub trait LexerEnumTrait {
    // Using references since self size is not known at compile time
    // TODO : evaluate lifetime parameters
    // fn get_patterns();
    // fn get_enum_from_pattern(pattern: String) -> Self;
    fn get_enum_from_pattern(input: &str) -> &'static Self;
    fn get_skips() -> Vec<&'static Self>;
    fn get_regexes() -> Vec<String>;
    fn get_literals() -> Vec<String>;
}
