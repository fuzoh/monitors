use std::io::Write;
use std::process::{Command, Stdio};

use std::string::String;
mod monitor;

use monitor::Monitor;

fn main() {
    // Get the monitors form hyprctl
    let hyprctl_output = Command::new("hyprctl")
        .arg("-j")
        .arg("monitors")
        .arg("all")
        .output()
        .expect("Failed to get monitors from hyprctl");

    let output_string = String::from_utf8(hyprctl_output.stdout)
        .expect("Failed to parse the command output as utf8 string");

    let monitors: Vec<Monitor> =
        serde_json::from_str(&output_string).expect("Failed to parse the command output as json");

    let monitors_string = monitors
        .iter()
        .map(|m| {
            format!(
                "{} {}\n",
                m.name,
                m.biggest_mode().expect("No mode found for this monitor")
            )
        })
        .collect::<String>();

    // Start tofi and give it the list of monitors
    let mut tofi_select_monitors = Command::new("tofi")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to launch tofi for monitor selection");

    let mut stdin_tofi = tofi_select_monitors
        .stdin
        .take()
        .expect("Failed to acess stdin");
    stdin_tofi
        .write_all(monitors_string.as_bytes())
        .expect("Failed to write data to stdin");

    let selected_monitor =
        String::from_utf8(tofi_select_monitors.wait_with_output().expect("").stdout)
            .expect("Failed to read tofi output");
    println!("Selected monitor {}", selected_monitor)
}
