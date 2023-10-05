## About

- The command-line utility __mem (Memorize)__ is used to store and run memorized command-lines. The basic objective of this command is to attempt to memorize long, difficult-to-remember command lines, store them in a specific location, and then execute the desired command that was memorized.

## Basic usage

```bash
Usage: mem [ALIAS] [X] [COMMAND]

Commands:
  add   Adding and memorize command
  del   Delete the specific memorized command
  set   Update the specific memorized command
  use   Execute the target memorized command by its alias
  list  Show a list of memorized commands and its alias
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [ALIAS]  Alias for `x` usage
  [X]      Executing a memorized command

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Adding execute only command

```bash
mem add --command "nmcli device wifi list" --alias wifis

```

#### Run a specific command

```bash
mem use wifis

# For short
mem wifis

```

