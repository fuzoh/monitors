use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;
use std::fmt::Display;

/// This structure represent data returned by the `hyprctl -j monitors all` command
/// Only the useful informations are deserialised.
/// See the command output for more informations.
/// Compatible with hyprland 0.4.0
#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub width: u32,
    pub height: u32,
    pub available_modes: Vec<Mode>,
}

impl PartialOrd for Monitor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for Monitor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(PartialEq, Serialize, Debug, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: String,
}

impl Mode {
    pub fn pixels(&self) -> u32 {
        self.width + self.height
    }
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.pixels().cmp(&other.pixels()))
    }
}

impl Ord for Mode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pixels().cmp(&other.pixels())
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{} {}", self.width, self.height))
    }
}

impl Monitor {
    pub fn biggest_mode(&self) -> Option<&Mode> {
        self.available_modes.iter().max()
    }
}

/// Custom deserialisation to transform monitor resolution string into Modes struct
/// Input "3840x2400@60.00Hz"
impl<'de> Deserialize<'de> for Mode {
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
        let refresh_rate: String = refresh_rate.trim_end_matches("Hz").parse().unwrap();
        Ok(Mode {
            width,
            height,
            refresh_rate,
        })
    }
}
