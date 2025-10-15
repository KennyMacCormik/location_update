use clap::Parser;
use override_key_derive::ApplyOverrides;
use crate::models::ApplyOverrides;

/// Command-line arguments for update_location
#[derive(Parser, ApplyOverrides)]
#[command(name = "update_location", version, about = "location loading and updating script")]
pub struct CLIArgs {
    /// Path to a configuration file
    #[arg(long)]
    pub config: Option<String>,

    /// IPRoyal API endpoint
    #[arg(long)]
    #[override_key("iproyal.endpoint")]
    pub iproyal_endpoint: Option<String>,

    /// IPRoyal token
    #[arg(long)]
    #[override_key("iproyal.token")]
    pub iproyal_token: Option<String>,

    /// timeout (e.g. 5m, 10s)
    #[arg(long)]
    #[override_key("iproyal.timeout")]
    pub iproyal_timeout: Option<String>,
}