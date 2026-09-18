//! Translates the element tree into on-screen components, and the theme.
//! Owns no knowledge of the filesystem or of Markdown syntax.

use std::ops::Range;

use gpui::*;
use gpui_component::{
    notification::Notification,
    scroll::{ScrollableElement, ScrollbarAxis},
    ActiveTheme, *,
};

use crate::{
    app::AppState,
    markdown::{Block, ImageRef, Inline, ListItem, Span, SpanStyle},
};

/// Maximum reading width, per AD-08.
const READING_WIDTH: f32 = 720.0;

/// Horizontal padding (both sides combined) around the reading column, from `p_8`.
const HORIZONTAL_PADDING: f32 = 64.0;

/// Base paragraph text size.
const BODY_SIZE: f32 = 16.0;

pub struct DocumentView {
    state: AppState,
    timing_scheduled: bool,
}

impl DocumentView {
    pub fn new(state: AppState) -> Self {
        Self { state, timing_scheduled: false }
    }
}

impl Render for DocumentView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(message) = self.state.pending_notice.take() {
            window.push_notification(Notification::error(message), cx);
        }

        if !self.timing_scheduled {
            self.timing_scheduled = true;
            if let Some(path) = self.state.timing_path.clone() {
                let start = self.state.start;
                // Fires after THIS frame (which already includes the
                // document below) is actually rendered, not when the view
                // was merely created.
                window.on_next_frame(move |_window, _cx| {
                    crate::app::record_timing(&path, start);
                });
            }
        }

        let available = f32::from(window.viewport_size().width) - HORIZONTAL_PADDING;
        let column_width = available.min(READING_WIDTH).max(0.0);

        let mut column = div().v_flex().gap_4().w(px(column_width));

        if self.state.no_path_given {
            column = column.child(render_empty_state(cx));
        } else {
            let mut code_index = 0;
            let mut text_index = 0;
            for block in &self.state.blocks {
                column = column.child(render_block(block, &mut code_index, &mut text_index, window, cx));
            }
        }

        // `Root` does not render notifications/dialogs/sheets on its own;
        // the top-level view is expected to composite them in. Without this,
        // `window.push_notification(...)` above updates state but nothing
        // ever draws it.
        div()
            .relative()
            .size_full()
            .child(
                div().size_full().overflow_y_scrollbar().child(
                    div()
                        .flex()
                        .w_full()
                        .justify_center()
                        .p_8()
                        .child(column),
                ),
            )
            .children(Root::render_notification_layer(window, cx))
    }
}

/// The window shown when MDView is launched without a file (RF-19).
fn render_empty_state(cx: &App) -> impl IntoElement {
    div()
        .v_flex()
        .gap_4()
        .w_full()
        .child(
            div()
                .whitespace_normal()
                .text_size(px(heading_size(1)))
                .child(render_spans(
                    &[Span { text: "MDView".to_string(), style: SpanStyle::default(), url: None }],
                    true,
                    cx,
                ).0),
        )
        .child(
            div().w_full().whitespace_normal().text_size(px(BODY_SIZE)).child(render_spans(
                &[Span {
                    text: "MDView muestra archivos Markdown (.md) con formato, sin necesidad \
                           de abrir un editor de código."
                        .to_string(),
                    style: SpanStyle::default(),
                    url: None,
                }],
                false,
                cx,
            ).0),
        )
        .child(
            div().w_full().whitespace_normal().text_size(px(BODY_SIZE)).child(render_spans(
                &[
                    Span { text: "Para abrir uno, indícale su ruta al iniciarlo: ".to_string(), style: SpanStyle::default(), url: None },
                    Span { text: "mdview ruta\\al\\archivo.md".to_string(), style: SpanStyle { code: true, ..Default::default() }, url: None },
                ],
                false,
                cx,
            ).0),
        )
}

