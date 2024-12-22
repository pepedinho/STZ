use std::collections::HashMap;

use super::structure::{Expr, SymbolTable};

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            var: HashMap::new(),
        }
    }

    pub fn set_var(&mut self, name: String, value: &Expr) {
        self.var.insert(name, value.clone());
    }

    pub fn get_var(&self, name: &str) -> Option<&Expr> {
        self.var.get(name)
    }
}
