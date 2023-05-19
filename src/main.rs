use clap::Parser;

use std::fs::{self, File};
use std::io::{self, Read};
use std::env;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Parser, Default)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Alias you want to use with easy_alias.
    alias: Option<String>,

    /// Bash script assigned to alias, enclosed in quotes if it contains spaces.
    cmd: Option<String>,

    /// Remove provided alias if flag is present
    #[arg(short, default_value_t = false)]
    remove: bool,

    /// List aliases
    #[arg(short, default_value_t = false)]
    list: bool,
}

impl Cli {
    fn process(self) {
        if self.remove {
            match self.alias {
                Some(_) => self.remove_alias(),
                _ => println!("Error! Please provide an alias to remove."),
            } 
            return;
        }
        if self.list {
            self.list_aliases();
            return;
        }
        if let Some(_) = &self.cmd {
            if self.alias_exists() {
                println!("Alias already in config. Would you like to overwrite it (y/n)?");
                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Invalid input.");
                    if input.to_lowercase() == "y\n" {
                        self.remove_alias();
                        self.add_alias();
                        return
                    } else if input.to_lowercase() == "n\n" {
                        return
                    } else {
                        println!("Please input a valid option (y/n).");
                    }
                }
            } else {
                self.add_alias();
            }
            return;
        } else if let Some(_) = self.alias {
            self.run_command()
        } else {
            println!("Invalid input. Try using the --help flag.");
        }
    }

    fn list_aliases(&self) {
        let (config, dir) = self.read_config().unwrap();
        println!("Aliases stored at {}:\n", &dir.to_str().unwrap());
        println!("{}", &config);
    }

    fn read_config(&self) -> Result<(String, PathBuf), std::io::Error> {
        let mut config_dir = PathBuf::new();
        // This line may error on Windows. This is a feature, not a bug.
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
            return Ok(("".to_string(), config_dir));
        }
        Ok((config, config_dir))

    }

    fn alias_exists(&self) -> bool {
        let (config, _) = self.read_config().unwrap();
        if config == "" {
            return false;
        }
        let mut start_str = self.alias.clone().unwrap();
        start_str.push_str("::");
        for line in config.split('\n') {
            if line.starts_with(&start_str) {
                return true
            }
        }

        return false
    }

    fn add_alias(&self) {
        let (mut config, dir) = self.read_config().unwrap();
        config.push_str(&self.alias.clone().unwrap());
        config.push_str("::");
        config.push_str(&self.cmd.clone().unwrap());
        config.push('\n');
        if let Ok(_) = fs::write(&dir, config) {
            println!("Command added to config at {}", &dir.to_str().unwrap());
        } else {
            println!("Failed to write to file!");
        }
    }
    
    fn remove_alias(&self) {
        let (config, dir) = self.read_config().unwrap();
        let mut start_str = self.alias.clone().unwrap();
        start_str.push_str("::");
        let mut new_config = config.split('\n')
            .filter(|x| !x.starts_with(&start_str))
            .fold(String::new(), |a, b| a + b + "\n");
        new_config.pop();
        if let Ok(_) = fs::write(&dir, new_config) {
            println!("Alias '{}' removed from config.", &self.alias.clone().unwrap());
        } else {
            println!("Failed to write to file!");
        }
    }

    fn run_command(&self) {
        let cmd = self.get_full_cmd();
        let mut cmd_quote = "\"".to_string();
        cmd_quote.push_str(&cmd);
        cmd_quote.push('"');
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()
            .expect("Failed to execute command.");
        if !output.success() {
            println!("Error! {}", output);
        }
    }

    fn get_full_cmd(&self) -> String {
        let (config, _) = self.read_config().unwrap();
        let mut start_str = self.alias.clone().unwrap();
        start_str.push_str("::");
        for line in config.split('\n') {
            if line.starts_with(&start_str) {
                return line.split("::").nth(1).unwrap().to_string();
            }
        }
        println!("\nError! Alias not found.");
        self.list_aliases();
        return "".to_string()
    }
}

fn main() {
    let args = Cli::parse();
    args.process();
}
