# tlc-rs
Rust TLA+ checker

!!! This project is paused indefinitely. TLA+'s specific brand of whitespace-sensitive syntax makes it hard to parse and I frankly just don't have the time.

## Organization

All modules which need to be integration tested (e.g. parsing) live in `lib/`, which is technically a separate project. The top-level `src/` directory contains information for the binary project which depends on the `lib/` project.

## Contributing

Right now the main goal -- the only goal -- is to complete the parser. The best way to help is to pick a test file in `lib/tests` and try to get at least one test working. See below for instructions on how to run a single test file.

To add to the parser:

1. Extend the grammar in `lib/grammar.pest`. You can learn about Pest [here](https://pest.rs/book/intro.html).
2. Extend the AST types in `lib/src/ast.rs`.
3. Extend the parse tree -> AST logic in `lib/src/parser.rs`.

In short: we need to tell Pest how to parse TLA+, we need a nice strongly typed place to put the resulting parse tree, and we need a layer to turn Pest's parse tree into our AST types.

## Testing

To run all tests which should be passing:

```sh
bin/passing_parser_tests.sh  # Please add to this file when you get a test passing!
```

To run a single test:

```sh
# Run only a single file
cargo test -p lib --test string  # Replace "string" with the file you want
```

To run all tests:

```sh
# Run all tests for application and library
cargo test --all --no-fail-fast
```

## Project goals

Goals of the project in roughly descending order of importance:

- Cover 80+% of common real-world use-cases.
- Trivially easy to install and run.
- Really really really good error reporting.
- Speed.
