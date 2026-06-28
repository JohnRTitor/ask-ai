use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use colored::Colorize;
use crate::render::syntax::highlight_code;

pub fn render_markdown(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut output = String::new();
    let mut in_code_block = false;
    let mut current_language = String::new();
    let mut current_code = String::new();

    let mut in_think_block = false;

    // A simple state machine to detect <think>...</think>
    // We do a pre-pass to handle `<think>` since pulldown_cmark might treat it as raw HTML
    // We'll replace it with a custom format before parsing, or handle it during parsing.
    
    // Actually, simpler to just parse HTML tags in pulldown_cmark
    
    for event in parser {
        match event {
            Event::Start(Tag::Heading { level: _level, .. }) => {
                output.push('\n');
            }
            Event::End(TagEnd::Heading(_level)) => {
                output.push('\n');
                output.push('\n');
            }
            Event::Start(Tag::CodeBlock(pulldown_cmark::CodeBlockKind::Fenced(lang))) => {
                in_code_block = true;
                current_language = lang.to_string();
                current_code.clear();
                output.push('\n');
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let highlighted = highlight_code(&current_code, &current_language);
                output.push_str(&highlighted);
                output.push('\n');
            }
            Event::Text(text) => {
                if in_code_block {
                    current_code.push_str(&text);
                } else if in_think_block {
                    output.push_str(&text.dimmed().to_string());
                } else {
                    output.push_str(&text);
                }
            }
            Event::Html(html) => {
                if html.contains("<think>") {
                    in_think_block = true;
                    output.push_str(&"\n[Thinking...]\n".dimmed().italic().to_string());
                } else if html.contains("</think>") {
                    in_think_block = false;
                    output.push_str(&"\n".to_string());
                } else {
                    output.push_str(&html);
                }
            }
            Event::Code(code) => {
                output.push_str(&code.yellow().to_string());
            }
            Event::Start(Tag::Strong) => {
                // Not perfectly supported with plain string builder due to nesting, but works well enough for basic usage
                // We'd need a stack of styles for perfect nesting
            }
            Event::End(TagEnd::Strong) => {}
            Event::Start(Tag::Emphasis) => {}
            Event::End(TagEnd::Emphasis) => {}
            Event::Start(Tag::List(_)) => output.push('\n'),
            Event::End(TagEnd::List(_)) => output.push('\n'),
            Event::Start(Tag::Item) => output.push_str("• "),
            Event::End(TagEnd::Item) => output.push('\n'),
            Event::SoftBreak | Event::HardBreak => output.push('\n'),
            Event::Start(Tag::Paragraph) => {}
            Event::End(TagEnd::Paragraph) => output.push('\n'),
            _ => {}
        }
    }

    output
}
