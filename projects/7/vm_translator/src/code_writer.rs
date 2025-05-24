use crate::parser::CommandType;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    output: BufWriter<File>,
    filename: String,
    label_counter: usize,
}

impl CodeWriter {
    pub fn new(output_file: File, filename: String) -> Self {
        Self {
            output: BufWriter::new(output_file),
            filename,
            label_counter: 0,
        }
    }

    pub fn write_arithmetic(&mut self, command: &str) {
        match command {
            "add" => {
                self.write_line("// add");
                self.pop_to_d();
                self.decrement_sp();
                self.write_line("A=M");
                self.write_line("M=D+M");
                self.increment_sp();
            }
            "sub" => {
                self.write_line("// sub");
                self.pop_to_d();
                self.decrement_sp();
                self.write_line("A=M");
                self.write_line("M=M-D");
                self.increment_sp();
            }
            "neg" => {
                self.write_line("// neg");
                self.decrement_sp();
                self.write_line("A=M");
                self.write_line("M=-M");
                self.increment_sp();
            }
            "eq" => self.write_comparison("JEQ"),
            "gt" => self.write_comparison("JGT"),
            "lt" => self.write_comparison("JLT"),
            "and" => {
                self.write_line("// and");
                self.pop_to_d();
                self.decrement_sp();
                self.write_line("A=M");
                self.write_line("M=D&M");
                self.increment_sp();
            }
            "or" => {
                self.write_line("// or");
                self.pop_to_d();
                self.decrement_sp();
                self.write_line("A=M");
                self.write_line("M=D|M");
                self.increment_sp();
            }
            "not" => {
                self.write_line("// not");
                self.decrement_sp();
                self.write_line("A=M");
                self.write_line("M=!M");
                self.increment_sp();
            }
            _ => panic!("Unknown arithmetic command: {}", command),
        }
    }

    pub fn write_push_pop(&mut self, command_type: CommandType, segment: &str, index: usize) {
        match command_type {
            CommandType::Push => self.write_push(segment, index),
            CommandType::Pop => self.write_pop(segment, index),
            _ => panic!("Invalid command type for push/pop"),
        }
    }

    fn write_push(&mut self, segment: &str, index: usize) {
        self.write_line(&format!("// push {} {}", segment, index));
        match segment {
            "constant" => {
                self.write_line(&format!("@{}", index));
                self.write_line("D=A");
                self.push_d();
            }
            "local" => self.push_from_segment("LCL", index),
            "argument" => self.push_from_segment("ARG", index),
            "this" => self.push_from_segment("THIS", index),
            "that" => self.push_from_segment("THAT", index),
            "temp" => {
                self.write_line(&format!("@{}", 5 + index));
                self.write_line("D=M");
                self.push_d();
            }
            "static" => {
                self.write_line(&format!("@{}.{}", self.filename, index));
                self.write_line("D=M");
                self.push_d();
            }
            "pointer" => {
                let addr = if index == 0 { "THIS" } else { "THAT" };
                self.write_line(&format!("@{}", addr));
                self.write_line("D=M");
                self.push_d();
            }
            _ => panic!("Unknown segment: {}", segment),
        }
    }

    fn write_pop(&mut self, segment: &str, index: usize) {
        self.write_line(&format!("// pop {} {}", segment, index));
        match segment {
            "local" => self.pop_to_segment("LCL", index),
            "argument" => self.pop_to_segment("ARG", index),
            "this" => self.pop_to_segment("THIS", index),
            "that" => self.pop_to_segment("THAT", index),
            "temp" => {
                self.pop_to_d();
                self.write_line(&format!("@{}", 5 + index));
                self.write_line("M=D");
            }
            "static" => {
                self.pop_to_d();
                self.write_line(&format!("@{}.{}", self.filename, index));
                self.write_line("M=D");
            }
            "pointer" => {
                let addr = if index == 0 { "THIS" } else { "THAT" };
                self.pop_to_d();
                self.write_line(&format!("@{}", addr));
                self.write_line("M=D");
            }
            _ => panic!("Unknown segment: {}", segment),
        }
    }

    fn push_from_segment(&mut self, segment: &str, index: usize) {
        self.write_line(&format!("@{}", index));
        self.write_line("D=A");
        self.write_line(&format!("@{}", segment));
        self.write_line("A=D+M");
        self.write_line("D=M");
        self.push_d();
    }

    fn pop_to_segment(&mut self, segment: &str, index: usize) {
        // Store the target address in R13
        self.write_line(&format!("@{}", index));
        self.write_line("D=A");
        self.write_line(&format!("@{}", segment));
        self.write_line("D=D+M");
        self.write_line("@R13");
        self.write_line("M=D");

        // Pop value to D
        self.pop_to_d();

        // Store value at target address
        self.write_line("@R13");
        self.write_line("A=M");
        self.write_line("M=D");
    }

    fn write_comparison(&mut self, jump_type: &str) {
        let true_label = format!("TRUE_{}", self.label_counter);
        let end_label = format!("END_{}", self.label_counter);
        self.label_counter += 1;

        self.write_line(&format!("// {}", jump_type.to_lowercase()));
        self.pop_to_d();
        self.decrement_sp();
        self.write_line("A=M");
        self.write_line("D=M-D");
        self.write_line(&format!("@{}", true_label));
        self.write_line(&format!("D;{}", jump_type));

        // False case
        self.write_line("@SP");
        self.write_line("A=M");
        self.write_line("M=0");
        self.write_line(&format!("@{}", end_label));
        self.write_line("0;JMP");

        // True case
        self.write_line(&format!("({})", true_label));
        self.write_line("@SP");
        self.write_line("A=M");
        self.write_line("M=-1");

        // End
        self.write_line(&format!("({})", end_label));
        self.increment_sp();
    }

    fn push_d(&mut self) {
        self.write_line("@SP");
        self.write_line("A=M");
        self.write_line("M=D");
        self.increment_sp();
    }

    fn pop_to_d(&mut self) {
        self.decrement_sp();
        self.write_line("A=M");
        self.write_line("D=M");
    }

    fn increment_sp(&mut self) {
        self.write_line("@SP");
        self.write_line("M=M+1");
    }

    fn decrement_sp(&mut self) {
        self.write_line("@SP");
        self.write_line("M=M-1");
    }

    fn write_line(&mut self, line: &str) {
        writeln!(self.output, "{}", line).expect("Failed to write line");
    }

    pub fn close(&mut self) {
        self.output.flush().expect("Failed to flush output");
    }
}
