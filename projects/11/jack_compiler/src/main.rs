use std::env;
use std::fs;
use std::path::Path;
use std::process;

mod compilation_engine;
mod jack_analyzer;
mod symbol_table;
mod tokenizer;
mod vm_writer;

use jack_analyzer::JackAnalyzer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <input.jack or directory>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let path = Path::new(input_path);

    if path.is_file() {
        if let Some(extension) = path.extension() {
            if extension == "jack" {
                analyze_file(path);
            } else {
                eprintln!("Input file must have .jack extension");
                process::exit(1);
            }
        }
    } else if path.is_dir() {
        analyze_directory(path);
    } else {
        eprintln!("Input must be a .jack file or a directory");
        process::exit(1);
    }
}

fn analyze_file(file_path: &Path) {
    println!("Analyzing file: {}", file_path.display());

    let input_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file {}: {}", file_path.display(), err);
            return;
        }
    };

    // Create output directory
    let parent_dir = file_path.parent().unwrap();
    let output_dir = parent_dir.join("output");
    if let Err(err) = fs::create_dir_all(&output_dir) {
        eprintln!("Error creating output directory {}: {}", output_dir.display(), err);
        return;
    }

    let mut analyzer = JackAnalyzer::new();
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap();

    // Generate tokens file
    let tokens_output = analyzer.tokenize(&input_content);
    let tokens_file = output_dir.join(format!("{}T.xml", file_stem));
    if let Err(err) = fs::write(&tokens_file, tokens_output) {
        eprintln!(
            "Error writing tokens file {}: {}",
            tokens_file.display(),
            err
        );
        return;
    }
    println!("Generated: {}", tokens_file.display());

    // Generate parse tree file
    let parse_output = analyzer.parse(&input_content);
    let parse_file = output_dir.join(format!("{}.xml", file_stem));
    if let Err(err) = fs::write(&parse_file, parse_output) {
        eprintln!("Error writing parse file {}: {}", parse_file.display(), err);
        return;
    }
    println!("Generated: {}", parse_file.display());

    // Generate VM code file
    let vm_output = analyzer.compile(&input_content, file_stem);
    let vm_file = output_dir.join(format!("{}.vm", file_stem));
    if let Err(err) = fs::write(&vm_file, vm_output) {
        eprintln!("Error writing VM file {}: {}", vm_file.display(), err);
        return;
    }
    println!("Generated: {}", vm_file.display());
}

fn analyze_directory(dir_path: &Path) {
    println!("Analyzing directory: {}", dir_path.display());

    let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Error reading directory {}: {}", dir_path.display(), err);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading directory entry: {}", err);
                continue;
            }
        };

        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "jack" {
                    analyze_file(&path);
                }
            }
        }
    }
}
