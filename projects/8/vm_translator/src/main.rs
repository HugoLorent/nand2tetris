use std::{env, fs, path::Path, process};

use code_writer::CodeWriter;
use parser::{CommandType, Parser};

mod code_writer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.vm or directory>", args[0]);
        process::exit(1);
    }

    let input_path = Path::new(&args[1]);

    if input_path.is_file() {
        // Single file mode (compatible with chapter 7)
        translate_single_file(&args[1]);
    } else if input_path.is_dir() {
        // Directory mode (chapter 8)
        translate_directory(&args[1]);
    } else {
        eprintln!("Error: {} is neither a file nor a directory", args[1]);
        process::exit(1);
    }
}

fn translate_single_file(input_file_name: &str) {
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
    let mut code_writer = CodeWriter::new(output_file, filename.clone());

    translate_file(&input_file_content, &mut code_writer);

    code_writer.close();
    println!(
        "Translation complete: {} -> {}",
        input_file_name, output_file_name
    );
}

fn translate_directory(dir_path: &str) {
    let dir = Path::new(dir_path);
    let dir_name = dir.file_name().unwrap().to_str().unwrap();
    let output_file_name = format!("{}/{}.asm", dir_path, dir_name);

    let output_file = fs::File::create(&output_file_name).expect("Failed to create output file");
    let mut code_writer = CodeWriter::new(output_file, String::new());

    // Write bootstrap code for directory mode
    code_writer.write_bootstrap();

    // Process all .vm files in the directory
    let entries = fs::read_dir(dir_path).expect("Failed to read directory");
    let mut vm_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "vm" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Sort files for consistent output
    vm_files.sort();

    for vm_file in vm_files {
        let filename = vm_file.file_stem().unwrap().to_str().unwrap().to_string();
        let content =
            fs::read_to_string(&vm_file).unwrap_or_else(|_| panic!("Failed to read file: {:?}", vm_file));

        code_writer.set_filename(filename.clone());
        translate_file(&content, &mut code_writer);
    }

    code_writer.close();
    println!("Translation complete: {} -> {}", dir_path, output_file_name);
}

fn translate_file(content: &str, code_writer: &mut CodeWriter) {
    let mut parser = Parser::new(content);

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
            CommandType::Label => {
                let label = parser.arg1();
                code_writer.write_label(&label);
            }
            CommandType::Goto => {
                let label = parser.arg1();
                code_writer.write_goto(&label);
            }
            CommandType::If => {
                let label = parser.arg1();
                code_writer.write_if(&label);
            }
            CommandType::Function => {
                let function_name = parser.arg1();
                let num_locals = parser.arg2();
                code_writer.write_function(&function_name, num_locals);
            }
            CommandType::Call => {
                let function_name = parser.arg1();
                let num_args = parser.arg2();
                code_writer.write_call(&function_name, num_args);
            }
            CommandType::Return => {
                code_writer.write_return();
            }
        }
        parser.advance();
    }
}
