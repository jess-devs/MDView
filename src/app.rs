//! Startup, arguments, application state, window.
//! Coordinates `document`, `markdown`, and `render`; does not replace them.

use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

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
    /// No path was given at all (RF-19): show the onboarding text instead of
    /// a silently blank window. Distinct from a load error, which also
    /// leaves `blocks` empty but must not show onboarding text over it.
    pub no_path_given: bool,
    /// Process entry time, for the `MDVIEW_TIMING` startup measurement (AD-07).
    pub start: SystemTime,
    /// Where to append the timing line, if `MDVIEW_TIMING` names a path.
    pub timing_path: Option<PathBuf>,
}

pub fn run(start: SystemTime) {
    let path = env::args().nth(1);
    let timing_path = env::var_os("MDVIEW_TIMING").map(PathBuf::from);

    let app = Application::new();
    app.run(move |cx| {
        gpui_component::init(cx);

        let mut blocks = Vec::new();
        let mut pending_notice = None;
        match path.as_deref() {
            Some(path) => match document::load(path) {
                Ok(text) => blocks = markdown::parse(&text),
                Err(error) => pending_notice = Some(error_message(path, error)),
            },
            None => {}
        }

        let state = AppState {
            mode: Mode::ReadOnly,
            blocks,
            pending_notice,
            no_path_given: path.is_none(),
            start,
            timing_path,
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

/// Appends one line to `MDVIEW_TIMING`'s file: entry time, first-rendered-frame
/// time, and their difference, all in epoch milliseconds (AD-07). Called once,
/// from `window.on_next_frame` after the first frame containing the document
/// has actually been rendered — not when the window or the view was created.
pub fn record_timing(path: &Path, start: SystemTime) {
    let now = SystemTime::now();
    let entry_ms = epoch_millis(start);
    let frame_ms = epoch_millis(now);
    let diff_ms = now.duration_since(start).map(|d| d.as_millis()).unwrap_or(0);

    let line = format!("entrada_ms={entry_ms} primer_frame_ms={frame_ms} diferencia_ms={diff_ms}\n");
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = file.write_all(line.as_bytes());
    }
}

fn epoch_millis(time: SystemTime) -> u128 {
    time.duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
}
