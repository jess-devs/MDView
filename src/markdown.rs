//! Turns document text into the app's own element tree.
//! Owns no knowledge of GPUI, colors, sizes, or fonts.
//!
//! Code blocks (RF-10) are parsed into `Block::CodeBlock` so the shared test
//! fixture round-trips, but their *rendering* is HU-03's job: HU-02 only
//! needs them not to break the rest of the document.

use pulldown_cmark::{Alignment, HeadingLevel, Options, Parser, Tag, TagEnd};
use std::{
    iter::Peekable,
    path::{Path, PathBuf},
};

#[derive(Clone, Default)]
pub struct SpanStyle {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub code: bool,
}

#[derive(Clone)]
pub struct Span {
    pub text: String,
    pub style: SpanStyle,
    /// The link destination this span is part of, if any (RF-08.2). `render`
    /// decides how to make it look distinguishable; this module only carries
    /// the data.
    pub url: Option<String>,
}

/// An image reference (RF-11.1). `alt` is always available, for the case
/// `render` can't show the image at all (CA-03.3, CA-03.4).
pub struct ImageRef {
    pub alt: String,
    /// `dest` resolved against the document's directory, or `None` if it
    /// couldn't be: no base directory was known, or `dest` is an `http(s)`
    /// URL, which RNF-02.1 forbids fetching (AD-20).
    pub resolved: Option<PathBuf>,
}

/// One piece of a paragraph's content, in document order. Plain text runs
/// (`Span`) and images (`ImageRef`) are kept apart, rather than folding an
/// image's alt text into the span stream, so `render` can show an actual
/// image instead of its alt text standing in for it (AD-20).
pub enum Inline {
    Span(Span),
    Image(ImageRef),
}

pub struct ListItem {
    pub checked: Option<bool>,
    pub children: Vec<Block>,
}

pub enum Block {
    /// The document's front matter (RF-12.1), as ordered (name, value) pairs.
    /// Only ever the first block, if present at all: see `extract_front_matter`.
    FrontMatter(Vec<(String, String)>),
    Heading(u8, Vec<Span>),
    Paragraph(Vec<Inline>),
    List { ordered: bool, start: u64, items: Vec<ListItem> },
    Quote(Vec<Block>),
    ThematicBreak,
    Table { header: Vec<Vec<Span>>, rows: Vec<Vec<Vec<Span>>> },
    CodeBlock(String),
    /// A `<div align="center">` (RF-13.1, HU-03): its content, centered.
    /// CommonMark has no equivalent construct, so this has no Markdown
    /// counterpart — unlike every other `Block` variant.
    Centered(Vec<Inline>),
}

/// Parses Markdown source into a list of top-level blocks. `base_dir` is the
/// directory of the `.md` file being shown, used to resolve relative image
/// paths (RF-11.1); pass `None` when there is no file on disk to resolve
/// against (e.g. in isolation, as the tests below do).
pub fn parse(source: &str, base_dir: Option<&Path>) -> Vec<Block> {
    let (front_matter, rest) = extract_front_matter(source);

    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TASKLISTS;
    let mut events = Parser::new_ext(rest, options).peekable();

    let mut blocks = Vec::new();
    if let Some(properties) = front_matter {
        blocks.push(Block::FrontMatter(properties));
    }
    blocks.extend(parse_blocks(&mut events, base_dir));
    blocks
}

