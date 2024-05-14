use std::collections::HashMap;
use std::io::{self, BufRead};
use std::{fmt::Error, fs::File, io::Read};

/*
expected commands:
help             - print help message
set <KEY> [PASS] - set a PASS by KEY. PASS can be provided with pipe or interactively
del <KEY>        - delete entry by KEY
gen <KEY>        - generate entry for KEY
get <KEY>        - copy a password to clipboard
*/

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

fn print_help() -> Result<(), ()> {
    let help_info = "expected commands:
 help             - print help message
 set <KEY> [PASS] - set a PASS by KEY. PASS can be provided with pipe or interactively
 del <KEY>        - delete entry by KEY
 gen <KEY>        - generate entry for KEY
 get <KEY>        - copy a password to clipboard";

    println!("{help_info}");

    return Ok(());
}

/*
 password file format

 key: value
 ...
*/

fn get_existing_passwords() -> HashMap<String, String> {
    let file = match File::open("./test_passwords") {
        Ok(f) => f,
        Err(_) => match File::create("./test_passwords") {
            Ok(f) => f,
            Err(_) => panic!("can't open and create passwords file"),
        },
    };

    let lines = io::BufReader::new(file).lines();

    let mut pwdCollection: HashMap<String, String> = HashMap::new();

    for line in lines {
        if let Ok(line) = line {
            if let Some(line) = line.replace("\n", "").split_once(": ") {
                pwdCollection.insert(String::from(line.0), String::from(line.1));
            }
        }
    }

    return pwdCollection;
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

fn get_password(key: &String) {
    let mut passwords = get_existing_passwords();

    let pwd_value = passwords.get(key);

    match pwd_value {
        Some(pwd) => println!("Found password: {pwd}"),
        _ => (),
    };
}

fn gen_password(key: &String) {
    let mut passwords = get_existing_passwords();

    let generated_password = String::from("generated pass");

    passwords.insert(key.clone(), generated_password);

    write_updated_passwords(&passwords);
}

fn set_password(key: &String, password: &String) {
    let mut passwords = get_existing_passwords();

    passwords.insert(key.clone(), password.clone());

    write_updated_passwords(&passwords);
}

fn execute_command(command: String, key: Option<String>, password: Option<String>) {
    let (passwords, key) = match (
        ["del", "gen", "set", "get"].contains(&command.as_str()),
        key,
    ) {
        (true, Some(key)) => (get_existing_passwords(), key),
        _ => {
            println!("KEY not provided");
            return;
        }
    };

    let _ = match command.as_str() {
        "del" => del_password(&key),
        "get" => get_password(&key),
        "set" => match password {
            Some(password) => set_password(&key, &password),
            None => Err(()),
        },
        "gen" => gen_password(&key),
        _ => (),
    };
}

fn write_updated_passwords(passwords: &HashMap<String, String>) {
    println!("new passwords:");
    for (key, val) in passwords {
        println!("{}: {}", key, val);
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
