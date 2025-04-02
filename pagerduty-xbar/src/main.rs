use chrono::{DateTime, Duration, Local, SecondsFormat, TimeDelta, Utc};
use reqwest::Client;
use std::env;
use types::{Oncall, Oncalls};

mod types;

const GREEN_CIRCLE: char = char::from_u32(0x1F7E2).unwrap();
const YELLOW_CIRCLE: char = char::from_u32(0x1F7E1).unwrap();
const CENTER_X: char = char::from_u32(0x00D7).unwrap();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pg_schedule_id = env_var("PG_SCHEDULE_ID")?;
    let pg_auth_token = env_var("PG_AUTH_TOKEN")?;
    let pg_user_id = env_var("PG_USER_ID")?;
    let pg_icons = env_var("PG_ICONS").unwrap_or("".to_owned());
    let now = Utc::now();

    let oncalls = get_oncalls(
        now,
        now + Duration::days(14),
        &pg_auth_token,
        &pg_schedule_id,
        &pg_user_id,
    )
    .await?;

    let is_oncall_now = is_oncall(now, &oncalls);
    let is_oncall_next_24h = is_oncall(now + Duration::days(1), &oncalls);

    let (icon_on_duty, icon_incoming_on_duty, icon_not_on_duty) = get_icons(&pg_icons);

    if is_oncall_now {
        println!("{icon_on_duty}");
    } else if is_oncall_next_24h {
        println!("{icon_incoming_on_duty}");
    } else {
        println!("{icon_not_on_duty}");
    }

    println!("---");
    println!("Upcoming:");

    for oncall in &oncalls {
        let delta = oncall.start - now;

        let time_remaining = if delta <= TimeDelta::zero() {
            format!("now, {}", format_timedelta(oncall.end - now))
        } else {
            format_timedelta(delta)
        };

        println!(
            "{} - ({}) {} - {}",
            oncall.schedule.summary,
            time_remaining,
            format_readable(oncall.start),
            format_readable(oncall.end)
        );
    }

    Ok(())
}

fn is_oncall(time: DateTime<Utc>, oncalls: &[Oncall]) -> bool {
    oncalls.iter().any(|o| time >= o.start && time < o.end)
}

fn format_readable(dt: DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.into();
    local.format("%a %d %b %H:%M").to_string()
}

fn format_iso(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Secs, true)
}

/// https://developer.pagerduty.com/api-reference/3a6b910f11050-list-all-of-the-on-calls
async fn get_oncalls(
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    pg_auth_token: &str,
    pg_schedule_id: &str,
    pg_user_id: &str,
) -> Result<Vec<Oncall>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client
        .get("https://api.pagerduty.com/oncalls")
        .query(&[
            ("schedule_ids[]", pg_schedule_id),
            ("since", &format_iso(from)),
            ("until", &format_iso(to)),
        ])
        .header("Authorization", format!("Token token={pg_auth_token}"))
        .send()
        .await?;

    let oncalls: Oncalls = res.json().await?;

    let user_oncalls = oncalls
        .oncalls
        .into_iter()
        .filter(|o| o.user.id == pg_user_id)
        .collect();

    Ok(user_oncalls)
}

fn env_var(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("Missing env var: {name}"))
}

fn get_icons(icons: &str) -> (char, char, char) {
    let icons: Vec<char> = icons.chars().collect();
    (
        *icons.first().unwrap_or(&GREEN_CIRCLE),
        *icons.get(1).unwrap_or(&YELLOW_CIRCLE),
        *icons.get(2).unwrap_or(&CENTER_X),
    )
}

fn format_timedelta(delta: TimeDelta) -> String {
    if delta < TimeDelta::hours(1) {
        format!("{}m", delta.num_minutes())
    } else if delta < TimeDelta::days(1) {
        format!("{}h", delta.num_hours())
    } else {
        format!("{}d", delta.num_days())
    }
}
