# Basalt 🪨

*key**B**o**A**rd **S**hortcut m**A**pping p**L**a**T**form*

Create hotkeys to launch programs or scripts that perform a range of different tasks.

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

## KeyMaps 
Please note that on MacOS, the `ALT` key is recognised as `META`, while on Linux it is registered as `LAlt` or `RAlt`.
## Usage
If new shortcuts are added, Basalt needs to be restarted for them to take effect. Here's an example of how to run Basalt in the background while still saving its PID to kill its process if needed:
```bash
#!/bin/bash

/PATH/TO/basalt &
basalt_pid=$!
echo $basalt_pid >> basalt.pid
```
If this is added to a bash file and executed, it will launch Basalt in the background and save the process ID in 'basalt.pid'. 
## OS
Tested on macOS Sonoma and Ubuntu 22.04
