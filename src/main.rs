use std::io;
use crossterm::event::{self,Event,KeyCode,KeyEvent,KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::Widget,
    DefaultTerminal,Frame,
};
use ratatui::prelude::*;
mod list;
mod map;
mod monitor;
mod resolutions;
mod utils;
mod scale;
mod configuration;
use list::MonitorList;
use map::Map;
use monitor::{Monitor,Position};

use resolutions::Resolutions; 
use scale::Scale;
use utils::TUIMode;
use utils::ScaleValue;
use configuration::Configuration;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
struct App {
    exit:bool,
    config: Configuration,
    monitors: Vec<Monitor>,
    selected_monitor: usize,
    selected_resolution : usize,
    selected_scale: usize,
    mode: TUIMode,
}

impl App{
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.monitors = Monitor::get_monitors();
        self.selected_resolution= 0;
        self.selected_monitor= 0;
        self.config = Configuration::get();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame){
        frame.render_widget(self,frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('w') => self.write(), 
            _ => {
                match self.mode {
                    TUIMode::View => self.handle_view_mode(key_event),
                    TUIMode::Move => self.handle_move_mode(key_event),
                    TUIMode::Resolution=> self.handle_resolution_mode(key_event),
                    TUIMode::Scale => self.handle_scale_mode(key_event), 
                }
            }
        }
    }
    fn handle_view_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k')=> self.previous_monitor(),
            KeyCode::Char('j')=> self.next_monitor(),
            KeyCode::Char('e')=> self.enable_monitor(),
            KeyCode::Char('d')=> self.disable_monitor(),
            KeyCode::Char('m') => self.change_mode(TUIMode::Move),
            KeyCode::Char('r') => self.change_mode(TUIMode::Resolution),
            KeyCode::Char('s') => self.change_mode(TUIMode::Scale),
            _ => {}
        }
    }
    fn handle_move_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k') => self.move_vertical(-10),
            KeyCode::Char('j') => self.move_vertical(10),
            KeyCode::Char('h') => self.move_horizontal(-10),
            KeyCode::Char('l') => self.move_horizontal(10),
            KeyCode::Char('K') => self.move_vertical(-100),
            KeyCode::Char('J') => self.move_vertical(100),
            KeyCode::Char('H') => self.move_horizontal(-100),
            KeyCode::Char('L') => self.move_horizontal(100),
            KeyCode::Esc => self.change_mode(TUIMode::View),
            _ => {}
        }
    }

    fn handle_resolution_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k')=> self.previous_resolution(),
            KeyCode::Char('j')=> self.next_resolution(),
            KeyCode::Char(' ')=> self.select_resolution(),
            KeyCode::Esc => self.change_mode(TUIMode::View),
            _ => {}
        }
    }

    fn handle_scale_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('k')=> self.previous_scale(),
            KeyCode::Char('j')=> self.next_scale(),
            KeyCode::Char(' ')=> self.select_scale(),
            KeyCode::Esc => self.change_mode(TUIMode::View),
            _ => {}
        }
    }
     
    fn exit(&mut self) {
        self.exit = true;
    }
    
    fn write(&mut self) {
        Monitor::save_hyprland_config(
            &self.config.monitors_config_path,
            &self.monitors
        ).expect("Failed to save Hyprland config");
    }         
  
    fn change_mode(&mut self, mode: TUIMode) {
        self.mode = mode;
    }

    fn next_monitor(&mut self) {
        self.selected_monitor = if self.selected_monitor >= self.monitors.len() - 1 {
            0
        } else {
            self.selected_monitor + 1
        }
    }

    fn previous_monitor(&mut self) {
        self.selected_monitor = if self.selected_monitor == 0 {
            self.monitors.len() - 1
        } else {
            self.selected_monitor - 1
        }
    }

    fn next_resolution(&mut self) {
        self.selected_resolution = if self.selected_resolution >= self.monitors[self.selected_monitor].modes.len() - 1 {
            0
        } else {
            self.selected_resolution + 1
        }
    }

    fn previous_resolution(&mut self) {
        self.selected_resolution = if self.selected_resolution == 0 {
            self.monitors[self.selected_monitor].modes.len() - 1
        } else {
            self.selected_resolution - 1
        }
    }

    fn select_resolution(&mut self) {
        self.monitors[self.selected_monitor].set_current_resolution(self.selected_resolution);
    }

    fn next_scale(&mut self) {
        self.selected_scale = if self.selected_scale >= ScaleValue::table().len() - 1 {
            0
        } else {
            self.selected_scale + 1
        }
    }

    fn previous_scale(&mut self) {
        self.selected_scale = if self.selected_scale == 0 {
            ScaleValue::table().len() - 1
        } else {
            self.selected_scale - 1
        }
    }

    fn select_scale(&mut self) {
        let scale_value = Some(ScaleValue::table()[self.selected_scale].value);
        self.monitors[self.selected_monitor].scale = scale_value;
    }

    fn move_vertical(&mut self, direction: i32) {
        self.monitors[self.selected_monitor].move_vertical(direction);
    }

    fn move_horizontal(&mut self, direction: i32) {
        self.monitors[self.selected_monitor].move_horizontal(direction);
    }

    fn disable_monitor(&mut self) {
        self.monitors[self.selected_monitor].enabled = false;
    }

    fn enable_monitor(&mut self) {
        self.monitors[self.selected_monitor].enabled = true;
        self.monitors[self.selected_monitor].position = Some(
            Position {
                x: 0,
                y: 0,
            }
        );
        self.monitors[self.selected_monitor].scale = Some(1.0);
    }
}

impl Widget for &App {

    fn render(self,area: Rect, buf: &mut Buffer) {
        let mut monitor_list = MonitorList::new(
            &self.monitors,
            self.mode,
            Some(self.selected_monitor), 
        );

        let canvas = Map {
            mode: self.mode,
            selected: self.selected_monitor,
            monitors: &self.monitors,
        };
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ])
            .split(area);

        match self.mode {
            TUIMode::Resolution=> {
                let selected = &self.monitors[self.selected_monitor];
                let mut resolutions = Resolutions::new(
                        selected,
                        Some(self.selected_resolution)
                );    
                let inner_top_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ])
                    .split(outer_layout[0]);
                canvas.render(inner_top_layout[0], buf);
                resolutions.render(inner_top_layout[1], buf);
            }
            TUIMode::Scale => {
                let mut scale = Scale::new(self.selected_scale);
                let inner_top_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Percentage(90),
                        Constraint::Percentage(10),
                    ])
                    .split(outer_layout[0]);
                canvas.render(inner_top_layout[0], buf);
                scale.render(inner_top_layout[1], buf);
            }
            _ => {
                canvas.render(outer_layout[0], buf);
            }
        }
        monitor_list.render(outer_layout[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('k').into());
        assert_eq!(app.selected_monitor, 1);

        app.handle_key_event(KeyCode::Char('j').into());
        assert_eq!(app.selected_monitor, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
