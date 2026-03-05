use crossterm::event::{KeyCode,KeyEvent,KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Stylize,Color,Style},
    symbols::{
        Marker,
        border,
    },
    text::Line,
    widgets::{
        Block,
        Widget,
        canvas::{
            Canvas,
            Rectangle,
        }
    },
};
use crate::{
    App,
    configuration::Configuration,
    monitor::{Monitor, MonitorCanvas},
    rotation::Rotation,
    utils::TUIMode,
};

#[derive(Debug)]
pub struct Map<'a>{
    pub mode: TUIMode,
    pub selected: usize,
    pub monitors:&'a Vec<Monitor>,
}

impl<'a> Widget for Map<'a>{

    fn render(self, area: Rect, buf: &mut Buffer) {

        let monitor_canvas = Monitor::get_monitors_canvas(self.monitors,&area);

        let title = Line::from(" Map ".white().bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK)
            .border_style(Style::default().fg(
                if self.mode == TUIMode::Move {Color::Yellow} else {Color::White}));


        Canvas::default()
            .marker(Marker::HalfBlock)
            .block(block)
            .x_bounds(monitor_canvas.x_bounds)
            .y_bounds(monitor_canvas.y_bounds)
            .paint(|ctx| {
                let mut index = 0;
                for monitor in self.monitors {
                    if self.selected != index && monitor.enabled {
                        self.render_enabled_monitor(ctx,&monitor_canvas, monitor, Color::Blue);
                    }
                    index += 1;
                }
                index = 0;
                for monitor in self.monitors {
                    if self.selected == index && monitor.enabled {
                            self.render_enabled_monitor(ctx,&monitor_canvas,monitor, Color::Yellow);
                    }
                    index += 1;
                }
            })
            .render(area, buf);
    } 

    
}
impl<'a> Map<'a> {
   
    pub fn handle_events(app:&mut App, key_event: KeyEvent) {
        let is_shift = key_event.modifiers.contains(KeyModifiers::SHIFT);
        match key_event.code {
            KeyCode::Char('k') => Map::snap_vertical(app, -1),
            KeyCode::Char('K') => Map::move_vertical(app, -10),
            KeyCode::Up => if is_shift { Map::move_vertical(app, -10) } else { Map::snap_vertical(app, -1) },

            KeyCode::Char('j') => Map::snap_vertical(app, 1),
            KeyCode::Char('J') => Map::move_vertical(app, 10),
            KeyCode::Down => if is_shift { Map::move_vertical(app, 10) } else { Map::snap_vertical(app, 1) },

            KeyCode::Char('h') => Map::snap_horizontal(app, -1),
            KeyCode::Char('H') => Map::move_horizontal(app, -10),
            KeyCode::Left => if is_shift { Map::move_horizontal(app, -10) } else { Map::snap_horizontal(app, -1) },

            KeyCode::Char('l') => Map::snap_horizontal(app, 1),
            KeyCode::Char('L') => Map::move_horizontal(app, 10),
            KeyCode::Right => if is_shift { Map::move_horizontal(app, 10) } else { Map::snap_horizontal(app, 1) },
            
            KeyCode::Esc => Map::change_mode(app,TUIMode::View),
            _ => {}
        }
    }
    fn change_mode(app:&mut App,mode: TUIMode) {
        // Save monitor state when exiting Move mode
        if app.mode == TUIMode::Move {
            let _ = Configuration::save_monitor_state(&app.monitors);
        }
        app.mode = mode;
    }
    fn move_vertical(app:&mut App, direction: i32) {
        app.monitors[app.selected_monitor].move_vertical(direction);
    }
    fn snap_vertical(app:&mut App, direction: i32) {
        let selected_index = app.selected_monitor;
        let mut targets = vec![0.0];
        
        for (i, monitor) in app.monitors.iter().enumerate() {
            if i == selected_index || !monitor.enabled { continue; }
            let (_, y, _, h) = monitor.get_geometry();
            targets.push(y);
            targets.push(y + h);
            targets.push(y + h / 2.0);
        }

        let (_, sy, _, sh) = app.monitors[selected_index].get_geometry();
        let sources = vec![sy, sy + sh, sy + sh / 2.0];

        let mut best_delta: Option<f64> = None;

        for s in &sources {
            for t in &targets {
                let diff = t - s;
                if (direction < 0 && diff < -0.1) || (direction > 0 && diff > 0.1) {
                     match best_delta {
                         None => best_delta = Some(diff),
                         Some(current) => {
                             if diff.abs() < current.abs() {
                                 best_delta = Some(diff);
                             }
                         }
                     }
                }
            }
        }
        
        if let Some(delta) = best_delta {
            app.monitors[selected_index].move_vertical(delta.round() as i32);
        }
    }

