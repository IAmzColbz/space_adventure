use std::io::{self, Write};

pub enum Command {
    Look(String),
    Go(String),
    Quit,
    Unknown(String),
}

pub fn parse(input_str: String) -> Command {
    let lc_input_str = input_str.to_lowercase();
    let mut split_input_iter = lc_input_str.trim().split_whitespace();

    let verb = split_input_iter.next().unwrap_or_default().to_string();
    let noun = split_input_iter.next().unwrap_or_default().to_string();

    match verb.as_str() {
        "quit" => Command::Quit,
        "look" => Command::Look(noun),
        "go" => Command::Go(noun),
        _ => Command::Unknown(input_str.trim().to_string()),
    }
}

pub fn get_input() -> Command {
    // prompt
    println!("");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input_str = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read move");
    println!("");

    // Parse and return
    parse(input_str)
}

pub fn update_state(command: &Command) -> String {
    let output: String;

    match command {
        Command::Look(_) => {output = format!("It is very cold and your breath is visible in fround of you.")},
        Command::Quit => output = format!("Quitting\nThanks for playing!"),
        Command::Go(_) => output = format!("I'm too weak to move like that..."),
        Command::Unknown(input_str) => output = format!("I dont know how to '{}'.", input_str),
    }

    // Return
    output
}

pub fn update_screen(output: String) {
    println!("{}", output);
}