## atomberg-cli

Control Atomberg smart fans over the local network from the terminal.
Written in rust with pure `human touch`.

## Why This Exists

This CLI was built to avoid cloud API rate limits in day-to-day usage. The cloud path is limited to 100 calls per day and 5 calls per second, which can be restrictive for frequent automation and local control workflows.  
`atomberg-cli` talks to devices directly on the local network to provide a faster and more reliable control path.

## Features

- Discover devices from local UDP beacons.
- Send fan commands to a single device alias or a group.
- Manage aliases (`set`, `rename`, `remove`, `list`, `show`).
- Manage groups (`create`, `rename`, `delete`, `add`, `remove`, `list`, `show`).
- Supports Aris Starlight light controls:
- `--set-brightness` (10 to 100)
- `--color` (`warm`, `cool`, `daylight`)

## Build

```bash
cargo build --release
```

Binary name:

```bash
./target/release/atomberg
```

Or run directly with Cargo:

```bash
cargo run -- <command>
```

## Configuration

By default, config is stored at:

```text
~/.atomberg/config
```

Override path with:

```bash
atomberg --config /path/to/config.toml <command>
```

If the config file does not exist, it is created automatically with defaults:

- `becon_port = 5625`
- `command_port = 5600`

Example config:

```toml
[network]
becon_port = 5625
command_port = 5600

[devices]

[groups]
```

## Command Overview

```text
atomberg discover
atomberg send
atomberg alias <subcommand>
atomberg group <subcommand>
```

### Discover

Listen for device beacons and optionally save discovered devices.

```bash
atomberg discover --interval 5
atomberg discover --interval 5 --save
```

### Send

Send requires:

- Exactly one target: `--alias` or `--group`
- At least one action:
- `--power on|off`
- `--speed <1..6>`
- `--sleep on|off`
- `--timer <0..4>`
- `--led on|off`
- `--set-brightness <10..100>`
- `--color warm|cool|daylight`

Examples:

```bash
# Power on a single fan
atomberg send --alias bedroom --power on

# Set speed for a group
atomberg send --group living-room --speed 4

# Aris Starlight brightness and color mode
atomberg send --alias starlight --set-brightness 40
atomberg send --alias starlight --color warm
atomberg send --alias starlight --color daylight
```

Notes:

- Timer values: `0=off`, `1=1hr`, `2=2hr`, `3=3hr`, `4=6hr`
- `send` waits for matching status packets and prints `Timeout waiting for status` if confirmation is not observed in time.

### Alias Management

```bash
atomberg alias set --device-id <DEVICE_ID> --alias <ALIAS>
atomberg alias rename --old <OLD_ALIAS> --new <NEW_ALIAS>
atomberg alias remove --alias <ALIAS>
atomberg alias list
atomberg alias show --alias <ALIAS>
atomberg alias show --device-id <DEVICE_ID>
```

### Group Management

```bash
atomberg group create --name <GROUP>
atomberg group rename --old <OLD> --new <NEW>
atomberg group delete --name <GROUP>

atomberg group add --name <GROUP> --alias <ALIAS>
atomberg group add --name <GROUP> --device-id <DEVICE_ID>

atomberg group remove --name <GROUP> --alias <ALIAS>
atomberg group remove --name <GROUP> --device-id <DEVICE_ID>

atomberg group list
atomberg group show --name <GROUP>
```

## Protocol Mapping (Implemented)

### Outgoing command payloads

- Brightness:

```json
{"brightness": 10}
```

- Color mode:

```json
{"light_mode": "warm"}
```

### Status decode for light mode

From `state` (first numeric field of status string):

- `cool = (state & 0x08) > 0`
- `warm = (state & 0x8000) > 0`

Mapping:

- `cool=true`, `warm=false` => `cool`
- `cool=false`, `warm=true` => `warm`
- `cool=true`, `warm=true` => `daylight`

## Contributing

Forks and pull requests are welcome.
