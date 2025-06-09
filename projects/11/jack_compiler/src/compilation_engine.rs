use crate::tokenizer::{Token, TokenType};
use crate::vm_writer::{VMWriter, Segment, Command};
use crate::symbol_table::{SymbolTable, Kind};

pub struct CompilationEngine {
    tokens: Vec<Token>,
    current: usize,
    output: String,
    indent_level: usize,
    vm_writer: VMWriter,
    symbol_table: SymbolTable,
    class_name: String,
    current_subroutine: String,
}

impl CompilationEngine {
    pub fn new(tokens: Vec<Token>) -> Self {
        CompilationEngine {
            tokens,
            current: 0,
            output: String::new(),
            indent_level: 0,
            vm_writer: VMWriter::new(),
            symbol_table: SymbolTable::new(),
            class_name: String::new(),
            current_subroutine: String::new(),
        }
    }

    pub fn compile_class(&mut self) -> String {
        self.output.clear();
        self.indent_level = 0;

        self.write_element_start("class");

        // 'class'
        self.consume_keyword("class");

        // className
        self.consume_identifier();

        // '{'
        self.consume_symbol("{");

        // classVarDec*
        while self.is_class_var_dec() {
            self.compile_class_var_dec();
        }

        // subroutineDec*
        while self.is_subroutine_dec() {
            self.compile_subroutine_dec();
        }

        // '}'
        self.consume_symbol("}");

        self.write_element_end("class");

        self.output.clone()
    }

    pub fn compile_class_to_vm(&mut self, class_name: &str) -> String {
        self.vm_writer.clear();
        self.symbol_table = SymbolTable::new();
        self.class_name = class_name.to_string();
        self.current = 0;

        // 'class'
        self.consume_keyword("class");

        // className
        self.class_name = self.consume_identifier_value();

        // '{'
        self.consume_symbol("{");

        // classVarDec*
        while self.is_class_var_dec() {
            self.compile_class_var_dec_vm();
        }

        // subroutineDec*
        while self.is_subroutine_dec() {
            self.compile_subroutine_dec_vm();
        }

        // '}'
        self.consume_symbol("}");

        self.vm_writer.get_output()
    }

    fn compile_class_var_dec_vm(&mut self) {
        // ('static' | 'field')
        let kind_token = self.consume_current_token_value();
        let kind = match kind_token.as_str() {
            "static" => Kind::Static,
            "field" => Kind::Field,
            _ => panic!("Expected static or field"),
        };

        // type
        let type_name = self.compile_type_vm();

        // varName
        let var_name = self.consume_identifier_value();
        self.symbol_table.define(var_name, type_name.clone(), kind.clone());

        // (',' varName)*
        while self.peek_symbol(",") {
            self.consume_symbol(",");
            let var_name = self.consume_identifier_value();
            self.symbol_table.define(var_name, type_name.clone(), kind.clone());
        }

        // ';'
        self.consume_symbol(";");
    }

    fn compile_subroutine_dec_vm(&mut self) {
        self.symbol_table.start_subroutine();

        // ('constructor' | 'function' | 'method')
        let subroutine_type = self.consume_current_token_value();

        // ('void' | type)
        let _return_type = if self.peek_keyword("void") {
            self.consume_keyword("void");
            "void".to_string()
        } else {
            self.compile_type_vm()
        };

        // subroutineName
        let subroutine_name = self.consume_identifier_value();
        self.current_subroutine = format!("{}.{}", self.class_name, subroutine_name);

        // '('
        self.consume_symbol("(");

        // For methods, 'this' is the first argument
        if subroutine_type == "method" {
            self.symbol_table.define("this".to_string(), self.class_name.clone(), Kind::Arg);
        }

        // parameterList
        self.compile_parameter_list_vm();

        // ')'
        self.consume_symbol(")");

        // subroutineBody
        self.compile_subroutine_body_vm(&subroutine_type);
    }

    fn compile_parameter_list_vm(&mut self) {
        if !self.peek_symbol(")") {
            // type
            let type_name = self.compile_type_vm();

            // varName
            let var_name = self.consume_identifier_value();
            self.symbol_table.define(var_name, type_name, Kind::Arg);

            // (',' type varName)*
            while self.peek_symbol(",") {
                self.consume_symbol(",");
                let type_name = self.compile_type_vm();
                let var_name = self.consume_identifier_value();
                self.symbol_table.define(var_name, type_name, Kind::Arg);
            }
        }
    }

