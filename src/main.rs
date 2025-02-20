use std::collections::HashMap;

fn main() {
    println!("Reagle");
    println!(
        "{}",
        Expr::parse(r#"5 repeat { args[0] print }"#)
            .map(|x| x.compile())
            .unwrap()
    );
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

impl Expr {
    fn parse(source: &str) -> Option<Expr> {
        let tokens_list = tokenize(source.trim(), &[' ', '　', '\n', '\t', '\r'])?;
        if tokens_list.len() >= 2 {
            let operator_to_method = HashMap::from([
                ("+", "add"),
                ("-", "sub"),
                ("*", "mul"),
                ("/", "div"),
                ("=", "eql"),
                ("!", "not"),
                ("&", "and"),
                ("|", "or"),
            ]);
            Some(Expr::Send {
                obj: Box::new(Expr::parse(&tokens_list[0])?),
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
                        result.push(Expr::parse(&line)?);
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
                    result.push(Expr::parse(&line)?);
                }
                Some(Expr::Block(result))
            } else if token.starts_with("(") && token.ends_with(")") {
                let token: String = token[1..token.len() - 1].to_string();
                Some(Expr::parse(&token)?)
            } else {
                Some(Expr::Value(token))
            }
        } else {
            None
        }
    }

    fn compile(&self) -> String {
        match self {
            Expr::Send { obj, msg, args } => format!(
                "({}.{msg}({}))",
                obj.compile(),
                args.iter()
                    .map(Expr::compile)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expr::Block(block) => format!("function(...args){{{}}}", {
                let mut result = String::new();
                let mut index = 0;
                while index < block.len() {
                    let code = block[index].compile();
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
                    let token: String = value[1..value.len() - 1].to_string();
                    format!("(new ReagleString('{token}'))")
                } else {
                    value.to_string()
                }
            }
        }
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
