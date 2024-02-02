use reqwest::Client;

use crate::config::Env;

pub struct ApiEnv {
    pub cf_api_key: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
}

impl Env for ApiEnv {
    fn new() -> Result<Self, String> {
        let mut error_messages = Vec::<String>::new();

        let cf_api_key = Self::get_env_var_or_default("CLOUDFLARE_API_KEY", &mut error_messages);
        let r2_access_key_id =
            Self::get_env_var_or_default("R2_ACCESS_KEY_ID", &mut error_messages);
        let r2_secret_access_key =
            Self::get_env_var_or_default("R2_SECRET_ACCESS_KEY", &mut error_messages);

        if !error_messages.is_empty() {
            return Err(error_messages.join("\n"));
        }

        Ok(Self {
            cf_api_key,
            r2_access_key_id,
            r2_secret_access_key,
        })
    }
}

pub async fn fetch_logs(
    client: &Client,
    endpoint: &str,
    cf_api_key: &str,
    r2_access_key_id: &str,
    r2_secret_access_key: &str,
) -> Result<String, reqwest::Error> {
    let res = client
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", cf_api_key))
        .header("R2-Access-Key-Id", r2_access_key_id)
        .header("R2-Secret-Access-Key", r2_secret_access_key)
        .send()
        .await?;

    if !res.status().is_success() {
        let status_code = res.status();
        let error_detail = res
            .text()
            .await
            .unwrap_or_else(|_| "Error Undifined".to_string());
        eprintln!("Failed to retrieve logs: {:?}", status_code);
        eprintln!("Error Detail: {}", error_detail);
        return Ok("".to_string());
    }
    let text = res.text().await?;
    if text.is_empty() {
        eprintln!("No logs found");
        eprintln!("Please check time range");
        return Ok("".to_string());
    }
    Ok(text)
}

#[cfg(test)]
mod reqwest_tests {
    use chrono::TimeZone;
    use chrono::Utc;
    use mockito::Matcher;

    use super::*;
    use crate::commands::Commands;
    use crate::ParsedArgs;

    #[tokio::test]
    async fn test_fetch_logs() {
        let mut server = mockito::Server::new_async().await;
        let data = r#"
        {
            "Event": {
                "RayID": "",
                "Request": {
                    "URL": "",
                    "Method": "GET"
                },
                "Response": {
                    "Status": 200
                }
            },
            "EventTimestampMs": 1704985180778,
            "EventType": "fetch",
            "Exceptions": [],
            "Logs": [
                {
                    "Level": "log",
                    "Message": [
                        ""
                    ],
                    "TimestampMs": 1704985180778
                },
                {
                    "Level": "log",
                    "Message": [
                        ""
                    ],
                    "TimestampMs": 1704985181064
                }
            ],
            "Outcome": "ok",
            "ScriptName": "",
            "ScriptTags": []
        }
        "#;
        let mock = server
            .mock("GET", "/")
            .match_header("Authorization", "Bearer cf_api_key")
            .match_header("R2-Access-Key-Id", "r2_access_key_id")
            .match_header("R2-Secret-Access-Key", "r2_secret_access_key")
            .with_body(data)
            .create_async()
            .await;
        let client = Client::new();
        let endpoint = server.url();
        let text = fetch_logs(
            &client,
            &endpoint,
            "cf_api_key",
            "r2_access_key_id",
            "r2_secret_access_key",
        )
        .await
        .unwrap();
        mock.assert();
        assert!(!text.is_empty());
        assert_eq!(text, data);
    }

    #[tokio::test]
    async fn test_fetch_logs_with_invalid_endpoint() {
        let mut server = mockito::Server::new_async().await;
        let data = r#"
        {
            "Event": {
                "RayID": "",
                "Request": {
                    "URL": "",
                    "Method": "GET"
                },
                "Response": {
                    "Status": 200
                }
            },
            "EventTimestampMs": 1704985180778,
            "EventType": "fetch",
            "Exceptions": [],
            "Logs": [
                {
                    "Level": "log",
                    "Message": [
                        ""
                    ],
                    "TimestampMs": 1704985180778
                },
                {
                    "Level": "log",
                    "Message": [
                        ""
                    ],
                    "TimestampMs": 1704985181064
                }
            ],
            "Outcome": "ok",
            "ScriptName": "",
            "ScriptTags": []
        }
        "#;
        let mock = server
            .mock("GET", "/")
            .match_header("Authorization", "Bearer cf_api_key")
            .match_header("R2-Access-Key-Id", "r2_access_key_id")
            .match_header("R2-Secret-Access-Key", "r2_secret_access_key")
            .with_body(data)
            .create_async()
            .await;
        let client = Client::new();
        let args = ParsedArgs {
            start_time: Utc
                .with_ymd_and_hms(2024, 1, 11, 15, 5, 0)
                .unwrap()
                .to_string(),
            end_time: Utc
                .with_ymd_and_hms(2024, 1, 11, 15, 10, 0)
                .unwrap()
                .to_string(),
            verbose: false,
            commands: Some(Commands::Retrieve),
        };
        let endpoint = server.url()
            + "/invalid_endpoint"
            + format!("?start={}", args.start_time).as_str()
            + format!("&end={}", args.end_time).as_str()
            + format!("&bucket={}", "bucket_name").as_str()
            + format!("&prefix={}", "{DATE}").as_str();
        let text = fetch_logs(
            &client,
            &endpoint,
            "cf_api_key",
            "r2_access_key_id",
            "r2_secret_access_key",
        )
        .await
        .unwrap();

        assert!(!mock.matched());
        assert!(text.is_empty());
        assert_ne!(text, data);
    }

