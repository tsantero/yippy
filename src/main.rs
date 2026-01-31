use evalexpr::{eval_with_context_mut, ContextWithMutableVariables, HashMapContext, Value};
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

/// Evaluates mustache templates. If the whole string is {{ expr }}, returns
/// the typed value. If mixed with text, interpolates and returns a string.
/// No mustaches means literal string.
fn interpolate(input: &str, ctx: &mut HashMapContext) -> Result<Value, String> {
    let trimmed = input.trim();

    if trimmed.starts_with("{{") && trimmed.ends_with("}}") {
        let inner = &trimmed[2..trimmed.len()-2];
        if !inner.contains("{{") && !inner.contains("}}") {
            let expr = inner.trim();
            return eval_with_context_mut(expr, ctx)
                .map_err(|e| format!("eval '{}': {}", expr, e));
        }
    }

    if !input.contains("{{") {
        return Ok(Value::String(input.to_string()));
    }

    let mut result = String::new();
    let mut remaining = input;

    while let Some(start) = remaining.find("{{") {
        result.push_str(&remaining[..start]);

        let after_open = &remaining[start + 2..];
        let end = after_open.find("}}")
            .ok_or_else(|| "unclosed {{ in template".to_string())?;

        let expr = after_open[..end].trim();
        let value = eval_with_context_mut(expr, ctx)
            .map_err(|e| format!("eval '{}': {}", expr, e))?;

        match value {
            Value::String(s) => result.push_str(&s),
            other => result.push_str(&other.to_string()),
        }

        remaining = &after_open[end + 2..];
    }

    result.push_str(remaining);

    Ok(Value::String(result))
}

fn execute(program: Vec<Instruction>, ctx: &mut HashMapContext) {
    for instruction in program {
        match instruction {
            Instruction::Set(data) => {
                let value = interpolate(&data.val, ctx)
                    .expect("Failed to evaluate expression");
                ctx.set_value(data.var, value)
                    .expect("Failed to set variable");
            }
            Instruction::Print(input) => {
                let value = interpolate(&input, ctx)
                    .expect("Failed to evaluate expression");
                match value {
                    Value::String(s) => println!("{}", s),
                    other => println!("{}", other),
                }
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
