use std::process::Command;
use crate::monitor::Monitor;

/// Get monitors
pub fn get_monitors() -> Vec<Monitor> {
    // Get monitors list in json format
    // `hyprctl monitors all -j` https://wiki.hyprland.org/Configuring/Using-hyprctl/
    let hyprctl_output = Command::new("hyprctl")
        .arg("-j")
        .arg("monitors")
        .arg("all")
        .output()
        .expect("Failed to get monitors from hyprctl.");

    let output_string = String::from_utf8(hyprctl_output.stdout)
        .expect("Failed to parse the command output as utf8 string.");

    serde_json::from_str(&output_string)
        .expect("Failed to parse the command output as json")
}
