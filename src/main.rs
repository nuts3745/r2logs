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
//! - `CLOUDFLARE_API_KEY`: Cloudflare API key
//! - `R2_ACCESS_KEY_ID`: R2 Access Key ID
//! - `R2_SECRET_ACCESS_KEY`: R2 Secret Access Key
//! - `CLOUDFLARE_ACCOUNT_ID`: Cloudflare Account ID
//! - `BUCKET_NAME`: Bucket name
//! ## References
//! - [Cloudflare Logs Engine](https://developers.cloudflare.com/logs/r2-log-retrieval/)
//! - [R2](https://developers.cloudflare.com/r2/)

mod api;
mod commands;
mod config;

use crate::{api::ApiEnv, config::Env};
use commands::{Args, Commands};
use config::UrlEnv;

struct ParsedArgs {
    start_time: String,
    end_time: String,
    verbose: bool,
    commands: Option<Commands>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // the environment configuration
    let url_env = UrlEnv::get_env();
    let api_env = ApiEnv::get_env();
    // the command line arguments
    let args = Args::get_parsed();

    // the command to be executed
    // If `args.commands` is `Some`, it returns the cloned value of `args.commands`.
    // Otherwise, it returns the default value `Commands::Retrieve`.
    let command = args.commands.clone().unwrap_or(Commands::Retrieve);
    // the endpoint for the command
    let endpoint = command.get_endpoint(&args, &url_env);

    let client = reqwest::Client::new();
    let text = api::fetch_logs(
        &client,
        &endpoint,
        &api_env.cf_api_key,
        &api_env.r2_access_key_id,
        &api_env.r2_secret_access_key,
    )
    .await?;

    println!("{}", text);

    Ok(())
}
