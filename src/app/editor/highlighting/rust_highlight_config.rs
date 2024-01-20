use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
use tree_sitter_rust::{language, HIGHLIGHT_QUERY, INJECTIONS_QUERY, TAGGING_QUERY};

use super::{HighlightGroup, HighlightSpan};

const RUST_HIGHLIGHT_NAMES: [&str; 25] = [
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "escape",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "keyword",
    "label",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

pub fn get_rust_highlight_config() -> HighlightConfiguration {
    let lang = language();
    let mut config =
        HighlightConfiguration::new(lang, HIGHLIGHT_QUERY, INJECTIONS_QUERY, TAGGING_QUERY)
            .unwrap();
    config.configure(&RUST_HIGHLIGHT_NAMES);
    config
}

pub fn calculate_rust_highlights(
    content: &[u8],
    highlighter: &mut Highlighter,
    highlighter_config: &HighlightConfiguration,
) -> Vec<HighlightSpan> {
    let highlights = highlighter
        .highlight(highlighter_config, content, None, |_| None)
        .unwrap();

    let mut current_style = HighlightGroup::None;
    let mut highlit_groups = vec![];
    for event in highlights {
        match event.unwrap() {
            HighlightEvent::Source { start, end } => highlit_groups.push(HighlightSpan {
                group: current_style,
                start,
                end,
            }),
            HighlightEvent::HighlightStart(s) => {
                current_style = match s.0 {
                    0 => HighlightGroup::Attribute,
                    1 => HighlightGroup::Comment,
                    2 => HighlightGroup::Constant,
                    3 => HighlightGroup::ConstantBuiltin,
                    4 => HighlightGroup::Constructor,
                    5 => HighlightGroup::Escape,
                    6 => HighlightGroup::Function,
                    7 => HighlightGroup::FunctionBuiltin,
                    8 => HighlightGroup::FunctionMethod,
                    9 => HighlightGroup::FunctionMacro,
                    10 => HighlightGroup::Keyword,
                    11 => HighlightGroup::Label,
                    12 => HighlightGroup::Operator,
                    13 => HighlightGroup::Property,
                    14 => HighlightGroup::Punctuation,
                    15 => HighlightGroup::PunctuationBracket,
                    16 => HighlightGroup::PunctuationDelimiter,
                    17 => HighlightGroup::String,
                    18 => HighlightGroup::StringSpecial,
                    19 => HighlightGroup::Tag,
                    20 => HighlightGroup::Type,
                    21 => HighlightGroup::TypeBuiltin,
                    22 => HighlightGroup::Variable,
                    23 => HighlightGroup::VariableBuiltin,
                    24 => HighlightGroup::VariableParameter,
                    _ => HighlightGroup::None,
                };
            }
            HighlightEvent::HighlightEnd => current_style = HighlightGroup::None,
        }
    }
    highlit_groups
}
