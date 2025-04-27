//! The parser module handles the parsing of Hack assembly language commands.

/// Represents the type of command in the Hack assembly language.
#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum CommandType {
    /// A-instruction: @value (loads value into A register)
    ACommand,
    /// C-instruction: dest=comp;jump (computation instruction)
    CCommand,
    /// L-instruction: (label) (pseudo-command for labels)
    LCommand,
}

pub struct Parser {
    commands: Vec<String>,
    current_command: usize,
}

impl Parser {
    /// Creates a new Parser instance from input string.
    ///
    /// # Arguments
    ///
    /// * `input` - The input assembly code as a string
    ///
    /// # Returns
    ///
    /// A new Parser instance with the cleaned up commands
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

    /// Checks if there are more commands to process.
    ///
    /// # Returns
    ///
    /// `true` if there are more commands, `false` otherwise
    pub fn has_more_commands(&self) -> bool {
        self.current_command < self.commands.len()
    }

    /// Advances to the next command.
    pub fn advance(&mut self) {
        if self.has_more_commands() {
            self.current_command += 1;
        }
    }

    /// Returns the type of the current command.
    ///
    /// # Returns
    ///
    /// An Option containing the CommandType of the current command
    pub fn command_type(&self) -> Option<CommandType> {
        let command = self.commands.get(self.current_command)?;

        if command.starts_with('@') {
            Some(CommandType::ACommand)
        } else if command.starts_with('(') && command.ends_with(')') {
            Some(CommandType::LCommand)
        } else {
            Some(CommandType::CCommand)
        }
    }

    /// Returns the symbol or decimal number of the current command.
    /// Should be called only when command_type() is A_COMMAND or L_COMMAND.
    ///
    /// # Returns
    ///
    /// An Option containing the symbol as a string
    pub fn symbol(&self) -> Option<&str> {
        let command = self.commands.get(self.current_command)?;

        match self.command_type()? {
            CommandType::ACommand => Some(&command[1..]), // Skip the '@'
            CommandType::LCommand => Some(&command[1..command.len() - 1]), // Remove '(' and ')'
            CommandType::CCommand => None,
        }
    }

    /// Returns the dest mnemonic in the current C-command.
    /// If no dest mnemonic is present, returns empty string.
    /// Should be called only when command_type() is C_COMMAND.
    ///
    /// # Returns
    ///
    /// An Option containing the dest mnemonic as a string
    pub fn dest(&self) -> Option<&str> {
        let command = self.commands.get(self.current_command)?;
        if let Some(i) = command.find('=') {
            Some(&command[..i])
        } else {
            Some("")
        }
    }

    /// Returns the comp mnemonic in the current C-command.
    /// Should be called only when command_type() is C_COMMAND.
    ///
    /// # Returns
    ///
    /// An Option containing the comp mnemonic as a string
    pub fn comp(&self) -> Option<&str> {
        let command = self.commands.get(self.current_command)?;
        let start = command.find('=').map_or(0, |i| i + 1);
        let end = command.find(';').unwrap_or(command.len());
        Some(&command[start..end])
    }

    /// Returns the jump mnemonic in the current C-command.
    /// If no jump mnemonic is present, returns empty string.
    /// Should be called only when command_type() is C_COMMAND.
    ///
    /// # Returns
    ///
    /// An Option containing the jump mnemonic as a string
    pub fn jump(&self) -> Option<&str> {
        let command = self.commands.get(self.current_command)?;
        if let Some(i) = command.find(';') {
            Some(&command[i + 1..])
        } else {
            Some("")
        }
    }
}
