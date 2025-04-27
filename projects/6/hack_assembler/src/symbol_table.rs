//! The symbol table module manages symbols and their corresponding addresses in the Hack assembly.

use std::collections::HashMap;

/// Maintains the symbol table for the Hack assembler.
/// Maps symbols to their numeric addresses.
pub struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    /// Creates a new symbol table with predefined symbols.
    ///
    /// # Returns
    ///
    /// A new SymbolTable instance initialized with all predefined symbols
    pub fn new() -> Self {
        let mut table = HashMap::new();

        // Predefined symbols
        table.insert("SP".to_string(), 0); // Stack pointer
        table.insert("LCL".to_string(), 1); // Local segment
        table.insert("ARG".to_string(), 2); // Argument segment
        table.insert("THIS".to_string(), 3); // This segment
        table.insert("THAT".to_string(), 4); // That segment

        // Register symbols R0-R15
        for i in 0..16 {
            table.insert(format!("R{}", i), i);
        }

        // I/O mappings
        table.insert("SCREEN".to_string(), 16384);
        table.insert("KBD".to_string(), 24576);

        SymbolTable { table }
    }

    /// Adds a new symbol-address pair to the table.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to add
    /// * `address` - The corresponding address
    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }

    /// Checks if a symbol exists in the table.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to look up
    ///
    /// # Returns
    ///
    /// `true` if the symbol exists, `false` otherwise
    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    /// Gets the address associated with a symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to look up
    ///
    /// # Returns
    ///
    /// An Option containing the address if the symbol exists
    pub fn get_address(&self, symbol: &str) -> Option<u16> {
        self.table.get(symbol).copied()
    }
}
