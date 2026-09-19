//! Translates the element tree into on-screen components, and the theme.
//! Owns no knowledge of the filesystem or of Markdown syntax.

use std::{
    ops::Range,
    path::{Path, PathBuf},
};

use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    notification::Notification,
    scroll::{ScrollableElement, ScrollbarAxis},
    tab::{Tab, TabBar},
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
    // `pub(crate)`, not private: RF-14.1's `app::activate_document_link`
    // needs to reach it from inside an `Entity<DocumentView>::update`
    // closure (AD-26's pattern), the same way `render_tab_bar`'s own
    // closures already do from within this module.
    pub(crate) state: AppState,
    timing_scheduled: bool,
}

/// What a clicked link needs beyond its own URL (RF-14.1, RF-15.1):
/// the active document's directory, to resolve a relative `.md` destination
/// against, and a handle to mutate `AppState` when one should open —
/// `app::activate_document_link` does the deciding, this just carries what
/// it needs into every place a link can appear. Threaded alongside `cx`
/// through the same functions that already take it, rather than adding a
/// second, unrelated parameter to each — `render_text` is the only one that
/// reads it, but it's reachable from a heading, a table cell, or a `<div
/// align="center">`, not only a paragraph.
#[derive(Clone)]
struct LinkCtx {
    doc_dir: Option<PathBuf>,
    view: Entity<DocumentView>,
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
        } else if let Some(tab) = self.state.tabs.get(self.state.active_tab) {
            if tab.raw_view {
                column = column.child(render_raw(&tab.raw, self.state.active_tab, window, cx));
            } else {
                let link_ctx = LinkCtx { doc_dir: tab.path.parent().map(Path::to_path_buf), view: cx.entity() };
                let mut code_index = 0;
                let mut text_index = 0;
                for block in &tab.blocks {
                    column = column.child(render_block(block, &mut code_index, &mut text_index, &link_ctx, window, cx));
                }
            }
        }

        let mut root = div().relative().size_full().v_flex();
        if let Some(tab) = self.state.tabs.get(self.state.active_tab) {
            root = root.child(
                div()
                    .flex()
                    .items_center()
                    .w_full()
                    .child(div().flex_1().child(render_tab_bar(&self.state.tabs, self.state.active_tab, cx.entity())))
                    .child(render_view_toggle(tab.raw_view, cx.entity())),
            );
        }
        // `Root` does not render notifications/dialogs/sheets on its own;
        // the top-level view is expected to composite them in. Without this,
        // `window.push_notification(...)` above updates state but nothing
        // ever draws it.
        root.child(
            div().flex_1().overflow_y_scrollbar().child(
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

/// Renders the tab bar (RF-05.1, RF-06.1): one `Tab` per open document,
/// labeled with its file name — never a full path or the document's first
/// heading (RF-06.1, `01-alcance.md` supuesto 5). Clicking one activates it
/// (CA-02.1): `TabBar::on_click` needs to mutate `AppState`, which a `render`
/// function only reaches through the view's own `Entity` (AD-26) — a plain
/// `&App` in scope here isn't enough, the way it was for `app::activate_link`.
fn render_tab_bar(tabs: &[crate::app::DocumentTab], active_tab: usize, view: Entity<DocumentView>) -> impl IntoElement {
    let close_view = view.clone();
    TabBar::new("document-tabs")
        .selected_index(active_tab)
        .children(tabs.iter().enumerate().map(|(ix, tab)| {
            let label = tab.path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            let close_view = close_view.clone();
            Tab::new().label(label).suffix(render_close_tab_button(ix, close_view))
        }))
        .on_click(move |ix, _window, cx| {
            let ix = *ix;
            view.update(cx, |view, cx| {
                if ix < view.state.tabs.len() && ix != view.state.active_tab {
                    view.state.active_tab = ix;
                    cx.notify();
                }
            });
        })
}

/// Closes tab `ix` (RF-05.1, RF-07.1). Its own click handler, not
/// `TabBar::on_click`'s: that one only ever reports which tab was clicked,
/// with no way to tell the close button apart from the rest of the tab.
/// Reuses the switching pattern from AD-26 — `Entity<DocumentView>::update`
/// — for the same reason: closing a tab mutates `AppState`.
fn render_close_tab_button(ix: usize, view: Entity<DocumentView>) -> impl IntoElement {
    // A plain "×" glyph, not `IconName::Close`: gpui-component's bundled
    // icon SVGs resolve through an asset source MDView never registers (it
    // has no reason to embed the whole icon set for one button), so the
    // icon would render blank. Confirmed by observation, not assumed —
    // ver CA-03.1 en 04-calidad.md.
    Button::new(("close-tab", ix)).label("×").ghost().xsmall().on_click(move |_event, _window, cx| {
        view.update(cx, |view, cx| {
            if crate::app::close_tab(&mut view.state, ix) {
                cx.quit();
            } else {
                cx.notify();
            }
        });
    })
}

/// The RF-23.1 toggle between rendered and raw view, next to the tab bar
/// (AD-30) rather than inside each `Tab`: it acts on the active tab, not on
/// how a tab looks in the strip.
fn render_view_toggle(raw_view: bool, view: Entity<DocumentView>) -> impl IntoElement {
    let label = if raw_view { "Ver renderizado" } else { "Ver crudo" };
    Button::new("toggle-raw-view").label(label).ghost().xsmall().on_click(move |_event, _window, cx| {
        view.update(cx, |view, cx| {
            if let Some(tab) = view.state.tabs.get_mut(view.state.active_tab) {
                tab.raw_view = !tab.raw_view;
            }
            cx.notify();
        });
    })
}

/// Renders a document's exact text (RF-23.1, RF-23.2): the same
/// monospaced/horizontal-scroll treatment as `Block::CodeBlock` (AD-10),
/// applied to the whole file instead of one block, since RF-23.2 reuses
/// RF-10's rule verbatim — a wide line stays whole and scrolls, not wrapped.
/// Keyed by `tab_ix` so each tab keeps its own scroll position, the same
/// reasoning `render_block`'s `code_index` already uses.
fn render_raw(raw: &str, tab_ix: usize, window: &mut Window, cx: &mut App) -> AnyElement {
    let handle = window
        .use_keyed_state(("raw-view-scroll", tab_ix), cx, |_, _| ScrollHandle::default())
        .read(cx)
        .clone();

    div()
        .id(("raw-view", tab_ix))
        .w_full()
        .relative()
        .bg(cx.theme().muted)
        .child(
            div()
                .id(("raw-view-scroll-area", tab_ix))
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
                        .child(raw.to_string()),
                ),
        )
        .scrollbar(&handle, ScrollbarAxis::Horizontal)
        .into_any_element()
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
    link_ctx: &LinkCtx,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    match block {
        Block::FrontMatter(properties) => render_front_matter(properties, text_index, link_ctx, cx).into_any_element(),
        Block::Heading(level, spans) => {
            let id = *text_index;
            *text_index += 1;
            div()
                .w_full()
                .whitespace_normal()
                .text_size(px(heading_size(*level)))
                .child(render_text(spans, true, ("text-block", id), link_ctx, cx))
                .into_any_element()
        }
        Block::Paragraph(inlines) => render_paragraph(inlines, text_index, link_ctx, cx),
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
                quote = quote.child(render_block(b, code_index, text_index, link_ctx, window, cx));
            }
            quote.into_any_element()
        }
        Block::List { ordered, start, items } => {
            render_list(*ordered, *start, items, code_index, text_index, link_ctx, window, cx).into_any_element()
        }
        Block::Table { header, rows } => render_table(header, rows, text_index, link_ctx, cx).into_any_element(),
        Block::Centered(inlines) => render_centered(inlines, text_index, link_ctx, cx),
        Block::Details { summary, children } => {
            let mut container = div().v_flex().gap_2().w_full();
            if !summary.is_empty() {
                let id = *text_index;
                *text_index += 1;
                container = container.child(
                    div()
                        .w_full()
                        .whitespace_normal()
                        .text_size(px(BODY_SIZE))
                        .child(render_text(summary, true, ("text-block", id), link_ctx, cx)),
                );
            }
            for child in children {
                container = container.child(render_block(child, code_index, text_index, link_ctx, window, cx));
            }
            container.into_any_element()
        }
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
    link_ctx: &LinkCtx,
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
            content = content.child(render_block(block, code_index, text_index, link_ctx, window, cx));
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
fn render_front_matter(properties: &[(String, String)], text_index: &mut usize, link_ctx: &LinkCtx, cx: &App) -> AnyElement {
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

    render_table(&[], &rows, text_index, link_ctx, cx).into_any_element()
}

fn render_table(
    header: &[Vec<Span>],
    rows: &[Vec<Vec<Span>>],
    text_index: &mut usize,
    link_ctx: &LinkCtx,
    cx: &App,
) -> impl IntoElement {
    let mut table = div().v_flex().w_full();

    if !header.is_empty() {
        table = table.child(render_table_row(header, true, text_index, link_ctx, cx));
    }
    for row in rows {
        table = table.child(render_table_row(row, false, text_index, link_ctx, cx));
    }

    table
}

fn render_table_row(
    cells: &[Vec<Span>],
    is_header: bool,
    text_index: &mut usize,
    link_ctx: &LinkCtx,
    cx: &App,
) -> impl IntoElement {
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
                .child(render_text(cell, is_header, ("text-block", id), link_ctx, cx)),
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
fn render_paragraph(inlines: &[Inline], text_index: &mut usize, link_ctx: &LinkCtx, cx: &App) -> AnyElement {
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
                    children.push(render_text_block(&run, text_index, link_ctx, cx));
                    run.clear();
                }
                children.push(render_image(image));
            }
        }
    }
    if !run.is_empty() {
        children.push(render_text_block(&run, text_index, link_ctx, cx));
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
fn render_text_block(spans: &[Span], text_index: &mut usize, link_ctx: &LinkCtx, cx: &App) -> AnyElement {
    let id = *text_index;
    *text_index += 1;
    div()
        .w_full()
        .whitespace_normal()
        .text_size(px(BODY_SIZE))
        .child(render_text(spans, false, ("text-block", id), link_ctx, cx))
        .into_any_element()
}

/// Renders a `<div align="center">` (RF-13.1, HU-03): the same span/image
/// splitting as `render_paragraph`, but each text run gets `.text_center()`
/// instead of `.w_full()`, and an image sits in a row centered on the cross
/// axis — `render_paragraph`'s text blocks are already `w_full()`, which
/// leaves nothing for centering to do.
fn render_centered(inlines: &[Inline], text_index: &mut usize, link_ctx: &LinkCtx, cx: &App) -> AnyElement {
    let mut column = div().v_flex().gap_2().w_full().items_center();
    let mut run: Vec<Span> = Vec::new();

    let flush = |run: &mut Vec<Span>, text_index: &mut usize, cx: &App| -> Option<AnyElement> {
        if run.is_empty() {
            return None;
        }
        let id = *text_index;
        *text_index += 1;
        let element = div()
            .w_full()
            .text_center()
            .whitespace_normal()
            .text_size(px(BODY_SIZE))
            .child(render_text(run, false, ("text-block", id), link_ctx, cx))
            .into_any_element();
        run.clear();
        Some(element)
    };

    for inline in inlines {
        match inline {
            Inline::Span(span) => run.push(span.clone()),
            Inline::Image(image) => {
                if let Some(element) = flush(&mut run, text_index, cx) {
                    column = column.child(element);
                }
                column = column.child(render_image(image));
            }
        }
    }
    if let Some(element) = flush(&mut run, text_index, cx) {
        column = column.child(element);
    }

    column.into_any_element()
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
fn render_text(spans: &[Span], bold: bool, id: impl Into<ElementId>, link_ctx: &LinkCtx, cx: &App) -> AnyElement {
    let (styled, links) = render_spans(spans, bold, cx);
    if links.is_empty() {
        return styled.into_any_element();
    }

    let ranges: Vec<Range<usize>> = links.iter().map(|(range, _)| range.clone()).collect();
    let urls: Vec<String> = links.into_iter().map(|(_, url)| url).collect();
    let link_ctx = link_ctx.clone();

    InteractiveText::new(id, styled)
        .on_click(ranges, move |ix, _window, cx| {
            crate::app::activate_document_link(&urls[ix], link_ctx.doc_dir.as_deref(), &link_ctx.view, cx);
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
