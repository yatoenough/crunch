use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
};

use crate::tui::{
    App,
    widgets::{render_help, selected_item},
};

impl App {
    pub fn render_device_select(&self, frame: &mut Frame, input: bool) {
        let title = if input {
            "Select Input Device"
        } else {
            "Select Output Device"
        };
        let names = if input {
            &self.input_names
        } else {
            &self.output_names
        };
        let selected = if input {
            self.selected_input
        } else {
            self.selected_output
        };
        let default = if input {
            self.default_input
        } else {
            self.default_output
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());

        let items: Vec<_> = if names.is_empty() {
            vec![ListItem::new("No devices found")]
        } else {
            names
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    let suffix = if Some(i) == default { " (default)" } else { "" };
                    selected_item(format!("{}{}", name, suffix), i == selected)
                })
                .collect()
        };

        let list = List::new(items).block(Block::default().title(title).borders(Borders::ALL));
        frame.render_widget(list, chunks[0]);
        render_help(
            frame,
            chunks[1],
            "Up/Down select  Enter confirm  Esc back  q quit",
        );
    }
}
