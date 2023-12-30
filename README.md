# yambar-disk

*A disk space module for [Yambar](https://codeberg.org/dnkl/yambar) status panel.*


## Installation

First build the software:

```bash
cargo build --release
```

Then install it:

```bash
sudo cp target/release/yambar-disk /usr/bin/
```

## Usage

The `yambar-disk` program will output disk space information at regular interval.

It produces the following tags that can be used by Yambar:

| Name         | Type  | Description                |
| ------------ | ----- | -------------------------- |
| free         | int   | Free disk space in bytes   |
| used         | int   | Used disk space in bytes   |
| total        | int   | Total disk space in bytes  |
| percent_free | range | Free disk space in percent |
| percent_used | range | Used disk space in percent |


## Configuration

The `yambar-disk` command accepts two optional arguments:

| Option                       | Type   | Description                                                                                       |
| ---------------------------- | ------ | ------------------------------------------------------------------------------------------------- |
| `--path <name>`              | string | The path name towards a file or directory located on the mounted disk. By default, `"/"` is used. |
| `--poll-interval <interval>` | int    | The interval (in milliseconds) between each update. By default, `1000` is used.                   |


See also `yambar-disk --help`.

## Example

Here is a possible `config.yaml` for Yambar:

```yaml
bar:
  height: 32
  location: bottom
  background: 111111cc

  left:
    - script:
        path: /usr/bin/yambar-disk
        args: [--poll-interval, 2000, --path, /]
        content:
          - string:
              text: "[Disk] {used:gb} GB / {total:gb} GB ({percent_used}%)"
```