/// Pulls a leading front matter block off `source` (RF-12.1), by hand rather
/// than with `pulldown_cmark`'s own `ENABLE_YAML_STYLE_METADATA_BLOCKS`: that
/// option recognizes a `---`-delimited block at *any* block boundary in the
/// document, not only the first — reading `pulldown-cmark`'s `firstpass.rs`
/// confirms `scan_metadata_block` carries no such restriction. RF-12.1 only
/// ever means the document's very first line, so this checks that directly on
/// the raw text instead of trusting the library's broader definition.
///
/// Returns the parsed properties (`None` if there is no front matter) and the
/// remaining source with the front matter block —delimiters included— cut
/// off the front, ready for `pulldown_cmark` to parse as before.
fn extract_front_matter(source: &str) -> (Option<Vec<(String, String)>>, &str) {
    let Some(after_open) = source.strip_prefix("---") else {
        return (None, source);
    };
    // The opening `---` must be alone on its line: only a newline (or EOF)
    // may follow it, not e.g. `----` or `--- title`.
    let after_open = match after_open.strip_prefix("\r\n").or_else(|| after_open.strip_prefix('\n')) {
        Some(rest) => rest,
        None => return (None, source),
    };

    let mut properties = Vec::new();
    let mut cursor = after_open;
    loop {
        let line_end = cursor.find('\n').map(|i| i + 1).unwrap_or(cursor.len());
        let (line, remainder) = cursor.split_at(line_end);
        let trimmed = line.trim_end_matches(['\n', '\r']);

        if trimmed == "---" {
            return (Some(properties), remainder);
        }
        if remainder.is_empty() && trimmed != "---" {
            // Reached EOF without a closing delimiter: not front matter.
            return (None, source);
        }
        if let Some((key, value)) = trimmed.split_once(':') {
            properties.push((key.trim().to_string(), value.trim().to_string()));
        }
        cursor = remainder;
    }
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
fn parse_blocks(events: &mut Events, base_dir: Option<&Path>) -> Vec<Block> {
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
                        let inlines = parse_paragraph_inline(events, base_dir);
                        consume_end(events, |e| matches!(e, TagEnd::Paragraph));
                        if !inlines.is_empty() {
                            blocks.push(Block::Paragraph(inlines));
                        }
                    }
                    Tag::Heading { level, .. } => {
                        let spans = parse_inline_spans(events);
                        consume_end(events, |e| matches!(e, TagEnd::Heading(_)));
                        blocks.push(Block::Heading(heading_level(level), spans));
                    }
                    Tag::BlockQuote(_) => {
                        let inner = parse_blocks(events, base_dir);
                        consume_end(events, |e| matches!(e, TagEnd::BlockQuote(_)));
                        blocks.push(Block::Quote(inner));
                    }
                    Tag::List(start) => {
                        let items = parse_list_items(events, base_dir);
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
                        // pulldown_cmark hands the block's content over as
                        // raw text, one `Html` event per source line, tags
                        // and text mixed in the same string — never split at
                        // tag boundaries the way `InlineHtml` is (confirmed
                        // against 0.13.4 before writing this, see AD-22).
                        // Concatenating first and tokenizing the whole block
                        // means a tag never accidentally lands split across
                        // two lines.
                        let mut raw = String::new();
                        loop {
                            match events.next() {
                                Some(Html(t)) => raw.push_str(&t),
                                Some(End(TagEnd::HtmlBlock)) | None => break,
                                _ => {}
                            }
                        }
                        let tokens = crate::html::tokenize(&raw);
                        blocks.extend(parse_html_block_tokens(&tokens, base_dir));
                    }
                    _ => {}
                }
            }
            Some(Start(_)) => {
                // Not a recognized block start: treat as an implicit paragraph
                // (e.g. a tight list item's bare inline content).
                let inlines = parse_paragraph_inline(events, base_dir);
                if !inlines.is_empty() {
                    blocks.push(Block::Paragraph(inlines));
                } else {
                    events.next();
                }
            }
            _ => {
                let inlines = parse_paragraph_inline(events, base_dir);
                if !inlines.is_empty() {
                    blocks.push(Block::Paragraph(inlines));
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

/// Parses inline-level events (text, emphasis/strong/strikethrough/code,
/// links and images) into a flat list of `Inline`s, stopping (without
/// consuming) at the first event that is not inline-level. Used for
/// paragraphs, where an image is shown as an actual image (RF-11.1).
fn parse_paragraph_inline(events: &mut Events, base_dir: Option<&Path>) -> Vec<Inline> {
    use pulldown_cmark::Event::*;

    let mut out = Vec::new();
    let mut style = SpanStyle::default();
    // The URL of the link this text is currently inside, if any.
    let mut link: Option<String> = None;
    // Set while inside a Start(Image)..End(Image) pair: its destination and
    // the alt text accumulated from the Text events in between.
    let mut image: Option<(String, String)> = None;

    loop {
        match events.peek() {
            Some(Text(_)) | Some(Code(_)) | Some(SoftBreak) | Some(HardBreak) => {}
            Some(Start(Tag::Emphasis)) | Some(End(TagEnd::Emphasis)) => {}
            Some(Start(Tag::Strong)) | Some(End(TagEnd::Strong)) => {}
            Some(Start(Tag::Strikethrough)) | Some(End(TagEnd::Strikethrough)) => {}
            Some(Start(Tag::Link { .. })) | Some(End(TagEnd::Link)) => {}
            Some(Start(Tag::Image { .. })) | Some(End(TagEnd::Image)) => {}
            Some(InlineHtml(_)) => {}
            _ => break,
        }

        match events.next().unwrap() {
            Text(t) => match &mut image {
                Some((_, alt)) => alt.push_str(&t),
                None => out.push(Inline::Span(Span { text: t.into_string(), style: style.clone(), url: link.clone() })),
            },
            Code(t) => {
                let mut s = style.clone();
                s.code = true;
                out.push(Inline::Span(Span { text: t.into_string(), style: s, url: link.clone() }));
            }
            SoftBreak => match &mut image {
                Some((_, alt)) => alt.push(' '),
                None => out.push(Inline::Span(Span { text: " ".to_string(), style: style.clone(), url: link.clone() })),
            },
            HardBreak => out.push(Inline::Span(Span { text: "\n".to_string(), style: style.clone(), url: link.clone() })),
            Start(Tag::Emphasis) => style.italic = true,
            End(TagEnd::Emphasis) => style.italic = false,
            Start(Tag::Strong) => style.bold = true,
            End(TagEnd::Strong) => style.bold = false,
            Start(Tag::Strikethrough) => style.strikethrough = true,
            End(TagEnd::Strikethrough) => style.strikethrough = false,
            Start(Tag::Link { dest_url, .. }) => link = Some(dest_url.into_string()),
            End(TagEnd::Link) => link = None,
            Start(Tag::Image { dest_url, .. }) => image = Some((dest_url.into_string(), String::new())),
            End(TagEnd::Image) => {
                let (dest, alt) = image.take().expect("End(Image) without a matching Start(Image)");
                out.push(Inline::Image(ImageRef { resolved: resolve_image(base_dir, &dest), alt }));
            }
            InlineHtml(raw) => {
                if let Some(tag) = crate::html::parse_tag(&raw) {
                    apply_inline_html_tag(&tag, base_dir, &mut style, &mut link, &mut out);
                }
            }
            _ => unreachable!(),
        }
    }

    out
}

/// Reacts to one embedded HTML tag inside a run of inline content (RF-13.1):
/// a paragraph's flow (via `pulldown_cmark`'s `InlineHtml` events), or the
/// content of an HTML block tag like `<p>`/`<li>`/`<div>` (via
/// `collect_html_inline`, over tags `html::tokenize` already split out).
/// Only `b`/`strong`, `i`/`em`, `code`, `a`, `img` and `br` are recognized
/// here — the ones with a place in `Inline`/`Span` already, per AD-22;
/// anything else, open or close, is silently ignored, which is exactly what
/// shows its text without its markup: the text between an unrecognized
/// tag's open and close arrives as ordinary text this function never sees,
/// untouched by whatever the tag was.
fn apply_inline_html_tag(
    tag: &crate::html::Tag,
    base_dir: Option<&Path>,
    style: &mut SpanStyle,
    link: &mut Option<String>,
    out: &mut Vec<Inline>,
) {
    match tag.name.as_str() {
        "b" | "strong" => style.bold = !tag.closing,
        "i" | "em" => style.italic = !tag.closing,
        "code" => style.code = !tag.closing,
        "a" => *link = if tag.closing { None } else { tag.attr("href").map(str::to_string) },
        "br" => out.push(Inline::Span(Span { text: "\n".to_string(), style: style.clone(), url: link.clone() })),
        "img" if !tag.closing => {
            let alt = tag.attr("alt").unwrap_or("").to_string();
            let src = tag.attr("src").unwrap_or("");
            out.push(Inline::Image(ImageRef { resolved: resolve_image(base_dir, src), alt }));
        }
        _ => {}
    }
}

type HtmlTokens<'a> = std::iter::Peekable<std::slice::Iter<'a, crate::html::Token>>;

/// Parses a whole HTML block's tokens (RF-13.1, HU-03: `p`, `ul`/`li`, `hr`,
/// `div align="center"`) into `Block`s, in document order. A `pulldown_cmark`
/// HTML block can hold several of these top-level tags in a row (any two
/// HTML lines with no blank line between them share one block, confirmed
/// against 0.13.4), so this loops rather than expecting exactly one.
fn parse_html_block_tokens(tokens: &[crate::html::Token], base_dir: Option<&Path>) -> Vec<Block> {
    let mut iter = tokens.iter().peekable();
    let mut blocks = Vec::new();

    while let Some(token) = iter.peek() {
        match token {
            crate::html::Token::Text(_) => {
                iter.next(); // stray text outside any recognized tag: dropped
            }
            crate::html::Token::Tag(tag) if tag.closing => {
                iter.next(); // an unmatched closing tag: nothing open to close
            }
            crate::html::Token::Tag(tag) => {
                let name = tag.name.clone();
                let centered = tag.attr("align") == Some("center");
                iter.next();
                match name.as_str() {
                    "p" => {
                        let inlines = collect_html_inline(&mut iter, base_dir, "p");
                        if !inlines.is_empty() {
                            blocks.push(Block::Paragraph(inlines));
                        }
                    }
                    "hr" => blocks.push(Block::ThematicBreak),
                    "ul" => {
                        let items = parse_html_list_items(&mut iter, base_dir);
                        consume_html_close(&mut iter, "ul");
                        blocks.push(Block::List { ordered: false, start: 1, items });
                    }
                    "div" if centered => {
                        let inlines = collect_html_inline(&mut iter, base_dir, "div");
                        blocks.push(Block::Centered(inlines));
                    }
                    // Any other tag — including a `div` without centering,
                    // which RF-13.1 doesn't ask for — shows no markup of its
                    // own. Its content still surfaces: the loop keeps
                    // reading the tokens that follow, own closing tag
                    // included, exactly like an unlisted inline tag.
                    _ => {}
                }
            }
        }
    }

    blocks
}

fn parse_html_list_items(iter: &mut HtmlTokens, base_dir: Option<&Path>) -> Vec<ListItem> {
    let mut items = Vec::new();
    loop {
        match iter.peek() {
            Some(crate::html::Token::Tag(tag)) if !tag.closing && tag.name == "li" => {
                iter.next();
                let inlines = collect_html_inline(iter, base_dir, "li");
                items.push(ListItem {
                    checked: None,
                    children: if inlines.is_empty() { Vec::new() } else { vec![Block::Paragraph(inlines)] },
                });
            }
            Some(crate::html::Token::Text(t)) if t.trim().is_empty() => {
                iter.next(); // whitespace between </li> and the next <li>
            }
            _ => break,
        }
    }
    items
}

fn consume_html_close(iter: &mut HtmlTokens, name: &str) {
    if let Some(crate::html::Token::Tag(tag)) = iter.peek() {
        if tag.closing && tag.name == name {
            iter.next();
        }
    }
}

/// Reads inline content (text, and the tags `apply_inline_html_tag`
/// recognizes) until `stop_name`'s closing tag, which it consumes, or the
/// tokens run out. Shares `apply_inline_html_tag` with
/// `parse_paragraph_inline`: a `<b>` inside `<p>...</p>` means the same
/// thing as one inside a Markdown paragraph.
fn collect_html_inline(iter: &mut HtmlTokens, base_dir: Option<&Path>, stop_name: &str) -> Vec<Inline> {
    let mut out = Vec::new();
    let mut style = SpanStyle::default();
    let mut link: Option<String> = None;

    while let Some(token) = iter.peek() {
        match token {
            crate::html::Token::Text(text) => {
                out.push(Inline::Span(Span { text: text.clone(), style: style.clone(), url: link.clone() }));
                iter.next();
            }
            crate::html::Token::Tag(tag) if tag.closing && tag.name == stop_name => {
                iter.next();
                break;
            }
            crate::html::Token::Tag(tag) => {
                let tag = tag.clone();
                iter.next();
                apply_inline_html_tag(&tag, base_dir, &mut style, &mut link, &mut out);
            }
        }
    }

    out
}

/// The same inline grammar as `parse_paragraph_inline`, flattened to plain
/// `Span`s: an image's alt text stands in for it, exactly as it did before
/// this module could show an image at all. Used for headings and table
/// cells, which stay text-only (AD-20) — RF-11.1's criteria only ever put an
/// image in its own paragraph.
fn parse_inline_spans(events: &mut Events) -> Vec<Span> {
    parse_paragraph_inline(events, None)
        .into_iter()
        .map(|inline| match inline {
            Inline::Span(span) => span,
            Inline::Image(image) => Span { text: image.alt, style: SpanStyle::default(), url: None },
        })
        .collect()
}

/// Resolves an image's `dest` against the document's directory (RF-11.1).
/// Returns `None` when there is nothing to resolve against, or when `dest`
/// is an `http(s)` URL: RNF-02.1 forbids fetching it, so it always falls
/// back to its alt text instead (AD-20).
fn resolve_image(base_dir: Option<&Path>, dest: &str) -> Option<PathBuf> {
    if dest.starts_with("http://") || dest.starts_with("https://") {
        return None;
    }
    Some(base_dir?.join(dest))
}

fn parse_list_items(events: &mut Events, base_dir: Option<&Path>) -> Vec<ListItem> {
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

        let children = parse_blocks(events, base_dir);
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
        cells.push(parse_inline_spans(events));
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
        assert_eq!(heading_texts(&parse(source, None)), vec!["uno".to_string(), "dos".to_string()]);
    }

    fn front_matter_of(blocks: &[Block]) -> Option<&Vec<(String, String)>> {
        blocks.iter().find_map(|b| match b {
            Block::FrontMatter(properties) => Some(properties),
            _ => None,
        })
    }

    #[test]
    fn front_matter_becomes_ordered_properties_and_is_the_first_block() {
        let source = "---\ntitulo: Ejemplo\nautor: Jesus\nfecha: 2026-09-18\n---\n# Encabezado\n";
        let blocks = parse(source, None);

        assert!(matches!(blocks[0], Block::FrontMatter(_)), "el front matter debe ser el primer bloque");
        let properties = front_matter_of(&blocks).unwrap();
        assert_eq!(
            properties,
            &vec![
                ("titulo".to_string(), "Ejemplo".to_string()),
                ("autor".to_string(), "Jesus".to_string()),
                ("fecha".to_string(), "2026-09-18".to_string()),
            ]
        );

        assert_eq!(heading_texts(&blocks), vec!["Encabezado".to_string()]);
    }

    #[test]
    fn a_value_with_a_colon_is_not_truncated_at_the_first_one() {
        let source = "---\nhora: 10:30:00\n---\ntexto\n";
        let blocks = parse(source, None);
        let properties = front_matter_of(&blocks).unwrap();
        assert_eq!(properties, &vec![("hora".to_string(), "10:30:00".to_string())]);
    }

    #[test]
    fn a_thematic_break_that_is_not_the_first_line_is_not_front_matter() {
        // The same shape as front matter -- `---`, some lines, `---` -- but
        // starting after a paragraph. RF-12.1 is explicit that this doesn't
        // count: only the document's very first line does.
        let source = "un parrafo\n\n---\nclave: valor\n---\n";
        let blocks = parse(source, None);
        assert!(front_matter_of(&blocks).is_none());
    }

    #[test]
    fn an_unclosed_leading_dashes_block_is_not_front_matter() {
        let source = "---\nclave: valor\nsin cerrar\n";
        let blocks = parse(source, None);
        assert!(front_matter_of(&blocks).is_none());
        // Falls through to ordinary parsing instead of vanishing.
        let paragraphs: Vec<&Vec<Inline>> = blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(inlines) => Some(inlines),
                _ => None,
            })
            .collect();
        assert!(!paragraphs.is_empty(), "el contenido no reconocido como front matter debe seguir mostrandose");
    }

    fn image_in(inlines: &[Inline]) -> &ImageRef {
        inlines
            .iter()
            .find_map(|inline| match inline {
                Inline::Image(image) => Some(image),
                _ => None,
            })
            .expect("falta la imagen")
    }

    #[test]
    fn link_and_image_stay_in_one_paragraph_with_the_rest_of_the_text() {
        let source = "un [enlace](https://x.dev) y ![alt](img/a.png) final\n";
        let blocks = parse(source, None);

        let paragraphs: Vec<&Vec<Inline>> = blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(inlines) => Some(inlines),
                _ => None,
            })
            .collect();
        assert_eq!(paragraphs.len(), 1, "el enlace y la imagen no deben partir el párrafo en dos");
        let inlines = paragraphs[0];

        // The image no longer folds its alt text into the span stream (that
        // was the pre-HU-03 behavior, back when showing the image itself
        // wasn't implemented yet): it becomes its own `Inline::Image`.
        let span_texts: Vec<&str> = inlines
            .iter()
            .filter_map(|inline| match inline {
                Inline::Span(span) => Some(span.text.as_str()),
                Inline::Image(_) => None,
            })
            .collect();
        assert_eq!(span_texts, vec!["un ", "enlace", " y ", " final"]);
        assert!(
            !span_texts.iter().any(|t| t.contains("https://x.dev")),
            "la URL no debe aparecer en el cuerpo del documento"
        );

        let link_span = inlines
            .iter()
            .filter_map(|inline| match inline {
                Inline::Span(span) if span.text == "enlace" => Some(span),
                _ => None,
            })
            .next()
            .expect("falta el texto del enlace");
        assert_eq!(link_span.url.as_deref(), Some("https://x.dev"));

        let image = image_in(inlines);
        assert_eq!(image.alt, "alt");
        assert_eq!(image.resolved, None, "sin base_dir no hay nada que resolver");
    }

    #[test]
    fn a_lone_image_is_its_own_paragraph_not_mixed_with_text() {
        // The common README pattern: an image alone on its own line. It
        // must not need a surrounding text container to be shown.
        let blocks = parse("![captura](img/foto.png)\n", None);
        assert_eq!(blocks.len(), 1);
        match &blocks[0] {
            Block::Paragraph(inlines) => {
                assert_eq!(inlines.len(), 1);
                assert!(matches!(inlines[0], Inline::Image(_)));
            }
            _ => panic!("se esperaba un párrafo con la imagen"),
        }
    }

    #[test]
    fn image_path_resolves_against_the_documents_directory_not_the_cwd() {
        let base_dir = Path::new("/docs/proyecto");
        let blocks = parse("![captura](img/foto.png)\n", Some(base_dir));

        let paragraphs: Vec<&Vec<Inline>> = blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(inlines) => Some(inlines),
                _ => None,
            })
            .collect();
        let image = image_in(paragraphs[0]);
        assert_eq!(image.resolved.as_deref(), Some(Path::new("/docs/proyecto/img/foto.png")));
    }

    #[test]
    fn a_remote_image_is_never_resolved_to_a_local_path() {
        // RNF-02.1: MDView never requests a URL image, so there is no local
        // path to load — it always falls back to the alt text (AD-20).
        let base_dir = Path::new("/docs/proyecto");
        let blocks = parse("![remota](https://example.com/foto.png)\n", Some(base_dir));

        let paragraphs: Vec<&Vec<Inline>> = blocks
            .iter()
            .filter_map(|b| match b {
                Block::Paragraph(inlines) => Some(inlines),
                _ => None,
            })
            .collect();
        assert_eq!(image_in(paragraphs[0]).resolved, None);
    }

    fn first_paragraph(blocks: &[Block]) -> &Vec<Inline> {
        blocks
            .iter()
            .find_map(|b| match b {
                Block::Paragraph(inlines) => Some(inlines),
                _ => None,
            })
            .expect("falta el parrafo")
    }

    fn spans_in(inlines: &[Inline]) -> Vec<&Span> {
        inlines
            .iter()
            .filter_map(|inline| match inline {
                Inline::Span(span) => Some(span),
                Inline::Image(_) => None,
            })
            .collect()
    }

    #[test]
    fn inline_html_b_and_strong_apply_bold() {
        let blocks = parse("<b>uno</b> y <strong>dos</strong>\n", None);
        let spans = spans_in(first_paragraph(&blocks));
        let bold: Vec<&str> = spans.iter().filter(|s| s.style.bold).map(|s| s.text.as_str()).collect();
        assert_eq!(bold, vec!["uno", "dos"]);
    }

    #[test]
    fn inline_html_i_and_em_apply_italic() {
        let blocks = parse("<i>uno</i> y <em>dos</em>\n", None);
        let spans = spans_in(first_paragraph(&blocks));
        let italic: Vec<&str> = spans.iter().filter(|s| s.style.italic).map(|s| s.text.as_str()).collect();
        assert_eq!(italic, vec!["uno", "dos"]);
    }

    #[test]
    fn inline_html_code_applies_code_style() {
        let blocks = parse("texto <code>codigo</code> mas texto\n", None);
        let spans = spans_in(first_paragraph(&blocks));
        let code = spans.iter().find(|s| s.text == "codigo").expect("falta el span de codigo");
        assert!(code.style.code);
    }

    #[test]
    fn inline_html_a_carries_the_href_as_url() {
        let blocks = parse(r#"<a href="https://x.dev">enlace</a>"#, None);
        let spans = spans_in(first_paragraph(&blocks));
        let link = spans.iter().find(|s| s.text == "enlace").expect("falta el span del enlace");
        assert_eq!(link.url.as_deref(), Some("https://x.dev"));
    }

    #[test]
    fn inline_html_img_becomes_a_real_image_with_alt_and_resolved_path() {
        // Mixed into running text, not alone on its own line: a lone `<img>`
        // surrounded by blank lines is a block-level HTML block under
        // CommonMark's own rules (type 7), not inline HTML — that shape is
        // HU-03's territory, not this one.
        let base_dir = Path::new("/docs/proyecto");
        let blocks = parse(r#"foto: <img src="img/foto.png" alt="una foto"> fin"#, Some(base_dir));
        let image = image_in(first_paragraph(&blocks));
        assert_eq!(image.alt, "una foto");
        assert_eq!(image.resolved.as_deref(), Some(Path::new("/docs/proyecto/img/foto.png")));
    }

    #[test]
    fn inline_html_br_becomes_a_line_break() {
        let blocks = parse("antes<br>despues\n", None);
        let spans = spans_in(first_paragraph(&blocks));
        assert!(spans.iter().any(|s| s.text == "\n"), "el <br> debe producir un salto de linea");
    }

    #[test]
    fn an_unlisted_tag_shows_its_text_but_not_its_markup() {
        let blocks = parse("uno <span class=\"x\">dos</span> tres\n", None);
        let spans = spans_in(first_paragraph(&blocks));
        let text: String = spans.iter().map(|s| s.text.as_str()).collect();
        assert_eq!(text, "uno dos tres");
    }

    #[test]
    fn html_block_p_becomes_a_paragraph_with_inline_formatting() {
        let blocks = parse("<p>Hola <b>mundo</b></p>\n", None);
        assert_eq!(blocks.len(), 1);
        let spans = spans_in(first_paragraph(&blocks));
        assert_eq!(spans.iter().map(|s| s.text.as_str()).collect::<Vec<_>>(), vec!["Hola ", "mundo"]);
        assert!(spans[1].style.bold);
    }

    #[test]
    fn html_block_hr_becomes_a_thematic_break() {
        let blocks = parse("texto\n\n<hr>\n\notro texto\n", None);
        assert!(blocks.iter().any(|b| matches!(b, Block::ThematicBreak)));
    }

    #[test]
    fn html_block_ul_li_becomes_an_unordered_list_of_two_items() {
        let blocks = parse("<ul>\n<li>uno</li>\n<li>dos</li>\n</ul>\n", None);
        let list = blocks
            .iter()
            .find_map(|b| match b {
                Block::List { ordered, items, .. } => Some((ordered, items)),
                _ => None,
            })
            .expect("falta la lista");
        assert!(!*list.0);
        assert_eq!(list.1.len(), 2);
        let first_item_text: String = list.1[0]
            .children
            .iter()
            .flat_map(|b| match b {
                Block::Paragraph(inlines) => spans_in(inlines).into_iter().map(|s| s.text.clone()).collect(),
                _ => vec![],
            })
            .collect();
        assert_eq!(first_item_text, "uno");
    }

    #[test]
    fn html_block_div_center_becomes_a_centered_block() {
        let blocks = parse("<div align=\"center\">centrado</div>\n", None);
        let inlines = blocks
            .iter()
            .find_map(|b| match b {
                Block::Centered(inlines) => Some(inlines),
                _ => None,
            })
            .expect("falta el bloque centrado");
        assert_eq!(spans_in(inlines)[0].text, "centrado");
    }

    #[test]
    fn several_html_block_level_tags_in_a_row_all_produce_their_own_block() {
        // pulldown-cmark groups consecutive HTML lines (no blank line
        // between them) into a single Tag::HtmlBlock — confirmed before
        // writing parse_html_block_tokens, not assumed.
        let source = "<p>uno</p>\n<hr>\n<p>dos</p>\n";
        let blocks = parse(source, None);
        let shapes: Vec<&str> = blocks
            .iter()
            .map(|b| match b {
                Block::Paragraph(_) => "p",
                Block::ThematicBreak => "hr",
                _ => "?",
            })
            .collect();
        assert_eq!(shapes, vec!["p", "hr", "p"]);
    }
}
