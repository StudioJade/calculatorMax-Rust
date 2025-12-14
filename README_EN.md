# Calculator Max - Rust Version

## Language Versions

- [简体中文](README.md)
- [繁體中文（台灣）](README_TW.md)
- [繁體中文（香港）](README_HK.md)
- [English](README_EN.md)
- [Русский](README_RU.md)
- [Cat Language (喵星语)](README_CAT.md)

A powerful calculator application written in Rust, featuring a graphical user interface and extensive mathematical capabilities.

## Features

- **Safe Expression Evaluation**: Uses the `meval` crate for secure mathematical expression parsing
- **Extensive Math Functions**: Supports trigonometric, logarithmic, and other advanced functions
- **Geometric Calculations**: Area calculations for triangles, rectangles, circles, and trapezoids
- **Special Formulas**: Heron's formula for triangle area, Pythagorean theorem
- **History Tracking**: Keeps track of previous calculations
- **Configurable Safety**: Toggle between safe and extended evaluation modes
- **Modern GUI**: Built with egui for a responsive user interface

## Installation

### Prerequisites

- Rust toolchain (latest stable version recommended)

### Building

```bash
cargo build --release
```

### Running

```bash
cargo run
```

## Usage

1. Enter a mathematical expression in the input field
2. Click "Calculate" or press Enter
3. View the result in the result field
4. Use the "History" button to view previous calculations
5. Toggle "Safe Mode" to control evaluation restrictions

## Mathematical Functions

The calculator supports a wide range of functions:

- Basic arithmetic: `+`, `-`, `*`, `/`, `%`, `^`
- Constants: `pi`, `e`
- Trigonometric: `sin`, `cos`, `tan`, etc.
- Logarithmic: `log`, `log10`, `ln`

## Architecture

The application is structured into several modules:

- `calculator`: Core calculation logic
- `ui`: Graphical user interface
- `config`: Application settings

## License

MIT