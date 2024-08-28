use std::error::Error;
use clap::Parser;

mod cli_arguments;
mod configuration;
mod monitor;
mod hyprctl_service;

use crate::cli_arguments::Args;
use crate::configuration::Configuration;
use crate::hyprctl_service::get_monitors;
use crate::monitor::Monitor;
// use monitor::Monitor;

fn main() {
    // Load or create the configuration file
    let config: Configuration = confy::load("monitors", None)
        .expect("Failed loading the configuration file (check syntax and available keys).");

    // Parse command arguments
    let args = Args::parse();

    let available_monitors = get_monitors();

    if available_monitors.len() < 1 {
        eprintln!("No monitors found.");
        panic!();
    }

    // If specified in config, find it
    // If none specified, the first one
    let mut default_monitor: Option<&Monitor> = None;

    if let Some(id) = config.default_monitor.id {
        default_monitor = available_monitors.iter().find(|monitor| {
            monitor.id == id
        })
    } else if let Some(name) = config.default_monitor.name  {
        default_monitor = available_monitors.iter().find(|monitor| {
            monitor.name == name
        })
    } else if let Some(description) = config.default_monitor.description {
        default_monitor = available_monitors.iter().find(|monitor| {
            monitor.description == description
        })
    } else { 
        default_monitor = available_monitors.iter().min() // Take the first ID
    }

    println!("{}", config.default_scaling_factor);
    println!("{}", args.auto);
    match default_monitor {
        Some(m) => println!("Default monitor is {}", m.name) ,
        None => println!("No default monitor selected")
    }


    /*    // Get the monitors form hyprctl


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
    println!("Selected monitor {}", selected_monitor)*/
}
