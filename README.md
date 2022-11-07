# buli - Show bundesliga results on the command line

buli is a command line tool that shows results for the Fußball Bundesliga on
the command line. Currently it can show the results for the current matchday
and the curent table standings.

## Installing

To install run:

```sh
cargo install buli
```

## Usage

`buli` to show the table or `buli -d` for the current matchday.
You can also use `buli -h` for help.

## Changelog

Version 0.1.1: Move from reqwest to ureq which saves a little binary size and a
lot in build time and dependencies.
