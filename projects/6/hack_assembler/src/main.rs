//! Hack Assembler main module.
//! Translates Hack assembly language into Hack machine code.
//!
//! This assembler implements the assembly process in two passes:
//! 1. First pass: Collects all labels and their corresponding ROM addresses
//! 2. Second pass: Generates binary code while handling variables and symbols
//!
//! The assembler handles three types of instructions:
//! - A-instructions: @value (translated to 0vvvvvvvvvvvvvvv where v is the 15-bit value)
//! - C-instructions: dest=comp;jump (translated to 111accccccdddjjj)
//! - L-instructions: (LABEL) (pseudo-command that defines a label)

use std::{env, fs, process};

use code::Code;
use parser::{CommandType, Parser};
use symbol_table::SymbolTable;

mod code;
mod parser;
mod symbol_table;

/// Converts a decimal number to its 16-bit binary representation.
///
/// # Arguments
///
/// * `num` - A u16 number to convert to binary
///
/// # Returns
///
/// A String containing the 16-bit binary representation with leading zeros
fn decimal_to_binary(num: u16) -> String {
    format!("{:016b}", num)
}

/// Main function that handles the assembly process:
/// 1. Validates command line arguments
/// 2. Reads the input file
/// 3. Performs two passes over the assembly code
/// 4. Writes the resulting binary code to the output file
///
/// # Command line arguments
///
/// * First argument: Input file path (must end with .asm)
///
/// # Output
///
/// Creates a .hack file with the same name as the input file
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.asm>", args[0]);
        process::exit(1);
    }

    let input_file_name = &args[1];
    let output_file_name = input_file_name.replace(".asm", ".hack");

    if !input_file_name.ends_with(".asm") {
        eprintln!("Invalid file: should be an asm file");
        process::exit(1);
    }

    let input_file_content =
        fs::read_to_string(input_file_name).expect("Failed to read input file");

    let mut symbol_table = SymbolTable::new();
    let mut parser = Parser::new(&input_file_content);

    // First pass: collect labels
    // ROM address starts at 0 and increments for each actual instruction (A or C)
    let mut rom_address = 0;
    while parser.has_more_commands() {
        match parser.command_type() {
            Some(CommandType::LCommand) => {
                let label = parser.symbol().expect("Failed to parse label");
                symbol_table.add_entry(label.to_string(), rom_address);
            }
            Some(CommandType::ACommand) | Some(CommandType::CCommand) => {
                rom_address += 1;
            }
            None => {}
        }
        parser.advance();
    }

    // Second pass: generate binary code
    // RAM address starts at 16 for variables (0-15 are reserved)
    let mut ram_address = 16;
    let mut parser = Parser::new(&input_file_content);
    let mut output = String::new();

    while parser.has_more_commands() {
        match parser.command_type() {
            Some(CommandType::ACommand) => {
                let symbol = parser.symbol().expect("Failed to parse symbol");
                let address = if let Ok(num) = symbol.parse::<u16>() {
                    // If symbol is a number, use it directly
                    num
                } else if symbol_table.contains(symbol) {
                    // If symbol exists in table, use its address
                    symbol_table.get_address(symbol).unwrap()
                } else {
                    // If symbol is new, allocate next available RAM address
                    symbol_table.add_entry(symbol.to_string(), ram_address);
                    ram_address += 1;
                    ram_address - 1
                };
                output.push_str(&decimal_to_binary(address));
                output.push('\n');
            }
            Some(CommandType::CCommand) => {
                // C-instructions always start with '111'
                let mut binary = String::from("111");
                binary.push_str(&Code::comp(parser.comp().unwrap()));
                binary.push_str(&Code::dest(parser.dest().unwrap()));
                binary.push_str(&Code::jump(parser.jump().unwrap()));
                output.push_str(&binary);
                output.push('\n');
            }
            Some(CommandType::LCommand) => {} // Labels are handled in first pass
            None => {}
        }
        parser.advance();
    }

    fs::write(&output_file_name, output).expect("Failed to write output file");
}
