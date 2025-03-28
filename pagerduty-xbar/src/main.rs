use chrono::{DateTime, Duration, SecondsFormat, Utc};
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
const YELLOW_CIRCLE: char = char::from_u32(0x1F7E1).unwrap();
const CENTER_X: char = char::from_u32(0x00D7).unwrap();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pg_schedule_id = env_var("PG_SCHEDULE_ID")?;
    let pg_auth_token = env_var("PG_AUTH_TOKEN")?;
    let pg_user = env_var("PG_USER")?;
    let pg_icons = env_var("PG_ICONS").unwrap_or("".to_owned());
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

    let (icon_on_duty, icon_incoming_on_duty, icon_not_on_duty) = get_icons(&pg_icons);

    if is_oncall_now {
        println!("pg{icon_on_duty}");
    } else if is_oncall_next_24h {
        println!("pg{icon_incoming_on_duty}");
    } else {
        println!("pg{icon_not_on_duty}");
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
    let since_iso = shift_from.to_rfc3339_opts(SecondsFormat::Secs, true);
    let until_iso = shift_to.to_rfc3339_opts(SecondsFormat::Secs, true);
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

fn env_var(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("Missing env var: {name}"))
}

fn get_icons(icons: &str) -> (char, char, char) {
    let icons: Vec<char> = icons.chars().collect();
    (
        *icons.get(0).unwrap_or(&GREEN_CIRCLE),
        *icons.get(1).unwrap_or(&YELLOW_CIRCLE),
        *icons.get(2).unwrap_or(&CENTER_X),
    )
}
