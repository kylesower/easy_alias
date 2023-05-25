use clap::Parser;
use std::fs::{self, File};
use std::io::{self, Read, Error, ErrorKind};
use std::env;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Parser, Default)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Alias you want to use with easy_alias.
    alias: Option<String>,

    /// Bash script assigned to alias, enclosed 
    /// in quotes if it contains spaces. To add
    /// substitutions, use '**x' syntax in the 
    /// command, where x can be any single letter.
    #[arg(verbatim_doc_comment)]
    cmd: Option<String>,

    /// Remove provided alias.
    #[arg(short, default_value_t = false)]
    remove: bool,

    /// List aliases.
    #[arg(short, default_value_t = false)]
    list: bool,

    /// Use this flag to pass substitutions 
    /// in the form "x=<value>,y=<value>..."
    /// If the value contains a comma or dollar 
    /// sign, escape it with a \\
    #[arg(short, verbatim_doc_comment)]
    subs: Option<String>
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
            println!("Invalid input. Try using the -h or --help flags.");
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
        let res = self.get_full_cmd();
        if let Ok(cmd) = res {
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
        } else if let Err(x) = res {
            println!("{}", x);
        }
    }

    fn expand_subs(&self, line: &str) -> Result<String, Error> {
        if let Some(subs) = self.subs.clone() {
            let mut new_line = line.clone().to_string();
            let safe_subs = subs.replace("\\,", "::COMMA::");
            let sub_iter = safe_subs.split(',').map(|val| val.replace("::COMMA::", ","));
            for sub in sub_iter {
                let mut new_val = String::new();
                let mut str_to_replace = "**".to_string();
                for (i, char) in sub.chars().enumerate() {
                    if i == 0 {
                        str_to_replace.push(char);
                    } else if i > 1 {
                        new_val.push(char);
                    }
                }
                new_line = new_line.replace(&str_to_replace, &new_val);
            }
            println!("Command after subs: {}", &new_line);
            return Ok(new_line)
        } else {
            if let None = line.find("**") {
                return Ok(line.to_string())
            } else {
                let err_string = format!("Error!{}There are substitutions you must make for any variables following **{}The command for this alias is: {}", '\n', '\n', &line);
                return Err(Error::new(ErrorKind::InvalidInput, err_string))
            }
        }
    }

    fn get_full_cmd(&self) -> Result<String, Error> {
        let (config, _) = self.read_config().unwrap();
        let mut start_str = self.alias.clone().unwrap();
        start_str.push_str("::");
        for line in config.split('\n') {
            if line.starts_with(&start_str) {
                let full_cmd = self.expand_subs(line.split("::").nth(1).unwrap());
                return full_cmd;
            }
        }
        return Err(Error::new(ErrorKind::InvalidInput, "Alias not found! List aliases with the -l flag."))
    }
}

fn main() {
    let args = Cli::parse();
    args.process();
}
