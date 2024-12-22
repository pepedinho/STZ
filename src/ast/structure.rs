use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Str(String),
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct BinearyExpr {
    pub left: Expr,
    pub operator: String,
    pub right: Expr,
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}

#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
pub enum Expr {
    Literal(Literal),                // const representation
    Binary(Box<BinearyExpr>),        // binary op (1 + 1) representation
    Variable(String),                // simple variable representation
    FunctionCall(Box<FunctionCall>), // function call representation
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Stmt {
    Assignment {
        line: usize,
        name: String,
        value: Expr,
    }, // affetation like x = 42
    If {
        line: usize,
        condition: Expr,
        body: Vec<Stmt>,
    }, // condition block like if (cond) { ... }
    Expression {
        line: usize,
        expression: Expr,
    }, // Expression execution like f(1, 2)
}

#[allow(dead_code)]
pub struct Function {
    pub name: String,            //function name
    pub parameters: Vec<String>, // parameters list
    pub body: Vec<Stmt>,         // instructions in the function body
}

#[allow(dead_code)]
pub struct Programm {
    pub file: PathBuf,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub var: HashMap<String, Expr>,
}
