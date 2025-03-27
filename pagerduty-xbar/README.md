# pagerduty-xbar

Show Pager duty status in xbar.

Requires the following environment variables to work (in `~/.config/pg_xbar.conf`)
- `PG_SCHEDULE_ID` - ID of the schedule, can be take from PagerDuty URL.
- `PG_AUTH_TOKEN` - Authentication token, generate it in PagerDuty settings.
- `PG_USER` - The full name of the user that is looked up in the PagerDuty response.

## Running

```
cargo build --relase
./xbar_pagerduty.sh
```

For **development** modify the script to use `cargo run` instead of release target.
