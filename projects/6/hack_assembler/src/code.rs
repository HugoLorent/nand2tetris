//! The code module handles the translation of Hack assembly mnemonics to binary code.

/// Provides methods to translate Hack assembly language mnemonics into binary codes.
pub struct Code;

impl Code {
    /// Translates the destination part of a C-instruction into binary.
    ///
    /// # Arguments
    ///
    /// * `mnemonic` - The destination mnemonic (e.g., "M", "D", "MD", etc.)
    ///
    /// # Returns
    ///
    /// A string containing the 3-bit binary code
    pub fn dest(mnemonic: &str) -> String {
        match mnemonic {
            "M" => "001",   // Memory
            "D" => "010",   // Data Register
            "MD" => "011",  // Memory and Data Register
            "A" => "100",   // Address Register
            "AM" => "101",  // Address Register and Memory
            "AD" => "110",  // Address and Data Registers
            "AMD" => "111", // Address, Memory, and Data Register
            "" => "000",    // No destination
            _ => panic!("Invalid dest mnemonic"),
        }
        .to_string()
    }

    /// Translates the computation part of a C-instruction into binary.
    ///
    /// # Arguments
    ///
    /// * `mnemonic` - The computation mnemonic (e.g., "D+1", "A+1", "D&M", etc.)
    ///
    /// # Returns
    ///
    /// A string containing the 7-bit binary code
    pub fn comp(mnemonic: &str) -> String {
        match mnemonic {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            _ => panic!("Invalid comp mnemonic"),
        }
        .to_string()
    }

    /// Translates the jump part of a C-instruction into binary.
    ///
    /// # Arguments
    ///
    /// * `mnemonic` - The jump mnemonic (e.g., "JMP", "JEQ", "JGT", etc.)
    ///
    /// # Returns
    ///
    /// A string containing the 3-bit binary code
    pub fn jump(mnemonic: &str) -> String {
        match mnemonic {
            "JGT" => "001", // Jump if greater than zero
            "JEQ" => "010", // Jump if equal to zero
            "JGE" => "011", // Jump if greater or equal to zero
            "JLT" => "100", // Jump if less than zero
            "JNE" => "101", // Jump if not equal to zero
            "JLE" => "110", // Jump if less or equal to zero
            "JMP" => "111", // Jump unconditionally
            "" => "000",    // No jump
            _ => panic!("Invalid jump mnemonic"),
        }
        .to_string()
    }
}
