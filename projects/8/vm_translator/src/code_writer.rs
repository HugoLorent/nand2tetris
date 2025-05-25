use crate::parser::CommandType;
use std::fs::File;
use std::io::{BufWriter, Write};

pub struct CodeWriter {
    output: BufWriter<File>,
    filename: String,
    label_counter: usize,
    function_name: String,
    call_counter: usize,
}

impl CodeWriter {
    pub fn new(output_file: File, filename: String) -> Self {
        Self {
            output: BufWriter::new(output_file),
            filename,
            label_counter: 0,
            function_name: String::new(),
            call_counter: 0,
        }
    }

    pub fn set_filename(&mut self, filename: String) {
        self.filename = filename;
    }

    pub fn write_bootstrap(&mut self) {
        self.write_line("// Bootstrap code");
        self.write_line("@256");
        self.write_line("D=A");
        self.write_line("@SP");
        self.write_line("M=D");
        self.write_call("Sys.init", 0);
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

    pub fn write_label(&mut self, label: &str) {
        self.write_line(&format!("// label {}", label));
        self.write_line(&format!("({}${})", self.function_name, label));
    }

    pub fn write_goto(&mut self, label: &str) {
        self.write_line(&format!("// goto {}", label));
        self.write_line(&format!("@{}${}", self.function_name, label));
        self.write_line("0;JMP");
    }

    pub fn write_if(&mut self, label: &str) {
        self.write_line(&format!("// if-goto {}", label));
        self.pop_to_d();
        self.write_line(&format!("@{}${}", self.function_name, label));
        self.write_line("D;JNE");
    }

    pub fn write_function(&mut self, function_name: &str, num_locals: usize) {
        self.function_name = function_name.to_string();
        self.write_line(&format!("// function {} {}", function_name, num_locals));
        self.write_line(&format!("({})", function_name));

        // Initialize local variables to 0
        for _ in 0..num_locals {
            self.write_line("@SP");
            self.write_line("A=M");
            self.write_line("M=0");
            self.increment_sp();
        }
    }

    pub fn write_call(&mut self, function_name: &str, num_args: usize) {
        let return_label = format!("{}$ret.{}", function_name, self.call_counter);
        self.call_counter += 1;

        self.write_line(&format!("// call {} {}", function_name, num_args));

        // Push return address
        self.write_line(&format!("@{}", return_label));
        self.write_line("D=A");
        self.push_d();

        // Push LCL
        self.write_line("@LCL");
        self.write_line("D=M");
        self.push_d();

        // Push ARG
        self.write_line("@ARG");
        self.write_line("D=M");
        self.push_d();

        // Push THIS
        self.write_line("@THIS");
        self.write_line("D=M");
        self.push_d();

        // Push THAT
        self.write_line("@THAT");
        self.write_line("D=M");
        self.push_d();

        // ARG = SP - 5 - num_args
        self.write_line("@SP");
        self.write_line("D=M");
        self.write_line(&format!("@{}", 5 + num_args));
        self.write_line("D=D-A");
        self.write_line("@ARG");
        self.write_line("M=D");

        // LCL = SP
        self.write_line("@SP");
        self.write_line("D=M");
        self.write_line("@LCL");
        self.write_line("M=D");

        // goto function
        self.write_line(&format!("@{}", function_name));
        self.write_line("0;JMP");

        // return label
        self.write_line(&format!("({})", return_label));
    }

    pub fn write_return(&mut self) {
        self.write_line("// return");

        // Store LCL in temp variable (R11)
        self.write_line("@LCL");
        self.write_line("D=M");
        self.write_line("@R11"); // endFrame = LCL
        self.write_line("M=D");

        // Get return address (endFrame - 5)
        self.write_line("@5");
        self.write_line("A=D-A");
        self.write_line("D=M");
        self.write_line("@R12"); // retAddr = *(endFrame - 5)
        self.write_line("M=D");

        // Reposition return value for caller
        self.pop_to_d();
        self.write_line("@ARG");
        self.write_line("A=M");
        self.write_line("M=D"); // *ARG = pop()

        // Restore SP for caller
        self.write_line("@ARG");
        self.write_line("D=M+1");
        self.write_line("@SP");
        self.write_line("M=D"); // SP = ARG + 1

        // Restore THAT
        self.write_line("@R11");
        self.write_line("D=M-1");
        self.write_line("A=D");
        self.write_line("D=M");
        self.write_line("@THAT");
        self.write_line("M=D");

        // Restore THIS
        self.write_line("@R11");
        self.write_line("D=M");
        self.write_line("@2");
        self.write_line("A=D-A");
        self.write_line("D=M");
        self.write_line("@THIS");
        self.write_line("M=D");

        // Restore ARG
        self.write_line("@R11");
        self.write_line("D=M");
        self.write_line("@3");
        self.write_line("A=D-A");
        self.write_line("D=M");
        self.write_line("@ARG");
        self.write_line("M=D");

        // Restore LCL
        self.write_line("@R11");
        self.write_line("D=M");
        self.write_line("@4");
        self.write_line("A=D-A");
        self.write_line("D=M");
        self.write_line("@LCL");
        self.write_line("M=D");

        // Jump to return address
        self.write_line("@R12");
        self.write_line("A=M");
        self.write_line("0;JMP");
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
