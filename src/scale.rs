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
    fn render() {
        let mut app = Scale {
            state: TableState::default(),
        }; 
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
        
        
        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━━ Properties ━━━━━━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
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
