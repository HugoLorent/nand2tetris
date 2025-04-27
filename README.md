# NAND2Tetris Project Repository

## Overview
This repository contains my implementations and solutions for the NAND2Tetris course, which guides students through building a complete computer system from the ground up – from basic NAND gates to a functional modern computer.

## Project Structure
- `/01`: Boolean Logic - Basic logic gates implementation
- `/02`: Boolean Arithmetic - ALU and adders
- `/03`: Sequential Logic - Registers and memory
- `/04`: Machine Language - Assembly programs
- `/05`: Computer Architecture - CPU and memory implementation
- `/06`: Assembler - Translating assembly to machine code (implemented in Rust)
- `/07-08`: VM Translator - Handling stack-based virtual machine
- `/09`: High-Level Language - Jack applications
- `/10-11`: Compiler - Jack language to VM code translation
- `/12`: Operating System - OS implementation in Jack

## Usage
Each project folder contains the necessary implementation files along with test scripts provided by the course. To test implementations:

1. Use the Hardware Simulator for projects 1-5
2. Use the provided testing tools for software components in projects 6-12

For the Assembler (Project 6):
```bash
cd projects/6/hack_assembler
cargo run -- <input_file.asm>
```

## Dependencies
- NAND2Tetris Software Suite (available at [nand2tetris.org](https://www.nand2tetris.org/))

## Resources
- Official course website: [nand2tetris.org](https://www.nand2tetris.org/)
- Book: "The Elements of Computing Systems" by Noam Nisan and Shimon Schocken

## License
This project contains my personal implementations for educational purposes.
