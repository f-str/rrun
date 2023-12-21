<p align="center">
  <img src="https://raw.githubusercontent.com/f-str/rrun/main/img/rrun_icon.png" alt="RRun icon"/>
</p>

# RRun

`rrun` is a resource-friendly application launcher for the [i3wm](https://i3wm.org/), written in pure Rust. 
It provides a simple way to start applications from directly from the command line.

## Installation

_WIP_

## Getting Started

Every time you install a new application, you need to execute `rrun generate` to update the list of available commands.

### i3wm

Just add the following line to your i3 config file:

```
# Start rrun when pressing superkey + d
bindsym $mod+d exec <your-prefered-terminal e.g. kitty> --class 'launcher' rrun

# Set rrun window to floating and center it
for_window [class="^launcher$"] floating enable, border none, resize set width 25 ppt height 20 ppt, move position center
```

## CLI

### Commands


| Command  | Description                                                                             |
| -------- |-----------------------------------------------------------------------------------------|
| list     | List all collected commands which could be executed in the users default shell.         |
| generate | Regengerates the list of all collected commands. These are stored in `$HOME/.rrun/tmp`. |
| help     | Shows help message and exits.                                                           |

### Options


| Option        | Description                                       |
| ------------- | ------------------------------------------------- |
| -h, --help    | Shows help message and exits.                     |
| -v, --version | Displays information about the version and exits. |


## Current TODOs:

- [ ]  AUR Package + Automated CI build
- [ ]  finish README

## License

This project is licensed under the MIT. See the [LICENSE](LICENSE) file for details.
