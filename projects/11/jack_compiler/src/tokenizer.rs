use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    Symbol,
    IntegerConstant,
    StringConstant,
    Identifier,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct JackTokenizer {
    input: Vec<char>,
    current: usize,
    tokens: Vec<Token>,
    keywords: HashSet<String>,
    symbols: HashSet<char>,
}

impl JackTokenizer {
    pub fn new(input: &str) -> Self {
        let mut keywords = HashSet::new();
        keywords.insert("class".to_string());
        keywords.insert("constructor".to_string());
        keywords.insert("function".to_string());
        keywords.insert("method".to_string());
        keywords.insert("field".to_string());
        keywords.insert("static".to_string());
        keywords.insert("var".to_string());
        keywords.insert("int".to_string());
        keywords.insert("char".to_string());
        keywords.insert("boolean".to_string());
        keywords.insert("void".to_string());
        keywords.insert("true".to_string());
        keywords.insert("false".to_string());
        keywords.insert("null".to_string());
        keywords.insert("this".to_string());
        keywords.insert("let".to_string());
        keywords.insert("do".to_string());
        keywords.insert("if".to_string());
        keywords.insert("else".to_string());
        keywords.insert("while".to_string());
        keywords.insert("return".to_string());

        let mut symbols = HashSet::new();
        symbols.insert('{');
        symbols.insert('}');
        symbols.insert('(');
        symbols.insert(')');
        symbols.insert('[');
        symbols.insert(']');
        symbols.insert('.');
        symbols.insert(',');
        symbols.insert(';');
        symbols.insert('+');
        symbols.insert('-');
        symbols.insert('*');
        symbols.insert('/');
        symbols.insert('&');
        symbols.insert('|');
        symbols.insert('<');
        symbols.insert('>');
        symbols.insert('=');
        symbols.insert('~');

        let cleaned_input = Self::remove_comments(input);

        JackTokenizer {
            input: cleaned_input.chars().collect(),
            current: 0,
            tokens: Vec::new(),
            keywords,
            symbols,
        }
    }

    fn remove_comments(input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '/' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '/' {
                        // Line comment - skip until end of line
                        chars.next(); // consume second '/'
                        while let Some(ch) = chars.next() {
                            if ch == '\n' {
                                result.push(ch);
                                break;
                            }
                        }
                    } else if next_ch == '*' {
                        // Block comment - skip until */
                        chars.next(); // consume '*'
                        let mut found_end = false;
                        while let Some(ch) = chars.next() {
                            if ch == '*' {
                                if let Some(&next_ch) = chars.peek() {
                                    if next_ch == '/' {
                                        chars.next(); // consume '/'
                                        found_end = true;
                                        break;
                                    }
                                }
                            }
                        }
                        if !found_end {
                            panic!("Unterminated block comment");
                        }
                    } else {
                        result.push(ch);
                    }
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while self.current < self.input.len() {
            self.skip_whitespace();

            if self.current >= self.input.len() {
                break;
            }

            let ch = self.input[self.current];

            if ch.is_ascii_digit() {
                self.read_integer_constant();
            } else if ch == '"' {
                self.read_string_constant();
            } else if self.symbols.contains(&ch) {
                self.read_symbol();
            } else if ch.is_ascii_alphabetic() || ch == '_' {
                self.read_identifier_or_keyword();
            } else {
                panic!("Unexpected character: {}", ch);
            }
        }

        self.tokens.clone()
    }

    fn skip_whitespace(&mut self) {
        while self.current < self.input.len() && self.input[self.current].is_whitespace() {
            self.current += 1;
        }
    }

    fn read_integer_constant(&mut self) {
        let start = self.current;
        while self.current < self.input.len() && self.input[self.current].is_ascii_digit() {
            self.current += 1;
        }

        let value: String = self.input[start..self.current].iter().collect();
        let int_val: i32 = value.parse().expect("Invalid integer");

        if int_val < 0 || int_val > 32767 {
            panic!("Integer constant out of range: {}", int_val);
        }

        self.tokens.push(Token {
            token_type: TokenType::IntegerConstant,
            value,
        });
    }

    fn read_string_constant(&mut self) {
        self.current += 1; // skip opening quote
        let start = self.current;

        while self.current < self.input.len() && self.input[self.current] != '"' {
            if self.input[self.current] == '\n' {
                panic!("Unterminated string constant");
            }
            self.current += 1;
        }

        if self.current >= self.input.len() {
            panic!("Unterminated string constant");
        }

        let value: String = self.input[start..self.current].iter().collect();
        self.current += 1; // skip closing quote

        self.tokens.push(Token {
            token_type: TokenType::StringConstant,
            value,
        });
    }

    fn read_symbol(&mut self) {
        let ch = self.input[self.current];
        self.current += 1;

        self.tokens.push(Token {
            token_type: TokenType::Symbol,
            value: ch.to_string(),
        });
    }

    fn read_identifier_or_keyword(&mut self) {
        let start = self.current;

        while self.current < self.input.len() {
            let ch = self.input[self.current];
            if ch.is_ascii_alphanumeric() || ch == '_' {
                self.current += 1;
            } else {
                break;
            }
        }

        let value: String = self.input[start..self.current].iter().collect();

        let token_type = if self.keywords.contains(&value) {
            TokenType::Keyword
        } else {
            TokenType::Identifier
        };

        self.tokens.push(Token { token_type, value });
    }

    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<tokens>\n");

        for token in &self.tokens {
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

            xml.push_str(&format!("<{tag}> {escaped_value} </{tag}>\n"));
        }

        xml.push_str("</tokens>\n");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let mut tokenizer = JackTokenizer::new("class Main { }");
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "class");
        assert_eq!(tokens[0].token_type, TokenType::Keyword);
        assert_eq!(tokens[1].value, "Main");
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
    }

    #[test]
    fn test_remove_comments() {
        let input = "// This is a comment\nclass Main /* block comment */ { }";
        let cleaned = JackTokenizer::remove_comments(input);
        assert_eq!(cleaned, "\nclass Main  { }");
    }
}
