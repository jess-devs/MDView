//! Turns document text into the app's own element tree.
//! Owns no knowledge of GPUI, colors, sizes, or fonts.
//!
//! Code blocks (RF-10) are parsed into `Block::CodeBlock` so the shared test
//! fixture round-trips, but their *rendering* is HU-03's job: HU-02 only
//! needs them not to break the rest of the document.

use pulldown_cmark::{Alignment, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::iter::Peekable;

#[derive(Clone, Default)]
pub struct SpanStyle {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub code: bool,
}

pub struct Span {
    pub text: String,
    pub style: SpanStyle,
    /// The link destination this span is part of, if any (RF-08.2). `render`
    /// decides how to make it look distinguishable; this module only carries
    /// the data.
    pub url: Option<String>,
}

pub struct ListItem {
    pub checked: Option<bool>,
    pub children: Vec<Block>,
}

pub enum Block {
    Heading(u8, Vec<Span>),
    Paragraph(Vec<Span>),
    List { ordered: bool, start: u64, items: Vec<ListItem> },
    Quote(Vec<Block>),
    ThematicBreak,
    Table { header: Vec<Vec<Span>>, rows: Vec<Vec<Vec<Span>>> },
    CodeBlock(String),
}

/// Parses Markdown source into a list of top-level blocks.
pub fn parse(source: &str) -> Vec<Block> {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    let mut events = Parser::new_ext(source, options).peekable();
    parse_blocks(&mut events)
}

type Events<'a> = Peekable<Parser<'a>>;

fn is_block_start(tag: &Tag) -> bool {
    matches!(
        tag,
        Tag::Paragraph
            | Tag::Heading { .. }
            | Tag::BlockQuote(_)
            | Tag::List(_)
            | Tag::Table(_)
            | Tag::CodeBlock(_)
            | Tag::HtmlBlock
    )
}

/// Parses a sequence of blocks until the iterator is exhausted or the next
/// event is an `End` that belongs to an enclosing caller (which does not
/// consume it: the caller does, after this returns).
fn parse_blocks(events: &mut Events) -> Vec<Block> {
    use pulldown_cmark::Event::*;

    let mut blocks = Vec::new();
    loop {
        match events.peek() {
            None => break,
            Some(End(_)) => break,
            Some(Rule) => {
                events.next();
                blocks.push(Block::ThematicBreak);
            }
            Some(Start(tag)) if is_block_start(tag) => {
                let tag = tag.clone();
                events.next();
                match tag {
                    Tag::Paragraph => {
                        let spans = parse_inline(events);
                        consume_end(events, |e| matches!(e, TagEnd::Paragraph));
                        if !spans.is_empty() {
                            blocks.push(Block::Paragraph(spans));
                        }
                    }
                    Tag::Heading { level, .. } => {
                        let spans = parse_inline(events);
                        consume_end(events, |e| matches!(e, TagEnd::Heading(_)));
                        blocks.push(Block::Heading(heading_level(level), spans));
                    }
                    Tag::BlockQuote(_) => {
                        let inner = parse_blocks(events);
                        consume_end(events, |e| matches!(e, TagEnd::BlockQuote(_)));
                        blocks.push(Block::Quote(inner));
                    }
                    Tag::List(start) => {
                        let items = parse_list_items(events);
                        consume_end(events, |e| matches!(e, TagEnd::List(_)));
                        blocks.push(Block::List {
                            ordered: start.is_some(),
                            start: start.unwrap_or(1),
                            items,
                        });
                    }
                    Tag::Table(alignments) => {
                        blocks.push(parse_table(events, &alignments));
                        consume_end(events, |e| matches!(e, TagEnd::Table));
                    }
                    Tag::CodeBlock(_) => {
                        let mut code = String::new();
                        loop {
                            match events.next() {
                                Some(Text(t)) => code.push_str(&t),
                                Some(End(TagEnd::CodeBlock)) | None => break,
                                _ => {}
                            }
                        }
                        blocks.push(Block::CodeBlock(code));
                    }
                    Tag::HtmlBlock => {
                        // Raw HTML has no element-tree representation yet
                        // (RF-13, a later feature): consume it so its `End`
                        // doesn't get mistaken for an enclosing caller's, and
                        // move on without producing a block.
                        loop {
                            match events.next() {
                                Some(End(TagEnd::HtmlBlock)) | None => break,
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            Some(Start(_)) => {
                // Not a recognized block start: treat as an implicit paragraph
                // (e.g. a tight list item's bare inline content).
                let spans = parse_inline(events);
                if !spans.is_empty() {
                    blocks.push(Block::Paragraph(spans));
                } else {
                    events.next();
                }
            }
            _ => {
                let spans = parse_inline(events);
                if !spans.is_empty() {
                    blocks.push(Block::Paragraph(spans));
                } else {
                    events.next();
                }
            }
        }
    }
    blocks
}

fn consume_end(events: &mut Events, matches_end: impl Fn(&TagEnd) -> bool) {
    if let Some(pulldown_cmark::Event::End(end)) = events.peek() {
        if matches_end(end) {
            events.next();
        }
    }
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Parses inline-level events (text and emphasis/strong/strikethrough/code)
/// into a flat list of styled spans, stopping (without consuming) at the
/// first event that is not inline-level.
fn parse_inline(events: &mut Events) -> Vec<Span> {
    use pulldown_cmark::Event::*;

    let mut spans = Vec::new();
    let mut style = SpanStyle::default();
    // The URL of the link this text is currently inside, if any. Images are
    // consumed the same way as links so their `End` doesn't leak out (see
    // `parse_blocks`), but they don't carry a URL: showing the image itself
    // is a later feature, so their alt text falls back to plain text.
    let mut link: Option<String> = None;

    loop {
        match events.peek() {
            Some(Text(_)) | Some(Code(_)) | Some(SoftBreak) | Some(HardBreak) => {}
            Some(Start(Tag::Emphasis)) | Some(End(TagEnd::Emphasis)) => {}
            Some(Start(Tag::Strong)) | Some(End(TagEnd::Strong)) => {}
            Some(Start(Tag::Strikethrough)) | Some(End(TagEnd::Strikethrough)) => {}
            Some(Start(Tag::Link { .. })) | Some(End(TagEnd::Link)) => {}
            Some(Start(Tag::Image { .. })) | Some(End(TagEnd::Image)) => {}
            _ => break,
        }

        match events.next().unwrap() {
            Text(t) => spans.push(Span { text: t.into_string(), style: style.clone(), url: link.clone() }),
            Code(t) => {
                let mut s = style.clone();
                s.code = true;
                spans.push(Span { text: t.into_string(), style: s, url: link.clone() });
            }
            SoftBreak => spans.push(Span { text: " ".to_string(), style: style.clone(), url: link.clone() }),
            HardBreak => spans.push(Span { text: "\n".to_string(), style: style.clone(), url: link.clone() }),
            Start(Tag::Emphasis) => style.italic = true,
            End(TagEnd::Emphasis) => style.italic = false,
            Start(Tag::Strong) => style.bold = true,
            End(TagEnd::Strong) => style.bold = false,
            Start(Tag::Strikethrough) => style.strikethrough = true,
            End(TagEnd::Strikethrough) => style.strikethrough = false,
            Start(Tag::Link { dest_url, .. }) => link = Some(dest_url.into_string()),
            End(TagEnd::Link) => link = None,
            Start(Tag::Image { .. }) | End(TagEnd::Image) => {}
            _ => unreachable!(),
        }
    }

    spans
}

fn parse_list_items(events: &mut Events) -> Vec<ListItem> {
    use pulldown_cmark::Event::*;

    let mut items = Vec::new();
    while let Some(Start(Tag::Item)) = events.peek() {
        events.next();

        let checked = match events.peek() {
            Some(TaskListMarker(_)) => match events.next() {
                Some(TaskListMarker(checked)) => Some(checked),
                _ => None,
            },
            _ => None,
        };

        let children = parse_blocks(events);
        consume_end(events, |e| matches!(e, TagEnd::Item));
        items.push(ListItem { checked, children });
    }
    items
}

fn parse_table(events: &mut Events, _alignments: &[Alignment]) -> Block {
    use pulldown_cmark::Event::*;

    let mut header = Vec::new();
    let mut rows = Vec::new();

    if let Some(Start(Tag::TableHead)) = events.peek() {
        events.next();
        header = parse_table_cells(events);
        consume_end(events, |e| matches!(e, TagEnd::TableHead));
    }

    while let Some(Start(Tag::TableRow)) = events.peek() {
        events.next();
        rows.push(parse_table_cells(events));
        consume_end(events, |e| matches!(e, TagEnd::TableRow));
    }

    Block::Table { header, rows }
}

fn parse_table_cells(events: &mut Events) -> Vec<Vec<Span>> {
    use pulldown_cmark::Event::*;

    let mut cells = Vec::new();
    while let Some(Start(Tag::TableCell)) = events.peek() {
        events.next();
        cells.push(parse_inline(events));
        consume_end(events, |e| matches!(e, TagEnd::TableCell));
    }
    cells
}

// Regression coverage for the bug in "Defecto observado antes de empezar"
// (docs/features/enlaces-e-imagenes/plan.md): `parse_blocks` broke out of its
// loop on ANY `End` event, including ones that belonged to a Link, Image or
// HtmlBlock rather than to the caller, silently discarding the rest of the
// document.
#[cfg(test)]
mod tests {
    use super::*;

    fn heading_texts(blocks: &[Block]) -> Vec<String> {
        blocks
            .iter()
            .filter_map(|b| match b {
                Block::Heading(_, spans) => Some(spans.iter().map(|s| s.text.as_str()).collect()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn html_block_does_not_truncate_the_document() {
        let source = "# uno\n\n<div align=\"center\">hola</div>\n\n# dos\n";
        assert_eq!(heading_texts(&parse(source)), vec!["uno".to_string(), "dos".to_string()]);
    }

    #[test]
    fn link_and_image_stay_in_one_paragraph_with_the_rest_of_the_text() {
        let source = "un [enlace](https://x.dev) y ![alt](img/a.png) final\n";
        let blocks = parse(source);

        let paragraphs: Vec<&Vec<Span>> = blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(spans) => Some(spans),
                _ => None,
            })
            .collect();
        assert_eq!(paragraphs.len(), 1, "el enlace no debe partir el párrafo en dos");

        let spans = paragraphs[0];
        let text: String = spans.iter().map(|s| s.text.as_str()).collect();
        assert_eq!(text, "un enlace y alt final");
        assert!(!text.contains("https://x.dev"), "la URL no debe aparecer en el cuerpo del documento");

        let link_span = spans.iter().find(|s| s.text == "enlace").expect("falta el texto del enlace");
        assert_eq!(link_span.url.as_deref(), Some("https://x.dev"));
    }
}