    #[tokio::test]
    async fn test_fetch_logs_with_invalid_cf_api_key() {
        let mut server = mockito::Server::new_async().await;
        let data = r#"
        {
            "Event": {
                "RayID": "",
                "Request": {
                    "URL": "",
                    "Method": "GET"
                },
                "Response": {
                    "Status": 200
                }
            },
            "EventTimestampMs": 1704985180778,
            "EventType": "fetch",
            "Exceptions": [],
            "Logs": [
                {
                    "Level": "log",
                    "Message": [
                        ""
                    ],
                    "TimestampMs": 1704985180778
                },
                {
                    "Level": "log",
                    "Message": [
                        ""
                    ],
                    "TimestampMs": 1704985181064
                }
            ],
            "Outcome": "ok",
            "ScriptName": "",
            "ScriptTags": []
        }
        "#;
        let mock = server
            .mock("GET", "/invalid_cf_api_key")
            .match_header("Authorization", "Bearer cf_api_key")
            .match_header("R2-Access-Key-Id", "r2_access_key_id")
            .match_header("R2-Secret-Access-Key", "r2_secret_access_key")
            .with_body(data)
            .create_async()
            .await;
        let client = Client::new();
        let args = ParsedArgs {
            start_time: Utc
                .with_ymd_and_hms(2024, 1, 11, 15, 5, 0)
                .unwrap()
                .to_string(),
            end_time: Utc
                .with_ymd_and_hms(2024, 1, 11, 15, 10, 0)
                .unwrap()
                .to_string(),
            verbose: false,
            commands: Some(Commands::Retrieve),
        };
        let endpoint = server.url()
            + "/invalid_cf_api_key"
            + format!("?start={}", args.start_time).as_str()
            + format!("&end={}", args.end_time).as_str()
            + format!("&bucket={}", "bucket_name").as_str()
            + format!("&prefix={}", "{DATE}").as_str();
        let text = fetch_logs(
            &client,
            &endpoint,
            "invalid_cf_api_key",
            "r2_access_key_id",
            "r2_secret_access_key",
        )
        .await
        .unwrap();

        assert!(!mock.matched());
        assert!(text.is_empty());
        assert_ne!(text, data);
    }

    #[tokio::test]
    async fn test_fetch_logs_with_invalid_r2_access_key_id() {
        let mut server = mockito::Server::new_async().await;
        let data = r#"
        {
            "Event": {
                "RayID": "",
                "Request": {
                    "URL": "",
                    "Method": "GET"
                },
                "Response": {
                    "Status": 200
                }
            },
            "EventTimestampMs": 1704985180778,
            "EventType": "fetch",
            "Exceptions": [],
            "Logs": [
                {
                    "Level": "log",
                    "Message": [
                        ""
                    ],
                    "TimestampMs": 1704985180778
                },
                {
                    "Level": "log",
                    "Message": [
                        ""
                    ],
                    "TimestampMs": 1704985181064
                }
            ],
            "Outcome": "ok",
            "ScriptName": "",
            "ScriptTags": []
        }
        "#;
        let mock = server
            .mock("GET", "/invalid_r2_access_key_id")
            .match_header("Authorization", "Bearer cf_api_key")
            .match_header("R2-Access-Key-Id", Matcher::Missing)
            .match_header("R2-Secret-Access-Key", "r2_secret_access_key")
            .with_body(data)
            .create_async()
            .await;
        let client = Client::new();
        let args = ParsedArgs {
            start_time: Utc
                .with_ymd_and_hms(2024, 1, 11, 15, 5, 0)
                .unwrap()
                .to_string(),
            end_time: Utc
                .with_ymd_and_hms(2024, 1, 11, 15, 10, 0)
                .unwrap()
                .to_string(),
            verbose: false,
            commands: Some(Commands::Retrieve),
        };
        let endpoint = server.url()
            + "/invalid_r2_access_key_id"
            + format!("?start={}", args.start_time).as_str()
            + format!("&end={}", args.end_time).as_str()
            + format!("&bucket={}", "bucket_name").as_str()
            + format!("&prefix={}", "{DATE}").as_str();
        let text = fetch_logs(
            &client,
            &endpoint,
            "cf_api_key",
            "invalid_r2_access_key_id",
            "r2_secret_access_key",
        )
        .await
        .unwrap();

        assert!(!mock.matched());
        assert!(text.is_empty());
        assert_ne!(text, data);
    }
}
