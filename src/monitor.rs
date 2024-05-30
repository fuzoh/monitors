use serde::{Deserialize, Deserializer, Serialize};

/// This structure represent data returned by the `hyprctl -j monitors all` command
/// Only the useful informations are deserialised.
/// See the command output for more informations.
/// Compatible with hyprland 0.4.0
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    id: u32,
    name: String,
    description: String,
    width: u32,
    height: u32,
    scale: f32,
    available_modes: Vec<Modes>,
}

#[derive(PartialEq, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Modes {
    width: u32,
    height: u32,
    refresh_rate: f32,
}

/// Custom deserialisation to transform monitor resolution string into Modes struct
/// Input "3840x2400@60.00Hz"
impl<'de> Deserialize<'de> for Modes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let mut parts = s.split('@');
        let resolution = parts.next().unwrap();
        let refresh_rate = parts.next().unwrap();
        let mut resolution_parts = resolution.split('x');
        let width: u32 = resolution_parts.next().unwrap().parse().unwrap();
        let height: u32 = resolution_parts.next().unwrap().parse().unwrap();
        let refresh_rate: f32 = refresh_rate.trim_end_matches("Hz").parse().unwrap();
        Ok(Modes {
            width,
            height,
            refresh_rate,
        })
    }
}
