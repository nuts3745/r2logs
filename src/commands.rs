use crate::ParsedArgs;
use crate::UrlEnv;
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use clap::{Parser, Subcommand};

/// ## CLI Arguments and Options
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// e.g. 2024-01-11T15:00:00Z
    ///
    /// RFC3339 datetime format (UTC)
    ///
    /// default: 5 minutes ago
    pub start_time: Option<DateTime<Utc>>,
    /// e.g. 2024-01-11T15:05:00Z
    ///
    /// RFC3339 datetime format (UTC)
    ///
    /// default: now
    pub end_time: Option<DateTime<Utc>>,
    /// Verbose output, print time range and endpoint
    #[arg(short, long)]
    pub verbose: bool,
    /// Subcommands
    #[command(subcommand)]
    pub commands: Option<Commands>,
}
impl Args {
    pub fn get_parsed() -> ParsedArgs {
        let parsed_args = Self::parsed();
        if parsed_args.verbose {
            println!();
            println!(
                "Retrieve logs from \x1b[32m{:?}\x1b[0m to \x1b[32m{:?}\x1b[0m ",
                &parsed_args.start_time, &parsed_args.end_time
            );
        }
        parsed_args
    }

    fn parsed() -> ParsedArgs {
        let args = Self::parse();
        let parsed_start_time = args
            .start_time
            .map_or(Utc::now() - Duration::minutes(5), |t| t)
            .to_rfc3339_opts(SecondsFormat::Secs, true);
        let parsed_end_time = args
            .end_time
            .map_or(Utc::now(), |t| t)
            .to_rfc3339_opts(SecondsFormat::Secs, true);

        ParsedArgs {
            start_time: parsed_start_time,
            end_time: parsed_end_time,
            verbose: args.verbose,
            commands: args.commands,
        }
    }
}

/// ## Subcommands
/// - `Retrieve`: Stream logs stored in R2 that match the provided query parameters.
///   - This is the default subcommand.
/// - `List`: List relevant R2 objects containing logs matching the provided query parameters.
#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// (default) Stream logs stored in R2 that match the provided query parameters.
    Retrieve,
    /// List relevant R2 objects containing logs matching the provided query parameters.
    List,
}

impl Commands {
    pub fn get_endpoint(&self, args: &ParsedArgs, env: &UrlEnv) -> String {
        let endpoint = self.build_endpoint(args, env);
        if args.verbose {
            println!();
            println!("Accessing endpoint: \x1b[32m{}\x1b[0m", endpoint);
            println!();
        }
        endpoint
    }

    fn build_endpoint(&self, args: &ParsedArgs, env: &UrlEnv) -> String {
        let base_url = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/logs",
            env.cf_account_id
        );
        let params = format!(
            "start={}&end={}&bucket={}&prefix={}",
            args.start_time, args.end_time, env.bucket_name, "{DATE}"
        );

        match self {
            Self::Retrieve => format!("{}/retrieve?{}", base_url, params),
            Self::List => format!("{}/list?{}", base_url, params),
        }
    }
}

#[cfg(test)]
mod clap_tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_default_args() {
        let args = Args::get_parsed();
        let now = Utc::now();
        let five_minutes_ago = now - Duration::minutes(5);
        assert_eq!(
            args.start_time,
            five_minutes_ago.to_rfc3339_opts(SecondsFormat::Secs, true)
        );
        assert_eq!(
            args.end_time,
            now.to_rfc3339_opts(SecondsFormat::Secs, true)
        );
        assert!(!args.verbose);
        assert_eq!(args.commands, None);
    }

    #[test]
    fn test_verbose_args() {
        let args = Args::parse_from(["r2logs", "-v"]);
        assert!(args.verbose);
    }

    #[test]
    fn test_time_range_args() {
        let args = Args::parse_from(["r2logs", "2024-01-11T15:00:00Z", "2024-01-11T15:05:00Z"]);
        assert_eq!(args.start_time.unwrap().year(), 2024);
        assert_eq!(args.start_time.unwrap().month(), 1);
        assert_eq!(args.start_time.unwrap().day(), 11);
        assert_eq!(args.start_time.unwrap().hour(), 15);
        assert_eq!(args.start_time.unwrap().minute(), 0);
        assert_eq!(args.start_time.unwrap().second(), 0);
        assert_eq!(args.end_time.unwrap().year(), 2024);
        assert_eq!(args.end_time.unwrap().month(), 1);
        assert_eq!(args.end_time.unwrap().day(), 11);
        assert_eq!(args.end_time.unwrap().hour(), 15);
        assert_eq!(args.end_time.unwrap().minute(), 5);
        assert_eq!(args.end_time.unwrap().second(), 0);
    }

    #[test]
    fn test_commands_args() {
        let args = Args::parse_from(["r2logs", "list"]);
        assert_eq!(args.commands.unwrap(), Commands::List);
    }

    #[test]
    fn test_commands_args_with_time_range() {
        let args = Args::parse_from([
            "r2logs",
            "2024-01-11T15:00:00Z",
            "2024-01-11T15:05:00Z",
            "list",
        ]);
        assert_eq!(args.commands.unwrap(), Commands::List);
        assert_eq!(args.start_time.unwrap().year(), 2024);
        assert_eq!(args.start_time.unwrap().month(), 1);
        assert_eq!(args.start_time.unwrap().day(), 11);
        assert_eq!(args.start_time.unwrap().hour(), 15);
        assert_eq!(args.start_time.unwrap().minute(), 0);
        assert_eq!(args.start_time.unwrap().second(), 0);
        assert_eq!(args.end_time.unwrap().year(), 2024);
        assert_eq!(args.end_time.unwrap().month(), 1);
        assert_eq!(args.end_time.unwrap().day(), 11);
        assert_eq!(args.end_time.unwrap().hour(), 15);
        assert_eq!(args.end_time.unwrap().minute(), 5);
        assert_eq!(args.end_time.unwrap().second(), 0);
    }
}