    fn compile_subroutine_body_vm(&mut self, subroutine_type: &str) {
        // '{'
        self.consume_symbol("{");

        // varDec*
        while self.peek_keyword("var") {
            self.compile_var_dec_vm();
        }

        // Write function declaration
        let n_locals = self.symbol_table.var_count(&Kind::Var);
        self.vm_writer.write_function(&self.current_subroutine, n_locals);

        // Handle constructor/method setup
        match subroutine_type {
            "constructor" => {
                let n_fields = self.symbol_table.var_count(&Kind::Field);
                self.vm_writer.write_push(Segment::Constant, n_fields);
                self.vm_writer.write_call("Memory.alloc", 1);
                self.vm_writer.write_pop(Segment::Pointer, 0);
            }
            "method" => {
                self.vm_writer.write_push(Segment::Argument, 0);
                self.vm_writer.write_pop(Segment::Pointer, 0);
            }
            _ => {} // function - no setup needed
        }

        // statements
        self.compile_statements_vm();

        // '}'
        self.consume_symbol("}");
    }

    fn compile_var_dec_vm(&mut self) {
        // 'var'
        self.consume_keyword("var");

        // type
        let type_name = self.compile_type_vm();

        // varName
        let var_name = self.consume_identifier_value();
        self.symbol_table.define(var_name, type_name.clone(), Kind::Var);

        // (',' varName)*
        while self.peek_symbol(",") {
            self.consume_symbol(",");
            let var_name = self.consume_identifier_value();
            self.symbol_table.define(var_name, type_name.clone(), Kind::Var);
        }

        // ';'
        self.consume_symbol(";");
    }

    fn compile_type_vm(&mut self) -> String {
        if let Some(token) = self.peek_token() {
            match token.value.as_str() {
                "int" | "char" | "boolean" => {
                    self.consume_current_token_value()
                }
                _ => {
                    // className
                    self.consume_identifier_value()
                }
            }
        } else {
            panic!("Expected type");
        }
    }

    fn compile_statements_vm(&mut self) {
        while self.is_statement() {
            match self.peek_token().unwrap().value.as_str() {
                "let" => self.compile_let_statement_vm(),
                "if" => self.compile_if_statement_vm(),
                "while" => self.compile_while_statement_vm(),
                "do" => self.compile_do_statement_vm(),
                "return" => self.compile_return_statement_vm(),
                _ => break,
            }
        }
    }

    fn compile_let_statement_vm(&mut self) {
        // 'let'
        self.consume_keyword("let");

        // varName
        let var_name = self.consume_identifier_value();

        // Check if it's an array assignment
        let is_array = self.peek_symbol("[");
        
        if is_array {
            // Array assignment: let arr[i] = expression
            self.consume_symbol("[");
            
            // Push array base address
            self.compile_var_access(&var_name);
            
            // Compile index expression
            self.compile_expression_vm();
            
            self.consume_symbol("]");
            
            // arr + i
            self.vm_writer.write_arithmetic(Command::Add);
            
            // '='
            self.consume_symbol("=");
            
            // Compile RHS expression
            self.compile_expression_vm();
            
            // Set up that pointer and store
            self.vm_writer.write_pop(Segment::Temp, 0);  // Store value
            self.vm_writer.write_pop(Segment::Pointer, 1); // Set that
            self.vm_writer.write_push(Segment::Temp, 0);   // Restore value
            self.vm_writer.write_pop(Segment::That, 0);    // Store at that[0]
        } else {
            // Simple assignment: let var = expression
            // '='
            self.consume_symbol("=");
            
            // Compile RHS expression
            self.compile_expression_vm();
            
            // Store in variable
            if let Some(kind) = self.symbol_table.kind_of(&var_name) {
                let segment = kind.to_segment();
                let index = self.symbol_table.index_of(&var_name).unwrap();
                self.vm_writer.write_pop(segment, index);
            } else {
                panic!("Undefined variable: {}", var_name);
            }
        }

        // ';'
        self.consume_symbol(";");
    }

