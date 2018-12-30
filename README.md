# timerr

[![Build Status](https://travis-ci.org/prikhi/timerr.svg?branch=master)](https://travis-ci.org/prikhi/timerr)

Timerr is a simple CLI timer that sends notifications when the timer is up.

First build and install the application:

```sh
rustup default stable
cargo build
cargo install --path .
export PATH="${HOME}/.cargo/bin/:${PATH}"
```

Then you can run `timerr`, either with a minute duration, or specific time to
go off:

```sh
# Send notification in 45 minutes
timerr 45 "Laundry is done"
# Send notification at 3:45pm
timerr 15:45 "Meeting in 15 minutes"
```

Note that quoting the notification title is not necessary:

```sh
timerr 15 Grab pizza from oven
```

Make sure you've got a notification daemon running!


## TODO

* Support duration suffixes. E.g., `30s`, `1.5hr`, `20m`
* Support am/pm times
* Optional body text, icon, & sound via CLI flags
* Default icon & sound via config file


## License

GPL-3.0 or newer
