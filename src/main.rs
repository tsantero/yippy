use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
struct SetData {
    var: String,
    val: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Instruction {
    Set(SetData),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: yippy <script.yaml>");
        std::process::exit(1);
    }

    let content = fs::read_to_string(&args[1]).expect("Failed to read file");
    let program: Vec<Instruction> = yaml_serde::from_str(&content).expect("Failed to parse YAML");

    println!("{:#?}", program);
}
