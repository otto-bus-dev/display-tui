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
mod test_utils;

use list::MonitorList;
use map::Map;
use monitor::Monitor;

use resolutions::Resolutions; 
use scale::Scale;
use utils::TUIMode;
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
                    TUIMode::View => MonitorList::handle_events(self,key_event),
                    TUIMode::Move => Map::handle_events(self,key_event),
                    TUIMode::Resolution=> Resolutions::handle_events(self,key_event),
                    TUIMode::Scale => Scale::handle_events(self,key_event), 
                }
            }
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
    use crate::test_utils::tests::test_monitors;
   
    #[test]
    fn handle_mode_view_key_event() -> io::Result<()> {
        let mut app = App{
            monitors: test_monitors(),
            selected_monitor: 0,
            ..Default::default()
        };

        app.handle_key_event(KeyCode::Char('k').into());
        assert_eq!(app.selected_monitor, 1);

        app.handle_key_event(KeyCode::Char('j').into());
        assert_eq!(app.selected_monitor, 0);

        app.handle_key_event(KeyCode::Char('j').into());
        assert_eq!(app.selected_monitor, app.monitors.len() - 1);

        app.handle_key_event(KeyCode::Char('k').into());
        assert_eq!(app.selected_monitor, 0);
       
        app.handle_key_event(KeyCode::Char('m').into());
        assert_eq!(app.mode, TUIMode::Move);
    
        app.handle_key_event(KeyCode::Esc.into());
        assert_eq!(app.mode, TUIMode::View);

        app.handle_key_event(KeyCode::Char('r').into());
        assert_eq!(app.mode, TUIMode::Resolution);
    
        app.handle_key_event(KeyCode::Esc.into());
        assert_eq!(app.mode, TUIMode::View);

        app.handle_key_event(KeyCode::Char('s').into());
        assert_eq!(app.mode, TUIMode::Scale);
    
        app.handle_key_event(KeyCode::Esc.into());
        assert_eq!(app.mode, TUIMode::View);

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
     
         
    #[test]
    fn handle_mode_move_key_event() -> io::Result<()> {
        let mut app = App{
            monitors: test_monitors(),
            selected_monitor: 0,
            ..Default::default()
        };
 
        app.handle_key_event(KeyCode::Char('m').into());
        assert_eq!(app.mode, TUIMode::Move);

        app.handle_key_event(KeyCode::Char('k').into());
        let monitor = app.monitors[app.selected_monitor].clone();
        assert_eq!(monitor.position.unwrap().y, -10);

        app.handle_key_event(KeyCode::Char('j').into());
        let monitor = app.monitors[app.selected_monitor].clone();
        assert_eq!(monitor.position.unwrap().y, 0);

        app.handle_key_event(KeyCode::Char('h').into());
        let monitor = app.monitors[app.selected_monitor].clone();
        assert_eq!(monitor.position.unwrap().x, -10);

        app.handle_key_event(KeyCode::Char('l').into());
        let monitor = app.monitors[app.selected_monitor].clone();
        assert_eq!(monitor.position.unwrap().x, 0);

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }       
    #[test]
    fn handle_mode_resolution_key_event() -> io::Result<()> {
        let mut app = App{
            monitors: test_monitors(),
            selected_monitor: 0,
            ..Default::default()
        };

        app.handle_key_event(KeyCode::Char('r').into());
        assert_eq!(app.mode, TUIMode::Resolution);

        app.selected_resolution = 0;
        app.handle_key_event(KeyCode::Char('j').into());
        assert_eq!(app.selected_resolution, 1);

        app.handle_key_event(KeyCode::Char('k').into());
        assert_eq!(app.selected_resolution, 0);

        app.handle_key_event(KeyCode::Char(' ').into());
        let monitor = app.monitors[0].clone();
        assert_eq!(monitor.modes[0].current, true);

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }    

    #[test]
    fn handle_mode_scale_key_event() -> io::Result<()> {
        let mut app = App{
            monitors: test_monitors(),
            selected_monitor: 0,
            ..Default::default()
        };

        app.handle_key_event(KeyCode::Char('s').into());
        assert_eq!(app.mode, TUIMode::Scale);

        app.selected_scale = 0;
        app.handle_key_event(KeyCode::Char('j').into());
        assert_eq!(app.selected_scale, 1);

        app.handle_key_event(KeyCode::Char('k').into());
        assert_eq!(app.selected_scale, 0);

        app.handle_key_event(KeyCode::Char(' ').into());
        let monitor = app.monitors[0].clone();
        assert_eq!(monitor.scale, Some(0.5));

        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }       
}
