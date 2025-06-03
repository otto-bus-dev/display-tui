# Display TUI

A simple TUI to manage display settings for Hyprland.
Built with Rust and the `crossterm` and `ratatui` libraries, it provides a user-friendly interface to control your display configurations.

# Features

- enable/disable display
- set display position
- set display resolution
- set display scale

# Preview

![Preview of Display TUI](/assets/preview.png)

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
2. Create a display-tui configuration file or run display-config to generate the default one :
   The configuration file is a json file that contains the tui settings.
   It contains only one field `monitors_config_path` which is the path where display-tui will save the monitors configuration for hyprland.
   the default path is `~/.config/hypr/hyprland/monitors.conf`.

   ```bash
   mkdir -p ~/.config/display-tui
   echo '{"monitors_config_path": "~/.config/hypr/hyprland/monitors.conf"}' > ~/.config/display-tui/config.json
   ```

3. Add reference to monitor configuration in your Hyprland config file:
   You need to add the following line to your Hyprland config file (usually located at `~/.config/hypr/hyprland.conf`):

   ```bash
   source ~/.config/hypr/hyprland/monitors.conf
   ```

   Here we have the default path, if you changed the `monitors_config_path` in the configuration file, you need to change it here too.

4. Run the TUI and Save your configuration:
   ```bash
   display-tui
   ```
