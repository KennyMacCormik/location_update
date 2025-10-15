use clap::Parser;

/// Command-line arguments for update_location
#[derive(Parser)]
#[command(name = "update_location", version, about = "location loading and updating script")]
pub struct CLIArgs {
    /// Path to a configuration file
    #[arg(long)]
    pub config: Option<String>,

    /// IPRoyal API endpoint
    #[arg(long)]
    pub iproyal_endpoint: Option<String>,

    /// IPRoyal token
    #[arg(long)]
    pub iproyal_token: Option<String>,

    /// timeout (e.g. 5m, 10s)
    #[arg(long)]
    pub iproyal_timeout: Option<String>,
}