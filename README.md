# r2logs
Simple CLI tool for retrieving logs from Cloudflare Logs Engine.

## Installation 🛠️
Before installing `r2logs`, make sure Rust and Cargo are set up on your system. [Install Rust and Cargo here](https://www.rust-lang.org/tools/install).

Install `r2logs` with:
```zsh
$ cargo install r2logs --path .
```

## Usage 🔍
Retrieve logs within a specified time range using `r2logs`. Time format: `YYYY-MM-DDTHH:MM:SSZ`.

```zsh
$ r2logs [OPTIONS] [START_TIME] [END_TIME]
$ r2logs # retrieve logs from 5 minutes ago to now
$ r2logs --help # print help
$ r2logs list # list relevant R2 objects containing logs
```
## Examples 📝

Specific time range, verbose output:
  ```zsh
  $ r2logs -v 2024-01-11T15:00:00Z 2024-01-11T15:01:00Z
  ```
Pretty print JSON output with [jq](https://github.com/jqlang/jq) and
Fuzzy search logs with [fzf](https://github.com/junegunn/fzf)
  ```zsh
  $ r2logs | jq . | fzf
  ```

List relevant R2 objects containing logs matching the provided query parameters:
  ```zsh
  $ r2logs list
  $ r2logs 2024-01-11T15:00:00Z 2024-01-11T15:01:00Z list
  ```

## Environment Variables 🌐
Set up these variables before using `r2logs`:

- `CF_API_KEY`: Your Cloudflare API key.
- `R2_ACCESS_KEY_ID`: Your R2 Access Key ID.
- `R2_SECRET_ACCESS_KEY`: Your R2 Secret Access Key.
- `CF_ACCOUNT_ID`: Your Cloudflare Account ID.
- `BUCKET_NAME`: Name of the bucket for log retrieval.

## Contributing 👐
Your contributions to `r2logs` are highly appreciated! If you've got ideas for improvements or wish to contribute code, please feel free to open issues or submit PRs on our [GitHub repository](https://github.com/nuts3745/r2logs).

## References 📚
- [Cloudflare Logs Engine](https://developers.cloudflare.com/logs/r2-log-retrieval/)
- [Workers Trace Events Logpush](https://developers.cloudflare.com/workers/observability/logpush/)
- [Cloudflare R2](https://developers.cloudflare.com/r2/)