mod models;
mod init;

use clap::Parser;
use crate::init::load_config;
use crate::models::{CLIArgs};

fn main() {
    let args = CLIArgs::parse();

    match load_config(&args) {
        Ok(cfg) => {
            println!("Endpoint: {}", cfg.iproyal.get_endpoint());
            println!("Token: {}", cfg.iproyal.get_token());
            println!("Timeout: {:?}", cfg.iproyal.get_timeout());
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}