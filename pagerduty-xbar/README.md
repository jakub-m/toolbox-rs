# pagerduty-xbar

Show Pager duty status in xbar.

Requires the following environment variables to work (in `~/.config/pg_xbar.conf`)
- `PG_SCHEDULE_ID` - ID of the schedule, can be take from PagerDuty URL.
- `PG_AUTH_TOKEN` - Authentication token, generate it in PagerDuty settings.
- `PG_USER_ID` - your user ID, available in your profile URL.
- `PG_ICONS` - Optional set of there icons: on-duty, tomorrow-on-duty, not-on-duty.

## Running

```
cargo build --relase
./xbar_pagerduty.sh
```

For **development** (when run from within the git repository) the script uses
`cargo run` instead of release target.
