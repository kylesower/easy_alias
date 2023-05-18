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
        let mut args = cmd.split(' ');
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
        //let output = Command::new(args.nth(0).unwrap()).output().expect("Command failed to execute");
        //io::stdout().write_all(&output.stdout).unwrap();
        //io::stderr().write_all(&output.stderr).unwrap();
        //let output = Command::new(args.nth(0).unwrap()).status().expect("Failed to execute command.");
        let output = Command::new(prog)
            .args(prog_args)
            .status()
            .expect("Failed to execute command.");
        //println!("{}", output);
    }

    fn get_full_cmd(self) -> String {
        let mut config_dir = PathBuf::new();
        config_dir.push(env::home_dir().unwrap());
        config_dir.push(".config");
        config_dir.push("eaconfig");
        let file = File::open(config_dir).unwrap();
        let lines = BufReader::new(file).lines();
        let start_str = self.cmd + ",";
        for line in lines {
            if let Ok(cmdline) = line {
                if cmdline.starts_with(&start_str) {
                    return cmdline.split(',').nth(1).unwrap().to_string();
                }
            }
        }

        return "".to_string()
    }
}



fn main() {
    let args = Cli::parse();
    args.process();
}
