// src/symbol_table.rs
use std::collections::HashMap;
use crate::ast::Type;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, Symbol>>,
    pub errors: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { scopes: vec![HashMap::new()], errors: Vec::new() }
    }

    pub fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 { self.scopes.pop(); }
    }

    pub fn declare(&mut self, sym: Symbol) {
        self.scopes.last_mut().unwrap().insert(sym.name.clone(), sym);
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(s) = scope.get(name) { return Some(s); }
        }
        None
    }

    pub fn error(&mut self, msg: String) { self.errors.push(msg); }

    pub fn has_errors(&self) -> bool { !self.errors.is_empty() }
}
