use crate::compilation_engine::CompilationEngine;
use crate::tokenizer::JackTokenizer;

pub struct JackAnalyzer;

impl JackAnalyzer {
    pub fn new() -> Self {
        JackAnalyzer
    }

    pub fn tokenize(&mut self, input: &str) -> String {
        let mut tokenizer = JackTokenizer::new(input);
        tokenizer.tokenize();
        tokenizer.to_xml()
    }

    pub fn parse(&mut self, input: &str) -> String {
        let mut tokenizer = JackTokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let mut compilation_engine = CompilationEngine::new(tokens);
        compilation_engine.compile_class()
    }

    pub fn compile(&mut self, input: &str, class_name: &str) -> String {
        let mut tokenizer = JackTokenizer::new(input);
        let tokens = tokenizer.tokenize();

        let mut compilation_engine = CompilationEngine::new(tokens);
        compilation_engine.compile_class_to_vm(class_name)
    }
}

impl Default for JackAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
