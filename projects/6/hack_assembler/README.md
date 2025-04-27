# Hack Assembler

A Rust implementation of the Hack Assembler from the Nand2Tetris course (Project 6).

## Description

This assembler translates Hack assembly language (.asm files) into Hack machine code (.hack files). It handles:
- A-instructions: `@value`
- C-instructions: `dest=comp;jump`
- Labels: `(LABEL)`

## Usage

```bash
cargo run -- <input_file.asm>
```

The assembler will generate a `.hack` file with the same name as the input file.

## Implementation Details

The assembly process is done in two passes:
1. First pass: Collects all labels and their corresponding ROM addresses
2. Second pass: Translates instructions to binary while handling variables and symbols

## Project Structure

- `main.rs`: Main program logic and assembly process
- `parser.rs`: Parses assembly commands
- `code.rs`: Translates mnemonics to binary
- `symbol_table.rs`: Manages symbols and their addresses

## Requirements

- Rust 2021 edition or later
