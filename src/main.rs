//! # Cloudflare Logs CLI
//! A simple CLI tool to retrieve logs from Cloudflare Logs Engine.
//! ## Usage
//! ```zsh
//! $ r2logs [OPTIONS] [START_TIME] [END_TIME] [COMMAND]
//! $ r2logs # retrieve logs from 5 minutes ago to now
//! $ r2logs list # list relevant R2 objects containing logs
//!
//! # retrieve logs from 2024-01-11T15:00:00Z to 2024-01-11T15:05:00Z
//! $ r2logs 2024-01-11T15:00:00Z 2024-01-11T15:05:00Z
//! # list relevant R2 objects containing logs from 2024-01-11T15:00:00Z to 2024-01-11T15:05:00Z
//! $ r2logs 2024-01-11T15:00:00Z 2024-01-11T15:05:00Z list
//!
//! $ r2logs | jq . # pretty print JSON
//! $ r2logs --help # print help
//! ```
//! ## Commands
//! Commands:
//! - retrieve (default)
//!   - Stream logs stored in R2 that match the provided query parameters
//! - list
//!   - List relevant R2 objects containing logs matching the provided query parameters
//! - help
//!   - Print this message or the help of the given subcommand(s)
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
use clap::{Parser, Subcommand};
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
    start_time: Option<String>,
    /// e.g. 2024-01-11T15:05:00Z
    ///
    /// RFC3339 datetime format (UTC)
    ///
    /// default: now
    end_time: Option<String>,
    /// Verbose output, print time range and endpoint
    #[arg(short, long)]
    verbose: bool,
    /// Subcommands
    #[command(subcommand)]
    commands: Option<Commands>,
}

struct ParsedArgs {
    start_time: String,
    end_time: String,
    verbose: bool,
    commands: Option<Commands>,
}

impl Args {
    fn get_parsed() -> ParsedArgs {
        let args = Self::parse();
        args.parse_and_check()
    }

    fn parse_and_check(self) -> ParsedArgs {
        let start_time = match self.start_time {
            Some(start_time) => {
                Self::format_datetime(&start_time.parse::<DateTime<Utc>>().unwrap())
            }
            None => Self::format_datetime(&(Utc::now() - Duration::minutes(5))),
        };
        let end_time = match self.end_time {
            Some(end_time) => Self::format_datetime(&end_time.parse::<DateTime<Utc>>().unwrap()),
            None => Self::format_datetime(&Utc::now()),
        };
        let verbose = self.verbose;

        ParsedArgs {
            start_time,
            end_time,
            verbose,
            commands: self.commands,
        }
    }

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
        let mut error_messages = Vec::<String>::new();
        let cf_api_key = Self::get_env_var("CF_API_KEY", &mut error_messages);
        let r2_access_key_id = Self::get_env_var("R2_ACCESS_KEY_ID", &mut error_messages);
        let r2_secret_access_key = Self::get_env_var("R2_SECRET_ACCESS_KEY", &mut error_messages);
        let cf_account_id = Self::get_env_var("CF_ACCOUNT_ID", &mut error_messages);
        let bucket_name = Self::get_env_var("BUCKET_NAME", &mut error_messages);

        match error_messages.len() {
            0 => Ok(Self {
                cf_api_key,
                r2_access_key_id,
                r2_secret_access_key,
                cf_account_id,
                bucket_name,
            }),
            _ => Err(error_messages.join("\n")),
        }
    }

    fn get_env_var(var_name: &str, error_messages: &mut Vec<String>) -> String {
        match env::var(var_name) {
            Ok(value) => value,
            Err(_) => {
                error_messages.push(format!("{} is not set", var_name));
                "".to_string()
            }
        }
    }
}

/// ## Subcommands
/// - `Retrieve`: Stream logs stored in R2 that match the provided query parameters.
///   - This is the default subcommand.
/// - `List`: List relevant R2 objects containing logs matching the provided query parameters.
#[derive(Subcommand, Debug)]
enum Commands {
    /// (default) Stream logs stored in R2 that match the provided query parameters.
    Retrieve,
    /// List relevant R2 objects containing logs matching the provided query parameters.
    List,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let env = match Env::new() {
        Ok(env) => env,
        Err(e) => {
            eprintln!("{}", e);
            println!();
            eprintln!("Please set environment variables");
            return Ok(());
        }
    };
    let args = Args::get_parsed();

    if args.verbose {
        println!();
        println!(
            "Retrieving logs from \x1b[32m{:?}\x1b[0m to \x1b[32m{:?}\x1b[0m ",
            &args.start_time, &args.end_time
        );
    }

    let endpoint = match &args.commands {
        Some(Commands::Retrieve) => {
            format!(
                "https://api.cloudflare.com/client/v4/accounts/{}/logs/retrieve?start={}&end={}&bucket={}&prefix={}",
                env.cf_account_id, args.start_time, args.end_time, env.bucket_name, "{DATE}"
            )
        }
        Some(Commands::List) => {
            format!(
                "https://api.cloudflare.com/client/v4/accounts/{}/logs/list?start={}&end={}&bucket={}&prefix={}",
                env.cf_account_id, args.start_time, args.end_time, env.bucket_name, "{DATE}"
            )
        }
        None => {
            format!(
                "https://api.cloudflare.com/client/v4/accounts/{}/logs/retrieve?start={}&end={}&bucket={}&prefix={}",
                env.cf_account_id, args.start_time, args.end_time, env.bucket_name, "{DATE}"
            )
        }
    };
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

#[cfg(test)]
mod tests {
    use super::*;

    mod clap_test {
        use super::*;
        use chrono::TimeZone;

        #[test]
        fn test_parse_and_check() {
            let args = Args {
                start_time: None,
                end_time: None,
                verbose: false,
                commands: None,
            };

            let parsed_args = args.parse_and_check();
            let now = Utc::now();
            let five_minutes_ago = now - Duration::minutes(5);

            assert_eq!(
                parsed_args.start_time,
                Args::format_datetime(&five_minutes_ago)
            );
            assert_eq!(parsed_args.end_time, Args::format_datetime(&now));
            assert!(!parsed_args.verbose);
        }

        #[test]
        fn test_time_range() {
            let args = Args {
                start_time: Some("2024-01-11T15:00:00Z".to_string()),
                end_time: Some("2024-01-11T15:05:00Z".to_string()),
                verbose: true,
                commands: None,
            };
            let parsed_args = args.parse_and_check();
            assert_eq!(parsed_args.start_time, "2024-01-11T15:00:00Z");
            assert_eq!(parsed_args.end_time, "2024-01-11T15:05:00Z");
            assert!(parsed_args.verbose);
        }

        #[test]
        fn test_get_parsed() {
            let parsed_args = Args::get_parsed();
            let now = Utc::now();
            let five_minutes_ago = now - Duration::minutes(5);

            assert_eq!(
                parsed_args.start_time,
                Args::format_datetime(&five_minutes_ago)
            );
            assert_eq!(parsed_args.end_time, Args::format_datetime(&now));
            assert!(!parsed_args.verbose);
        }

        #[test]
        fn test_format_datetime() {
            let datetime = Utc.with_ymd_and_hms(2024, 1, 11, 15, 5, 0).unwrap();
            assert_eq!(Args::format_datetime(&datetime), "2024-01-11T15:05:00Z");
        }
    }
}
