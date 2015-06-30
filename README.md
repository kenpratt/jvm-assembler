JVM Assembler
=============

Tools for working with Java Virtual Machine Class files from Rust.

Converts back and forth between binary `.class` files and Rust structs.

Supports JVM version 8.

Examples
--------

To run the examples:

```
cargo run --example hello_world && java hello_world
cargo run --example simple_addition && java simple_addition
```

Inspecting existing `.class` files
----------------------------------

The following command will print out a nicely-formatted representation of the structure of a `.class` file:

```
cargo run read myfile.class
```
