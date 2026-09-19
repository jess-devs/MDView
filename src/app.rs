//! Startup, arguments, application state, window.
//! Coordinates `document`, `markdown`, and `render`; does not replace them.

use std::{
    env,
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use gpui::{App, Application, AppContext, Entity, WindowOptions};
use gpui_component::{Root, Theme};
#[cfg(windows)]
use std::{ffi::OsStr, os::windows::ffi::OsStrExt};
#[cfg(windows)]
use windows::{
    core::PCWSTR,
    Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS},
    Win32::System::Threading::CreateMutexW,
};

use crate::{document, markdown, render::DocumentView};

/// The mode the application is in. Only one value exists today: see AD-05.
pub enum Mode {
    ReadOnly,
}

/// One open document (RF-01.1): its element tree, and the path that both
/// names its tab (RF-06.1) and identifies it for RF-04.1's dedup — always
/// canonicalized (`std::fs::canonicalize`), so two paths that resolve to the
/// same file on disk compare equal without either caller re-deriving that.
/// What a tab currently shows (RF-23.1, RF-24.1). An enum, not two
/// independent `bool`s, so "raw and editing at once" is unrepresentable
/// rather than merely undocumented (AD-31). The `Entity<InputState>` behind
/// `Editing` does not live here: `app` coordinates, it does not know GPUI's
/// UI components (AD-04) — `render::DocumentView` keeps that map instead.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewMode {
    #[default]
    Rendered,
    Raw,
    Editing,
}

pub struct DocumentTab {
    pub path: PathBuf,
    pub blocks: Vec<markdown::Block>,
    /// The file's exact text, kept alongside `blocks` for RF-23.1: showing
    /// it raw doesn't re-read the file or re-run `markdown::parse`, so it
    /// can never drift from what was actually loaded (AD-30). Also the
    /// baseline RF-24.4 compares an editor's live text against, to decide
    /// whether there are unsaved changes.
    pub raw: String,
    /// What this tab currently shows. Per-tab, not global: switching tabs
    /// must not change another tab's mode.
    pub view: ViewMode,
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

/// How often the running instance checks for a later invocation's request
/// (RF-03.1). Not a criterion of any historia — plan.md's risk note is
/// explicit that this is the first thing to adjust if it's noticeable, not
/// the mechanism itself.
const INSTANCE_POLL_INTERVAL: Duration = Duration::from_millis(200);

pub fn run(start: SystemTime) {
    let paths: Vec<String> = env::args().skip(1).collect();
    let timing_path = env::var_os("MDVIEW_TIMING").map(PathBuf::from);

    // RF-03.1: a request file nobody picked up belongs to a run that ended
    // before it could — clear it before it's mistaken for one meant for
    // this run (plan.md's risk note).
    let _ = std::fs::remove_file(instance_request_path());
    if is_another_instance_running() {
        send_instance_request(&paths);
        return; // No window: the running instance handles this invocation.
    }

    let app = Application::new();
    app.run(move |cx| {
        gpui_component::init(cx);
        crate::render::init(cx);

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
                    tabs.push(DocumentTab { path: canonical, blocks, raw: text, view: ViewMode::Rendered });
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
            // Built here, not inside `open_window`'s closure, so a clone
            // survives outside it for the polling loop below to reach
            // (RF-03.1) — `Root` only exposes its child view as an opaque
            // `AnyView`, not this concrete type.
            let view = cx
                .update(|cx| cx.new(|_| DocumentView::new(state)))
                .expect("app was released before the window could open");
            let poll_view = view.clone();

            let window_handle = cx
                .open_window(WindowOptions::default(), move |window, cx| {
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

                    cx.new(|cx| Root::new(view, window, cx))
                })
                .expect("failed to open window");

            // RF-03.1: for as long as this instance runs, react to a later
            // invocation's request instead of letting it open a window of
            // its own.
            loop {
                cx.background_executor().timer(INSTANCE_POLL_INTERVAL).await;
                let Some(request_paths) = take_instance_request() else { continue };
                let _ = window_handle.update(cx, |_root, window, cx| {
                    window.activate_window();
                    poll_view.update(cx, |view, cx| {
                        for path in &request_paths {
                            open_or_activate_tab(&mut view.state, Path::new(path));
                        }
                        cx.notify();
                    });
                });
            }
        })
        .detach();
    });
}

/// `true` if another MDView process already holds the single-instance
/// mutex. The handle this creates is deliberately never closed — Win32
/// releases a named mutex once its last handle closes, so an open handle
/// for as long as the process runs is exactly what makes a *third*
/// invocation see this one as "already running" too. `windows::Win32::Foundation::HANDLE`
/// has no `Drop` impl (confirmed reading its definition, not assumed), so
/// letting the `Result` fall out of scope already leaves it open; nothing
/// further to do.
#[cfg(windows)]
fn is_another_instance_running() -> bool {
    let name: Vec<u16> = OsStr::new("MDView-Instancia-Unica").encode_wide().chain(std::iter::once(0)).collect();
    unsafe {
        let _ = CreateMutexW(None, false, PCWSTR(name.as_ptr()));
        GetLastError() == ERROR_ALREADY_EXISTS
    }
}

