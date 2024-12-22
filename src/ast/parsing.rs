use std::fs;

use super::structure::{BinearyExpr, Expr, FunctionCall, Literal, Programm, Stmt};

fn parse_expr(expr: &str, type_key: Option<&str>) -> Result<Expr, String> {
    if let Some(key) = type_key {
        if key == "str" {
            return Ok(Expr::Literal(Literal::Str(
                expr[1..expr.len() - 1].to_string(),
            )));
        } else if key == "int" {
            if expr.contains("+") || expr.contains("-") {
                //binary expression
                let parts: Vec<&str> = expr.split_whitespace().collect();
                if parts.len() == 3 {
                    let left = match parse_expr(parts[0], type_key) {
                        Ok(l) => l,
                        Err(e) => return Err(e),
                    };
                    let right = match parse_expr(parts[2], type_key) {
                        Ok(r) => r,
                        Err(e) => return Err(e),
                    };
                    return Ok(Expr::Binary(Box::new(BinearyExpr {
                        left,
                        operator: parts[1].to_string(),
                        right,
                    })));
                }
                return Ok(Expr::Variable(expr.to_string())); // default
            }

            if let Ok(i) = expr.parse::<i64>() {
                return Ok(Expr::Literal(Literal::Integer(i)));
            } else {
                return Err(format!("Invalid type expected '{}' ! expr : {}", key, expr));
            }
        }
    }
    if expr.starts_with("\"") && expr.ends_with("\"") {
        // litteral String
        Ok(Expr::Literal(Literal::Str(
            expr[1..expr.len() - 1].to_string(),
        )))
    } else if let Ok(i) = expr.parse::<i64>() {
        // litteral integer
        Ok(Expr::Literal(Literal::Integer(i)))
    } else if expr.contains("+") || expr.contains("-") {
        //binary expression
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() == 3 {
            let left = match parse_expr(parts[0], type_key) {
                Ok(l) => l,
                Err(e) => return Err(e),
            };
            let right = match parse_expr(parts[2], type_key) {
                Ok(r) => r,
                Err(e) => return Err(e),
            };
            return Ok(Expr::Binary(Box::new(BinearyExpr {
                left,
                operator: parts[1].to_string(),
                right,
            })));
        }
        Ok(Expr::Variable(expr.to_string())) // default
    } else if expr.contains("(") && expr.contains(")") {
        //function call
        let name_end = expr.find("(").unwrap();
        let name = expr[..name_end].to_string();
        let args_content = &expr[name_end + 1..expr.len() - 1];
        let arguments = args_content
            .split(',')
            .map(|arg| parse_expr(arg.trim(), None).unwrap())
            .collect();
        Ok(Expr::FunctionCall(Box::new(FunctionCall {
            name,
            arguments,
        })))
    } else {
        // variable
        Ok(Expr::Variable(expr.to_string()))
    }
}

fn is_type_keyword(word: &str) -> bool {
    matches!(word, "int" | "float" | "str" | "bool")
}

fn parse_statement(line: &str, n: usize) -> Result<Option<Stmt>, String> {
    if let Some((type_key, rest)) = line.split_once(' ') {
        if is_type_keyword(type_key) {
            if line.contains("=") {
                // assignement
                let parts: Vec<&str> = rest.split("=").map(str::trim).collect();
                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    let value = match parse_expr(parts[1], Some(type_key)) {
                        //ici
                        Ok(l) => l,
                        Err(e) => return Err(format!("line {}: {}", n, e)),
                    };
                    return Ok(Some(Stmt::Assignment {
                        name,
                        value,
                        line: n,
                    }));
                }
            }
        }
    }
    if line.starts_with("if") {
        // condition
        let condition = parse_expr(&line[3..], None).unwrap();
        return Ok(Some(Stmt::If {
            condition,
            body: vec![],
            line: n,
        }));
    } else if line.contains("(") && line.contains(")") {
        let expr = parse_expr(line, None).unwrap();
        return Ok(Some(Stmt::Expression {
            expression: expr,
            line: n,
        }));
    }
    Ok(None)
}

fn pars_prog(content: &str) -> Vec<Stmt> {
    let mut statements = Vec::new();

    for (i, line) in content.lines().enumerate() {
        // iteration ligne par ligne
        if line.trim().is_empty() {
            continue; // si vide on ignore
        }

        match parse_statement(line.trim(), i) {
            Ok(stmt) => {
                if let Some(st) = stmt {
                    statements.push(st);
                }
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
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
