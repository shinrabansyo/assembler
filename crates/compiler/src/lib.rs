use assembler::{assembly, Inst};

pub fn compile(program: &str) -> Vec<Inst> {
    println!("{:?}", lex("10"));
    println!("{:?}", lex(""));
    println!("{:?}", lex("10+"));
    println!("{:?}", lex("10-"));
    println!("{:?}", lex("10 -"));
    println!("{:?}", lex("10 - \n     \t"));
    println!("{:?}", lex("10 - 20 + 30 - 40"));

    vec![]
}

#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
}

// 字句解析
fn lex(program: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut token = String::new();
    for c in program.chars() {
        match c {
            '0'..='9' => {
                token.push(c);
            }
            '+' => {
                tokens.push(Token::Number(token.parse::<i32>().unwrap()));
                tokens.push(Token::Plus);
                token = "".to_string();
            }
            '-' => {
                tokens.push(Token::Number(token.parse::<i32>().unwrap()));
                tokens.push(Token::Minus);
                token = "".to_string();
            }
            ' ' | '\n' | '\t' => {
                // 何もしない
            }
            _ => {
                panic!("Error!");
            }
        }
    }
    if token.len() > 0 {
        tokens.push(Token::Number(token.parse::<i32>().unwrap()));
    }

    tokens
}

fn parse(tokens: Vec<Token>) -> Vec<Inst> {
    vec![]
}

// if ( 10 == 10 ) { printf ( "hello" ) ; }
// if (10 == 20) { printf("hello"); }
