use std::sync::LazyLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{LinesWithEndings, as_24_bit_terminal_escaped};

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

pub fn highlight_code(code: &str, language: &str) -> String {
    let syntax = SYNTAX_SET
        .find_syntax_by_token(language)
        .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

    let theme = &THEME_SET.themes["base16-ocean.dark"];
    let mut h = HighlightLines::new(syntax, theme);

    let mut output = String::new();
    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &SYNTAX_SET).unwrap_or_default();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        output.push_str(&escaped);
    }

    // Reset terminal color at the end
    output.push_str("\x1b[0m");
    output
}
