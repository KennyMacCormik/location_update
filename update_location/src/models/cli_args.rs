use clap::Parser;
use override_key_derive::ApplyOverrides;

/// Command-line arguments for update_location
#[derive(Parser, ApplyOverrides)]
#[command(name = "update_location", version, about = "location loading and updating script")]
#[apply_overrides(infer_keys)]
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

    /// Infatica API endpoint
    #[arg(long)]
    pub infatica_endpoint: Option<String>,

    /// IPRoyal token
    #[arg(long)]
    pub infatica_email: Option<String>,

    /// IPRoyal token
    #[arg(long)]
    pub infatica_password: Option<String>,

    /// timeout (e.g. 5m, 10s)
    #[arg(long)]
    pub infatica_timeout: Option<String>,
}