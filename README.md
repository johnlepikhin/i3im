
# About the Project

The utility extends the standard functionality of the i3 and Sway window managers.

Main features include:

- Workspace groups. Useful when working on multiple projects simultaneously, allowing you to extend the standard
  workspace functionality ($mod+1, $mod+2, … $mod+9) to multiple groups, organized by project. You can configure it so
  that $mod+N switches workspaces only within the current group, while also enabling the creation of additional groups
  and switching between them.
- Event handler. A simple way to define event-handling logic via a configuration file.


# Installation

Currently installation is only possible directly from the source.

Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install the project:

```bash
cargo install i3im
```

# Getting started

The utility has a robust command-line interface. Don’t hesitate to call it with `help`.

To start, you need to create a configuration file. You can do this with the following command:

```bash
i3im config generate > $HOME/.config/i3im.yaml
```

# Workspace groups

Add the following to your i3 configuration:

```ini
bindsym $mod+1 exec i3im focus workspace 1
bindsym $mod+2 exec i3im focus workspace 2
bindsym $mod+3 exec i3im focus workspace 3
bindsym $mod+4 exec i3im focus workspace 4
bindsym $mod+5 exec i3im focus workspace 5
bindsym $mod+6 exec i3im focus workspace 6
bindsym $mod+7 exec i3im focus workspace 7
bindsym $mod+8 exec i3im focus workspace 8
bindsym $mod+9 exec i3im focus workspace 9
bindsym $mod+0 exec i3im focus workspace 10

bindsym $mod+Shift+s exec i3im focus wm-workspace "`i3im list wm-workspaces | rofi -dmenu -p 'Switch to workspace'`"
```

Note that the last command uses rofi.

Now, with the $mod+Shift+s key combination, you can create a new workspace group. Within the group, switching between
workspaces works with the usual keys. You can return to the default group (which has no name) using the same key
combination — $mod+Shift+s. Similarly, you can create multiple workspace groups if needed.


# Event handler

i3im can operate in event-handler mode. To do this, run:

```bash
i3im listen
```

By default, the list of handlers is empty. They need to be described in the configuration file under the
`window_event_handlers` and `workspace_event_handlers` sections. Each entry consists of two elements:

- `condition_list`: A list of conditions that must be met for the handler to activate.
- `action`: The action to be performed when the conditions are met.

You can see the full list of available conditions and actions by calling the configuration help:

```bash
i3im config documentation
```

## Example: Increasing brightness for fullscreen windows

Add the following to your configuration:

```yaml
window_event_handlers:
  # Event handler for entering fullscreen mode
  - condition_list:
      # Event type: fullscreen mode change
      - EventType: FullscreenMode
      # New fullscreen mode status: activated
      - NodeFullscreenMode: Fullscreen
    action:
      # Execute a shell command
      ShellCommand:
        command: ~/.config/i3/scripts/brightness_autoset.sh 100

  # Event handler for exiting fullscreen mode
  - condition_list:
      # Event type: fullscreen mode change
      - EventType: FullscreenMode
      # New fullscreen mode status: not activated
      - NodeFullscreenMode: None
    action:
      # Execute a shell command
      ShellCommand:
        command: ~/.config/i3/scripts/brightness_autoset.sh
```

Content of the script `~/.config/i3/scripts/brightness_autoset.sh`:

```bash
#! /bin/sh

DEFAULT_BRIGHTNESS='20'
STATE_FILE="$XDG_STATE_HOME/brightness_autoset"

BRIGHTNESS="$1"
if [ -z "$BRIGHTNESS" ]; then
    BRIGHTNESS="$(cat "$STATE_FILE" 2>/dev/null || echo "$DEFAULT_BRIGHTNESS")"
    brightnessctl set "${BRIGHTNESS}%"
    exit 0
fi

brightnessctl brightnessctl -m | cut -d, -f4 | tr -d % > "$STATE_FILE"

brightnessctl set "${BRIGHTNESS}%"
```

## Example: Automatically closing Jitsi sharing notification windows

Many find it annoying that the Jitsi video conferencing system automatically shows a notification window about sharing.
You can automatically close it using the following workaround:

```yaml
window_event_handlers:
  - condition_list:
      # Event type: creation of a new window
      - EventType: New
      # The window title must match the regular expression
      - Title:
          Regex:
            regex: 'jitsi.*is sharing'
    action:
      # Execute a shell command
      ShellCommand:
        command: xdotool windowclose "$I3IM_CONTAINER_WINDOW_ID"
```

## Example: Retrieving environment variables

In the example above, you may have noticed the environment variable `$I3IM_CONTAINER_WINDOW_ID`. To retrieve the full
list of variables, you can attach an empty condition to an event:

```yaml
window_event_handlers:
  - condition_list: []
    action:
      # Execute a shell command
      ShellCommand:
        command: env | grep I3IM_ > /tmp/i3im-environment-list.txt
```

You can do the same for workspace events.

# Debugging

By default, the utility writes logs to syslog. You can force it to write logs to the console by running it with the
following environment variable:

```bash
RUST_LOG=debug i3im listen

# The same, with logging level "info":
RUST_LOG=info i3im listen
```
