use clap::Parser;

/// CMD Flags, shutdown, restart etc.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    shutdown: Option<bool>,
    #[arg(short, long)]
    restart: Option<bool>,
    // NOTE: Figure out how to make these mutually exclusive
}
