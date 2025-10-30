mod models;
mod init;
mod iproyal;
mod infatica;

use clap::Parser;
use tokio;
use crate::init::load_config;
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

    match iproyal::get_raw_data(&cfg.iproyal).await {
        Ok(r) => {
            println!("iproyal request succeeded");
            println!("iproyal countries {}", r.countries.len());
            println!(
                "iproyal first country: {{ code: \"{}\", name: \"{}\", cities: \"{}\", states: \"{}\", ip_availability: \"{}\" }}",
                &r.countries[0].code,
                &r.countries[0].name,
                &r.countries[0].cities.as_ref().map(|c| c.options.len()).unwrap_or(0),
                &r.countries[0].states.as_ref().map(|c| c.options.len()).unwrap_or(0),
                &r.countries[0].ip_availability.as_deref().map(|c| c).unwrap_or("no data"),
            );
            println!();
        },
        Err(e) => eprintln!("iproyal request failed: {e:?}"),
    }

    match infatica::get_raw_data(&cfg.infatica).await {
        Ok(r) => {
            println!("infatica request succeeded");
            println!("infatica records {}", r.len());
            println!("infatica first record: {:?}", &r[0]);
            println!();
        },
        Err(e) => eprintln!("infatica request failed: {e:?}"),
    }
}