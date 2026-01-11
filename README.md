# Desktop Files Creator (Rust Version)

This is a Rust port of the Desktop Files Creator application, built with GTK 4 and Libadwaita.

## Requirements

You need to have the following development packages installed on your system:

*   Rust and Cargo
*   GTK 4 development files (`libgtk-4-dev` on Debian/Ubuntu, `gtk4-devel` on Fedora)
*   Libadwaita development files (`libadwaita-1-dev` on Debian/Ubuntu, `libadwaita-devel` on Fedora)
*   Meson and Ninja (optional, for installation)

## Build and Run (Development)

To build and run the application for development:

```bash
cargo run
```

## Installation

To build and install the application globally on your system using Meson:

```bash
meson setup build
ninja -C build
sudo ninja -C build install
```

## Structure

*   `src/main.rs`: Application entry point.
*   `src/window.rs`: Main window logic and UI interaction.
*   `resources/window.ui`: UI definition (XML).
*   `data/`: Desktop entry and icon files.
