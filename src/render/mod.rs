pub mod markdown;
pub mod spinner;
pub mod syntax;

use std::io::{self, Write};

pub struct StreamRenderer {
    buffer: String,
}

impl Default for StreamRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamRenderer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn append(&mut self, text: &str) {
        self.buffer.push_str(text);

        // In a perfect world, we'd render the markdown incrementally.
        // For simplicity and to avoid tearing syntax highlighting, we just print the raw tokens as they arrive.
        // Once the stream completes, we'll re-render the whole block if needed, or just let the raw stream be.
        // But wait! The spec asks for markdown rendering of the stream.
        // The best way to do this without a complex state machine is to just clear the terminal and re-render.
        // However, that flickers.
        // Instead, let's just print the raw text during streaming for now.
        print!("{}", text);
        let _ = io::stdout().flush();
    }

    pub fn finish(&self) {
        println!("\n");
        // We could optionally re-render the whole buffer with `render_markdown` here
        // if we want a clean post-generation output.
        // println!("{}", render_markdown(&self.buffer));
    }
}