    fn compile_if_statement_vm(&mut self) {
        // 'if'
        self.consume_keyword("if");

        // '('
        self.consume_symbol("(");

        // expression
        self.compile_expression_vm();

        // ')'
        self.consume_symbol(")");

        // Negate condition for if-goto
        self.vm_writer.write_arithmetic(Command::Not);

        let else_label = self.vm_writer.generate_label("IF_ELSE");
        let end_label = self.vm_writer.generate_label("IF_END");

        self.vm_writer.write_if(&else_label);

        // '{'
        self.consume_symbol("{");

        // statements
        self.compile_statements_vm();

        // '}'
        self.consume_symbol("}");

        self.vm_writer.write_goto(&end_label);
        self.vm_writer.write_label(&else_label);

        // ('else' '{' statements '}')?
        if self.peek_keyword("else") {
            self.consume_keyword("else");
            self.consume_symbol("{");
            self.compile_statements_vm();
            self.consume_symbol("}");
        }

        self.vm_writer.write_label(&end_label);
    }

    fn compile_while_statement_vm(&mut self) {
        let loop_label = self.vm_writer.generate_label("WHILE_LOOP");
        let end_label = self.vm_writer.generate_label("WHILE_END");

        self.vm_writer.write_label(&loop_label);

        // 'while'
        self.consume_keyword("while");

        // '('
        self.consume_symbol("(");

        // expression
        self.compile_expression_vm();

        // ')'
        self.consume_symbol(")");

        // Negate condition
        self.vm_writer.write_arithmetic(Command::Not);
        self.vm_writer.write_if(&end_label);

        // '{'
        self.consume_symbol("{");

        // statements
        self.compile_statements_vm();

        // '}'
        self.consume_symbol("}");

        self.vm_writer.write_goto(&loop_label);
        self.vm_writer.write_label(&end_label);
    }

    fn compile_do_statement_vm(&mut self) {
        // 'do'
        self.consume_keyword("do");

        // subroutineCall
        self.compile_subroutine_call_vm();

        // Pop return value (do statements ignore return value)
        self.vm_writer.write_pop(Segment::Temp, 0);

        // ';'
        self.consume_symbol(";");
    }

    fn compile_return_statement_vm(&mut self) {
        // 'return'
        self.consume_keyword("return");

        // expression?
        if !self.peek_symbol(";") {
            self.compile_expression_vm();
        } else {
            // Void function - push 0
            self.vm_writer.write_push(Segment::Constant, 0);
        }

        // ';'
        self.consume_symbol(";");

        self.vm_writer.write_return();
    }

    fn compile_expression_vm(&mut self) {
        // term
        self.compile_term_vm();

        // (op term)*
        while self.is_op() {
            let op = self.consume_current_token_value();
            self.compile_term_vm();
            
            // Generate VM command for operator
            match op.as_str() {
                "+" => self.vm_writer.write_arithmetic(Command::Add),
                "-" => self.vm_writer.write_arithmetic(Command::Sub),
                "*" => self.vm_writer.write_call("Math.multiply", 2),
                "/" => self.vm_writer.write_call("Math.divide", 2),
                "&" => self.vm_writer.write_arithmetic(Command::And),
                "|" => self.vm_writer.write_arithmetic(Command::Or),
                "<" => self.vm_writer.write_arithmetic(Command::Lt),
                ">" => self.vm_writer.write_arithmetic(Command::Gt),
                "=" => self.vm_writer.write_arithmetic(Command::Eq),
                _ => panic!("Unknown operator: {}", op),
            }
        }
    }

