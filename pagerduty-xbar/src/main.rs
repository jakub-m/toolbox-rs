use std::env;

use reqwest::Client;
use reqwest::Error;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pg_schedule_id = env::var("PG_SCHEDULE_ID")?;
    let pg_auth_token = env::var("PG_AUTH_TOKEN")?;
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let authorization = format!("Token token={pg_auth_token}");
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&authorization)?);
    let url = format!("https://api.pagerduty.com/schedules/{pg_schedule_id}/users");
    let res = client.get(url).headers(headers).send().await?;
    println!("{}", res.text().await?);
    Ok(())
}
