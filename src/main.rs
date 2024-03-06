use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

/// Parser for the config file
/// Lines starting with a '#' will be ignored and can be used as comments.
/// Keymaps need the exact structure like:
/// KeyCode-KeyCode = [ program, command ]
/// L-T = [ touch, myTestFile.txt ]
/// :parameter
///     *   file_path: path to the key.map file
/// :return
///     *   shortcut_map: map containing shortcuts and commands  
fn parse_map_file(file_path: &str) -> HashMap<Vec<String>, Vec<String>> {
    let mut shortcut_map: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    let file = File::open(file_path).expect("Couldn't read map file");
    let buffer = BufReader::new(file).lines();
    'outer: for (ci, i) in buffer.enumerate() {
        let line = match i {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{:?}", e);
                eprintln!("Cannot read line [{}] - skipping...", ci);
                continue;
            }
        };
        if !line.starts_with('#') && !line.is_empty() {
            // one line of key - value pair from the file split into key and value
            let kv_line = &line.split(" = ").collect::<Vec<&str>>();
            if kv_line.len() != 2 {
                eprintln!("Malformatted line [{}] - skipping...", ci);
                continue;
            };
            // the shortcut (key)
            let mut shortcut = kv_line[0].split('-').collect::<Vec<&str>>();
            shortcut.sort();
            let mut shortcut_kc: Vec<String> = Vec::new();
            // create keycode vector
            for kc in shortcut {
                match Keycode::from_str(kc) {
                    Ok(k) => shortcut_kc.push(k.to_string()),
                    Err(e) => {
                        eprintln!("{:?}", e);
                        eprintln!("Invalid keycode [{}] in line [{}] - skipping... ", kc, ci);
                        continue 'outer;
                    }
                };
            }
            // the command (value)
            let cmd_line = kv_line[1]
                .split(", ")
                .map(|k| k.to_string().replace("[ ", "").replace(" ]", ""))
                .collect::<Vec<String>>();
            if cmd_line.len() != 2 {
                eprintln!("Malformatted command in line [{}] - skipping...", ci);
                continue;
            };
            shortcut_map.insert(shortcut_kc, cmd_line);
        }
    }
    shortcut_map
}

fn main() {
    //  construct the file path to $HOME/.config/basalt/key.map
    let home_path = env::var_os("HOME").expect("$HOME is not defined in this environment");
    let home_path = home_path.to_str().expect("Cannt convert $HOME bath");
    let mut base_path: PathBuf = [&home_path, ".config", "basalt"].iter().collect();
    base_path.extend(&["key.map"]);
    // get keymaps
    let mapped = parse_map_file(
        base_path
            .into_os_string()
            .to_str()
            .expect("Cannot join basepath and key.map "),
    );
    // listen to keyboard inputs
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            // sort the keys so the are in the same order as when read from the config file
            let mut keys = keys.iter().map(|x| x.to_string()).collect::<Vec<String>>();
            keys.sort();
            // execute mapped command
            if let Some(command) = mapped.get(&keys) {
                match Command::new(&command[0]).arg(&command[1]).spawn() {
                    Ok(_f) => {}
                    Err(e) => {
                        eprintln!("{:?}", e)
                    }
                }
            }
        }
        prev_keys = keys;
        // sleep so it doesn't use 100% of the CPU
        thread::sleep(Duration::from_millis(50))
    }
}