fn render_block(
    block: &Block,
    code_index: &mut usize,
    text_index: &mut usize,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    match block {
        Block::FrontMatter(properties) => render_front_matter(properties, text_index, cx).into_any_element(),
        Block::Heading(level, spans) => {
            let id = *text_index;
            *text_index += 1;
            div()
                .w_full()
                .whitespace_normal()
                .text_size(px(heading_size(*level)))
                .child(render_text(spans, true, ("text-block", id), cx))
                .into_any_element()
        }
        Block::Paragraph(inlines) => render_paragraph(inlines, text_index, cx),
        Block::ThematicBreak => div().w_full().h(px(1.0)).bg(cx.theme().border).into_any_element(),
        Block::Quote(blocks) => {
            let mut quote = div()
                .v_flex()
                .gap_4()
                .w_full()
                .pl_4()
                .border_l_2()
                .border_color(cx.theme().border);
            for b in blocks {
                quote = quote.child(render_block(b, code_index, text_index, window, cx));
            }
            quote.into_any_element()
        }
        Block::List { ordered, start, items } => {
            render_list(*ordered, *start, items, code_index, text_index, window, cx).into_any_element()
        }
        Block::Table { header, rows } => render_table(header, rows, text_index, cx).into_any_element(),
        Block::CodeBlock(text) => {
            // Each code block needs its own horizontal ScrollHandle: the
            // convenience `overflow_x_scrollbar()` derives its state key from
            // this call site alone, so with several code blocks in one
            // document they would all share one scroll position. Keying our
            // own handle by index avoids that. See AD-10.
            let id = *code_index;
            *code_index += 1;

            let handle = window
                .use_keyed_state(("code-block-scroll", id), cx, |_, _| ScrollHandle::default())
                .read(cx)
                .clone();

            // Structure mirrors gpui-component's own `Scrollable::render`: a
            // `.relative()` outer wrapper, an inner scrolling area holding
            // the content, and the scrollbar as a sibling overlay (it is
            // absolutely positioned against the outer wrapper) — not a child
            // of the scrolling div itself, or it scrolls away with the text.
            div()
                .id(("code-block", id))
                .w_full()
                .relative()
                .bg(cx.theme().muted)
                .child(
                    div()
                        .id(("code-block-scroll-area", id))
                        .flex()
                        .flex_row()
                        .w_full()
                        .overflow_x_scroll()
                        .track_scroll(&handle)
                        .p_2()
                        .child(
                            div()
                                .flex_1()
                                .whitespace_nowrap()
                                .font_family(cx.theme().mono_font_family.clone())
                                .text_size(px(BODY_SIZE))
                                .child(text.clone()),
                        ),
                )
                .scrollbar(&handle, ScrollbarAxis::Horizontal)
                .into_any_element()
        }
    }
}

fn render_list(
    ordered: bool,
    start: u64,
    items: &[ListItem],
    code_index: &mut usize,
    text_index: &mut usize,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let mut list = div().v_flex().gap_2().w_full().pl_5();

    for (i, item) in items.iter().enumerate() {
        let marker = match item.checked {
            Some(true) => "\u{2611}".to_string(),
            Some(false) => "\u{2610}".to_string(),
            None if ordered => format!("{}.", start + i as u64),
            None => "\u{2022}".to_string(),
        };

        let mut content = div().v_flex().gap_2().w_full();
        for block in &item.children {
            content = content.child(render_block(block, code_index, text_index, window, cx));
        }

        list = list.child(
            div()
                .flex()
                .items_start()
                .gap_2()
                .w_full()
                .child(div().flex_shrink_0().child(marker))
                .child(content),
        );
    }

    list
}

/// Renders the document's front matter as a two-column properties table
/// (RF-12.1), reusing the same row styling as a Markdown table: front matter
/// is metadata, not something a reader edits, so it doesn't need its own look.
fn render_front_matter(properties: &[(String, String)], text_index: &mut usize, cx: &App) -> AnyElement {
    if properties.is_empty() {
        // A `---`/`---` block with nothing recognizable as `key: value` in
        // between: nothing to show, and nothing worth an empty table for.
        return div().into_any_element();
    }

    let rows: Vec<Vec<Vec<Span>>> = properties
        .iter()
        .map(|(name, value)| {
            vec![
                vec![Span { text: name.clone(), style: SpanStyle { bold: true, ..Default::default() }, url: None }],
                vec![Span { text: value.clone(), style: SpanStyle::default(), url: None }],
            ]
        })
        .collect();

    render_table(&[], &rows, text_index, cx).into_any_element()
}

fn render_table(header: &[Vec<Span>], rows: &[Vec<Vec<Span>>], text_index: &mut usize, cx: &App) -> impl IntoElement {
    let mut table = div().v_flex().w_full();

    if !header.is_empty() {
        table = table.child(render_table_row(header, true, text_index, cx));
    }
    for row in rows {
        table = table.child(render_table_row(row, false, text_index, cx));
    }

    table
}

fn render_table_row(cells: &[Vec<Span>], is_header: bool, text_index: &mut usize, cx: &App) -> impl IntoElement {
    let mut row = div().flex().w_full().gap_4().p_2().text_size(px(BODY_SIZE));
    if is_header {
        row = row.bg(cx.theme().muted).border_b_2().border_color(cx.theme().border);
    } else {
        row = row.border_b_1().border_color(cx.theme().border);
    }

    for cell in cells {
        let id = *text_index;
        *text_index += 1;
        row = row.child(
            div()
                .flex_1()
                .whitespace_normal()
                .child(render_text(cell, is_header, ("text-block", id), cx)),
        );
    }

    row
}

/// Renders a paragraph's content in document order (CA-03.1): consecutive
/// text runs are grouped into one `render_text` block each, same as before
/// images existed, and each image is its own element in between (AD-20).
/// GPUI's `StyledText` can't mix an actual image into a run of text, so a
/// paragraph that mixes both renders as stacked blocks rather than flowing
/// inline — the common case this feature's criteria test, an image alone on
/// its own line, still renders as just that one image, nothing stacked
/// around it.
fn render_paragraph(inlines: &[Inline], text_index: &mut usize, cx: &App) -> AnyElement {
    if let [Inline::Image(image)] = inlines {
        return render_image(image);
    }

    let mut children: Vec<AnyElement> = Vec::new();
    let mut run: Vec<Span> = Vec::new();

    for inline in inlines {
        match inline {
            Inline::Span(span) => run.push(span.clone()),
            Inline::Image(image) => {
                if !run.is_empty() {
                    children.push(render_text_block(&run, text_index, cx));
                    run.clear();
                }
                children.push(render_image(image));
            }
        }
    }
    if !run.is_empty() {
        children.push(render_text_block(&run, text_index, cx));
    }

    let mut column = div().v_flex().gap_2().w_full();
    for child in children {
        column = column.child(child);
    }
    column.into_any_element()
}

