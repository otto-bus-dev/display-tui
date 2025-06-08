use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style,Stylize,Color},
    symbols::border,
    text::Line,
    widgets::{Cell,Block,StatefulWidget,Row,Table,TableState},
};

use ratatui::layout::Constraint;
use crate::monitor::Monitor;
use crate::utils::TUIMode;

#[derive(Debug)]
pub struct MonitorList<'a> {
    pub mode: TUIMode,
    pub selected_row: Option<usize>,
    pub state: TableState,
    pub monitors:&'a Vec<Monitor>,
}


impl<'a> MonitorList<'a> {
    pub fn new(monitors: &'a Vec<Monitor>,mode:TUIMode,selected_row:Option<usize>) -> Self {
        MonitorList{
            mode,
            selected_row,
            state: TableState::default()
                .with_selected(selected_row),
            monitors,
        }
    }

    fn monitors_to_rows(&self) -> Vec<Row<'static>> {
        self.monitors
            .iter()
            .map(|monitor| {
                let name = monitor.name.clone();
                let description = monitor.description.clone().unwrap_or_else(|| "No description".to_string());
                let scale = monitor.scale.unwrap_or(1.0).to_string();
                let enabled = monitor.enabled.to_string();
                
                let position = match monitor.position.as_ref() {
                    Some(pos) => format!("({},{})", pos.x, pos.y),
                    None => "N/A".to_string(),
                };

                let mut mode = monitor.get_current_resolution(); 
                if mode.is_none() {
                    mode = monitor.get_prefered_resolution();
                }
                let resolution = match mode{
                    Some(res) => format!("{}x{}", res.width, res.height),
                    None => "N/A".to_string(),
                };
                Row::new(vec![
                    Cell::default().content(
                        Line::from(
                            if enabled == "true" {
                                "".green().to_string()
                            } else {
                                "".red().to_string()
                            }
                        )
                        .centered()
                        .style(
                            Style::default().fg(
                                if enabled == "true" {Color::Green} else {Color::Red}
                            )
                        ),
                    ),
                    Cell::from(name),
                    Cell::from(description),
                    Cell::from(resolution), 
                    Cell::from(position),
                    Cell::from(scale),
                ])
            }
            )
            .collect()
    }
    
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(if self.monitors.len()>1 {" Displays "}else{" Display "}.white().bold());
        let mut instructions_items = vec![];

        match self.mode {
            TUIMode::View => {
                let selected_monitor = &self.monitors[self.selected_row.unwrap_or(0)];
                instructions_items.push(" Up ".white());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Move ".white());
                instructions_items.push("<m> ".blue().bold());
                instructions_items.push(" Resolution ".white());
                instructions_items.push("<r> ".blue().bold());
                instructions_items.push(" Scale ".white());
                instructions_items.push("<s> ".blue().bold());
                if selected_monitor.enabled {
                    instructions_items.push(" Disable ".white());
                    instructions_items.push("<d> ".blue().bold());
                } else {
                    instructions_items.push(" Enable ".white());
                    instructions_items.push("<e> ".blue().bold());
                }
            },

            TUIMode::Resolution=> {
                instructions_items.push(" Up ".white());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Select ".white());
                instructions_items.push("<Space> ".blue().bold());
                instructions_items.push(" Quit Resolution Mode ".white());
                instructions_items.push("<Esc> ".blue().bold());
            },

            TUIMode::Move => {
                instructions_items.push(" Fast ".white());
                instructions_items.push("<MAJ>+<*> ".blue().bold());
                instructions_items.push(" Up ".white());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Left ".white());
                instructions_items.push("<h> ".blue().bold());
                instructions_items.push(" Right ".white());
                instructions_items.push("<l> ".blue().bold());
                instructions_items.push(" Quit Move Mode ".white());
                instructions_items.push("<Esc> ".blue().bold());
            },
            TUIMode::Scale => {
                instructions_items.push(" Up ".white());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Select ".white());
                instructions_items.push("<Space> ".blue().bold());
                instructions_items.push(" Quit Scale Mode ".white());
                instructions_items.push("<Esc> ".blue().bold());
            },
        }

        instructions_items.push(" Save ".white());
        instructions_items.push("<w> ".blue().bold());
        instructions_items.push(" Quit ".white());
        instructions_items.push("<q> ".blue().bold());

        let instructions = Line::from(instructions_items);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .border_style(Style::default().fg(
                if self.mode == TUIMode::View {Color::Yellow} else {Color::White}));

        let widths = [
            
            Constraint::Percentage(5),
            Constraint::Percentage(15),
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
        ];   

        let table = Table::new(self.monitors_to_rows(),widths) 
            .column_spacing(1)
            .header(
                Row::new(vec![
                    Cell::default().content(
                        Line::from("  ")
                        .centered()
                    ),
                    Cell::from("name"),
                    Cell::from("description"),
                    Cell::from("resolution"),
                    Cell::from("position"),
                    Cell::from("scale")])
                    .bottom_margin(1)
                    .bold()
                    .green()
                    .reversed()
            )
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
    use crate::test_utils::test_monitors;

    #[test]
    fn render_list() {
        let mut list = MonitorList{
            state: TableState::default(),
            selected_row: Some(0),
            mode: TUIMode::View,
            monitors: &test_monitors(),
        }; 
        let mut buf = Buffer::empty(Rect::new(0, 0, 110, 7));
        
        list.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Displays ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
            "┃     name              description                      resolution             position         scale      ┃",
            "┃                                                                                                            ┃",
            "┃     Monitor 1         Description 1                    1920x1080              (0,0)            1          ┃",
            "┃     Monitor 2         Description 2                    1280x720               (1920,0)         1.25       ┃",
            "┃                                                                                                            ┃",
            "┗━━━━━━━━━━ Up <k>  Down <j>  Move <m>  Resolution <r>  Scale <s>  Disable <d>  Save <w>  Quit <q> ━━━━━━━━━━┛",
        ]);

        let border_style = Style::new().fg(Color::Yellow);
        let title_style = Style::new().bold().fg(Color::White);
        let header_style = Style::new().green().bold().reversed();
        let empty_style = Style::new();
        let instructions_label_style = Style::new().fg(Color::White);
        let instructions_key_style = Style::new().blue().bold();
        let connected_style = Style::new().fg(Color::Green);
        let disconnected_style = Style::new().fg(Color::Red);
        let row_style = Style::new();

        // first line : title
        expected.set_style(Rect::new(0, 0, 50, 1), border_style);
        expected.set_style(Rect::new(50, 0, 10, 1), title_style);
        expected.set_style(Rect::new(60, 0, 50, 1), border_style);       

        // second line : header
        expected.set_style(Rect::new(0, 1, 1, 1), border_style);
        expected.set_style(Rect::new(1, 1, 108, 1), header_style);
        expected.set_style(Rect::new(109, 1, 1, 1), border_style);
        
        // third line : empty
        expected.set_style(Rect::new(0, 2, 1, 1), border_style);
        expected.set_style(Rect::new(1, 2, 108, 1), empty_style);
        expected.set_style(Rect::new(109, 2, 1, 1), border_style);
         
        // fourth line : first row 
        expected.set_style(Rect::new(0, 3, 1, 1), border_style);
        expected.set_style(Rect::new(1, 3, 5, 1), connected_style);
        expected.set_style(Rect::new(6, 3, 103, 1), row_style);
        expected.set_style(Rect::new(109, 3, 1, 1), border_style);      

        // fifth line : second row 
        expected.set_style(Rect::new(0, 4, 1, 1), border_style);
        expected.set_style(Rect::new(1, 4, 5, 1), disconnected_style);
        expected.set_style(Rect::new(6, 4, 103, 1), row_style);
        expected.set_style(Rect::new(109, 4, 1, 1), border_style);   
         
        // fifth line : empty
        expected.set_style(Rect::new(0, 5, 1, 1), border_style);
        expected.set_style(Rect::new(1, 5, 108, 1), empty_style);
        expected.set_style(Rect::new(109, 5, 1, 1), border_style);

        // last line : instructions 
        expected.set_style(Rect::new(0,6,  11, 1), border_style);
        expected.set_style(Rect::new(11, 6, 4, 1), instructions_label_style);
        expected.set_style(Rect::new(15, 6, 4, 1), instructions_key_style);

        expected.set_style(Rect::new(19,6, 6, 1), instructions_label_style);
        expected.set_style(Rect::new(25,6, 4, 1), instructions_key_style);

        expected.set_style(Rect::new(29,6, 6, 1), instructions_label_style);
        expected.set_style(Rect::new(35,6, 4, 1), instructions_key_style);

        expected.set_style(Rect::new(39,6, 12, 1), instructions_label_style);
        expected.set_style(Rect::new(51,6, 4, 1), instructions_key_style);

        expected.set_style(Rect::new(55,6, 7, 1), instructions_label_style);
        expected.set_style(Rect::new(62,6, 4, 1), instructions_key_style);

        expected.set_style(Rect::new(66,6, 9, 1), instructions_label_style);
        expected.set_style(Rect::new(75,6, 4, 1), instructions_key_style);

        expected.set_style(Rect::new(79,6, 6, 1), instructions_label_style);
        expected.set_style(Rect::new(85,6, 4, 1), instructions_key_style);

        expected.set_style(Rect::new(89,6, 6, 1), instructions_label_style);
        expected.set_style(Rect::new(95,6, 4, 1), instructions_key_style);

        expected.set_style(Rect::new(99,6, 11, 1), border_style);

        assert_eq!(buf, expected);
    }
}
