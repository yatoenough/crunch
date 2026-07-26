#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    MainMenu,
    InputDeviceSelect,
    OutputDeviceSelect,
    Pedalboard,
    AddEffect,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PedalFocus {
    Effects,
    Params,
}

pub const MENU_ITEMS: [&str; 2] = ["Pedalboard", "Exit"];
pub const EFFECT_KINDS: [&str; 5] = ["Overdrive", "Distortion", "Fuzz", "Chorus", "Delay"];
