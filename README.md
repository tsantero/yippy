# yippy

An interpreter that executes YAML as code.

Turing complete YAML. Yeah, that's right. And no, I'm not sorry.

## But why??

Because it's the year 2026 and why wouldn't you want your config files execute
code?

## Installation

```
cargo install --path .
```

Or just `cargo build --release` and grab the binary from `target/release/yippy`.

## Usage

```
yippy script.yaml
```

## Instructions

- `!set { var: name, val: "{{ expr }}" }` — set a variable
- `!print "text or {{ expr }}"` — print to stdout
- `!if { cond: "{{ bool }}", then: [...], else: [...] }` — conditional
- `!while { cond: "{{ bool }}", do: [...] }` — loop

Expressions use `{{ }}` mustaches for evaluation. Plain strings are literals.

## Limits

Currently uses a hardcoded 100k step limit to prevent infinite loops.

## License

MIT
