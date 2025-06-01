use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style,Stylize,Color},
    symbols::border,
    text::Line,
    widgets::{Block,StatefulWidget,Row,Table,Cell,TableState},
};

use ratatui::layout::Constraint;
use crate::monitor::Monitor;

#[derive(Debug)]
pub struct Resolutions<'a> {
    pub state: TableState,
    pub monitor:&'a Monitor,
}

impl<'a> Resolutions<'a> {

    pub fn new(monitor: &'a Monitor,selected:Option<usize>) -> Self {
        Resolutions {
            state: TableState::default()
                .with_selected(selected),
            monitor,
        }
    }

    fn resolutions_to_rows(&self) -> Vec<Row<'static>> {
        self.monitor.modes.clone()
            .into_iter()
            .map(|mode| {
                Row::new(vec![
                    Cell::from(format!("{}x{}",mode.width, mode.height)),
                    Cell::from(mode.refresh.to_string()),
                Cell::from(
                if mode.preferred {
                    "".green().to_string()
                } else {
                    "".red().to_string()
                })
                    .style(
                        Style::default().fg(
                            if mode.preferred {Color::Green} else {Color::Red}
                        )
                    ),
                Cell::from(
                if mode.current  {
                    "".green().to_string()
                } else {
                    "".red().to_string()
                })
                    .style(
                        Style::default().fg(
                            if mode.current  {Color::Green} else {Color::Red}
                        )
                    ),
                            ])
            }
            )
            .collect()
    }

    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Resolutions ".bold());
        let block = Block::bordered()
            .title(title.white().centered())
            .border_set(border::THICK)
            .border_style(Style::default().fg(Color::Yellow));


        let widths = [
            
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ];   

        let table = Table::new(self.resolutions_to_rows(),widths) 
            .column_spacing(1)
            //.style(Style::new().blue())
            .header(
                Row::new(vec!["resolution","refresh", "preferred", "current"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
            )
 
            .row_highlight_style(Style::new().yellow())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(" ")           
            //.row_highlight_style(Style::new().reversed())
            //.highlight_symbol(">>")
            .block(block);

        StatefulWidget::render(
            table,
            area,
            buf,
            &mut self.state,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;
    use crate::monitor::{Monitor, Position, Resolution};

    #[test]
    fn render() {
        let mut resolutions = Resolutions {
            state: TableState::default(),
            monitor:&Monitor {
                    name: "Monitor 2".to_string(),
                    description: None,
                    enabled: false,
                    modes: vec![
                        // Example modes
                        Resolution {
                            width: 1920,
                            height: 1080,
                            refresh: 60.0,
                            preferred: true,
                            current: false,
                        },
                        Resolution {
                            width: 1280,
                            height: 720,
                            refresh: 60.0,
                            preferred: false,
                            current: true,
                        },
                    ],
                    position: Some(Position { x: 1920, y: 0 }),
                    scale: Some(1.0),
                },
            
        }; 
        let mut buf = Buffer::empty(Rect::new(0, 0, 84, 4));
        
        resolutions.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━━━━━━━━━ Resolutions ━━━━━━━━━━━━━━━━━━━━━━━━━┓",
            "┃  resolution      refresh       preferred       current        ┃",
            "┃                                                               ┃",
            "┃  1920x1080       60            true            false          ┃",
            "┃>>1280x720        60            false           true           ┃",
            "┃                                                               ┃",
            "┃                                                               ┃",
            "┃                                                               ┃",
            "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(19, 0, 12, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
}
