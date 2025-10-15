mod models;
mod init;

use clap::Parser;
use crate::init::load_config;
use crate::models::CLIArgs;

fn main() -> anyhow::Result<()> {
    let args = CLIArgs::parse();

    let cfg = load_config(&args)?;
    println!("Endpoint: {}", cfg.iproyal.get_endpoint());
    println!("Timeout: {:?}", cfg.iproyal.get_timeout());
    Ok(())
}