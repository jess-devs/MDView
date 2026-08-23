//! Translates the element tree into on-screen components, and the theme.
//! Owns no knowledge of the filesystem or of Markdown syntax.

use gpui::*;
use gpui_component::{scroll::ScrollableElement, *};

use crate::{app::AppState, markdown::Block};

/// Maximum reading width, per AD-08.
const READING_WIDTH: f32 = 720.0;

/// Horizontal padding (both sides combined) around the reading column, from `p_8`.
const HORIZONTAL_PADDING: f32 = 64.0;

pub struct DocumentView {
    state: AppState,
}

impl DocumentView {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Render for DocumentView {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let available = f32::from(window.viewport_size().width) - HORIZONTAL_PADDING;
        let column_width = available.min(READING_WIDTH).max(0.0);

        let mut column = div().v_flex().gap_4().w(px(column_width));

        for block in &self.state.blocks {
            column = column.child(render_block(block));
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

fn render_block(block: &Block) -> impl IntoElement {
    match block {
        Block::Heading(level, text) => div()
            .w_full()
            .whitespace_normal()
            .font_bold()
            .text_size(px(heading_size(*level)))
            .child(text.clone()),
        Block::Paragraph(text) => div().w_full().whitespace_normal().child(text.clone()),
    }
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
