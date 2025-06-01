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
            .into_iter()
            .map(|monitor| {
                let name = monitor.name.clone();
                let description = monitor.description.clone().unwrap_or_else(|| "No description".to_string());
                let scale = monitor.scale.unwrap_or(1.0).to_string();
                let enabled = monitor.enabled.to_string();
                
                let position = match monitor.position.clone() {
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
                Cell::from(
name),
                Cell::from(
description),
                Cell::from(
                if enabled == "true" {
                    "".green().to_string()
                } else {
                    "".red().to_string()
                })
                    .style(
                        Style::default().fg(
                            if enabled == "true" {Color::Green} else {Color::Red}
                        )
                    ),
                Cell::from(
resolution), 
                Cell::from(
position),
                Cell::from(
scale),
            ])
            }
            )
            .collect()
    }
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Properties ".white().bold());
        let mut instructions_items = vec![];

        match self.mode {
            TUIMode::View => {
                let selected_monitor = &self.monitors[self.selected_row.unwrap_or(0)];
                instructions_items.push(" Up ".white().into());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white().into());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Move ".white().into());
                instructions_items.push("<m> ".blue().bold());
                instructions_items.push(" Resolution ".white().into());
                instructions_items.push("<r> ".blue().bold());
                instructions_items.push(" Scale ".white().into());
                instructions_items.push("<s> ".blue().bold());
                if selected_monitor.enabled {
                    instructions_items.push(" Disable ".white().into());
                    instructions_items.push("<d> ".blue().bold());
                } else {
                    instructions_items.push(" Enable ".white().into());
                    instructions_items.push("<e> ".blue().bold());
                }
            },
            TUIMode::Resolution=> {
                instructions_items.push(" Up ".white().into());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white().into());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Select ".white().into());
                instructions_items.push("<Space> ".blue().bold());
                instructions_items.push(" Quit Resolution Mode ".white().into());
                instructions_items.push("<Esc> ".blue().bold());
            },
            TUIMode::Move => {
                instructions_items.push(" Fast ".white().into());
                instructions_items.push("<MAJ>+<*> ".blue().bold());
                instructions_items.push(" Up ".white().into());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white().into());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Left ".white().into());
                instructions_items.push("<h> ".blue().bold());
                instructions_items.push(" Right ".white().into());
                instructions_items.push("<l> ".blue().bold());
                instructions_items.push(" Quit Move Mode ".white().into());
                instructions_items.push("<Esc> ".blue().bold());
            },
            TUIMode::Scale => {
                instructions_items.push(" Up ".white().into());
                instructions_items.push("<k> ".blue().bold());
                instructions_items.push(" Down ".white().into());
                instructions_items.push("<j> ".blue().bold());
                instructions_items.push(" Select ".white().into());
                instructions_items.push("<Space> ".blue().bold());
                instructions_items.push(" Quit Scale Mode ".white().into());
                instructions_items.push("<Esc> ".blue().bold());
            },
        }

        instructions_items.push(" Save ".white().into());
        instructions_items.push("<w> ".blue().bold());
        instructions_items.push(" Quit ".white().into());
        instructions_items.push("<q> ".blue().bold());

        let instructions = Line::from(instructions_items);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .border_style(Style::default().fg(
                if self.mode == TUIMode::View {Color::Yellow} else {Color::White}));

        let widths = [
            
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(10),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
        ];   
        let table = Table::new(self.monitors_to_rows(),widths) 
            .column_spacing(1)
            //.style(Style::new().blue())
            .header(
                Row::new(vec!["name","description", "connected", "resolution", "position","scale"])
                    .style(Style::new().bold())
                    .bottom_margin(1),
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
    use crate::monitor::{Monitor, Position};

    #[test]
    fn render() {
        let mut app = MonitorList{
            state: TableState::default(),
            selected_row: Some(0),
            mode: TUIMode::View,
            monitors: &vec![
                Monitor {
                    name: "Monitor 1".to_string(),
                    description: None,
                    // make: None,
                    // model: None,
                    // serial: None,
                    // physical_size: None,
                    enabled: true,
                    modes: vec![],
                    position: Some(Position { x: 0, y: 0 }),
                    // transform: Some("normal".to_string()),
                    scale: Some(1.0),
                    // adaptive_sync: Some(false),
                },
                Monitor {
                    name: "Monitor 2".to_string(),
                    description: None,
                    // make: None,
                    // model: None,
                    // serial: None,
                    // physical_size: None,
                    enabled: false,
                    modes: vec![],
                    position: Some(Position { x: 0, y: 0 }),
                    // transform: Some("normal".to_string()),
                    scale: Some(1.0),
                    // adaptive_sync: Some(false),
                },
            ],
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
