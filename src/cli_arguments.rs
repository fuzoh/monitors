use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Monitors")]
#[command(about = "Quickly manage your monitors")]
#[command(version, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub auto: bool,
}
