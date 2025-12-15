use crate::rotation::Rotation;
use serde::Deserialize;
use std::process::Command;
use std::io::Write;
use ratatui::layout::Rect;
#[derive(Debug,Default, Clone, Deserialize)]
pub struct Monitor {
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub modes: Vec<Resolution>,
    pub position: Option<Position>,
    pub scale: Option<f32>,
    pub transform: Option<String>,
}



#[derive(Debug, Clone, Deserialize)]
pub struct Position{
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
    pub refresh: f32,
    pub preferred: bool,
    pub current: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MonitorCanvas{
    pub top: i32,
    pub x_bounds: [f64; 2],
    pub y_bounds: [f64; 2],
    pub offset_y: i32,
}


impl Monitor {

    pub fn get_monitors() -> Vec<Monitor> {
        let output = Command::new("wlr-randr")
            .arg("--json")
            .output().expect("Failed to execute wlr-randr command");
        let stdout = String::from_utf8(output.stdout).expect("Failed to convert output to string");
        let new_monitors: Vec<Monitor> = match serde_json::from_str(&stdout) {
            Ok(monitors) => monitors,
            Err(e) => {
                eprintln!("Deserialization error: {}", e);
                Vec::new()
            }
        };

        new_monitors
    }
    pub fn get_monitors_canvas(monitors: &Vec<Monitor>, area: &Rect) -> MonitorCanvas {
        let mut left = 10000.0;
        let mut bottom = 10000.0;
        let mut right = -10000.0;
        let mut top = -10000.0;

        for monitor in monitors {
            if !monitor.enabled {
                continue;
            }
            let mut mode = monitor.get_current_resolution();
            if mode.is_none() {
                mode = monitor.get_prefered_resolution();
            }

            let rotation = Rotation::from_transform(&monitor.transform);
            let (width, height) = if rotation == Rotation::Deg90 || rotation == Rotation::Deg270 {
                (mode.unwrap().height, mode.unwrap().width)
            } else {
                (mode.unwrap().width, mode.unwrap().height)
            };

            let monitor_left = monitor.position.clone().unwrap().x as f64;
            let monitor_right = monitor_left  + (width as f64 / monitor.scale.unwrap() as f64);

            let monitor_bottom = monitor.position.clone().unwrap().y as f64;
            let monitor_top = monitor_bottom + (height as f64 / monitor.scale.unwrap() as f64);
            
            if monitor_right > right {
                right= monitor_right;
            }
            if monitor_top > top {
                top= monitor_top;
            }
            if monitor_left < left {
                left= monitor_left;
            }
            if monitor_bottom < bottom {
                bottom= monitor_bottom;
            }
        }


        let margin = 50.0;
        left -= margin;
        bottom -= margin;
        right += margin;
        top += margin;

        let x_bounds = [left, right];
        let y_bounds = [bottom, top];

        let mut offset_y = 0.0;
        if bottom < 0.0 {
             offset_y = -bottom;
        }
       
        MonitorCanvas {
            top: top as i32,
            x_bounds,
            y_bounds,
            offset_y: offset_y as i32,
        }

    }

    pub fn get_current_resolution(&self) -> Option<&Resolution> {
        self.modes
            .iter()
            .find(|m| m.current)
    }

    pub fn get_prefered_resolution(&self) -> Option<&Resolution> {
        self.modes
            .iter()
            .find(|m| m.preferred)
    }
    
    pub fn set_current_resolution(&mut self, index: usize) {
        if index < self.modes.len() {
            for mode in &mut self.modes {
                mode.current = false;
            }
            self.modes[index].current = true;
        } else {
            eprintln!("Index out of bounds: {}", index);
        }
    }

    pub fn to_hyprland_config(&self) -> String {
        let mode = match self.get_current_resolution() {
            Some(m) => m,
            None => {
                self.get_prefered_resolution().expect("No preferred resolution found")
            }
        };
        if self.enabled {
            let rotation = Rotation::from_transform(&self.transform);
            format!(
                "monitor = {}, {}x{}@{}, {}x{}, {}, transform,{}",
                self.name,
                mode.width, mode.height, mode.refresh,
                self.position.clone().unwrap().x, self.position.clone().unwrap().y,
                self.scale.unwrap_or(1.0),
                rotation.to_hyprland()
            )
        } else {
            format!(
                "monitor = {}, disabled",
                self.name
            )
        }
        
    }
    pub fn save_hyprland_config(path:&String,monitors: &Vec<Monitor>) -> std::io::Result<()> {
        let expanded_path = shellexpand::tilde(path).to_string();
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(expanded_path)?;
        for monitor in monitors {
            let config_line = monitor.to_hyprland_config();
            writeln!(file, "{}", config_line)?;
        }
        Ok(())
    }

    pub fn move_vertical(&mut self, direction: i32) {
        if let Some(ref mut pos) = self.position { pos.y += direction};
    }

    pub fn move_horizontal(&mut self, direction: i32) {
        if let Some(ref mut pos) = self.position { pos.x += direction};
    }

    pub fn get_geometry(&self) -> (f64, f64, f64, f64) {
        let mut mode = self.get_current_resolution();
        if mode.is_none() {
            mode = self.get_prefered_resolution();
        }
        
        if mode.is_none() { return (0.0,0.0,0.0,0.0); }

        let rotation = Rotation::from_transform(&self.transform);
        let (width, height) = if rotation == Rotation::Deg90 || rotation == Rotation::Deg270 {
            (mode.unwrap().height, mode.unwrap().width)
        } else {
            (mode.unwrap().width, mode.unwrap().height)
        };

        let scale = self.scale.unwrap_or(1.0);
        let logical_width = width as f64 / scale as f64;
        let logical_height = height as f64 / scale as f64;
        let x = self.position.clone().unwrap().x as f64;
        let y = self.position.clone().unwrap().y as f64;

        (x, y, logical_width, logical_height)
    }
}
