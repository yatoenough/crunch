use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use pedalboard::{
    EffectChain,
    gain::{Distortion, Fuzz, Overdrive},
    modulation::{Chorus, Delay},
};
use ratatui::Frame;

use crate::tui::{
    App, PedalFocus, Screen,
    types::{EFFECT_KINDS, MENU_ITEMS},
    widgets::{default_index, device_names, move_index, next_index},
};
use crate::{audio::device::config_with_min_buffer, engine::AudioStreams};

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let input_devices: Vec<_> = host.input_devices()?.collect();
        let output_devices: Vec<_> = host.output_devices()?.collect();
        let default_input_device = host.default_input_device();
        let default_output_device = host.default_output_device();
        let default_input = default_index(&input_devices, &default_input_device);
        let default_output = default_index(&output_devices, &default_output_device);
        let selected_input = default_input.unwrap_or(0);
        let selected_output = default_output.unwrap_or(0);
        let input_names = device_names(&input_devices);
        let output_names = device_names(&output_devices);

        Ok(Self {
            screen: Screen::MainMenu,
            menu_index: 0,
            input_devices,
            output_devices,
            input_names,
            output_names,
            default_input,
            default_output,
            selected_input,
            selected_output,
            selected_effect: 0,
            selected_param: 0,
            selected_add_effect: 0,
            pedal_focus: PedalFocus::Params,
            sample_rate: 48_000.0,
            effects: Arc::new(Mutex::new(EffectChain::new())),
            streams: None,
            running: false,
            message: None,
        })
    }

    pub fn render(&self, frame: &mut Frame) {
        match self.screen {
            Screen::MainMenu => self.render_main_menu(frame),
            Screen::InputDeviceSelect => self.render_device_select(frame, true),
            Screen::OutputDeviceSelect => self.render_device_select(frame, false),
            Screen::Pedalboard | Screen::AddEffect => self.render_pedalboard(frame),
        }

        if self.screen == Screen::AddEffect {
            self.render_add_effect(frame);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        self.message = None;

        if matches!(key.code, KeyCode::Char('q'))
            || matches!(key.code, KeyCode::Char('c'))
                && key.modifiers.contains(KeyModifiers::CONTROL)
        {
            return Ok(true);
        }

        match self.screen {
            Screen::MainMenu => self.handle_main_menu_key(key),
            Screen::InputDeviceSelect => self.handle_input_select_key(key),
            Screen::OutputDeviceSelect => self.handle_output_select_key(key),
            Screen::Pedalboard => self.handle_pedalboard_key(key),
            Screen::AddEffect => self.handle_add_effect_key(key),
        }
    }

    fn handle_main_menu_key(&mut self, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match key.code {
            KeyCode::Up => self.menu_index = self.menu_index.saturating_sub(1),
            KeyCode::Down => self.menu_index = (self.menu_index + 1).min(MENU_ITEMS.len() - 1),
            KeyCode::Enter => match self.menu_index {
                0 => self.screen = Screen::InputDeviceSelect,
                _ => return Ok(true),
            },
            KeyCode::Esc => return Ok(true),
            _ => {}
        }
        Ok(false)
    }

    fn handle_input_select_key(
        &mut self,
        key: KeyEvent,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match key.code {
            KeyCode::Up => self.selected_input = self.selected_input.saturating_sub(1),
            KeyCode::Down => {
                self.selected_input = next_index(self.selected_input, self.input_devices.len())
            }
            KeyCode::Enter if !self.input_devices.is_empty() => {
                self.screen = Screen::OutputDeviceSelect;
            }
            KeyCode::Esc => self.screen = Screen::MainMenu,
            _ => {}
        }
        Ok(false)
    }

    fn handle_output_select_key(
        &mut self,
        key: KeyEvent,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        match key.code {
            KeyCode::Up => self.selected_output = self.selected_output.saturating_sub(1),
            KeyCode::Down => {
                self.selected_output = next_index(self.selected_output, self.output_devices.len())
            }
            KeyCode::Enter if !self.output_devices.is_empty() => self.open_pedalboard()?,
            KeyCode::Esc => self.screen = Screen::InputDeviceSelect,
            _ => {}
        }
        Ok(false)
    }

    fn handle_pedalboard_key(&mut self, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match key.code {
            KeyCode::Char(' ') => self.toggle_audio()?,
            KeyCode::Char('a') => self.screen = Screen::AddEffect,
            KeyCode::Char('d') | KeyCode::Delete => self.remove_selected_effect(),
            KeyCode::Tab => self.toggle_focus(),
            KeyCode::BackTab => self.toggle_focus(),
            KeyCode::Up => self.move_pedal_selection(-1),
            KeyCode::Down => self.move_pedal_selection(1),
            KeyCode::Left => self.adjust_selected_param(-1.0),
            KeyCode::Right => self.adjust_selected_param(1.0),
            KeyCode::Esc => {
                self.stop_audio()?;
                self.streams = None;
                self.screen = Screen::MainMenu;
            }
            _ => {}
        }
        Ok(false)
    }

    fn handle_add_effect_key(&mut self, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
        match key.code {
            KeyCode::Up => self.selected_add_effect = self.selected_add_effect.saturating_sub(1),
            KeyCode::Down => {
                self.selected_add_effect =
                    (self.selected_add_effect + 1).min(EFFECT_KINDS.len() - 1)
            }
            KeyCode::Enter => {
                self.add_selected_effect();
                self.screen = Screen::Pedalboard;
            }
            KeyCode::Esc => self.screen = Screen::Pedalboard,
            _ => {}
        }
        Ok(false)
    }

    fn open_pedalboard(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.stop_audio()?;

        let input_device = &self.input_devices[self.selected_input];
        let output_device = &self.output_devices[self.selected_output];
        let input_config = config_with_min_buffer(input_device.default_input_config()?);
        let output_config = config_with_min_buffer(output_device.default_output_config()?);

        self.sample_rate = output_config.sample_rate as f32;
        self.streams = Some(AudioStreams::new(
            input_device,
            output_device,
            input_config,
            output_config,
            Arc::clone(&self.effects),
        )?);
        self.running = false;
        self.screen = Screen::Pedalboard;
        Ok(())
    }

    fn toggle_audio(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.running {
            self.stop_audio()?;
        } else if let Some(streams) = &self.streams {
            streams.start()?;
            self.running = true;
        }
        Ok(())
    }

    pub fn stop_audio(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(streams) = &self.streams {
            streams.pause()?;
        }
        self.running = false;
        Ok(())
    }

    fn toggle_focus(&mut self) {
        self.pedal_focus = match self.pedal_focus {
            PedalFocus::Effects => PedalFocus::Params,
            PedalFocus::Params => PedalFocus::Effects,
        };
    }

    fn move_pedal_selection(&mut self, amount: isize) {
        let effects = self.effects.lock().unwrap();
        match self.pedal_focus {
            PedalFocus::Effects => {
                self.selected_effect =
                    move_index(self.selected_effect, effects.effects().len(), amount);
                self.selected_param = 0;
            }
            PedalFocus::Params => {
                let params_len = effects
                    .effects()
                    .get(self.selected_effect)
                    .map(|effect| effect.params().len())
                    .unwrap_or(0);
                self.selected_param = move_index(self.selected_param, params_len, amount);
            }
        }
    }

    fn adjust_selected_param(&mut self, direction: f32) {
        let mut effects = self.effects.lock().unwrap();
        let Some(effect) = effects.effects_mut().get_mut(self.selected_effect) else {
            return;
        };
        let params = effect.params();
        let Some(param) = params.get(self.selected_param) else {
            return;
        };
        effect.set_param(param.name, param.value + param.step * direction);
    }

    fn add_selected_effect(&mut self) {
        let effect: Box<dyn pedalboard::Effect> = match EFFECT_KINDS[self.selected_add_effect] {
            "Overdrive" => Box::new(Overdrive::new(15.0, 0.5)),
            "Distortion" => Box::new(Distortion::new(15.0, 0.5)),
            "Fuzz" => Box::new(Fuzz::new(15.0, 0.5)),
            "Chorus" => Box::new(Chorus::new(self.sample_rate)),
            "Delay" => Box::new(Delay::new(self.sample_rate, 100.0, 0.1, 0.2, 0.43)),
            _ => return,
        };

        let mut effects = self.effects.lock().unwrap();
        effects.push_effect(effect);
        self.selected_effect = effects.effects().len().saturating_sub(1);
        self.selected_param = 0;
        self.pedal_focus = PedalFocus::Params;
    }

    fn remove_selected_effect(&mut self) {
        let mut effects = self.effects.lock().unwrap();
        effects.remove_effect(self.selected_effect);
        self.selected_effect = self
            .selected_effect
            .min(effects.effects().len().saturating_sub(1));
        self.selected_param = 0;
    }
}
