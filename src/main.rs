use passwords::PasswordGenerator;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::Command;

const PWD_FILE_PATH: &str = "/usr/local/.spm_rs_passwords";
struct CmdArgs {
    command: Option<String>,
    key: Option<String>,
    password: Option<String>,
}

fn parse_cmd_args() -> CmdArgs {
    let mut cmd_args = std::env::args().skip(1);

    return CmdArgs {
        command: cmd_args.nth(0),
        key: cmd_args.nth(0),
        password: cmd_args.nth(0),
    };
}

fn print_help() {
    let help_info = "expected commands:
 help             - print help message
 set <KEY> [PASS] - set a PASS by KEY. PASS can be provided with pipe or interactively
 del <KEY>        - delete entry by KEY
 gen <KEY>        - generate entry for KEY
 get <KEY>        - copy a password to clipboard
 list             - list available entries";

    println!("{help_info}");
}

/*
 password file format

 key: value
*/

fn get_existing_passwords() -> HashMap<String, String> {
    let mut pwd_collection: HashMap<String, String> = HashMap::new();
    
    let file = match File::options().read(true).open(PWD_FILE_PATH) {
        Ok(f) => f,
        Err(_) => match File::create(PWD_FILE_PATH) {
            Ok(_) => {
                return pwd_collection;
            },
            Err(_) => {
                panic!("can't open or create passwords file");
            }
        },
    };

    let lines = io::BufReader::new(file).lines();


    for line in lines {
        if let Ok(line) = line {
            if let Some(line) = line.replace("\n", "").split_once(": ") {
                pwd_collection.insert(String::from(line.0), String::from(line.1));
            }
        }
    }

    return pwd_collection;
}

fn del_password(key: &String) {
    let mut passwords = get_existing_passwords();

    match passwords.remove(key) {
        Some(_) => {
            write_updated_passwords(&passwords);
            println!("{key} deleted");
        }
        None => {
            println!("{key} not found")
        }
    };
}

fn list_passwords() {
    let passwords = get_existing_passwords();
    for (key, _) in passwords {
        println!("{key}");
    }
}

fn get_password(key: &String) {
    let passwords = get_existing_passwords();

    let pwd_value = passwords.get(key);

    match pwd_value {
        Some(pwd) => {
            let shell_command = format!(
                "echo -e '{}' | xclip -selection clipboard",
                pwd.replace("'", "'\"'\"'")
            );

            let _ = Command::new("bash")
                .arg("-c")
                .arg(shell_command.as_str())
                .status();

            println!("Copied to clipboard");
        }
        _ => println!("entry {key} not found"),
    };
}

fn gen_password(key: &String) {
    let mut passwords = get_existing_passwords();

    let pg = PasswordGenerator {
        length: 24,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: true,
        exclude_similar_characters: false,
        strict: true,
    };

    let generated_password = pg
        .generate_one()
        .expect("failed to generate password with pg");

    passwords.insert(key.clone(), generated_password);

    write_updated_passwords(&passwords);

    println!("generated password for {key}");
}

fn set_password(key: &String, password: &String) {
    let mut passwords = get_existing_passwords();

    passwords.insert(key.clone(), password.clone());

    write_updated_passwords(&passwords);
}

fn execute_command(command: String, key: Option<String>, password: Option<String>) {
    if ["del", "set", "get", "gen"].contains(&command.as_str()) {
        let key = match key {
            Some(v) => v,
            None => {
                println!("KEY is not provided");
                return;
            }
        };

        let _ = match command.as_str() {
            "del" => del_password(&key),
            "set" => {
                if let Some(password) = password {
                    set_password(&key, &password);
                } else {
                    println!("PASSWORD not provided");
                }
            }
            "gen" => gen_password(&key),
            "get" => get_password(&key),
            _ => print_help(),
        };
    } else {
        match command.as_str() {
            "list" => list_passwords(),
            _ => print_help(),
        }
    }
}

fn write_updated_passwords(passwords: &HashMap<String, String>) {
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(PWD_FILE_PATH)
        .expect("filed to write passwords");

    for (key, val) in passwords {
        let line_to_write = format!("{}: {}", key, val);

        let _ = writeln!(&mut file, "{line_to_write}");
    }
}
fn main() {
    let parsed_commands = parse_cmd_args();

    match parsed_commands.command {
        Some(command) => execute_command(command, parsed_commands.key, parsed_commands.password),
        _ => {
            print_help();
        }
    }
}
