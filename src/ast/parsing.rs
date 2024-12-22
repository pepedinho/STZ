use std::fs;

use super::structure::{BinearyExpr, Expr, FunctionCall, Literal, Programm, Stmt};

fn parse_expr(expr: &str) -> Expr {
    if expr.starts_with("\"") && expr.ends_with("\"") {
        // litteral String
        Expr::Literal(Literal::Str(expr[1..expr.len() - 1].to_string()))
    } else if let Ok(i) = expr.parse::<i64>() {
        // litteral integer
        Expr::Literal(Literal::Integer(i))
    } else if expr.contains("+") || expr.contains("-") {
        //binary expression
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() == 3 {
            return Expr::Binary(Box::new(BinearyExpr {
                left: parse_expr(parts[0]),
                operator: parts[1].to_string(),
                right: parse_expr(parts[2]),
            }));
        }
        Expr::Variable(expr.to_string()) // default
    } else if expr.contains("(") && expr.contains(")") {
        //function call
        let name_end = expr.find("(").unwrap();
        let name = expr[..name_end].to_string();
        let args_content = &expr[name_end + 1..expr.len() - 1];
        let arguments = args_content
            .split(',')
            .map(|arg| parse_expr(arg.trim()))
            .collect();
        Expr::FunctionCall(Box::new(FunctionCall { name, arguments }))
    } else {
        // variable
        Expr::Variable(expr.to_string())
    }
}

fn parse_statement(line: &str, n: usize) -> Option<Stmt> {
    if line.contains("=") {
        // assignement
        let parts: Vec<&str> = line.split("=").map(str::trim).collect();
        if parts.len() == 2 {
            let name = parts[0].to_string();
            let value = parse_expr(parts[1]);
            return Some(Stmt::Assignment {
                name,
                value,
                line: n,
            });
        }
    } else if line.starts_with("if") {
        // condition
        let condition = parse_expr(&line[3..]);
        return Some(Stmt::If {
            condition,
            body: vec![],
            line: n,
        });
    } else if line.contains("(") && line.contains(")") {
        let expr = parse_expr(line);
        return Some(Stmt::Expression {
            expression: expr,
            line: n,
        });
    }

    None
}

fn pars_prog(content: &str) -> Vec<Stmt> {
    let mut statements = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some(stmt) = parse_statement(line.trim(), i) {
            statements.push(stmt);
        }
    }
    statements
}

impl Programm {
    pub fn pars(filename: String) {
        let f_content = fs::read_to_string(filename).unwrap();
        println!("file readed succesfully !");

        let parsed_statements = pars_prog(&f_content);
        println!("==== Parsed Programm ====");
        for stmt in parsed_statements {
            println!("{:#?}", stmt);
        }
    }
}
