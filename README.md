# cargo-r2logs
A simple CLI tool to retrieve logs from Cloudflare Logs Engine.

## Installation
Before installing `cargo-r2logs`, ensure you have Rust and Cargo installed on your system. You can install Rust and Cargo from [here](https://www.rust-lang.org/tools/install).

To install `cargo-r2logs`, run the following command:
```
$ cargo install cargo-r2logs --path .
```

## Usage
Use `cargo r2logs` to retrieve logs for a specified time range. The time format should be `YYYY-MM-DDTHH:MM:SSZ`.

```zsh
cargo r2logs [start_time] [end_time] [--pretty | --verbose]
cargo r2logs [--pretty] # Retrieves logs from the last 5 minutes
cargo r2logs [--verbose] # Prints the time range and endpoint used for the request
```

### Examples

Retrieve logs from the last 5 minutes in pretty format:
```zsh
cargo r2logs --pretty
```

Retrieve logs from a specific time range with verbose output:
```zsh
cargo r2logs 2024-01-11T15:00:00Z 2024-01-11T15:05:00Z --verbose
```

> [!NOTE]
> To handle high log volumes in R2, adjust the time range accordingly.

## Environment Variables
Set the following environment variables before using `cargo-r2logs`:

- `CF_API_KEY`: Your Cloudflare API key.
- `R2_ACCESS_KEY_ID`: Your R2 Access Key ID.
- `R2_SECRET_ACCESS_KEY`: Your R2 Secret Access Key.
- `CF_ACCOUNT_ID`: Your Cloudflare Account ID.
- `BUCKET_NAME`: The name of the bucket to retrieve logs from.

## Contributions
Contributions to `cargo-r2logs` are welcome! If you have suggestions for improvement or want to contribute to the code, please feel free to create issues or submit pull requests on our [GitHub repository](https://github.com/nuts3745/cargo-r2logs).

## References
- [Cloudflare Logs Engine](https://developers.cloudflare.com/logs/r2-log-retrieval/)
- [Workers Trace Events Logpush](https://developers.cloudflare.com/workers/observability/logpush/)
- [R2](https://developers.cloudflare.com/r2/)