#![windows_subsystem = "windows"]

mod app;
mod document;
mod html;
mod markdown;
mod render;

fn main() {
    let start = std::time::SystemTime::now();
    app::run(start);
}