    fn compile_term_vm(&mut self) {
        if let Some(token) = self.peek_token() {
            match token.token_type {
                TokenType::IntegerConstant => {
                    let value = self.consume_current_token_value();
                    let int_val: usize = value.parse().expect("Invalid integer");
                    self.vm_writer.write_push(Segment::Constant, int_val);
                }
                TokenType::StringConstant => {
                    let string_val = self.consume_current_token_value();
                    self.compile_string_constant(&string_val);
                }
                TokenType::Keyword => {
                    let keyword = self.consume_current_token_value();
                    match keyword.as_str() {
                        "true" => {
                            self.vm_writer.write_push(Segment::Constant, 1);
                            self.vm_writer.write_arithmetic(Command::Neg);
                        }
                        "false" | "null" => {
                            self.vm_writer.write_push(Segment::Constant, 0);
                        }
                        "this" => {
                            self.vm_writer.write_push(Segment::Pointer, 0);
                        }
                        _ => panic!("Unexpected keyword in term: {}", keyword),
                    }
                }
                TokenType::Identifier => {
                    let identifier = self.consume_identifier_value();
                    
                    if self.peek_symbol("[") {
                        // Array access: arr[index]
                        self.consume_symbol("[");
                        
                        // Push array base
                        self.compile_var_access(&identifier);
                        
                        // Push index
                        self.compile_expression_vm();
                        
                        self.consume_symbol("]");
                        
                        // Add base + index
                        self.vm_writer.write_arithmetic(Command::Add);
                        
                        // Pop address to that pointer and push that[0]
                        self.vm_writer.write_pop(Segment::Pointer, 1);
                        self.vm_writer.write_push(Segment::That, 0);
                        
                    } else if self.peek_symbol("(") || self.peek_symbol(".") {
                        // Subroutine call
                        self.compile_identifier_subroutine_call(&identifier);
                    } else {
                        // Variable access
                        self.compile_var_access(&identifier);
                    }
                }
                TokenType::Symbol => {
                    if token.value == "(" {
                        // '(' expression ')'
                        self.consume_symbol("(");
                        self.compile_expression_vm();
                        self.consume_symbol(")");
                    } else if matches!(token.value.as_str(), "-" | "~") {
                        // unaryOp term
                        let op = self.consume_current_token_value();
                        self.compile_term_vm();
                        
                        match op.as_str() {
                            "-" => self.vm_writer.write_arithmetic(Command::Neg),
                            "~" => self.vm_writer.write_arithmetic(Command::Not),
                            _ => panic!("Unknown unary operator: {}", op),
                        }
                    }
                }
            }
        }
    }

    fn compile_string_constant(&mut self, string_val: &str) {
        // Create new string object
        self.vm_writer.write_push(Segment::Constant, string_val.len());
        self.vm_writer.write_call("String.new", 1);
        
        // Append each character
        for ch in string_val.chars() {
            self.vm_writer.write_push(Segment::Constant, ch as usize);
            self.vm_writer.write_call("String.appendChar", 2);
        }
    }

    fn compile_var_access(&mut self, var_name: &str) {
        if let Some(kind) = self.symbol_table.kind_of(var_name) {
            let segment = kind.to_segment();
            let index = self.symbol_table.index_of(var_name).unwrap();
            self.vm_writer.write_push(segment, index);
        } else {
            panic!("Undefined variable: {}", var_name);
        }
    }

    fn compile_identifier_subroutine_call(&mut self, identifier: &str) {
        if self.peek_symbol(".") {
            // Method/function call: object.method() or Class.function()
            self.consume_symbol(".");
            let method_name = self.consume_identifier_value();
            
            self.consume_symbol("(");
            
            let mut n_args = 0;
            
            // Check if identifier is a variable (method call) or class name (function call)
            if self.symbol_table.is_defined(identifier) {
                // Method call - push object reference
                self.compile_var_access(identifier);
                n_args += 1;
                
                let object_type = self.symbol_table.type_of(identifier).unwrap();
                let full_name = format!("{}.{}", object_type, method_name);
                
                n_args += self.compile_expression_list_vm();
                self.vm_writer.write_call(&full_name, n_args);
            } else {
                // Function call or constructor
                let full_name = format!("{}.{}", identifier, method_name);
                n_args += self.compile_expression_list_vm();
                self.vm_writer.write_call(&full_name, n_args);
            }
            
            self.consume_symbol(")");
        } else {
            // Method call on current object: method()
            self.consume_symbol("(");
            
            // Push 'this'
            self.vm_writer.write_push(Segment::Pointer, 0);
            let mut n_args = 1;
            
            let full_name = format!("{}.{}", self.class_name, identifier);
            n_args += self.compile_expression_list_vm();
            
            self.consume_symbol(")");
            self.vm_writer.write_call(&full_name, n_args);
        }
    }

