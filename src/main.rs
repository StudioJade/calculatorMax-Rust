//! Calculator Max - A powerful calculator application written in Rust
//!
//! This is the main entry point for the application.

use anyhow::Result;

mod calculator;
mod config;
mod ui;
mod i18n;

fn main() -> Result<()> {
    // Initialize the application
    println!("Starting Calculator Max...");
    
    // Run the GUI application
    ui::run_gui()
}