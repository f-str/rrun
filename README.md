<p align="center">
  <img src="https://raw.githubusercontent.com/f-str/rrun/main/img/rrun_icon.png" alt="RRun icon"/>
</p>

# RRun

`rrun` is a resource-friendly application and command launcher for the [i3wm](https://i3wm.org/), [sway](https://swaywm.org/) and [Hyprland](https://hyprland.org/), written in pure Rust. 
It provides a simple way to start applications directly from the command line or from a script.

## Installation

_WIP_

## Getting Started

### First encounter
When you first open `rrun`, no start commands are stored, but you can start directly to type commands/application names and rrun will execute them and remember them for the next time. 
You could also enter commands with arguments or muliple commands.
The default way of remembering the command/application is the full thing you entered.
But you could rename that to a shorter version using the `edit-name` `rrun` subcommand.


### Launching commands in a terminal
If you want to launch an program as a command in your terminal, you can use `@ <command>` and rrun will try to launch an terminal with this command. Per default `rrun` tries to launch `kitty`, but this can be reconfigured with your terminal emulator of choice in the configuration file (located at `~/.config/rrun/config`) (you may also need to enter arguments in order to launch your favorite terminal emulator with a command). 

### Sorting
After each reboot, on the first execution of `rrun` the order of the entries gets reordered, so that the most invoked applications is actually stored on top.

### i3wm / sway

Just add the following line to your config file:

```
# Start rrun when pressing superkey + d
bindsym $mod+d exec <your-prefered-terminal e.g. kitty> --class 'launcher' rrun

# Set rrun window to floating and center it
for_window [class="^launcher$"] floating enable, border none, resize set width 25 ppt height 20 ppt, move position center
```

### Hyprland

Just add the following lines to you hyprland config file:

- For executing `rrun`:
```
$launcher = <your-prefered-terminal e.g. kitty> --class 'launcher' rrun
```
(Note, that your terminal may requiere more arguments in order to start directly with the program)

- Window rules in order to make `rrun` pretty:
```
windowrulev2 = float, class:^(launcher)$  # Set the launcher window to float
windowrulev2 = size 700ppt 250ppt, class:^(launcher)$  # Set the size (you may adapt it to your liking)
windowrulev2 = center, class:^(launcher)$  # Center it to your current monitor
windowrulev2 = stayfocused, class:^(launcher)$  # Force the focus on the launcher window
```


## CLI

### Commands


| Command        | Description                                                                             |
| -------------- |-----------------------------------------------------------------------------------------|
| add-command    | Option for adding a new command/application                                             |
| edit-name      | Option for renaming the name of a command/application                                   |
| edit-command   | Option for changing the command of a command/application                                |
| delete-command | Option for deleting a command/application                                               |
| statistics     | Print some invocation statistics                                                        |
| exec           | Option for executing commands/applications. Useful during scripting.                    |

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
