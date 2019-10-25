# simple-db
A simple sql database written in rust.
One day I was wondering about how cool databases are and how do they work internally. And what better way to learn
about database internals than to build one for yourself. This is an attempt to create a very simple sql database to
learn about how they work.

## Progress
The project is in very early stage. The features that have been implemented:
- [x] REPL Interface
- [x] Distinguish between meta commands and db commands.
- [x] Columnar storage engine.
- [x] `Create Table` Command
- [x] Generic validation structure.
- [x] persistance with a command `.persist`
- [x] Serialization | Deserialization to and from binary encodings.
- [x] simple insert queries.
- [x] In memory btree indexes only for primary keys.
- [x] simple select queries ( only single where clause and no joins ).
- [x] unique key constraints.

## Roadmap
Features that are in the roadmap of the proejct:
- [ ] Joins
  - [ ] Inner
  - [ ] Left
  - [ ] Outer
- [ ] Indexing - cost and performance gain analysis
- [ ] Benchmarking
- [ ] Server Client / Connection Manager
- [ ] Lock manager
- [ ] Concurrency
- [ ] Pluggable storage engine
- [ ] Different implementations of storage engines to optimize different operations
  - [ ] Write Heavy - `LSM Tree && SSTable`
  - [ ] Read Heavy - `B-Tree` 

## Getting Started
The project is written in `rust`, so you need it in your system. You can get started with 
rust from here https://rustup.rs/. Then just clone the project and `cargo run` to run it.

- `.tables` - prints list of tables with schema
- `.data`   - prints all rows of all tables. Useful for debugging
- `.exit`   - to exit