/// A run of plain text, styled and sized exactly like a `Block::Paragraph`
/// always has been — factored out of `render_block`'s old paragraph arm so
/// `render_paragraph` can call it once per text run instead of once per
/// paragraph.
fn render_text_block(spans: &[Span], text_index: &mut usize, cx: &App) -> AnyElement {
    let id = *text_index;
    *text_index += 1;
    div()
        .w_full()
        .whitespace_normal()
        .text_size(px(BODY_SIZE))
        .child(render_text(spans, false, ("text-block", id), cx))
        .into_any_element()
}

/// Renders an image (RF-11.1), or its alt text if it can't be shown:
/// `resolved` is `None` (a remote URL, or no document directory to resolve
/// against — RNF-02.1), or loading it fails for any reason (CA-03.3,
/// CA-03.4) — a missing file, a permission error, a file that isn't a
/// decodable image. `img()`'s own loader classifies all of those as one
/// `ImageCacheError` and this only needs the fallback, not which one it was.
///
/// `with_fallback`'s closure can't borrow `cx` (it isn't `'static`), so the
/// fallback is a plain, unstyled text child rather than going through
/// `render_spans` — CA-03.3 only requires the alt text to be shown, not
/// styled like body text.
fn render_image(image: &ImageRef) -> AnyElement {
    let alt = image.alt.clone();
    match &image.resolved {
        Some(path) => img(path.clone())
            .max_w(px(READING_WIDTH))
            .with_fallback(move || div().child(alt.clone()).into_any_element())
            .into_any_element(),
        None => div().whitespace_normal().text_size(px(BODY_SIZE)).child(alt).into_any_element(),
    }
}

/// Builds the styled text for a run of spans, plus the byte range and URL of
/// each span that is part of a link (RF-08.2), for callers that want to make
/// those ranges clickable (RF-15.1).
fn render_spans(spans: &[Span], bold: bool, cx: &App) -> (StyledText, Vec<(Range<usize>, String)>) {
    let theme = cx.theme();
    let base = TextStyle {
        color: theme.foreground,
        font_family: theme.font_family.clone(),
        font_weight: if bold { FontWeight::BOLD } else { FontWeight::NORMAL },
        white_space: WhiteSpace::Normal,
        ..Default::default()
    };

    let mut combined = String::new();
    let mut runs = Vec::new();
    let mut links = Vec::new();

    for span in spans {
        let len = span.text.len();
        let start = combined.len();
        combined.push_str(&span.text);
        if let Some(url) = &span.url {
            links.push((start..start + len, url.clone()));
        }

        let mut style = base.clone();
        if span.style.code {
            style.font_family = theme.mono_font_family.clone();
            style.background_color = Some(theme.muted);
        }

        let mut highlight = HighlightStyle::default();
        if span.style.bold {
            highlight.font_weight = Some(FontWeight::BOLD);
        }
        if span.style.italic {
            highlight.font_style = Some(FontStyle::Italic);
        }
        if span.style.strikethrough {
            highlight.strikethrough = Some(StrikethroughStyle {
                thickness: px(1.0),
                color: None,
            });
        }
        if span.url.is_some() {
            highlight.color = Some(theme.link);
            highlight.underline = Some(UnderlineStyle {
                thickness: px(1.0),
                color: None,
                wavy: false,
            });
        }

        runs.push(style.highlight(highlight).to_run(len));
    }

    (StyledText::new(combined).with_runs(runs), links)
}

/// Renders a run of spans as clickable text when any of them carries a link
/// (RF-15.1): activating one of those ranges hands the URL to `app`, which
/// decides what "activate a link" means — `render` only knows how to detect
/// a click and where the link's text sits in the combined string.
fn render_text(spans: &[Span], bold: bool, id: impl Into<ElementId>, cx: &App) -> AnyElement {
    let (styled, links) = render_spans(spans, bold, cx);
    if links.is_empty() {
        return styled.into_any_element();
    }

    let ranges: Vec<Range<usize>> = links.iter().map(|(range, _)| range.clone()).collect();
    let urls: Vec<String> = links.into_iter().map(|(_, url)| url).collect();

    InteractiveText::new(id, styled)
        .on_click(ranges, move |ix, _window, _cx| {
            crate::app::activate_link(&urls[ix]);
        })
        .into_any_element()
}

fn heading_size(level: u8) -> f32 {
    match level {
        1 => 32.0,
        2 => 28.0,
        3 => 24.0,
        4 => 20.0,
        5 => 18.0,
        _ => 16.0,
    }
}