    fn compile_subroutine_call_vm(&mut self) {
        let identifier = self.consume_identifier_value();
        self.compile_identifier_subroutine_call(&identifier);
    }

    fn compile_expression_list_vm(&mut self) -> usize {
        let mut n_args = 0;
        
        if !self.peek_symbol(")") {
            self.compile_expression_vm();
            n_args += 1;
            
            while self.peek_symbol(",") {
                self.consume_symbol(",");
                self.compile_expression_vm();
                n_args += 1;
            }
        }
        
        n_args
    }

    // Helper methods for VM compilation
    fn consume_current_token_value(&mut self) -> String {
        if let Some(token) = self.tokens.get(self.current) {
            let value = token.value.clone();
            self.current += 1;
            value
        } else {
            panic!("No more tokens to consume");
        }
    }

    fn consume_identifier_value(&mut self) -> String {
        if let Some(token) = self.peek_token() {
            if token.token_type == TokenType::Identifier {
                self.consume_current_token_value()
            } else {
                panic!("Expected identifier, found: {:?}", token);
            }
        } else {
            panic!("Expected identifier, but no more tokens");
        }
    }

    fn is_class_var_dec(&self) -> bool {
        if let Some(token) = self.peek_token() {
            matches!(token.value.as_str(), "static" | "field")
        } else {
            false
        }
    }

    fn is_subroutine_dec(&self) -> bool {
        if let Some(token) = self.peek_token() {
            matches!(token.value.as_str(), "constructor" | "function" | "method")
        } else {
            false
        }
    }

    fn compile_class_var_dec(&mut self) {
        self.write_element_start("classVarDec");

        // ('static' | 'field')
        self.consume_current_token();

        // type
        self.compile_type();

        // varName
        self.consume_identifier();

        // (',' varName)*
        while self.peek_symbol(",") {
            self.consume_symbol(",");
            self.consume_identifier();
        }

        // ';'
        self.consume_symbol(";");

        self.write_element_end("classVarDec");
    }

    fn compile_type(&mut self) {
        if let Some(token) = self.peek_token() {
            match token.value.as_str() {
                "int" | "char" | "boolean" => {
                    self.consume_current_token();
                }
                _ => {
                    // className
                    self.consume_identifier();
                }
            }
        }
    }

    fn compile_subroutine_dec(&mut self) {
        self.write_element_start("subroutineDec");

        // ('constructor' | 'function' | 'method')
        self.consume_current_token();

        // ('void' | type)
        if self.peek_keyword("void") {
            self.consume_keyword("void");
        } else {
            self.compile_type();
        }

        // subroutineName
        self.consume_identifier();

        // '('
        self.consume_symbol("(");

        // parameterList
        self.compile_parameter_list();

        // ')'
        self.consume_symbol(")");

        // subroutineBody
        self.compile_subroutine_body();

        self.write_element_end("subroutineDec");
    }

    fn compile_parameter_list(&mut self) {
        self.write_element_start("parameterList");

        if !self.peek_symbol(")") {
            // type
            self.compile_type();

            // varName
            self.consume_identifier();

            // (',' type varName)*
            while self.peek_symbol(",") {
                self.consume_symbol(",");
                self.compile_type();
                self.consume_identifier();
            }
        }

        self.write_element_end("parameterList");
    }

    fn compile_subroutine_body(&mut self) {
        self.write_element_start("subroutineBody");

        // '{'
        self.consume_symbol("{");

        // varDec*
        while self.peek_keyword("var") {
            self.compile_var_dec();
        }

        // statements
        self.compile_statements();

        // '}'
        self.consume_symbol("}");

        self.write_element_end("subroutineBody");
    }

    fn compile_var_dec(&mut self) {
        self.write_element_start("varDec");

        // 'var'
        self.consume_keyword("var");

        // type
        self.compile_type();

        // varName
        self.consume_identifier();

        // (',' varName)*
        while self.peek_symbol(",") {
            self.consume_symbol(",");
            self.consume_identifier();
        }

        // ';'
        self.consume_symbol(";");

        self.write_element_end("varDec");
    }

