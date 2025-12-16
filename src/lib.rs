//! Calculator Max - A powerful calculator library

pub mod calculator;
pub mod config;
pub mod i18n;

// UI is optional to speed up development builds. Enable with `--features gui` or use
// default build which currently includes GUI. To skip GUI for faster builds: `cargo build --no-default-features`
#[cfg(feature = "gui")]
pub mod ui;

#[cfg(feature = "gui")]
pub use ui::run_gui;
