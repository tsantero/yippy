use serde::Deserialize;
use std::collections::HashMap;
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

fn execute(program: Vec<Instruction>, vars: &mut HashMap<String, String>) {
    for instruction in program {
        match instruction {
            Instruction::Set(data) => {
                vars.insert(data.var, data.val);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: yippy <script.yaml>");
        std::process::exit(1);
    }

    let content = fs::read_to_string(&args[1]).expect("Failed to read file");
    let program: Vec<Instruction> = yaml_serde::from_str(&content).expect("Failed to parse YAML");

    let mut vars: HashMap<String, String> = HashMap::new();
    execute(program, &mut vars);

    println!("{:#?}", vars);
}
