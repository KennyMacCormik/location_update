mod models;
mod init;
mod iproyal;

use clap::Parser;
use tokio;
use crate::init::load_config;
use crate::iproyal::{get_raw_data};
use crate::models::{CLIArgs};

#[tokio::main]
async fn main() {
    let args = CLIArgs::parse();

    let cfg = match load_config(&args) {
        Ok(c) => {c        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    match get_raw_data::get_raw_data(&cfg.iproyal).await {
        Ok(_) => println!("Request succeeded ✅"),
        Err(e) => eprintln!("Request failed: {e:?}"),
    }
}