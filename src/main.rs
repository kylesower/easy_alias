use clap::Parser;
use std::fs::{self, File};
use std::path::Path;
use std::io::{self, Read, Write, BufRead, BufReader};
use std::env;
use std::path::PathBuf;
use std::process::Command;
//use execute::Execute;

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
                println!("Alias already in config. Would you like to overwrite it (y/n)?");
                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Invalid input.");
                    if input == "y\n" {
                        self.remove_alias();
                        self.add_alias();
                        return
                    } else if input == "n\n" {
                        return
                    } else {
                        println!("Please input a valid option (y/n).");
                    }
                }
            } else {
                self.add_alias();
            }
            return;
        } else {
            self.run_command()
        }
    }
    
    fn read_file(&self) -> Result<File, std::io::Error> {
        let mut config_dir = PathBuf::new();
        // This line may error on Windows. This program is not designed for 
        // Windows, which isn't even a real operating system.
        config_dir.push(env::home_dir().unwrap());
        config_dir.push(".config");
        config_dir.push("eaconfig");
        File::open(&config_dir)
    }

    fn read_config(&self) -> Result<(String, PathBuf), std::io::Error> {
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
        let mut start_str = self.cmd.clone();
        start_str.push(',');
        for line in config.split('\n') {
            if line.starts_with(&start_str) {
                return true
            }
        }

        return false
    }

    fn add_alias(&self) {
        let (mut config, dir) = self.read_config().unwrap();
        config.push_str(&self.cmd);
        config.push(',');
        config.push_str(&self.alias.clone().unwrap());
        config.push('\n');
        if let Ok(_) = fs::write(&dir, config) {
            println!("Command added to config at {}", &dir.to_str().unwrap());
        } else {
            println!("Failed to write to file!");
        }
    }
    
    fn remove_alias(&self) {
        let (config, dir) = self.read_config().unwrap();
        let mut start_str = self.cmd.clone();
        start_str.push(',');
        let mut new_config = config.split('\n')
            .filter(|x| !x.starts_with(&start_str))
            .fold(String::new(), |a, b| a + b + "\n");
        new_config.pop();
        if let Ok(_) = fs::write(&dir, new_config) {
            println!("Command '{}' removed from config.", &self.cmd);
        } else {
            println!("Failed to write to file!");
        }
    }

    fn run_command(self) {
        let cmd = self.get_full_cmd();
        let args = cmd.split(' ');
        println!("cmd is {}", &cmd);
        let mut prog = "";
        let mut prog_args = Vec::new();
        for (i, arg) in args.enumerate() {
            if i == 0 {
                prog = arg;
            } else {
                prog_args.push(arg);
            }
        }
        println!("Prog args are {:?}", prog_args);
        let output = Command::new(prog)
            .args(prog_args)
            .status()
            .expect("Failed to execute command.");
        if !output.success(){
            println!("{}", output);
        }
    }

    fn get_full_cmd(&self) -> String {
        let (config, _) = self.read_config().unwrap();
        let mut start_str = self.cmd.clone();
        start_str.push(',');
        for line in config.split('\n') {
            if line.starts_with(&start_str) {
                return line.split(',').nth(1).unwrap().to_string();
            }
        }

        return "".to_string()
    }
}



fn main() {
    let args = Cli::parse();
    args.process();
}
