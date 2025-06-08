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
                    Cell::default().content(
                        Line::from(
                            if mode.current{
                                "".green().to_string()
                            } else {
                                "".red().to_string()
                            }
                        )
                        .centered()
                    )
                    .style(
                        Style::default().fg(
                            if mode.current {Color::Green} else {Color::Red}
                        )
                    ),
                    Cell::default().content(
                        Line::from(
                            format!("{}x{}",mode.width, mode.height)
                        )
                        .centered()
                    ),
                    Cell::default().content(
                        Line::from(
                            mode.refresh.to_string()
                        )
                        .centered()
                    ),
                    Cell::default().content(
                        Line::from(
                            if mode.preferred {
                                "".green().to_string()
                            } else {
                                "".red().to_string()
                            }
                        )
                        .centered()
                    )
                    .style(
                        Style::default().fg(
                            if mode.preferred {Color::Green} else {Color::Red}
                    )
                )])
            })
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
                Row::new(vec![
                    Cell::from(
                        Line::from("current")
                            .centered()
                    ),
                    Cell::from(
                        Line::from("resolution")
                            .centered()
                    ),
                    Cell::from(
                        Line::from("refresh")
                            .centered()
                    ), 
                    Cell::from(
                        Line::from("preferred")
                            .centered()
                    ), 
                ])
                    .style(Style::new().bold())
                    .bottom_margin(1)
                    .bold()
                    .green()
                    .reversed()
            )
 
            .row_highlight_style(Style::new().yellow())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol(" ")           
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
    use crate::test_utils::test_monitors;

    #[test]
    fn render_resolutions() {

        let mut resolutions = Resolutions {
            state: TableState::default(),
            monitor:&test_monitors()[0],
            
        }; 
        let mut buf = Buffer::empty(Rect::new(0, 0, 65, 7));
        
        resolutions.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━━━━━━━━━ Resolutions ━━━━━━━━━━━━━━━━━━━━━━━━━┓",
            "┃    current       resolution        refresh        preferred   ┃",
            "┃                                                               ┃",
            "┃                  1920x1080          60                      ┃",
            "┃                  1280x720           60                      ┃",
            "┃                                                               ┃",
            "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
        ]);
        let border_style = Style::new().fg(Color::Yellow);
        let title_style = Style::new().bold().fg(Color::White);
        let header_style = Style::new().green().bold().reversed();
        let empty_style = Style::new();
        let ok_style = Style::new().fg(Color::Green);
        let nok_style = Style::new().fg(Color::Red);
        let row_style = Style::new();

        // first line : title
        expected.set_style(Rect::new(0, 0, 26, 1), border_style);
        expected.set_style(Rect::new(26, 0, 13, 1), title_style);
        expected.set_style(Rect::new(39, 0, 26, 1), border_style);       

        // second line : header
        expected.set_style(Rect::new(0, 1, 1, 1), border_style);
        expected.set_style(Rect::new(1, 1, 63, 1), header_style);
        expected.set_style(Rect::new(64, 1, 1, 1), border_style);
        
        // third line : empty
        expected.set_style(Rect::new(0, 2, 1, 1), border_style);
        expected.set_style(Rect::new(1, 2, 63, 1), empty_style);
        expected.set_style(Rect::new(64, 2, 1, 1), border_style);
         
        // fourth line : first row 
        expected.set_style(Rect::new(0, 3, 1, 1), border_style);
        expected.set_style(Rect::new(1, 3, 15, 1), ok_style);
        expected.set_style(Rect::new(16, 3, 33, 1), row_style);
        expected.set_style(Rect::new(49, 3, 15, 1), ok_style);
        expected.set_style(Rect::new(64, 3, 1, 1), border_style);      

        // fifth line : second row 
        expected.set_style(Rect::new(0, 4, 1, 1), border_style);
        expected.set_style(Rect::new(1, 4, 15, 1), nok_style);
        expected.set_style(Rect::new(16, 4, 33, 1), row_style);
        expected.set_style(Rect::new(49, 4, 15, 1), nok_style);
        expected.set_style(Rect::new(64, 4, 1, 1), border_style);  
        
        // fifth line : empty
        expected.set_style(Rect::new(0, 5, 1, 1), border_style);
        expected.set_style(Rect::new(1, 5, 63, 1), empty_style);
        expected.set_style(Rect::new(64, 5, 1, 1), border_style);

        // last line : instructions 
        expected.set_style(Rect::new(0,6, 65, 1), border_style);

        assert_eq!(buf, expected);
    }
}
