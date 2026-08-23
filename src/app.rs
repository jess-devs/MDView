//! Startup, arguments, application state, window.
//! Coordinates `document`, `markdown`, and `render`; does not replace them.

use std::env;

use gpui::{Application, AppContext, WindowOptions};
use gpui_component::Root;

use crate::{document, markdown, render::DocumentView};

/// The mode the application is in. Only one value exists today: see AD-05.
pub enum Mode {
    ReadOnly,
}

pub struct AppState {
    // Unread until a second `Mode` value exists; see AD-05.
    #[allow(dead_code)]
    pub mode: Mode,
    pub blocks: Vec<markdown::Block>,
}

pub fn run() {
    let path = env::args().nth(1);

    let app = Application::new();
    app.run(move |cx| {
        gpui_component::init(cx);

        let blocks = path
            .as_deref()
            .and_then(|path| document::load(path).ok())
            .map(|text| markdown::parse(&text))
            .unwrap_or_default();

        let state = AppState {
            mode: Mode::ReadOnly,
            blocks,
        };

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| DocumentView::new(state));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("failed to open window");
        })
        .detach();
    });
}
