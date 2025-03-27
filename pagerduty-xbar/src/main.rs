use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
struct UsersWrapper {
    users: Vec<User>,
}

#[derive(Debug, Deserialize)]
struct User {
    name: String,
}

const GREEN_CIRCLE: char = char::from_u32(0x1F7E2).unwrap();
const DOTTED_CIRCLE: char = char::from_u32(0x25CC).unwrap();
const YELLOW_CIRCLE: char = char::from_u32(0x1F7E1).unwrap();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pg_schedule_id = env::var("PG_SCHEDULE_ID")?;
    let pg_auth_token = env::var("PG_AUTH_TOKEN")?;
    let pg_user = env::var("PG_USER")?;
    let now = Utc::now();
    let is_oncall_now = check_if_user_on_call(
        now,
        now + Duration::minutes(1),
        &pg_auth_token,
        &pg_schedule_id,
        &pg_user,
    )
    .await?;

    let is_oncall_next_24h = check_if_user_on_call(
        now,
        now + Duration::days(1),
        &pg_auth_token,
        &pg_schedule_id,
        &pg_user,
    )
    .await?;

    if is_oncall_now {
        println!("PG {}", GREEN_CIRCLE);
    } else if is_oncall_next_24h {
        println!("PG {}", YELLOW_CIRCLE);
    } else {
        println!("PG {}", DOTTED_CIRCLE);
    }

    Ok(())
}

/// https://developer.pagerduty.com/api-reference/e1ad560792567-list-users-on-call
async fn check_if_user_on_call(
    shift_from: DateTime<Utc>,
    shift_to: DateTime<Utc>,
    pg_auth_token: &str,
    pg_schedule_id: &str,
    pg_user: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let since_iso = shift_from.to_rfc3339();
    let until_iso = shift_to.to_rfc3339();
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let authorization = format!("Token token={pg_auth_token}");
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&authorization)?);
    let url = format!(
        "https://api.pagerduty.com/schedules/{pg_schedule_id}/users?since={since_iso}&until={until_iso}"
    );
    eprintln!("{url}");
    let res = client.get(url).headers(headers).send().await?;

    let response_body = res.text().await?;
    eprintln!("{response_body}");

    let parsed: UsersWrapper = serde_json::from_str(&response_body).unwrap();
    let is_oncall = parsed.users.iter().any(|user| user.name == pg_user);
    Ok(is_oncall)
}
