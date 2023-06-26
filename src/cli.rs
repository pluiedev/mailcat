use clap::Parser;

/// Simple program to cat email.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Number of emails to display.
    #[arg(short, long, default_value_t = 1)]
    entries: usize,
}