    fn compile_statements(&mut self) {
        self.write_element_start("statements");

        while self.is_statement() {
            match self.peek_token().unwrap().value.as_str() {
                "let" => self.compile_let_statement(),
                "if" => self.compile_if_statement(),
                "while" => self.compile_while_statement(),
                "do" => self.compile_do_statement(),
                "return" => self.compile_return_statement(),
                _ => break,
            }
        }

        self.write_element_end("statements");
    }

    fn is_statement(&self) -> bool {
        if let Some(token) = self.peek_token() {
            matches!(
                token.value.as_str(),
                "let" | "if" | "while" | "do" | "return"
            )
        } else {
            false
        }
    }

    fn compile_let_statement(&mut self) {
        self.write_element_start("letStatement");

        // 'let'
        self.consume_keyword("let");

        // varName
        self.consume_identifier();

        // ('[' expression ']')?
        if self.peek_symbol("[") {
            self.consume_symbol("[");
            self.compile_expression();
            self.consume_symbol("]");
        }

        // '='
        self.consume_symbol("=");

        // expression
        self.compile_expression();

        // ';'
        self.consume_symbol(";");

        self.write_element_end("letStatement");
    }

    fn compile_if_statement(&mut self) {
        self.write_element_start("ifStatement");

        // 'if'
        self.consume_keyword("if");

        // '('
        self.consume_symbol("(");

        // expression
        self.compile_expression();

        // ')'
        self.consume_symbol(")");

        // '{'
        self.consume_symbol("{");

        // statements
        self.compile_statements();

        // '}'
        self.consume_symbol("}");

        // ('else' '{' statements '}')?
        if self.peek_keyword("else") {
            self.consume_keyword("else");
            self.consume_symbol("{");
            self.compile_statements();
            self.consume_symbol("}");
        }

        self.write_element_end("ifStatement");
    }

    fn compile_while_statement(&mut self) {
        self.write_element_start("whileStatement");

        // 'while'
        self.consume_keyword("while");

        // '('
        self.consume_symbol("(");

        // expression
        self.compile_expression();

        // ')'
        self.consume_symbol(")");

        // '{'
        self.consume_symbol("{");

        // statements
        self.compile_statements();

        // '}'
        self.consume_symbol("}");

        self.write_element_end("whileStatement");
    }

    fn compile_do_statement(&mut self) {
        self.write_element_start("doStatement");

        // 'do'
        self.consume_keyword("do");

        // subroutineCall
        self.compile_subroutine_call();

        // ';'
        self.consume_symbol(";");

        self.write_element_end("doStatement");
    }

    fn compile_return_statement(&mut self) {
        self.write_element_start("returnStatement");

        // 'return'
        self.consume_keyword("return");

        // expression?
        if !self.peek_symbol(";") {
            self.compile_expression();
        }

        // ';'
        self.consume_symbol(";");

        self.write_element_end("returnStatement");
    }

    fn compile_expression(&mut self) {
        self.write_element_start("expression");

        // term
        self.compile_term();

        // (op term)*
        while self.is_op() {
            self.consume_current_token(); // op
            self.compile_term();
        }

        self.write_element_end("expression");
    }

    fn is_op(&self) -> bool {
        if let Some(token) = self.peek_token() {
            matches!(
                token.value.as_str(),
                "+" | "-" | "*" | "/" | "&" | "|" | "<" | ">" | "="
            )
        } else {
            false
        }
    }

