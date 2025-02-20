use std::collections::HashMap;

fn main() {
    let mut compiler = Compiler { vars: vec![] };
    let ast = compiler.parse(r#"10 repeat { x println }"#.trim()).unwrap();
    println!("{}", compiler.build(&ast));
}

#[derive(Debug, Clone)]
enum Expr {
    Value(String),
    Block(Vec<Expr>),
    Send {
        obj: Box<Expr>,
        msg: String,
        args: Vec<Expr>,
    },
}

struct Compiler {
    vars: Vec<String>,
}

impl Compiler {
    fn parse(&self, source: &str) -> Option<Expr> {
        let tokens_list = tokenize(source.trim(), &[' ', '　', '\n', '\t', '\r'])?;
        if tokens_list.len() >= 2 {
            let operator_to_method = HashMap::from([
                ("+", "add"),
                ("-", "sub"),
                ("*", "mul"),
                ("/", "div"),
                ("%", "mod"),
                ("^", "pow"),
                ("=", "eql"),
                ("!", "not"),
                ("&", "and"),
                ("|", "or"),
            ]);
            Some(Expr::Send {
                obj: Box::new(self.parse(&tokens_list[0])?),
                msg: {
                    let msg = tokens_list[1].trim().to_string();
                    operator_to_method
                        .get(&msg.as_str())
                        .unwrap_or(&msg.as_str())
                        .to_string()
                },
                args: {
                    let mut result = Vec::new();
                    for line in tokens_list[2..].iter() {
                        result.push(self.parse(&line)?);
                    }
                    result
                },
            })
        } else if tokens_list.len() == 1 {
            let token = tokens_list[0].clone();
            if token.starts_with("{") && token.ends_with("}") {
                let token: String = token[1..token.len() - 1].to_string();
                let mut result = Vec::new();
                for line in tokenize(&token, &[';'])? {
                    result.push(self.parse(&line)?);
                }
                Some(Expr::Block(result))
            } else if token.starts_with("(") && token.ends_with(")") {
                let token: String = token[1..token.len() - 1].to_string();
                Some(self.parse(&token)?)
            } else {
                Some(Expr::Value(token))
            }
        } else {
            None
        }
    }

    fn compile(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Send { obj, msg, args } if msg == ":=" => {
                format!("{} = ({})", self.compile(obj), self.compile(&args[0]))
            }
            Expr::Send { obj, msg, args } => format!(
                "({}.{msg}({}))",
                self.compile(obj),
                args.iter()
                    .map(|i| self.compile(i))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expr::Block(block) => format!("function(x){{{}}}", {
                let mut result = String::new();
                let mut index = 0;
                while index < block.len() {
                    let code = self.compile(&block[index]);
                    if index == (block.len() - 1) {
                        result.push_str(&format!("return {};", code))
                    } else {
                        result.push_str(&format!("{};", code))
                    }
                    index += 1;
                }
                result
            }),
            Expr::Value(value) => {
                if let Ok(n) = value.parse::<f64>() {
                    format!("(new ReagleNumber({n}))")
                } else if let Ok(n) = value.parse::<bool>() {
                    format!("(new ReagleBool({n}))")
                } else if value.starts_with("'") && value.ends_with("'") {
                    let value: String = value[1..value.len() - 1].to_string();
                    format!("(new ReagleString('{value}'))")
                } else {
                    self.vars.push(value.clone());
                    value.to_string()
                }
            }
        }
    }

    fn build(&mut self, expr: &Expr) -> String {
        let js_code = include_str!("./template.js");
        let body = self.compile(expr);
        format!(
            "{js_code}\n\n\n{};\n\n\n{}",
            self.vars
                .iter()
                .map(|i| format!("let {i}"))
                .collect::<Vec<String>>()
                .join(";\n"),
            body
        )
    }
}

fn tokenize(input: &str, delimiter: &[char]) -> Option<Vec<String>> {
    let mut tokens: Vec<String> = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;
    let mut is_escape = false;

    for c in input.chars() {
        if is_escape {
            current_token.push(match c {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                _ => c,
            });
            is_escape = false;
        } else {
            match c {
                '(' | '{' | '[' if !in_quote => {
                    current_token.push(c);
                    in_parentheses += 1;
                }
                ')' | '}' | ']' if !in_quote => {
                    current_token.push(c);
                    if in_parentheses != 0 {
                        in_parentheses -= 1;
                    } else {
                        return None;
                    }
                }
                '"' | '\'' | '`' => {
                    in_quote = !in_quote;
                    current_token.push(c);
                }
                '\\' if in_quote => {
                    current_token.push(c);
                    is_escape = true;
                }
                other => {
                    if delimiter.contains(&other) && !in_quote {
                        if in_parentheses != 0 {
                            current_token.push(c);
                        } else if !current_token.is_empty() {
                            tokens.push(current_token.clone());
                            current_token.clear();
                        }
                    } else {
                        current_token.push(c);
                    }
                }
            }
        }
    }

    // Syntax error check
    if is_escape || in_quote || in_parentheses != 0 {
        return None;
    }
    if !current_token.is_empty() {
        tokens.push(current_token.clone());
        current_token.clear();
    }
    Some(tokens)
}
