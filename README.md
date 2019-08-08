# simple-db
A simple sql database written in rust.
One day I was wondering about how cool databases are and how do they work internally. And what better way to learn
about database internals than to build one for yourself. This is an attempt to create a very simple sql database to
learn about how they work.

## Progress
The project is in very early stage. The features that have been implemented:
- REPL Interface
- Distinguish between meta commands and db commands.
- `Create Table` Command
- Generic validation structure.

Features that are in the roadmap of the proejct:
- `Persistence` duh.
- Serialization | Deserialization
- Joins
  - Inner
  - Left
  - Outer
- Indexing - cost and performance gain analysis
- Benchmarking
- Server Client
- Abstract away the storage engine
- Different implementations of storage engines to optimize different operations
  - Write Heavy - `LSM Tree && SSTable`
  - Read Heavy - `B-Tree` 

## Getting Started
The project is written in `rust`, so you need it in your system. You can get started with 
rust from here https://rustup.rs/. Then just clone the project and `cargo run` to run it.

- `.help` - prints the help dialog.
- `.tables` - prints list of tables.

