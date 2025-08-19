pub type CompileStaticString = &'static str;

pub const MALFORMED_FILE: CompileStaticString = "The buffer for parsing is malformed.";
pub const UNEXPECTED: CompileStaticString = "An unexpected error occurred.";
pub const USIZE_OVERFLOW: CompileStaticString = "The given usize is larger than a 64-bit integer.";