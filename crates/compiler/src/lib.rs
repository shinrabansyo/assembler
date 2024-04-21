use std::collections::VecDeque;

use thiserror::Error;

use assembler::{assembly, Inst};

pub fn compile(program: &str) -> Vec<Inst> {
    println!("{:?}", parse(lex("10 / (20 + 30) - 40")));
    vec![]
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Mul,
    Div,
    ParenthesisL,
    ParenthesisR,
}

// 字句解析
fn lex(program: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut reader = program.chars().into_iter().collect::<Vec<char>>();
    let mut idx = 0;
    loop {
        let c = match reader.get(idx) {
            Some(c) => c,
            None => break,
        };

        match c {
            '0'..='9' => {
                let mut num_s = String::from(*c);
                idx += 1;

                loop {
                    if let Some(c) = reader.get(idx) {
                        match c {
                            '0'..='9' => {
                                num_s.push(*c);
                                idx += 1;
                            }
                            _ => break,
                        }
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Number(num_s.parse::<i32>().unwrap()));
            }
            '+' => {
                tokens.push(Token::Plus);
                idx += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                idx += 1;
            }
            '*' => {
                tokens.push(Token::Mul);
                idx += 1;
            }
            '/' => {
                tokens.push(Token::Div);
                idx += 1;
            }
            '(' => {
                tokens.push(Token::ParenthesisL);
                idx += 1;
            }
            ')' => {
                tokens.push(Token::ParenthesisR);
                idx += 1;
            }
            ' ' | '\n' | '\t' => {
                idx += 1;
            }
            _ => {
                panic!("Error!");
            }
        }
    }

    tokens
}

#[derive(Debug)]
enum NodeKind {
    Num(i32),
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct Node {
    kind: NodeKind,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unexpected token: expected {expected:?}, but found {actual:?}!!")]
    UnexpectedToken {
        expected: Token,
        actual: Token,
    },
    #[error("Unexpected EOF!!")]
    UnexpectedEOF,
}

fn parse(tokens: Vec<Token>) -> anyhow::Result<Node> {
    // let mut node;

    parse_sum(&mut tokens.into_iter().collect::<VecDeque<Token>>())
}

fn parse_sum(tokens: &mut VecDeque<Token>) -> anyhow::Result<Node> {
    // term + or - term
    let l_term = parse_term(tokens)?;

    // ('+' or '-' 記号, 数字) の繰り返し
    let mut sum_node = l_term;
    loop {
        let kind = match tokens.get(0) {
            Some(token) if token == &Token::Plus => {
                tokens.pop_front();
                NodeKind::Add
            }
            Some(token) if token == &Token::Minus => {
                tokens.pop_front();
                NodeKind::Sub
            }
            _ => break,
        };

        // '+' の右側の数字
        let r_term = parse_term(tokens)?;

        sum_node = Node {
            kind,
            left: Some(Box::new(sum_node)),
            right: Some(Box::new(r_term)),
        };
    }

    Ok(sum_node)
}

fn parse_term(tokens: &mut VecDeque<Token>) -> anyhow::Result<Node> {
    // '*' の左側の数字
    let l_num = parse_num(tokens)?;
    
    // ('*' or '/' 記号 , 数字) の繰り返し
    let mut term_node = l_num;
    loop {
        let kind = match tokens.get(0) {
            Some(token) if token == &Token::Mul => {
                tokens.pop_front();
                NodeKind::Mul
            }
            Some(token) if token == &Token::Div => {
                tokens.pop_front();
                NodeKind::Div
            }
            _ => break,
        };

        // '*' の右側の数字
        let r_num = parse_num(tokens)?;

        // 節を作成
        term_node = Node {
            kind,
            left: Some(Box::new(term_node)),
            right: Some(Box::new(r_num)),
        };
    }

    Ok(term_node)
}

fn parse_num(tokens: &mut VecDeque<Token>) -> anyhow::Result<Node> {
    match tokens.pop_front() {
        Some(token) => match token {
            Token::Number(n) => Ok(Node {
                kind: NodeKind::Num(n),
                left: None,
                right: None,
            }),
            Token::ParenthesisL => {
                let sum = parse_sum(tokens)?;
                match tokens.pop_front() {
                    Some(Token::ParenthesisR) => Ok(sum),
                    Some(token) => Err(ParseError::UnexpectedToken { expected: Token::ParenthesisR, actual: token }.into()),
                    _ => Err(ParseError::UnexpectedEOF.into()),
                } 
            }
            _ => Err(ParseError::UnexpectedToken { expected: Token::Number(0), actual: token }.into()),
        },
        _ => Err(ParseError::UnexpectedEOF.into()),
    }
}

// if ( 10 == 10 ) { printf ( "hello" ) ; }
// if (10 == 20) { printf("hello"); }

// cargo test -p compiler
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex() {
        println!("{:?}", lex("10"));
        println!("{:?}", lex(""));
        println!("{:?}", lex("10+"));
        println!("{:?}", lex("10-"));
        println!("{:?}", lex("10 -"));
        println!("{:?}", lex("10 - \n     \t"));
        println!("{:?}", lex("10 - 20 + 30 - 40"));
        println!("{:?}", lex("10 - (20 + 30) - 40"));
        println!("{:?}", lex("10 * (20 + 30) - 40"));
        println!("{:?}", lex("10 / (20 + 30) - 40"));
    }
    
    #[test]
    fn test_parse() {
        // Ok 
        assert!(parse(lex("10")).is_ok());
        assert!(parse(lex("10 - 20 + 30 - 40")).is_ok());
        assert!(parse(lex("10 - (20 + 30) - 40")).is_ok());
        assert!(parse(lex("10 * (20 + 30) - 40")).is_ok());
        assert!(parse(lex("10 / (20 + 30) - 40")).is_ok());

        // Err
        assert!(parse(lex("")).is_err());
        assert!(parse(lex("10+")).is_err());
        assert!(parse(lex("10-")).is_err());
        assert!(parse(lex("10 -")).is_err());
        assert!(parse(lex("10 - \n     \t")).is_err());
    }
}
