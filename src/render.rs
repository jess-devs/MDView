//! Translates the element tree into on-screen components, and the theme.
//! Owns no knowledge of the filesystem or of Markdown syntax.

use gpui::*;
use gpui_component::{scroll::ScrollableElement, *};

use crate::{app::AppState, markdown::Block};

/// Maximum reading width, per AD-08.
const READING_WIDTH: f32 = 720.0;

pub struct DocumentView {
    state: AppState,
}

impl DocumentView {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

impl Render for DocumentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut column = div().v_flex().gap_4().w_full().max_w(px(READING_WIDTH));

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
            .font_bold()
            .text_size(px(heading_size(*level)))
            .child(text.clone()),
        Block::Paragraph(text) => div().child(text.clone()),
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
