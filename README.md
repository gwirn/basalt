# Basalt 🪨

*key**B**o**A**rd **S**hortcut m**A**pping p**L**a**T**form*

Create hotkeys to launch programs or scripts that perform a range of different task.

## Configuration
The config file in which the keymaps are defined will be searched in
```sh
$HOME/.config/basalt/key.map
```
In this file `#` are ignored and can be used as comments. The shortcuts need to be set as `KeyCode-KeyCode = [ command, argument ]`. Some example scripts and an example key.map can be found [here](https://github.com/gwirn/basalt/tree/master/examples). Malformatted lines will be ignored.
