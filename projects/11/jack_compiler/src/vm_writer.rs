use std::fmt;

/// VM commands for the Hack computer
#[derive(Debug, Clone)]
pub enum Segment {
    Constant,
    Local,
    Argument,
    Static,
    This,
    That,
    Pointer,
    Temp,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Segment::Constant => "constant",
            Segment::Local => "local",
            Segment::Argument => "argument",
            Segment::Static => "static",
            Segment::This => "this",
            Segment::That => "that",
            Segment::Pointer => "pointer",
            Segment::Temp => "temp",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Command::Add => "add",
            Command::Sub => "sub",
            Command::Neg => "neg",
            Command::Eq => "eq",
            Command::Gt => "gt",
            Command::Lt => "lt",
            Command::And => "and",
            Command::Or => "or",
            Command::Not => "not",
        };
        write!(f, "{}", s)
    }
}

/// VM Writer generates VM code
pub struct VMWriter {
    output: Vec<String>,
    label_count: usize,
}

impl VMWriter {
    pub fn new() -> Self {
        VMWriter {
            output: Vec::new(),
            label_count: 0,
        }
    }

    /// Write a VM push command
    pub fn write_push(&mut self, segment: Segment, index: usize) {
        self.output.push(format!("push {} {}", segment, index));
    }

    /// Write a VM pop command
    pub fn write_pop(&mut self, segment: Segment, index: usize) {
        self.output.push(format!("pop {} {}", segment, index));
    }

    /// Write a VM arithmetic command
    pub fn write_arithmetic(&mut self, command: Command) {
        self.output.push(format!("{}", command));
    }

    /// Write a VM label command
    pub fn write_label(&mut self, label: &str) {
        self.output.push(format!("label {}", label));
    }

    /// Write a VM goto command
    pub fn write_goto(&mut self, label: &str) {
        self.output.push(format!("goto {}", label));
    }

    /// Write a VM if-goto command
    pub fn write_if(&mut self, label: &str) {
        self.output.push(format!("if-goto {}", label));
    }

    /// Write a VM call command
    pub fn write_call(&mut self, name: &str, n_args: usize) {
        self.output.push(format!("call {} {}", name, n_args));
    }

    /// Write a VM function command
    pub fn write_function(&mut self, name: &str, n_locals: usize) {
        self.output.push(format!("function {} {}", name, n_locals));
    }

    /// Write a VM return command
    pub fn write_return(&mut self) {
        self.output.push("return".to_string());
    }

    /// Generate a unique label
    pub fn generate_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_count);
        self.label_count += 1;
        label
    }

    /// Get the complete VM code as a string
    pub fn get_output(&self) -> String {
        self.output.join("\n") + "\n"
    }

    /// Clear the output (useful for testing)
    pub fn clear(&mut self) {
        self.output.clear();
        self.label_count = 0;
    }
}

impl Default for VMWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut writer = VMWriter::new();
        writer.write_push(Segment::Constant, 7);
        writer.write_pop(Segment::Local, 0);
        
        let output = writer.get_output();
        assert_eq!(output, "push constant 7\npop local 0\n");
    }

    #[test]
    fn test_arithmetic() {
        let mut writer = VMWriter::new();
        writer.write_arithmetic(Command::Add);
        writer.write_arithmetic(Command::Neg);
        
        let output = writer.get_output();
        assert_eq!(output, "add\nneg\n");
    }

    #[test]
    fn test_function_call() {
        let mut writer = VMWriter::new();
        writer.write_function("Math.multiply", 2);
        writer.write_call("Output.printInt", 1);
        writer.write_return();
        
        let output = writer.get_output();
        assert_eq!(output, "function Math.multiply 2\ncall Output.printInt 1\nreturn\n");
    }

    #[test]
    fn test_labels() {
        let mut writer = VMWriter::new();
        let label1 = writer.generate_label("LOOP");
        let label2 = writer.generate_label("END");
        
        assert_eq!(label1, "LOOP_0");
        assert_eq!(label2, "END_1");
    }
}