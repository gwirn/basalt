# Basalt 🪨

*key**B**o**A**rd **S**hortcut m**A**pping p**L**a**T**form*

Create hotkeys to launch programs or scripts that perform a range of different tasks.

![TEST](https://github.com/gwirn/basalt/actions/workflows/rust.yml/badge.svg)
[![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](/LICENSE.md)
<a title="Code Size" target="_blank" href="https://github.com/gwirn/basalt"><img src="https://img.shields.io/github/languages/code-size/gwirn/basalt"></a>

## Installation
[Install rust and cargo](https://www.rust-lang.org/tools/install)

Run `cargo build --release` to create a binary of the program.

## Configuration
The configuration file where the keymaps are defined must be saved at
```sh
$HOME/.config/basalt/key.map
```
The file ignores `#` as comments. You can set shortcuts as `KeyCode-KeyCode = [ command, argument ]`. You can find example scripts and a key.map
 [here](https://github.com/gwirn/basalt/tree/master/examples). Malformatted lines will be ignored. You can find the keycodes to use [here](https://github.com/ostrosco/device_query/blob/b5ba13089c611b1deb3a6804e1f3032301d0fd5d/src/keymap.rs#L9).

## Usage
If new shortcuts are added, Basalt needs to be restarted for them to take effect. Here's an example of how to run Basalt in the background while still saving its PID to kill its process if needed:
```bash
#!/bin/bash

if ! pgrep "basalt" > /dev/null ; then
    "/PATH/TO/BASALT/basalt/target/release/basalt" &
    sleep_pid=$!
    echo $sleep_pid > "$HOME/.config/basalt/basalt.pid"
else
    echo "Basalt is already running"
fi
```
If this is added to a bash file and executed, it will launch Basalt in the background and save the process ID in 'basalt.pid'.

A possible bash script to stop basalt would then be:
```bash
#!/bin/bash

basalt_pid=$(cat "$HOME/.config/basalt/basalt.pid")

if pgrep "basalt" > /dev/null ; then
    kill $basalt_pid
else
    echo "Basalt is not running"
fi

```
## OS
Tested on macOS Sonoma and Ubuntu 22.04
