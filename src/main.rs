use evalexpr::{eval_with_context_mut, ContextWithMutableVariables, HashMapContext, Value};
use serde::Deserialize;
use std::{env, fs, process};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum Instruction {
    Set { var: String, val: String },
    Print(String),
    If {
        cond: String,
        then: Vec<Instruction>,
        #[serde(default, rename = "else")]
        else_: Vec<Instruction>,
    },
    While {
        cond: String,
        #[serde(rename = "do")]
        do_: Vec<Instruction>,
    },
}

const MAX_STEPS: u64 = 100_000;

fn interpolate(input: &str, ctx: &mut HashMapContext) -> Result<Value, String> {
    let trimmed = input.trim();

    if let Some(inner) = trimmed.strip_prefix("{{").and_then(|s| s.strip_suffix("}}")) {
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
        let rest = &remaining[start + 2..];

        let (expr, after) = rest.split_once("}}")
            .ok_or_else(|| "unclosed {{".to_string())?;

        let value = eval_with_context_mut(expr.trim(), ctx)
            .map_err(|e| format!("eval '{}': {}", expr.trim(), e))?;

        match value {
            Value::String(s) => result.push_str(&s),
            v => result.push_str(&v.to_string()),
        }

        remaining = after;
    }

    result.push_str(remaining);
    Ok(Value::String(result))
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Boolean(b) => *b,
        Value::Int(i) => *i != 0,
        Value::Float(f) => *f != 0.0,
        Value::String(s) => !s.is_empty(),
        _ => false,
    }
}

fn execute(program: Vec<Instruction>, ctx: &mut HashMapContext, steps: &mut u64) -> Result<(), String> {
    for instruction in program {
        *steps += 1;
        if *steps > MAX_STEPS {
            return Err(format!("step limit exceeded ({} instructions)", MAX_STEPS));
        }

        match instruction {
            Instruction::Set { var, val } => {
                let value = interpolate(&val, ctx)?;
                ctx.set_value(var, value).map_err(|e| e.to_string())?;
            }
            Instruction::Print(input) => {
                let value = interpolate(&input, ctx)?;
                match value {
                    Value::String(s) => println!("{}", s),
                    v => println!("{}", v),
                }
            }
            Instruction::If { cond, then, else_ } => {
                let cond_val = interpolate(&cond, ctx)?;
                if is_truthy(&cond_val) {
                    execute(then, ctx, steps)?;
                } else if !else_.is_empty() {
                    execute(else_, ctx, steps)?;
                }
            }
            Instruction::While { cond, do_ } => {
                loop {
                    let cond_val = interpolate(&cond, ctx)?;
                    if !is_truthy(&cond_val) {
                        break;
                    }
                    execute(do_.clone(), ctx, steps)?;
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: yippy <script.yaml>");
        process::exit(1);
    }

    let content = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("error reading file: {}", e);
        process::exit(1);
    });

    let program: Vec<Instruction> = yaml_serde::from_str(&content).unwrap_or_else(|e| {
        eprintln!("yaml parse error: {}", e);
        process::exit(1);
    });

    let mut ctx = HashMapContext::new();
    let mut steps = 0u64;

    if let Err(e) = execute(program, &mut ctx, &mut steps) {
        eprintln!("runtime error: {}", e);
        process::exit(1);
    }
}
