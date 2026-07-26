use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph},
};

use crate::tui::{
    App, PedalFocus,
    types::EFFECT_KINDS,
    widgets::{centered_rect, render_help, selected_item},
};

impl App {
    pub fn render_pedalboard(&self, frame: &mut Frame) {
        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(frame.area());
        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(34), Constraint::Percentage(66)])
            .split(outer[0]);

        self.render_effects(frame, main[0]);
        self.render_params(frame, main[1]);

        let state = if self.running { "Playing" } else { "Stopped" };
        let msg = self.message.as_deref().unwrap_or("");
        render_help(
            frame,
            outer[1],
            &format!(
                "{state} | Space play/stop  a add  d remove  Tab focus  Up/Down select  Left/Right edit  Esc menu  q quit  {msg}"
            ),
        );
    }

    fn render_effects(&self, frame: &mut Frame, area: Rect) {
        let effects = self.effects.lock().unwrap();
        let items: Vec<_> = if effects.effects().is_empty() {
            vec![ListItem::new("No effects. Press a to add one.")]
        } else {
            effects
                .effects()
                .iter()
                .enumerate()
                .map(|(i, effect)| selected_item(effect.name(), i == self.selected_effect))
                .collect()
        };
        let border = if self.pedal_focus == PedalFocus::Effects {
            Color::Yellow
        } else {
            Color::Gray
        };
        let list = List::new(items).block(
            Block::default()
                .title("Effects")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border)),
        );
        frame.render_widget(list, area);
    }

    fn render_params(&self, frame: &mut Frame, area: Rect) {
        let effects = self.effects.lock().unwrap();
        let Some(effect) = effects.effects().get(self.selected_effect) else {
            let empty = Paragraph::new("Add an effect to edit parameters.")
                .block(Block::default().title("Parameters").borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(empty, area);
            return;
        };

        let params = effect.params();
        let constraints = params
            .iter()
            .map(|_| Constraint::Length(3))
            .chain(std::iter::once(Constraint::Min(0)))
            .collect::<Vec<_>>();
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        let border = if self.pedal_focus == PedalFocus::Params {
            Color::Yellow
        } else {
            Color::Gray
        };
        frame.render_widget(
            Block::default()
                .title(format!("{} Parameters", effect.name()))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border)),
            area,
        );

        for (i, param) in params.iter().enumerate() {
            let ratio = ((param.value - param.min) / (param.max - param.min)).clamp(0.0, 1.0);
            let style = if i == self.selected_param && self.pedal_focus == PedalFocus::Params {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Cyan)
            };
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::NONE))
                .gauge_style(style)
                .label(format!("{}: {:.2}", param.name, param.value))
                .ratio(ratio as f64);
            let row = Rect {
                x: rows[i].x.saturating_add(1),
                y: rows[i].y.saturating_add(1),
                width: rows[i].width.saturating_sub(2),
                height: 1,
            };
            frame.render_widget(gauge, row);
        }
    }

    pub fn render_add_effect(&self, frame: &mut Frame) {
        let area = centered_rect(42, 45, frame.area());
        let items = EFFECT_KINDS
            .iter()
            .enumerate()
            .map(|(i, name)| selected_item(*name, i == self.selected_add_effect))
            .collect::<Vec<_>>();
        let list = List::new(items).block(
            Block::default()
                .title("Add Effect")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        );
        frame.render_widget(Clear, area);
        frame.render_widget(list, area);
    }
}
