use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style,Stylize,Color},
    symbols::border,
    text::Line,
    widgets::{Block,StatefulWidget,Row,Table,Cell,TableState},
};

use ratatui::layout::Constraint;
use crate::utils::ScaleValue;

#[derive(Debug)]
pub struct Scale{
    pub state: TableState,
}

impl Scale{
    pub fn new(selected:usize) -> Self {
        Scale {
            state: TableState::default()
                .with_selected(selected),
        }
    }



    fn scale_to_rows(&self) -> Vec<Row<'static>> {
        
        ScaleValue::table()
            .into_iter()
            .map(|scale| {
                Row::new(vec![
                    Cell::default().content(
                        Line::from(scale.name)
                            .centered()
                    ),
                ])
            })
            .collect()
    }
}

impl Scale{
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Scale ".bold());
        let block = Block::bordered()
            .title(title.white().centered())
            .border_set(border::THICK)
            .border_style(Style::default().fg(Color::Yellow));


        let widths = [
            Constraint::Percentage(100),
        ];   

        let table = Table::new(self.scale_to_rows(),widths) 
            .column_spacing(1)
            .row_highlight_style(Style::new().yellow())
            .cell_highlight_style(Style::new().blue())
            .highlight_symbol("  ")            
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

    #[test]
    fn render_scale() {

        let mut scales = Scale{
            state: TableState::default(),
        }; 
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 11));
        
        scales.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━ Scale ━━━━━━┓",
            "┃       50%        ┃",
            "┃       66%        ┃",
            "┃       75%        ┃",
            "┃       80%        ┃",
            "┃       100%       ┃",
            "┃       125%       ┃",
            "┃       160%       ┃",
            "┃       175%       ┃",
            "┃       200%       ┃",
            "┗━━━━━━━━━━━━━━━━━━┛",
        ]);

        let border_style = Style::new().fg(Color::Yellow);
        let title_style = Style::new().bold().fg(Color::White);
        let row_style = Style::new();

        // first line : title
        expected.set_style(Rect::new(0, 0, 6, 1), border_style);
        expected.set_style(Rect::new(6, 0, 7, 1), title_style);
        expected.set_style(Rect::new(13, 0, 7, 1), border_style);       

        // second line : row 
        for i in 0..ScaleValue::table().len() {
            expected.set_style(Rect::new(0, (i + 1) as u16, 1, 1), border_style);
            expected.set_style(Rect::new(1, (i + 1) as u16, 18, 1), row_style);
            expected.set_style(Rect::new(19, (i + 1) as u16, 1, 1), border_style);
        }

        // last line : instructions 
        expected.set_style(Rect::new(0,10, 20, 1), border_style);

        assert_eq!(buf, expected);
    }
}

