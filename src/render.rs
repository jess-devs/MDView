//! Translates the element tree into on-screen components, and the theme.
//! Owns no knowledge of the filesystem or of Markdown syntax.

use gpui::*;
use gpui_component::{
    scroll::{ScrollableElement, ScrollbarAxis},
    ActiveTheme, *,
};

use crate::{
    app::AppState,
    markdown::{Block, ListItem, Span},
};

/// Maximum reading width, per AD-08.
const READING_WIDTH: f32 = 720.0;

/// Horizontal padding (both sides combined) around the reading column, from `p_8`.
const HORIZONTAL_PADDING: f32 = 64.0;

/// Base paragraph text size.
const BODY_SIZE: f32 = 16.0;

pub struct DocumentView {
    state: AppState,
}

impl DocumentView {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Render for DocumentView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let available = f32::from(window.viewport_size().width) - HORIZONTAL_PADDING;
        let column_width = available.min(READING_WIDTH).max(0.0);

        let mut column = div().v_flex().gap_4().w(px(column_width));

        let mut code_index = 0;
        for block in &self.state.blocks {
            column = column.child(render_block(block, &mut code_index, window, cx));
        }

        div().size_full().overflow_y_scrollbar().child(
            div()
                .flex()
                .w_full()
                .justify_center()
                .p_8()
                .child(column),
        )
    }
}

fn render_block(block: &Block, code_index: &mut usize, window: &mut Window, cx: &mut App) -> AnyElement {
    match block {
        Block::Heading(level, spans) => div()
            .w_full()
            .whitespace_normal()
            .text_size(px(heading_size(*level)))
            .child(render_spans(spans, true, cx))
            .into_any_element(),
        Block::Paragraph(spans) => div()
            .w_full()
            .whitespace_normal()
            .text_size(px(BODY_SIZE))
            .child(render_spans(spans, false, cx))
            .into_any_element(),
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
                quote = quote.child(render_block(b, code_index, window, cx));
            }
            quote.into_any_element()
        }
        Block::List { ordered, start, items } => {
            render_list(*ordered, *start, items, code_index, window, cx).into_any_element()
        }
        Block::Table { header, rows } => render_table(header, rows, cx).into_any_element(),
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
            content = content.child(render_block(block, code_index, window, cx));
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

fn render_table(header: &[Vec<Span>], rows: &[Vec<Vec<Span>>], cx: &App) -> impl IntoElement {
    let mut table = div().v_flex().w_full();

    if !header.is_empty() {
        table = table.child(render_table_row(header, true, cx));
    }
    for row in rows {
        table = table.child(render_table_row(row, false, cx));
    }

    table
}

fn render_table_row(cells: &[Vec<Span>], is_header: bool, cx: &App) -> impl IntoElement {
    let mut row = div().flex().w_full().gap_4().p_2().text_size(px(BODY_SIZE));
    if is_header {
        row = row.bg(cx.theme().muted).border_b_2().border_color(cx.theme().border);
    } else {
        row = row.border_b_1().border_color(cx.theme().border);
    }

    for cell in cells {
        row = row.child(
            div()
                .flex_1()
                .whitespace_normal()
                .child(render_spans(cell, is_header, cx)),
        );
    }

    row
}

fn render_spans(spans: &[Span], bold: bool, cx: &App) -> StyledText {
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

    for span in spans {
        let len = span.text.len();
        combined.push_str(&span.text);

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

        runs.push(style.highlight(highlight).to_run(len));
    }

    StyledText::new(combined).with_runs(runs)
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
