#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Calculator Max - A powerful calculator application written in Rust
//!
//! This is the main entry point for the application.

use anyhow::Result;

pub mod calculator;
pub mod config;
pub mod i18n;

// UI module is optional (feature = "gui") to speed up builds during development.
// When GUI feature is disabled, provide a no-op run function so tests and library
// builds work without depending on GUI crates.
#[cfg(feature = "gui")]
pub mod ui;

#[cfg(feature = "gui")]
fn run_app() -> Result<()> {
    println!("Starting Calculator Max with GUI...");
    ui::run_gui()
}

#[cfg(not(feature = "gui"))]
fn run_app() -> Result<()> {
    println!("GUI feature is disabled. Nothing to run in main().");
    Ok(())
}

fn main() -> Result<()> {
    run_app()
}
