use std::collections::HashMap;

/// Kinds of identifiers in Jack
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Static,
    Field,
    Arg,
    Var,
}

impl Kind {
    pub fn to_segment(&self) -> crate::vm_writer::Segment {
        match self {
            Kind::Static => crate::vm_writer::Segment::Static,
            Kind::Field => crate::vm_writer::Segment::This,
            Kind::Arg => crate::vm_writer::Segment::Argument,
            Kind::Var => crate::vm_writer::Segment::Local,
        }
    }
}

/// Symbol entry containing type, kind, and index
#[derive(Debug, Clone)]
pub struct Symbol {
    pub type_name: String,
    pub kind: Kind,
    pub index: usize,
}

/// Symbol table for managing variable scopes
pub struct SymbolTable {
    class_scope: HashMap<String, Symbol>,
    subroutine_scope: HashMap<String, Symbol>,
    static_count: usize,
    field_count: usize,
    arg_count: usize,
    var_count: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            class_scope: HashMap::new(),
            subroutine_scope: HashMap::new(),
            static_count: 0,
            field_count: 0,
            arg_count: 0,
            var_count: 0,
        }
    }

    /// Start a new subroutine scope
    pub fn start_subroutine(&mut self) {
        self.subroutine_scope.clear();
        self.arg_count = 0;
        self.var_count = 0;
    }

    /// Define a new identifier
    pub fn define(&mut self, name: String, type_name: String, kind: Kind) {
        let index = match kind {
            Kind::Static => {
                let index = self.static_count;
                self.static_count += 1;
                index
            }
            Kind::Field => {
                let index = self.field_count;
                self.field_count += 1;
                index
            }
            Kind::Arg => {
                let index = self.arg_count;
                self.arg_count += 1;
                index
            }
            Kind::Var => {
                let index = self.var_count;
                self.var_count += 1;
                index
            }
        };

        let symbol = Symbol {
            type_name,
            kind: kind.clone(),
            index,
        };

        match kind {
            Kind::Static | Kind::Field => {
                self.class_scope.insert(name, symbol);
            }
            Kind::Arg | Kind::Var => {
                self.subroutine_scope.insert(name, symbol);
            }
        }
    }

    /// Get the number of variables of a given kind
    pub fn var_count(&self, kind: &Kind) -> usize {
        match kind {
            Kind::Static => self.static_count,
            Kind::Field => self.field_count,
            Kind::Arg => self.arg_count,
            Kind::Var => self.var_count,
        }
    }

    /// Get the kind of an identifier
    pub fn kind_of(&self, name: &str) -> Option<Kind> {
        if let Some(symbol) = self.subroutine_scope.get(name) {
            Some(symbol.kind.clone())
        } else if let Some(symbol) = self.class_scope.get(name) {
            Some(symbol.kind.clone())
        } else {
            None
        }
    }

    /// Get the type of an identifier
    pub fn type_of(&self, name: &str) -> Option<String> {
        if let Some(symbol) = self.subroutine_scope.get(name) {
            Some(symbol.type_name.clone())
        } else if let Some(symbol) = self.class_scope.get(name) {
            Some(symbol.type_name.clone())
        } else {
            None
        }
    }

    /// Get the index of an identifier
    pub fn index_of(&self, name: &str) -> Option<usize> {
        if let Some(symbol) = self.subroutine_scope.get(name) {
            Some(symbol.index)
        } else if let Some(symbol) = self.class_scope.get(name) {
            Some(symbol.index)
        } else {
            None
        }
    }

    /// Check if an identifier is defined
    pub fn is_defined(&self, name: &str) -> bool {
        self.subroutine_scope.contains_key(name) || self.class_scope.contains_key(name)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_variables() {
        let mut table = SymbolTable::new();
        
        table.define("x".to_string(), "int".to_string(), Kind::Field);
        table.define("y".to_string(), "int".to_string(), Kind::Field);
        table.define("count".to_string(), "int".to_string(), Kind::Static);
        
        assert_eq!(table.kind_of("x"), Some(Kind::Field));
        assert_eq!(table.index_of("x"), Some(0));
        assert_eq!(table.index_of("y"), Some(1));
        assert_eq!(table.index_of("count"), Some(0));
        assert_eq!(table.var_count(&Kind::Field), 2);
        assert_eq!(table.var_count(&Kind::Static), 1);
    }

    #[test]
    fn test_subroutine_variables() {
        let mut table = SymbolTable::new();
        
        table.start_subroutine();
        table.define("a".to_string(), "int".to_string(), Kind::Arg);
        table.define("b".to_string(), "boolean".to_string(), Kind::Arg);
        table.define("temp".to_string(), "int".to_string(), Kind::Var);
        
        assert_eq!(table.kind_of("a"), Some(Kind::Arg));
        assert_eq!(table.index_of("a"), Some(0));
        assert_eq!(table.index_of("b"), Some(1));
        assert_eq!(table.index_of("temp"), Some(0));
        assert_eq!(table.var_count(&Kind::Arg), 2);
        assert_eq!(table.var_count(&Kind::Var), 1);
    }

    #[test]
    fn test_scope_resolution() {
        let mut table = SymbolTable::new();
        
        // Class scope
        table.define("field_var".to_string(), "int".to_string(), Kind::Field);
        
        // Subroutine scope
        table.start_subroutine();
        table.define("local_var".to_string(), "int".to_string(), Kind::Var);
        
        // Subroutine scope should take precedence
        table.define("field_var".to_string(), "boolean".to_string(), Kind::Var);
        
        assert_eq!(table.kind_of("field_var"), Some(Kind::Var));
        assert_eq!(table.type_of("field_var"), Some("boolean".to_string()));
        assert_eq!(table.kind_of("local_var"), Some(Kind::Var));
    }
}