    fn compile_term(&mut self) {
        self.write_element_start("term");

        if let Some(token) = self.peek_token() {
            match token.token_type {
                TokenType::IntegerConstant => {
                    self.consume_current_token();
                }
                TokenType::StringConstant => {
                    self.consume_current_token();
                }
                TokenType::Keyword => {
                    // true | false | null | this
                    self.consume_current_token();
                }
                TokenType::Identifier => {
                    self.consume_identifier();

                    if self.peek_symbol("[") {
                        // varName '[' expression ']'
                        self.consume_symbol("[");
                        self.compile_expression();
                        self.consume_symbol("]");
                    } else if self.peek_symbol("(") || self.peek_symbol(".") {
                        // subroutineCall (we already consumed the first identifier)
                        if self.peek_symbol(".") {
                            self.consume_symbol(".");
                            self.consume_identifier();
                        }
                        self.consume_symbol("(");
                        self.compile_expression_list();
                        self.consume_symbol(")");
                    }
                }
                TokenType::Symbol => {
                    if token.value == "(" {
                        // '(' expression ')'
                        self.consume_symbol("(");
                        self.compile_expression();
                        self.consume_symbol(")");
                    } else if matches!(token.value.as_str(), "-" | "~") {
                        // unaryOp term
                        self.consume_current_token();
                        self.compile_term();
                    }
                }
            }
        }

        self.write_element_end("term");
    }

    fn compile_subroutine_call(&mut self) {
        // subroutineName '(' expressionList ')' |
        // (className | varName) '.' subroutineName '(' expressionList ')'

        self.consume_identifier();

        if self.peek_symbol(".") {
            self.consume_symbol(".");
            self.consume_identifier();
        }

        self.consume_symbol("(");
        self.compile_expression_list();
        self.consume_symbol(")");
    }

    fn compile_expression_list(&mut self) {
        self.write_element_start("expressionList");

        if !self.peek_symbol(")") {
            self.compile_expression();

            while self.peek_symbol(",") {
                self.consume_symbol(",");
                self.compile_expression();
            }
        }

        self.write_element_end("expressionList");
    }

    // Helper methods
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_keyword(&self, keyword: &str) -> bool {
        if let Some(token) = self.peek_token() {
            token.token_type == TokenType::Keyword && token.value == keyword
        } else {
            false
        }
    }

    fn peek_symbol(&self, symbol: &str) -> bool {
        if let Some(token) = self.peek_token() {
            token.token_type == TokenType::Symbol && token.value == symbol
        } else {
            false
        }
    }

    fn consume_current_token(&mut self) {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.write_token(&token);
            self.current += 1;
        } else {
            panic!("No more tokens to consume");
        }
    }

    fn consume_keyword(&mut self, expected: &str) {
        if let Some(token) = self.peek_token() {
            if token.token_type == TokenType::Keyword && token.value == expected {
                self.consume_current_token();
            } else {
                panic!("Expected keyword '{}', found: {:?}", expected, token);
            }
        } else {
            panic!("Expected keyword '{}', but no more tokens", expected);
        }
    }

    fn consume_symbol(&mut self, expected: &str) {
        if let Some(token) = self.peek_token() {
            if token.token_type == TokenType::Symbol && token.value == expected {
                self.consume_current_token();
            } else {
                panic!("Expected symbol '{}', found: {:?}", expected, token);
            }
        } else {
            panic!("Expected symbol '{}', but no more tokens", expected);
        }
    }

    fn consume_identifier(&mut self) {
        if let Some(token) = self.peek_token() {
            if token.token_type == TokenType::Identifier {
                self.consume_current_token();
            } else {
                panic!("Expected identifier, found: {:?}", token);
            }
        } else {
            panic!("Expected identifier, but no more tokens");
        }
    }

    fn write_element_start(&mut self, element: &str) {
        self.write_indent();
        self.output.push_str(&format!("<{}>\n", element));
        self.indent_level += 1;
    }

    fn write_element_end(&mut self, element: &str) {
        self.indent_level -= 1;
        self.write_indent();
        self.output.push_str(&format!("</{}>\n", element));
    }

    fn write_token(&mut self, token: &Token) {
        self.write_indent();

        let tag = match token.token_type {
            TokenType::Keyword => "keyword",
            TokenType::Symbol => "symbol",
            TokenType::IntegerConstant => "integerConstant",
            TokenType::StringConstant => "stringConstant",
            TokenType::Identifier => "identifier",
        };

        let escaped_value = match token.value.as_str() {
            "<" => "&lt;".to_string(),
            ">" => "&gt;".to_string(),
            "&" => "&amp;".to_string(),
            "\"" => "&quot;".to_string(),
            _ => token.value.clone(),
        };

        self.output
            .push_str(&format!("<{tag}> {escaped_value} </{tag}>\n"));
    }

    fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str("  ");
        }
    }
}
