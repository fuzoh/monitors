use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum MonitorDirections {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Deserialize, Serialize)]
pub enum MonitorModes {
    Extend,
    Mirror,
}

/// Witch monitor will pe chosen to define the mirroring resolution
#[derive(Deserialize, Serialize)]
pub enum MirroredMonitor {
    DefaultMonitor, // Will mirror your default monitor resolution
    External, // Will chose the first external monitor present resolution (your internal monitor will mirror it)
}

/// These structures represent the configuration file
#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub default_monitor: DefaultMonitor,
    // Specify the default direction to place the monitor if extended mode
    pub default_direction: MonitorDirections,
    // Specify the default mode. mirror or extend
    pub default_mode: MonitorModes,
    // Specify the screen to mirror by default (resolution of this screen will be chosen)
    pub default_mirrored: MirroredMonitor,
    // Define the default scaling factor
    pub default_scaling_factor: u32,
    // Specify the default mirroring scaling
    pub default_mirrored_scaling_factor: u32,
}

/// These configuration allows to identify a default monitor with the corresponding
/// hyprland reported keys. See `hyprctl monitors all -j`
#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultMonitor {
    pub id: Option<u32>,
    pub name: Option<String>,
    pub description: Option<String>,
}

/// DEFAULT CONFIGURATION IF NO EXISTING
impl Default for Configuration {
    fn default() -> Self {
        Self {
            default_monitor: Default::default(),
            default_direction: MonitorDirections::Up,
            default_mode: MonitorModes::Extend,
            default_mirrored: MirroredMonitor::External,
            default_scaling_factor: 2,
            default_mirrored_scaling_factor: 3,
        }
    }
}

impl Default for DefaultMonitor {
    fn default() -> Self {
        Self {
            id: Some(0),
            name: None,
            description: None,
        }
    }
}
