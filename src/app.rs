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

/// One open document (RF-01.1): its element tree, and the path that both
/// names its tab (RF-06.1) and identifies it for RF-04.1's dedup — always
/// canonicalized (`std::fs::canonicalize`), so two paths that resolve to the
/// same file on disk compare equal without either caller re-deriving that.
pub struct DocumentTab {
    pub path: PathBuf,
    pub blocks: Vec<markdown::Block>,
}

pub struct AppState {
    // Unread until a second `Mode` value exists; see AD-05.
    #[allow(dead_code)]
    pub mode: Mode,
    pub tabs: Vec<DocumentTab>,
    /// Index into `tabs` of the tab currently shown. Meaningless (and
    /// unread) while `tabs` is empty.
    pub active_tab: usize,
    /// Set when a requested file could not be shown (RF-17). `render`
    /// pushes it as a notification once, then clears it. One or more paths
    /// can fail in the same invocation (CA-01.5); their messages join into
    /// this single notice rather than one each.
    pub pending_notice: Option<String>,
    /// No path was given at all (RF-19): show the onboarding text instead of
    /// a silently blank window. Distinct from a load error, which also
    /// leaves `tabs` empty but must not show onboarding text over it.
    pub no_path_given: bool,
    /// Process entry time, for the `MDVIEW_TIMING` startup measurement (AD-07).
    pub start: SystemTime,
    /// Where to append the timing line, if `MDVIEW_TIMING` names a path.
    pub timing_path: Option<PathBuf>,
}

pub fn run(start: SystemTime) {
    let paths: Vec<String> = env::args().skip(1).collect();
    let timing_path = env::var_os("MDVIEW_TIMING").map(PathBuf::from);

    let app = Application::new();
    app.run(move |cx| {
        gpui_component::init(cx);

        let mut tabs: Vec<DocumentTab> = Vec::new();
        let mut errors = Vec::new();

        for path in &paths {
            match document::load(path) {
                // The document's directory, not the process's (CA-03.2):
                // relative image paths (RF-11.1) resolve against where the
                // `.md` file lives, wherever MDView was invoked from.
                Ok(text) => {
                    let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| PathBuf::from(path));
                    if tabs.iter().any(|tab| tab.path == canonical) {
                        continue; // RF-04.1: the same file, already open
                    }
                    let base_dir = canonical.parent();
                    let blocks = markdown::parse(&text, base_dir);
                    tabs.push(DocumentTab { path: canonical, blocks });
                }
                Err(error) => errors.push(error_message(path, error)),
            }
        }
        let pending_notice = if errors.is_empty() { None } else { Some(errors.join("\n")) };

        let state = AppState {
            mode: Mode::ReadOnly,
            tabs,
            active_tab: 0,
            pending_notice,
            no_path_given: paths.is_empty(),
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

/// Reacts to a click on a link span (RF-15.1). Only `http`/`https` are
/// handed to the system's default browser; anything else — a `mailto:`
/// link, or a relative `.md` link, which is RF-14 in another feature — does
/// nothing today. If the browser can't be launched, the failure is dropped:
/// there is nothing useful to show the user for it, and CA-02.4 requires
/// MDView to keep responding either way (errors as values, never `panic!`).
pub fn activate_link(url: &str) {
    if url.starts_with("http://") || url.starts_with("https://") {
        let _ = open::that_detached(url);
    }
}

/// Closes one tab (RF-05.1). Returns `true` when it was the only one open
/// (RF-07.1): the caller is responsible for actually quitting, since only
/// it has the `App`/`Context` to do that with — this only touches `state`.
/// An out-of-range `ix` does nothing and returns `false`, the same
/// defensive check as `render_tab_bar`'s switch handler (AD-26).
pub fn close_tab(state: &mut AppState, ix: usize) -> bool {
    if ix >= state.tabs.len() {
        return false;
    }
    state.tabs.remove(ix);
    if state.tabs.is_empty() {
        return true;
    }
    if ix < state.active_tab {
        state.active_tab -= 1;
    } else if state.active_tab >= state.tabs.len() {
        state.active_tab = state.tabs.len() - 1;
    }
    // ix == active_tab and still in range: active_tab now refers to what
    // was the next tab, which is exactly the "closing the active tab
    // activates its neighbor" behaviour CA-03.2 asks for — no change needed.
    false
}
