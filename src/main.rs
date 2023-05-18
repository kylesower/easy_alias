use clap::Parser;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::env;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Parser, Default)]
struct Cli {
    cmd: String,
    alias: Option<String>,
    #[clap(short, default_value_t = false)]
    remove: bool
}

impl Cli {
    fn process(self) {
        if self.remove {
            self.remove_alias();
            return;
        }
        if let Some(_) = &self.alias {
            if self.alias_exists() {
                println!("Alias already in config. Would you like to overwrite it?");
                // TODO: Overwrite aliases
            } else {
                self.add_alias();
            }
            return;
        } else {
            self.run_command()
        }
    }
    
    fn alias_exists(&self) -> bool {
        return false;
    }

    fn add_alias(&self) {
        let mut config_dir = PathBuf::new();
        // This line may error on Windows. This program is not designed for 
        // Windows, which isn't even a real operating system.
        config_dir.push(env::home_dir().unwrap());
        config_dir.push(".config");
        config_dir.push("eaconfig");
        let file = File::open(&config_dir);
        let mut file_exists = true;
        match file {
            Err(_) => file_exists = false,
            _ => (),
        }
        let mut config = String::new();
        if file_exists {
            if let Ok(_) = file.unwrap().read_to_string(&mut config){
                ();
            } else {
                println!("Failed to read config file!");
            };
        }
        config.push_str(&self.cmd);
        config.push(',');
        config.push_str(&self.alias.clone().unwrap());
        config.push('\n');
        if let Ok(_) = fs::write(config_dir, config) {
            println!("Command added to config.");
        } else {
            println!("Failed to write to file!");
        }
    }
    
    fn remove_alias(self) {

    }

    fn run_command(self) {
        let cmd = self.get_full_cmd();
    }

    fn get_full_cmd(self) {
        // TODO: abstract
        let mut config_dir = PathBuf::new();
        config_dir.push(env::home_dir().unwrap());
        config_dir.push(".config");
        config_dir.push("eaconfig");
        let file = File::open(&config_dir);
        let mut file_exists = true;
        match file {
            Err(_) => file_exists = false,
            _ => (),
        }
        let mut config = String::new();
        if file_exists {
            if let Ok(_) = file.unwrap().read_to_string(&mut config){
                ();
            } else {
                println!("Failed to read config file!");
            };
        } else {
            println!("Config file does not exist. Please add a command alias first.");
        }
        
    }
}



fn main() {
    let args = Cli::parse();
    args.process();
}
