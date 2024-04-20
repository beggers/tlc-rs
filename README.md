# tlc-rs
Rust TLA+ checker

## Organization

All modules which need to be integration tested (e.g. parsing) live in `lib/`, which is technically a separate project. The top-level `src/` directory contains information for the binary project which depends on the `lib/` project.

## Testing

```sh
# Run all tests for application and library
cargo test --all --no-fail-fast
```

```sh
# Run only a single file
cargo test -p lib --test string  # Replace "string" with the file you want
```

## Requirements

- Progress indicators like the TLA+ Toolkit (diameter, states found, percentage of states checked, yadda yadda)
- 

## Parts

- Lexer (with unicode support?)
- Parser
- Model checker? How does that even work?

## Testing

- https://github.com/ongardie/raft.tla/blob/master/raft.tla

## Magic

- Debugger
