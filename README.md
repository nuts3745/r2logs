# cargo-r2logs
Simple CLI tool for retrieving logs from Cloudflare Logs Engine.

## Installation 🛠️
Before installing `cargo-r2logs`, make sure Rust and Cargo are set up on your system. [Install Rust and Cargo here](https://www.rust-lang.org/tools/install).

Install `cargo-r2logs` with:
```zsh
$ cargo install cargo-r2logs --path .
```

## Usage 🔍
Retrieve logs within a specified time range using `cargo r2logs`. Time format: `YYYY-MM-DDTHH:MM:SSZ`.

```zsh
$ cargo r2logs [start_time] [end_time] [--pretty | --verbose]
$ cargo r2logs [--pretty] # Last 5 minutes
$ cargo r2logs [--verbose] # Time range and endpoint details
```
## Examples 📝

Last 5 minutes, pretty format:
  ```zsh
  $ cargo r2logs --pretty
  ```

Specific time range, verbose output:
  ```zsh
  $ cargo r2logs 2024-01-11T15:00:00Z 2024-01-11T15:01:00Z --verbose
  ```

> [!NOTE]
> Reduce the time range if the amount of data in R2 is too large.

## Environment Variables 🌐
Set up these variables before using `cargo-r2logs`:

- `CF_API_KEY`: Your Cloudflare API key.
- `R2_ACCESS_KEY_ID`: Your R2 Access Key ID.
- `R2_SECRET_ACCESS_KEY`: Your R2 Secret Access Key.
- `CF_ACCOUNT_ID`: Your Cloudflare Account ID.
- `BUCKET_NAME`: Name of the bucket for log retrieval.

## Contributing 👐
Your contributions to `cargo-r2logs` are highly appreciated! If you've got ideas for improvements or wish to contribute code, please feel free to open issues or submit PRs on our [GitHub repository](https://github.com/nuts3745/cargo-r2logs).

## References 📚
- [Cloudflare Logs Engine](https://developers.cloudflare.com/logs/r2-log-retrieval/)
- [Workers Trace Events Logpush](https://developers.cloudflare.com/workers/observability/logpush/)
- [Cloudflare R2](https://developers.cloudflare.com/r2/)