use crate::tokenizer::{Token, TokenType};

pub struct CompilationEngine {
    tokens: Vec<Token>,
    current: usize,
    output: String,
    indent_level: usize,
}

impl CompilationEngine {
    pub fn new(tokens: Vec<Token>) -> Self {
        CompilationEngine {
            tokens,
            current: 0,
            output: String::new(),
            indent_level: 0,
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
