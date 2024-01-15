//! # Cloudflare Logs CLI
//! A simple CLI tool to retrieve logs from Cloudflare Logs Engine.
//! ## Usage
//! ```zsh
//! r2logs [OPTIONS] [START_TIME] [END_TIME]
//! r2logs # retrieve logs from 5 minutes ago to now
//! r2logs 2024-01-11T15:00:00Z 2024-01-11T15:05:00Z # retrieve logs from 2024-01-11T15:00:00Z to 2024-01-11T15:05:00Z
//! r2logs | jq . # pretty print JSON
//! r2logs --help # print help
//! ```
//! ## Options
//! - -v, --verbose
//!   - Verbose output, print time range and endpoint
//! - -h, --help
//!   - Print help (see a summary with '-h')
//! - -V, --version
//!   - Print version
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
use std::env;

/// ## CLI Arguments and Options
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
    /// Verbose output, print time range and endpoint
    #[arg(short, long)]
    verbose: bool,
}

impl Args {
    fn format_datetime(datetime: &DateTime<Utc>) -> String {
        datetime.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }
}

/// ## Environment Variables
/// - `CF_API_KEY`: Cloudflare API key
/// - `R2_ACCESS_KEY_ID`: R2 Access Key ID
/// - `R2_SECRET_ACCESS_KEY`: R2 Secret Access Key
/// - `CF_ACCOUNT_ID`: Cloudflare Account ID
/// - `BUCKET_NAME`: Bucket name
struct Env {
    cf_api_key: String,
    r2_access_key_id: String,
    r2_secret_access_key: String,
    cf_account_id: String,
    bucket_name: String,
}

impl Env {
    fn new() -> Result<Self, String> {
        let cf_api_key = match env::var("CF_API_KEY") {
            Ok(val) => val,
            Err(_) => return Err("CF_API_KEY is not set".to_string()),
        };
        let r2_access_key_id = match env::var("R2_ACCESS_KEY_ID") {
            Ok(val) => val,
            Err(_) => return Err("R2_ACCESS_KEY_ID is not set".to_string()),
        };
        let r2_secret_access_key = match env::var("R2_SECRET_ACCESS_KEY") {
            Ok(val) => val,
            Err(_) => return Err("R2_SECRET_ACCESS_KEY is not set".to_string()),
        };
        let cf_account_id = match env::var("CF_ACCOUNT_ID") {
            Ok(val) => val,
            Err(_) => return Err("CF_ACCOUNT_ID is not set".to_string()),
        };
        let bucket_name = match env::var("BUCKET_NAME") {
            Ok(val) => val,
            Err(_) => return Err("BUCKET_NAME is not set".to_string()),
        };

        Ok(Self {
            cf_api_key,
            r2_access_key_id,
            r2_secret_access_key,
            cf_account_id,
            bucket_name,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let env = match Env::new() {
        Ok(env) => env,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("Please set environment variables");
            return Ok(());
        }
    };
    let args = Args::parse();

    let start_time = match args.start_time {
        Some(start_time) => Args::format_datetime(&start_time),
        None => Args::format_datetime(&(Utc::now() - Duration::minutes(5))),
    };
    let end_time = match args.end_time {
        Some(end_time) => Args::format_datetime(&end_time),
        None => Args::format_datetime(&Utc::now()),
    };

    if args.verbose {
        println!();
        println!(
            "Retrieving logs from \x1b[32m{}\x1b[0m to \x1b[32m{}\x1b[0m ",
            start_time, end_time
        );
    }

    let endpoint = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/logs/retrieve?start={}&end={}&bucket={}&prefix={}",
        env.cf_account_id, start_time, end_time, env.bucket_name, "{DATE}"
    );
    if args.verbose {
        println!();
        println!("Accessing endpoint: \x1b[32m{}\x1b[0m", endpoint);
        println!();
    }

    let client = reqwest::Client::new();
    let res = client
        .get(&endpoint)
        .header("Authorization", format!("Bearer {}", env.cf_api_key))
        .header("R2-Access-Key-Id", env.r2_access_key_id)
        .header("R2-Secret-Access-Key", env.r2_secret_access_key)
        .send()
        .await?;

    if !res.status().is_success() {
        eprintln!("Failed to retrieve logs: {:?}", res.status());
        return Ok(());
    }

    let text = res.text().await?;
    if text.is_empty() {
        eprintln!("No logs found");
        eprintln!("Please check time range");
        return Ok(());
    }

    println!("{}", text);

    Ok(())
}
