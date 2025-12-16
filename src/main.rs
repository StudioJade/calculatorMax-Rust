#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! Calculator Max - A powerful calculator application written in Rust
//!
//! This is the main entry point for the application.

use anyhow::Result;

pub mod calculator;
pub mod config;
pub mod i18n;
pub mod ui;

fn main() -> Result<()> {
    // Initialize the application
    println!("Starting Calculator Max...");

    // Run the GUI application
    ui::run_gui()
}
