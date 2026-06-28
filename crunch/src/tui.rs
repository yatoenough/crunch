pub mod app;
pub mod screens;
pub mod types;
pub mod widgets;

use std::{
    io,
    sync::{Arc, Mutex},
    time::Duration,
};

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use pedalboard::EffectChain;
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::engine::AudioStreams;
pub use types::{PedalFocus, Screen};

pub struct App {
    pub screen: Screen,
    pub menu_index: usize,
    pub input_devices: Vec<cpal::Device>,
    pub output_devices: Vec<cpal::Device>,
    pub input_names: Vec<String>,
    pub output_names: Vec<String>,
    pub default_input: Option<usize>,
    pub default_output: Option<usize>,
    pub selected_input: usize,
    pub selected_output: usize,
    pub selected_effect: usize,
    pub selected_param: usize,
    pub selected_add_effect: usize,
    pub pedal_focus: PedalFocus,
    pub sample_rate: f32,
    pub effects: Arc<Mutex<EffectChain>>,
    pub streams: Option<AudioStreams>,
    pub running: bool,
    pub message: Option<String>,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new()?;

    loop {
        terminal.draw(|frame| app.render(frame))?;

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && app.handle_key(key)?
        {
            break;
        }
    }

    if app.running {
        app.stop_audio()?;
    }

    Ok(())
}
