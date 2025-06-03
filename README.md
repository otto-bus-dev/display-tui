# Display TUI

A simple TUI to manage display settings for Hyprland.
Built with Rust and the `crossterm` and `ratatui` libraries, it provides a user-friendly interface to control your display configurations.

# Features

- enable/disable display
- set display position
- set display resolution
- set display scale

# Preview

![Preview of Display TUI]([https://github.com/otto-bus-dev/display-tui/blob/master/assets/preview.png))

# Requirements

- Hyprland
- Hyprctl
- wlr-randr
- Nerd Font
- Rust
- Cargo

# Installation

1. Clone the repository and build the project:
   ```bash
   git clone https://github.com/otto-bus-dev/display-tui.git
   cd display-tui
   cargo build --release
   cp target/release/display-tui /usr/local/bin/ # or your preferred location
   ```
2. Add reference to monitor configuration in your Hyprland config file:
   ```bash
   source ~/.config/hypr/hyprland/monitors.conf
   ```
3. Run the TUI and Save your configuration:
   ```bash
   display-tui
   ```
