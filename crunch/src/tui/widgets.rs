use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, ListItem, Paragraph, Wrap},
};

pub fn selected_item<'a>(text: impl Into<String>, selected: bool) -> ListItem<'a> {
    let text = text.into();
    if selected {
        ListItem::new(Line::from(vec![Span::styled(
            format!("> {text}"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]))
    } else {
        ListItem::new(format!("  {text}"))
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

pub fn render_help(frame: &mut ratatui::Frame, area: Rect, text: &str) {
    let help = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(help, area);
}

pub fn device_names(devices: &[cpal::Device]) -> Vec<String> {
    devices.iter().map(|device| format!("{device}")).collect()
}

pub fn default_index(devices: &[cpal::Device], default: &Option<cpal::Device>) -> Option<usize> {
    default
        .as_ref()
        .and_then(|default| devices.iter().position(|device| device == default))
}

pub fn next_index(current: usize, len: usize) -> usize {
    if len == 0 {
        0
    } else {
        (current + 1).min(len - 1)
    }
}

pub fn move_index(current: usize, len: usize, amount: isize) -> usize {
    if len == 0 {
        return 0;
    }
    current.saturating_add_signed(amount).min(len - 1)
}
