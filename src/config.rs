use std::env;

/// ## Environment Variables
/// - `CLOUDFLARE_API_KEY`: Cloudflare API key
/// - `R2_ACCESS_KEY_ID`: R2 Access Key ID
/// - `R2_SECRET_ACCESS_KEY`: R2 Secret Access Key
/// - `CLOUDFLARE_ACCOUNT_ID`: Cloudflare Account ID
/// - `BUCKET_NAME`: Bucket name
pub struct UrlEnv {
    pub cf_account_id: String,
    pub bucket_name: String,
}

pub trait Env {
    fn get_env() -> Self
    where
        Self: std::marker::Sized,
    {
        match Self::new() {
            Ok(env) => env,
            Err(e) => {
                eprintln!("{}", e);
                println!();
                eprintln!("Please set environment variables");
                std::process::exit(1);
            }
        }
    }
    fn new() -> Result<Self, String>
    where
        Self: Sized;
    fn get_env_var_or_default(var_name: &str, error_vec: &mut Vec<String>) -> String {
        env::var(var_name).unwrap_or_else(|_| {
            error_vec.push(format!("{} is not set", var_name));
            "".to_string()
        })
    }
}

impl Env for UrlEnv {
    fn new() -> Result<Self, String> {
        let mut error_messages = Vec::<String>::new();

        let cf_account_id = Self::get_env_var_or_default("CLOUDFLARE_ACCOUNT_ID", &mut error_messages);
        let bucket_name = Self::get_env_var_or_default("BUCKET_NAME", &mut error_messages);

        if !error_messages.is_empty() {
            return Err(error_messages.join("\n"));
        }

        Ok(Self {
            cf_account_id,
            bucket_name,
        })
    }
}

#[cfg(test)]
mod env_tests {
    use super::*;

    #[test]
    fn test_get_env_var_or_default() {
        let mut error_messages = Vec::<String>::new();
        let var_name = "TEST_VAR";
        let var_value = "test_value";
        let var = UrlEnv::get_env_var_or_default(var_name, &mut error_messages);
        assert_eq!(var, "");
        assert_eq!(error_messages.len(), 1);
        assert_eq!(error_messages[0], format!("{} is not set", var_name));

        env::set_var(var_name, var_value);
        let var = UrlEnv::get_env_var_or_default(var_name, &mut error_messages);
        assert_eq!(var, var_value);
        assert_eq!(error_messages.len(), 1);
    }
}
