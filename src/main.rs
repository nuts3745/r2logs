//! # Cloudflare Logs CLI
//! A simple CLI tool to retrieve logs from Cloudflare Logs Engine.
//! ## Usage
//! ```zsh
//! r2logs [start_time] [end_time] [--pretty | --verbose]
//! r2logs [--pretty] # last 5 minutes
//! r2logs [--verbose] # print time range and endpoint
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

use chrono::{DateTime, Duration, Utc};
use clap::Parser;
use serde_json::Value;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// e.g. 2024-01-11T15:00:00Z
    ///
    /// RFC3339 datetime format (UTC)
    ///
    /// default: 5 minutes ago
    start_time: Option<DateTime<Utc>>,
    /// e.g. 2024-01-11T15:05:00Z
    ///
    /// RFC3339 datetime format (UTC)
    ///
    /// default: now
    end_time: Option<DateTime<Utc>>,
    /// JSON Pretty print
    #[arg(short, long)]
    pretty: bool,
    /// Verbose output, print time range and endpoint
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = env::var("CF_API_KEY").expect("CF_API_KEY not set");
    let r2_access_key_id = env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID not set");
    let r2_secret_access_key =
        env::var("R2_SECRET_ACCESS_KEY").expect("R2_SECRET_ACCESS_KEY not set");
    let account_id = env::var("CF_ACCOUNT_ID").expect("CF_ACCOUNT_ID not set");
    let bucket = env::var("BUCKET_NAME").expect("BUCKET_NAME not set");

    let args = Args::parse();

    let time1 = args
        .start_time
        .unwrap_or_else(|| Utc::now() - Duration::minutes(5));
    let time2 = args.end_time.unwrap_or_else(Utc::now);
    let start_time = format_datetime(&time1);
    let end_time = format_datetime(&time2);

    if args.verbose {
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
    if args.verbose {
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
                if args.pretty {
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
