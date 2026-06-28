use ratatui::{
    Frame,
    style::Color,
    widgets::{Block, Borders, Clear, List},
};

use crate::tui::{
    App,
    types::MENU_ITEMS,
    widgets::{centered_rect, selected_item},
};

impl App {
    pub fn render_main_menu(&self, frame: &mut Frame) {
        let area = centered_rect(50, 35, frame.area());
        let items: Vec<_> = MENU_ITEMS
            .iter()
            .enumerate()
            .map(|(i, item)| selected_item(format!("{}. {item}", i + 1), i == self.menu_index))
            .collect();
        let list = List::new(items).block(
            Block::default()
                .title("Crunch")
                .borders(Borders::ALL)
                .border_style(ratatui::style::Style::default().fg(Color::Blue)),
        );
        frame.render_widget(Clear, area);
        frame.render_widget(list, area);
    }
}
