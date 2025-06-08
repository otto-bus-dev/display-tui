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


use crate::monitor::{
    Monitor
    ,MonitorCanvas
};
use crate::utils::TUIMode;

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
        let width = mode.unwrap().width as f64 / monitor.scale.unwrap() as f64;
        let x = monitor.position.clone().unwrap().x as f64;
        let height = mode.unwrap().height as f64 / monitor.scale.unwrap() as f64;
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
    use crate::test_utils::test_monitors;

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
            "┃                                                                                                  ┃",
            "┃                                                                                                  ┃",
            "┃                                                                                                  ┃",
            "┃                                                                                                  ┃",
            "┃                                █▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀█                                 ┃",
            "┃                                █  Monitor 1                    █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                █                               █                                 ┃",
            "┃                                ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀                                 ┃",
            "┃                                                                                                  ┃",
            "┃                                                                                                  ┃",
            "┃                                                                                                  ┃",
            "┃                                                                                                  ┃",
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

        expected.set_style(Rect::new(0, 1, 1, 5), border_style);
        expected.set_style(Rect::new(1, 1, 98, 5), empty_style);
        expected.set_style(Rect::new(99, 1, 1, 5), border_style);

        expected.set_style(Rect::new(0, 6, 1, 1), border_style);
        expected.set_style(Rect::new(1, 6, 32, 1), empty_style);
        expected.set_style(Rect::new(33, 6, 1, 1), vertical_line_style);
        expected.set_style(Rect::new(34, 6, 31, 1), horizontal_line_style);
        expected.set_style(Rect::new(65, 6, 1, 1), vertical_line_style);
        expected.set_style(Rect::new(66, 6, 33, 1), empty_style);
        expected.set_style(Rect::new(99, 6, 1, 1), border_style);
 
        expected.set_style(Rect::new(0, 7, 1, 1), border_style);
        expected.set_style(Rect::new(1, 7, 32, 1), empty_style);
        expected.set_style(Rect::new(33, 7, 1, 1), vertical_line_style);
        expected.set_style(Rect::new(34, 7, 2, 1), empty_style);
        expected.set_style(Rect::new(36, 7, 9, 1), horizontal_line_style);
        expected.set_style(Rect::new(45, 7, 20, 1), empty_style);
        expected.set_style(Rect::new(65, 7, 1, 1), vertical_line_style);
        expected.set_style(Rect::new(66, 7, 33, 1), empty_style);
        expected.set_style(Rect::new(99, 7, 1, 1), border_style);
        
        expected.set_style(Rect::new(0, 8, 1, 15), border_style);
        expected.set_style(Rect::new(1, 8, 32, 15), empty_style);
        expected.set_style(Rect::new(33, 8, 1, 15), vertical_line_style);
        expected.set_style(Rect::new(34, 8, 31, 15), empty_style);
        expected.set_style(Rect::new(65, 8, 1, 15), vertical_line_style);
        expected.set_style(Rect::new(66, 8, 33, 15), empty_style);
        expected.set_style(Rect::new(99, 8, 1, 15), border_style);
 
        expected.set_style(Rect::new(0, 23, 1, 1), border_style);
        expected.set_style(Rect::new(1, 23, 32, 1), empty_style);
        expected.set_style(Rect::new(33, 23, 33, 1), horizontal_line_style);
        expected.set_style(Rect::new(66, 23, 33, 1), empty_style);
        expected.set_style(Rect::new(99, 23, 1, 1), border_style);       

        expected.set_style(Rect::new(0, 24, 1, 5), border_style);
        expected.set_style(Rect::new(1, 24, 98, 5), empty_style);
        expected.set_style(Rect::new(99, 24, 1, 5), border_style);

        expected.set_style(Rect::new(0,29, 100, 1), border_style);

        assert_eq!(buf, expected);
    }
}
