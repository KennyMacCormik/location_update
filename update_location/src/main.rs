mod infatica;
mod init;
mod iproyal;
mod models;

use crate::init::load_config;
use crate::models::CLIArgs;
use clap::Parser;
use tokio;

#[tokio::main]
async fn main() {
    let args = CLIArgs::parse();

    let cfg = match load_config(&args) {
        Ok(c) => c,
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
                &r.countries[0]
                    .cities
                    .as_ref()
                    .map(|c| c.options.len())
                    .unwrap_or(0),
                &r.countries[0]
                    .states
                    .as_ref()
                    .map(|c| c.options.len())
                    .unwrap_or(0),
                &r.countries[0]
                    .ip_availability
                    .as_deref()
                    .map(|c| c)
                    .unwrap_or("no data"),
            );
            println!();
        }
        Err(e) => eprintln!("iproyal request failed: {e:?}"),
    }

    match infatica::get_all(&cfg.infatica).await {
        Ok(results) => {
            println!("Infatica queries succeeded");

            println!("--- GEO NODES ---");
            println!("Records: {}", results.geo_nodes().len());
            if let Some(first) = results.geo_nodes().first() {
                println!("First record: {:?}", first);
            }
            println!();

            println!("--- REGION CODES ---");
            println!("Records: {}", results.region_codes().len());
            if let Some(first) = results.region_codes().first() {
                println!("First record: {:?}", first);
            }
            println!();

            println!("--- ZIP CODES ---");
            println!("Records: {}", results.zip_codes().len());
            if let Some(first) = results.zip_codes().first() {
                println!("First record: {:?}", first);
            }
            println!();

            println!("--- ISP CODES ---");
            println!("Records: {}", results.isp_codes().len());
            if let Some(first) = results.isp_codes().first() {
                println!("First record: {:?}", first);
            }
            println!();
        }

        Err(errors) => {
            eprintln!("Infatica query failed with {} error(s):", errors.len());
            for err in errors {
                eprintln!("  - {err}");
            }
        }
    }
}
