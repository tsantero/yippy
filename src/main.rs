use evalexpr::{eval_with_context_mut, ContextWithMutableVariables, HashMapContext};
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
    Print(String),
}

fn execute(program: Vec<Instruction>, ctx: &mut HashMapContext) {
    for instruction in program {
        match instruction {
            Instruction::Set(data) => {
                let value = eval_with_context_mut(&data.val, ctx)
                    .expect("Failed to evaluate expression");
                ctx.set_value(data.var, value)
                    .expect("Failed to set variable");
            }
            Instruction::Print(expr) => {
                let value = eval_with_context_mut(&expr, ctx)
                    .expect("Failed to evaluate expression");
                println!("{}", value);
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

    let mut ctx = HashMapContext::new();
    execute(program, &mut ctx);
}
