use serde::Deserialize;

/// This structure represent data returned by the `hyprctl -j monitors all` command
/// Only the useful informations are deserialised.
/// See the command output for more informations.
/// Compatible with hyprland 0.4.0
#[derive(Deserialize)]
pub struct Monitor {
    id: u32,
    name: String,
    description: String,
    width: u32,
    height: u32,
    scale: f32,
    #[serde(rename = "availableModes")]
    available_modes: Vec<Modes>,
}

pub struct Modes {
    width: u32,
    height: u32,
    #[serde(rename = "refreshRate")]
    refresh_rate: f32,
}
