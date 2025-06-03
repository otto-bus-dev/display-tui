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

        let title = Line::from(" Map ".bold());

        Canvas::default()
            .marker(Marker::HalfBlock)
            .block(
                Block::bordered()
                    .style(Style::default().fg(if self.mode == TUIMode::Move{Color::Yellow} else {Color::White}))
                    .title(title.white().centered())
            )
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
