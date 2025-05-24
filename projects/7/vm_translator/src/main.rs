use std::{env, fs, path::Path, process};

use code_writer::CodeWriter;
use parser::{CommandType, Parser};

mod code_writer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.vm>", args[0]);
        process::exit(1);
    }

    let input_file_name = &args[1];
    let output_file_name = input_file_name.replace(".vm", ".asm");

    let input_file_content =
        fs::read_to_string(input_file_name).expect("Failed to read input file");

    // Extract filename without extension for static variables
    let filename = Path::new(input_file_name)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let output_file = fs::File::create(&output_file_name).expect("Failed to create output file");

    let mut code_writer = CodeWriter::new(output_file, filename);
    let mut parser = Parser::new(&input_file_content);

    while parser.has_more_lines() {
        match parser.command_type() {
            CommandType::Arithmetic => {
                let command = parser.arg1();
                code_writer.write_arithmetic(&command);
            }
            CommandType::Push | CommandType::Pop => {
                let command_type = parser.command_type();
                let segment = parser.arg1();
                let index = parser.arg2();
                code_writer.write_push_pop(command_type, &segment, index);
            }
            _ => {
                // For now, we only handle arithmetic and memory access commands
                // Other commands will be implemented in chapter 8
                println!("Skipping unsupported command type");
            }
        }
        parser.advance();
    }

    code_writer.close();
    println!(
        "Translation complete: {} -> {}",
        input_file_name, output_file_name
    );
}
