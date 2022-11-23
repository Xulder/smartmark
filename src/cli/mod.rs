use clap::Parser;

/// CMD Flags, shutdown, restart etc.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub shutdown: bool,
    #[arg(short, long)]
    pub restart: bool,
    // NOTE: Figure out how to make these mutually exclusive
}

/* TODO: This should actually just be its own crate...
 * Just make some routes for reloading the config and the cli can just send requests
 * Think about a web-view config.toml editor, for the less computer savvy
 * Low priority but I do want to eventually add functionality with the windows system tray
*/
