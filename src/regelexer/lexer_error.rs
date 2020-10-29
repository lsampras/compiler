use snafu::{Backtrace, Snafu};
use std::path::PathBuf;

#[derive(Debug, Snafu)]
pub enum Error {
    // error when trying to open file
    OpenFile {
        filename: PathBuf,
        source: std::io::Error,
    },
    // error on trying to read file to string
    Readfile {
        filename: PathBuf,
        source: std::io::Error,
    },
    // regex set fails to compile
    RegexSet {
        regex_pattern: String,
    },
    // regex pattern fails to compile
    Regex {
        source: regex::Error,
    },
}