    fn move_horizontal(app:&mut App, direction: i32) {
        app.monitors[app.selected_monitor].move_horizontal(direction);
    }
    fn snap_horizontal(app:&mut App, direction: i32) {
        let selected_index = app.selected_monitor;
        let mut targets = vec![0.0];
        
        for (i, monitor) in app.monitors.iter().enumerate() {
            if i == selected_index || !monitor.enabled { continue; }
            let (x, _, w, _) = monitor.get_geometry();
            targets.push(x);
            targets.push(x + w);
            targets.push(x + w / 2.0);
        }

        let (sx, _, sw, _) = app.monitors[selected_index].get_geometry();
        let sources = vec![sx, sx + sw, sx + sw / 2.0];

        let mut best_delta: Option<f64> = None;

        for s in &sources {
            for t in &targets {
                let diff = t - s;
                if (direction < 0 && diff < -0.1) || (direction > 0 && diff > 0.1) {
                     match best_delta {
                         None => best_delta = Some(diff),
                         Some(current) => {
                             if diff.abs() < current.abs() {
                                 best_delta = Some(diff);
                             }
                         }
                     }
                }
            }
        }
        
        if let Some(delta) = best_delta {
            app.monitors[selected_index].move_horizontal(delta.round() as i32);
        }
    }

    pub fn render_enabled_monitor(
        &self,
        ctx: &mut ratatui::widgets::canvas::Context,
        monitor_canvas: &MonitorCanvas,
        monitor: &Monitor,
        color: Color,
    ) {
        let mut mode = monitor.get_current_resolution();
        if mode.is_none() {
            mode = monitor.get_prefered_resolution();
        }

        let rotation = Rotation::from_transform(&monitor.transform);
        let (width, height) = if rotation == Rotation::Deg90 || rotation == Rotation::Deg270 {
            (
                mode.unwrap().height as f64 / monitor.scale.unwrap() as f64,
                mode.unwrap().width as f64 / monitor.scale.unwrap() as f64,
            )
        } else {
            (
                mode.unwrap().width as f64 / monitor.scale.unwrap() as f64,
                mode.unwrap().height as f64 / monitor.scale.unwrap() as f64,
            )
        };
        let x = monitor.position.clone().unwrap().x as f64;
        let y = (monitor_canvas.top - monitor_canvas.offset_y - monitor.position.clone().unwrap().y) as f64 - height ; 

        let x_margin = width * 0.07; 
        let y_margin = height * 0.07;

        ctx.print(
            x + x_margin, 
            y + height - y_margin, 
            Line::styled(
                monitor.name.to_string(),
                color
            )
        );

        ctx.draw(&Rectangle {
            x,
            y,
            width,
            height,
            color,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;
    use crate::test_utils::tests::test_monitors;

    #[test]
    fn render_map() {
        let map = Map {
            selected: 0,
            mode: TUIMode::View,
            monitors: &test_monitors(),
        }; 
        let mut buf = Buffer::empty(Rect::new(0, 0, 100, 30));
        
        map.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Map ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
            "┃                                                                                                  ┃",
            "┃  █▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█   ┃",
            "┃  █     Monitor 1                                                                             █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  █                                                                                           █   ┃",
            "┃  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀   ┃",
            "┃                                                                                                  ┃",
            "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
        ]);
        let vertical_line_style = Style::new().fg(Color::Yellow).bg(Color::Yellow);
        
        let horizontal_line_style = Style::new().fg(Color::Yellow);
        let border_style = Style::new().fg(Color::White);
        let title_style = Style::new().bold().fg(Color::White);
        let empty_style = Style::new();

        expected.set_style(Rect::new(0, 0, 47, 1), border_style);
        expected.set_style(Rect::new(47, 0, 5, 1), title_style);
        expected.set_style(Rect::new(52, 0, 48, 1), border_style);       

        expected.set_style(Rect::new(0, 1, 1, 28), border_style);
        expected.set_style(Rect::new(1, 1, 98, 28), empty_style);
        expected.set_style(Rect::new(99, 1, 1, 28), border_style);

        expected.set_style(Rect::new(0, 29, 100, 1), border_style);

        // Monitor styles
        // Top line y=2
        expected.set_style(Rect::new(3, 2, 1, 1), vertical_line_style);
        expected.set_style(Rect::new(4, 2, 91, 1), horizontal_line_style);
        expected.set_style(Rect::new(95, 2, 1, 1), vertical_line_style);

        // Sides y=3..26
        expected.set_style(Rect::new(3, 3, 1, 24), vertical_line_style);
        expected.set_style(Rect::new(95, 3, 1, 24), vertical_line_style);
        
        // Bottom line y=27
        expected.set_style(Rect::new(3, 27, 93, 1), horizontal_line_style);

        // Text y=3
        expected.set_style(Rect::new(9, 3, 9, 1), horizontal_line_style);

        assert_eq!(buf, expected);
    }
}
