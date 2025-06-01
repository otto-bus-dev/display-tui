use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Stylize,Color,Style},
    symbols::Marker,
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

use crate::monitor::Monitor;
use crate::utils::TUIMode;

#[derive(Debug)]
pub struct Map<'a>{
    pub mode: TUIMode,
    pub selected: usize,
    pub monitors:&'a Vec<Monitor>,
}

impl<'a> Widget for Map<'a>{

    fn render(self, area: Rect, buf: &mut Buffer) {

        let monitor_canvas = Monitor::get_monitors_canvas(&self.monitors);

        let left = monitor_canvas.left as f64;
        let bottom = monitor_canvas.bottom as f64;
        let right = monitor_canvas.right as f64;
        let top = monitor_canvas.top as f64;
        
        let area_ratio = area.width as f64 / area.height as f64;
        let canvas_ratio = monitor_canvas.width as f64 / monitor_canvas.height as f64;
        let canvas_area_ratio = canvas_ratio / area_ratio;
            
        let height = top - bottom;
        let added_height =  height * canvas_area_ratio  / 2.0;
        let y_bounds = [bottom - added_height, top + added_height];

        let width = right- left;
        let added_width =  width / canvas_area_ratio  / 2.0;
        let x_bounds = [left - added_width, right + added_width];

        let mut offset_y = 0.0;
        if bottom < 0.0 {
             offset_y = -bottom;
        }
        
        let title = Line::from(" Canvas ".bold());

        Canvas::default()
            .marker(Marker::HalfBlock)
            .block(
                Block::bordered()
                    .style(Style::default().fg(if self.mode == TUIMode::Move{Color::Yellow} else {Color::White}))
                    .title(title.white().centered())
            )
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(|ctx| {
                let mut index = 0;
                for monitor in self.monitors {
                    if self.selected != index {
                        if monitor.enabled {
                            self.render_enabled_monitor(ctx,top,offset_y, &monitor, Color::Blue);
                        } 
                    }
                    index += 1;
                }
                index = 0;
                for monitor in self.monitors {
                    if self.selected == index {
                        if monitor.enabled {
                            self.render_enabled_monitor(ctx,top,offset_y,&monitor, Color::Yellow);
                        }
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
        top: f64,
        offset_y: f64,
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
        let y = top - offset_y - height as f64 - monitor.position.clone().unwrap().y as f64; 
        self.render_monitor(ctx, x, y, width, height, &monitor.name, color);
    }

    pub fn render_monitor(
        &self,
        ctx: &mut ratatui::widgets::canvas::Context,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        label: &str,
        color: Color,
    ) {

        let x_margin = width * 0.07; 
        let y_margin = height * 0.07;

        ctx.print(
            (x + x_margin) as f64, 
            (y + height - y_margin) as f64, 
            Line::styled(
                format!("{}",
                    label,
                ),
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
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ratatui::style::Style;
//
//     #[test]
//     fn render() {
//         let map= Map::default();
//         let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
//
//         map.render(buf.area, &mut buf);
//
//         let mut expected = Buffer::with_lines(vec![
//             "┏━━━━━━━━━━━━━━━━━━━━ Canvas ━━━━━━━━━━━━━━━━━━━━┓",
//             "┃                                                ┃",
//             "┃                                                ┃",
//             "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
//         ]);
//         // let title_style = Style::new().bold();
//         // expected.set_style(Rect::new(21, 0, 8, 1), title_style);
//         // //
//         // assert_eq!(buf, expected);
//     }
// }
