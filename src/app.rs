//! Startup, arguments, application state, window.
//! Coordinates `document`, `markdown`, and `render`; does not replace them.

use std::env;

use gpui::{Application, AppContext, WindowOptions};
use gpui_component::{Root, Theme};

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
    /// Set when the requested file could not be shown (RF-17). `render`
    /// pushes it as a notification once, then clears it.
    pub pending_notice: Option<String>,
}

pub fn run() {
    let path = env::args().nth(1);

    let app = Application::new();
    app.run(move |cx| {
        gpui_component::init(cx);

        let mut blocks = Vec::new();
        let mut pending_notice = None;
        if let Some(path) = path.as_deref() {
            match document::load(path) {
                Ok(text) => blocks = markdown::parse(&text),
                Err(error) => pending_notice = Some(error_message(path, error)),
            }
        }

        let state = AppState {
            mode: Mode::ReadOnly,
            blocks,
            pending_notice,
        };

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                // gpui_component::init(cx) already synced once, before this
                // window existed. Re-sync now against the real window (per
                // AD-11), and keep syncing if the user changes the Windows
                // theme while MDView is open.
                Theme::sync_system_appearance(Some(window), cx);
                window
                    .observe_window_appearance(|window, cx| {
                        Theme::sync_system_appearance(Some(window), cx);
                    })
                    .detach();

                let view = cx.new(|_| DocumentView::new(state));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("failed to open window");
        })
        .detach();
    });
}

/// Turns a load failure into the message CA-05.1–CA-05.3 expect: it names
/// the file and states the cause.
fn error_message(path: &str, error: document::LoadError) -> String {
    match error {
        document::LoadError::NotFound => format!("No se encontró «{path}»."),
        document::LoadError::PermissionDenied => {
            format!("No se pudo leer «{path}»: no hay permiso para leerlo.")
        }
        document::LoadError::NotText => format!("«{path}» no es un archivo de texto."),
        document::LoadError::Unreadable => format!("No se pudo leer «{path}»."),
    }
}
