//! Turns document text into the app's own element tree.
//! Owns no knowledge of GPUI, colors, sizes, or fonts.
//!
//! HU-01 only needs enough of the tree to prove a document renders
//! "with formatting, not raw": headings and paragraphs. Lists, tables,
//! code blocks and the rest of CommonMark/GFM arrive with HU-02/HU-03.

use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};

/// A block-level element of a document.
pub enum Block {
    Heading(u8, String),
    Paragraph(String),
}

/// Parses Markdown source into a flat list of blocks.
pub fn parse(source: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut current: Option<(Option<HeadingLevel>, String)> = None;

    for event in Parser::new(source) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => current = Some((Some(level), String::new())),
            Event::Start(Tag::Paragraph) => current = Some((None, String::new())),
            Event::End(TagEnd::Heading(_)) | Event::End(TagEnd::Paragraph) => {
                if let Some((level, text)) = current.take() {
                    if !text.trim().is_empty() {
                        blocks.push(match level {
                            Some(level) => Block::Heading(level as u8, text),
                            None => Block::Paragraph(text),
                        });
                    }
                }
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some((_, buf)) = current.as_mut() {
                    buf.push_str(&text);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some((_, buf)) = current.as_mut() {
                    buf.push(' ');
                }
            }
            _ => {}
        }
    }

    blocks
}
