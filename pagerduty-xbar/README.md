# pagerduty-xbar

Show Pager duty status in [xbar][xbar].

[xbar]: https://github.com/matryer/xbar

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

## Installation

```sh
$ git clone git@github.com:jakub-m/toolbox-rs.git
$ cargo build --release
$ ln -s $(realpath xbar_pagerduty_plugin.sh) "$HOME/Library/Application Support/xbar/plugins/pagerduty.5m.sh"
```
