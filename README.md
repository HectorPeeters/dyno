# Dyno

Dyno is an experimental JIT compiler which is still very limited in capabilities. It parses code written in a custom programming language and outputs x86-64 assembly. The assembly can either be written to an ELF executable which is very unstable at the moment, or it can be executed in memory. The application can also run in repl mode which gives you a command-line like interface to execute single lines of code.

## Usage

To run the tests:

```
cargo test
```

To run the repl:

```
cargo run
```
