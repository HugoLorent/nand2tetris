pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

pub struct Parser {
    commands: Vec<String>,
    current_command: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let commands: Vec<String> = input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .map(|line| {
                if let Some(idx) = line.find("//") {
                    line[..idx].trim().to_string()
                } else {
                    line.to_string()
                }
            })
            .collect();

        Self {
            commands,
            current_command: 0,
        }
    }

    pub fn has_more_lines(&self) -> bool {
        self.current_command < self.commands.len()
    }

    pub fn advance(&mut self) {
        if self.has_more_lines() {
            self.current_command += 1;
        }
    }

    pub fn command_type(&self) -> CommandType {
        if let Some(command) = self.commands.get(self.current_command) {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if let Some(first_part) = parts.first() {
                match *first_part {
                    "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                        CommandType::Arithmetic
                    }
                    "push" => CommandType::Push,
                    "pop" => CommandType::Pop,
                    "label" => CommandType::Label,
                    "goto" => CommandType::Goto,
                    "if-goto" => CommandType::If,
                    "function" => CommandType::Function,
                    "return" => CommandType::Return,
                    "call" => CommandType::Call,
                    _ => panic!("Unknown command: {}", first_part),
                }
            } else {
                panic!("Empty command");
            }
        } else {
            panic!("No current command");
        }
    }

    pub fn arg1(&self) -> String {
        if let Some(command) = self.commands.get(self.current_command) {
            let parts: Vec<&str> = command.split_whitespace().collect();
            match self.command_type() {
                CommandType::Arithmetic => parts[0].to_string(),
                CommandType::Push
                | CommandType::Pop
                | CommandType::Label
                | CommandType::Goto
                | CommandType::If => {
                    if parts.len() > 1 {
                        parts[1].to_string()
                    } else {
                        panic!("Missing first argument");
                    }
                }
                CommandType::Function | CommandType::Call => {
                    if parts.len() > 1 {
                        parts[1].to_string()
                    } else {
                        panic!("Missing function name");
                    }
                }
                CommandType::Return => panic!("Return command has no arguments"),
            }
        } else {
            panic!("No current command");
        }
    }

    pub fn arg2(&self) -> usize {
        if let Some(command) = self.commands.get(self.current_command) {
            let parts: Vec<&str> = command.split_whitespace().collect();
            match self.command_type() {
                CommandType::Push
                | CommandType::Pop
                | CommandType::Function
                | CommandType::Call => {
                    if parts.len() > 2 {
                        parts[2].parse().expect("Invalid number in arg2")
                    } else {
                        panic!("Missing second argument");
                    }
                }
                _ => panic!("Command type does not have arg2"),
            }
        } else {
            panic!("No current command");
        }
    }
}
