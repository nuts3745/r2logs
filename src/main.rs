//! # Cloudflare Logs CLI
//! A simple CLI tool to retrieve logs from Cloudflare Logs Engine.
//! ## Usage
//! ```zsh
//! cargo r2logs [start_time] [end_time] [--pretty | --verbose]
//! cargo r2logs [--pretty] # last 5 minutes
//! cargo r2logs [--verbose] # print time range and endpoint
//! ```
//! ## Environment Variables
//! - `CF_API_KEY`: Cloudflare API key
//! - `R2_ACCESS_KEY_ID`: R2 Access Key ID
//! - `R2_SECRET_ACCESS_KEY`: R2 Secret Access Key
//! - `CF_ACCOUNT_ID`: Cloudflare Account ID
//! - `BUCKET_NAME`: Bucket name
//! ## References
//! - [Cloudflare Logs Engine](https://developers.cloudflare.com/logs/r2-log-retrieval/)
//! - [R2](https://developers.cloudflare.com/r2/)

use chrono::{Duration, Utc};
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = env::var("CF_API_KEY").expect("CF_API_KEY not set");
    let r2_access_key_id = env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID not set");
    let r2_secret_access_key =
        env::var("R2_SECRET_ACCESS_KEY").expect("R2_SECRET_ACCESS_KEY not set");
    let account_id = env::var("CF_ACCOUNT_ID").expect("CF_ACCOUNT_ID not set");
    let bucket = env::var("BUCKET_NAME").expect("BUCKET_NAME not set");

    // TODO: Use `clap` instead of `env::args()`
    let args: Vec<String> = env::args().collect();
    let pretty = args.contains(&"--pretty".to_string());
    let verbose = args.contains(&"--verbose".to_string());

    // in the case of `cargo <subcommand> [args]`,
    //   Args[0]: /path/to/cargo-subcommand
    //   Args[1]: <subcommand>
    //   Args[2]: [args]
    //
    // if `cargo run`, this workaround occurs an error.
    // so, use `cargo run -- --` or `cargo run -- -- [args]` instead.
    let _path = &args[0];
    let _command = &args[1];

    let (start_time, end_time) = if args.len() <= 3 {
        let end_time = Utc::now();
        let start_time = end_time - Duration::minutes(5);
        // start_time is 5 minutes ago, if no args
        (format_datetime(&start_time), format_datetime(&end_time))
    } else {
        (args[2].clone(), args[3].clone())
    };
    if verbose {
        println!();
        println!(
            "Retrieving logs from \x1b[32m{}\x1b[0m to \x1b[32m{}\x1b[0m ",
            start_time, end_time
        );
    }

    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/logs/retrieve?start={}&end={}&bucket={}",
        account_id, start_time, end_time, bucket
    );
    if verbose {
        println!();
        println!("Accessing endpoint: \x1b[32m{}\x1b[0m", endpoint);
        println!();
    }

    let client = reqwest::Client::new();
    let res = client
        .get(&endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("R2-Access-Key-Id", r2_access_key_id)
        .header("R2-Secret-Access-Key", r2_secret_access_key)
        .send()
        .await?;

    if !res.status().is_success() {
        eprintln!("Failed to retrieve logs: {:?}", res.status());
        return Ok(());
    }

    let text = res.text().await?;
    if text.is_empty() {
        eprintln!("No logs found");
        return Ok(());
    }

    for line in text.lines() {
        match serde_json::from_str::<Value>(line) {
            Ok(json) => {
                if pretty {
                    match serde_json::to_string_pretty(&json) {
                        Ok(formatted) => println!("{}", formatted),
                        Err(e) => eprintln!("Failed to format JSON: {}", e),
                    }
                } else {
                    match serde_json::to_string(&json) {
                        Ok(formatted) => println!("{}", formatted),
                        Err(e) => eprintln!("Failed to format JSON: {}", e),
                    }
                }
            }
            Err(e) => eprintln!("Failed to parse JSON: {}", e),
        }
    }

    Ok(())
}

fn format_datetime(datetime: &chrono::DateTime<Utc>) -> String {
    datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
