pub mod rust_highlight_config;

use tree_sitter_highlight::{HighlightConfiguration, Highlighter};

use self::rust_highlight_config::{calculate_rust_highlights, get_rust_highlight_config};

use super::filetypes::FileType;

pub type HighlightingFn =
    fn(&[u8], &mut Highlighter, &HighlightConfiguration) -> Vec<HighlightSpan>;

#[derive(Copy, Clone, Debug)]
pub enum HighlightGroup {
    Attribute,
    Comment,
    Constant,
    ConstantBuiltin,
    Constructor,
    Escape,
    Function,
    FunctionBuiltin,
    FunctionMethod,
    FunctionMacro,
    Keyword,
    Label,
    Operator,
    Property,
    Punctuation,
    PunctuationBracket,
    PunctuationDelimiter,
    String,
    StringSpecial,
    Tag,
    Type,
    TypeBuiltin,
    Variable,
    VariableBuiltin,
    VariableParameter,
    None,
}

#[derive(Debug)]
pub struct HighlightSpan {
    pub group: HighlightGroup,
    pub start: usize,
    pub end: usize,
}

pub fn get_highlighting_function(filetype: FileType) -> HighlightingFn {
    match filetype {
        FileType::Rust => calculate_rust_highlights,
    }
}

pub fn get_highlighting_config(filetype: FileType) -> HighlightConfiguration {
    match filetype {
        FileType::Rust => get_rust_highlight_config(),
    }
}