/// RF-03.1's single-instance behaviour is Windows-only for now (AD-28's
/// Mutex mechanism is Win32-specific); the `multiplataforma` feature still
/// has to design a Linux equivalent. Until then, a Linux build always acts
/// as the first instance — an explicit, documented gap, not a silent one.
#[cfg(not(windows))]
fn is_another_instance_running() -> bool {
    false
}

fn instance_request_path() -> PathBuf {
    std::env::temp_dir().join("mdview-instancia-peticion.txt")
}

/// Writes this invocation's paths — none, for RF-03.1's "just focus the
/// window" case — for the running instance to pick up. Written to a
/// temporary name first and renamed into place (`std::fs::rename` is
/// atomic on the same volume) so the other process never reads a half
/// written file.
fn send_instance_request(paths: &[String]) {
    let final_path = instance_request_path();
    let tmp_path = final_path.with_extension("tmp");
    if std::fs::write(&tmp_path, paths.join("\n")).is_ok() {
        let _ = std::fs::rename(&tmp_path, &final_path);
    }
}

/// Reads and removes a pending request, if there is one. `Some(vec![])`
/// means a request with no paths (RF-03.1 invoked with none): still a real
/// request — activate the window — distinct from `None`, meaning no
/// invocation is waiting on anything.
fn take_instance_request() -> Option<Vec<String>> {
    let path = instance_request_path();
    let content = std::fs::read_to_string(&path).ok()?;
    let _ = std::fs::remove_file(&path);
    Some(if content.is_empty() { Vec::new() } else { content.lines().map(String::from).collect() })
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

/// Reacts to a click on any link span, now that a document can navigate to
/// another one (RF-14.1): `http`/`https` still goes to the browser exactly
/// like `activate_link`; a destination that resolves, relative to
/// `doc_dir`, to a `.md` file opens it — a new tab, or its existing one
/// (RF-04.1). Anything else (`mailto:`, an `.md` that doesn't exist —
/// that still shows the RF-17 notice, not silence) falls through to doing
/// nothing further, same as before this historia. `doc_dir` is `None` only
/// when there is no current document to be relative to, which in practice
/// means there is no link to have clicked either.
pub fn activate_document_link(url: &str, doc_dir: Option<&Path>, view: &Entity<DocumentView>, cx: &mut App) {
    if url.starts_with("http://") || url.starts_with("https://") {
        activate_link(url);
        return;
    }

    let Some(doc_dir) = doc_dir else { return };
    let target = doc_dir.join(url);
    let is_md = target.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("md"));
    if !is_md {
        return;
    }

    view.update(cx, |view, cx| {
        open_or_activate_tab(&mut view.state, &target);
        cx.notify();
    });
}

/// Opens `target` in a new tab, or activates its tab if it's already open
/// (RF-04.1) — RF-14.1's navigation only, not the initial command-line load
/// in `run`: that has its own loop, because a repeated path there must
/// never move `active_tab` away from the first argument (CA-01.2), which is
/// exactly what activating an existing tab here is supposed to do.
fn open_or_activate_tab(state: &mut AppState, target: &Path) {
    let canonical = std::fs::canonicalize(target).unwrap_or_else(|_| target.to_path_buf());
    if let Some(ix) = state.tabs.iter().position(|tab| tab.path == canonical) {
        state.active_tab = ix;
        return;
    }
    match document::load(&canonical) {
        Ok(text) => {
            let base_dir = canonical.parent();
            let blocks = markdown::parse(&text, base_dir);
            state.tabs.push(DocumentTab { path: canonical, blocks, raw: text, view: ViewMode::Rendered });
            state.active_tab = state.tabs.len() - 1;
        }
        Err(error) => {
            // `doc_dir` is already canonicalized (RF-04.1 needs it to be),
            // so `target` carries Windows' `\\?\`-prefixed verbatim form
            // even though `canonicalize` itself just failed above. RF-17's
            // message should read like the one the initial command-line
            // load shows: the path as written, not as the filesystem API
            // happens to spell it.
            let shown = target.to_string_lossy();
            let shown = shown.strip_prefix(r"\\?\").unwrap_or(&shown);
            state.pending_notice = Some(error_message(shown, error));
        }
    }
}

/// Writes `text` to `tab.path` and updates `tab.raw`/`tab.blocks` to match
/// (RF-24.3): saving must leave the tab showing exactly what got written,
/// the same invariant a freshly opened document already keeps between
/// `raw` and `blocks`. Returns the write error, if any, without changing
/// `tab.view` — the caller decides what a failed save should look like.
pub fn save_tab(tab: &mut DocumentTab, text: String) -> std::io::Result<()> {
    std::fs::write(&tab.path, &text)?;
    let base_dir = tab.path.parent();
    tab.blocks = markdown::parse(&text, base_dir);
    tab.raw = text;
    tab.view = ViewMode::Rendered;
    Ok(())
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
