use std::io;
use std::process::Command;

use std::string::String;
mod tui;
mod app;
mod monitor;
mod actions;

use monitor::Monitor;
use app::App;


fn main() -> io::Result<()> {
    // Get the monitors form hyprctl
    let hyprctl_output = Command::new("hyprctl")
        .arg("-j")
        .arg("monitors")
        .arg("all")
        .output()
        .expect("Failed to get monitors from hyprctl");
    let output_string = String::from_utf8(hyprctl_output.stdout).expect("Failed to parse the command output as utf8 string");
    let monitors: Vec<Monitor> = serde_json::from_str(&output_string).expect("Failed to parse the command output as json");

    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
