use std::env;
use std::process;

#[derive(Debug, PartialEq)]
enum TokenKind {
    Reserved(char),
    Num(i64),
    Eof,
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    pos: usize,
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some(&(i, ch)) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }
        if ch == '+' || ch == '-' {
            tokens.push(Token { kind: TokenKind::Reserved(ch), pos: i });
            chars.next();
            continue;
        }
        if ch.is_ascii_digit() {
            let start = i;
            let mut end = i;
            while let Some(&(j, c)) = chars.peek() {
                if c.is_ascii_digit() {
                    chars.next();
                    end = j;
                } else {
                    break;
                }
            }
            let num_str = &input[start..=end];
            let value = num_str.parse::<i64>().unwrap();
            tokens.push(Token { kind: TokenKind::Num(value), pos: start });
            continue;
        }
        eprintln!("unexpected character: {}", ch);
        process::exit(1);
    }

    tokens.push(Token { kind: TokenKind::Eof, pos: input.len() });
    tokens
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <expression>", args[0]);
        process::exit(1);
    }

    let input = &args[1];
    let tokens = tokenize(input);
    let mut iter = tokens.iter().peekable();

    println!("    .globl main");
    println!("main:");

    let first = iter.next().unwrap();
    let acc = match first.kind {
        TokenKind::Num(val) => val,
        _ => {
            eprintln!("Expected a number at position {}", first.pos);
            process::exit(1);
        }
    };

    println!("    li a0, {}", acc);

    while let Some(tok) = iter.next() {
        match tok.kind {
            TokenKind::Reserved(op) => {
                let next_tok = iter.next().unwrap();
                let val = match next_tok.kind {
                    TokenKind::Num(v) => v,
                    _ => {
                        eprintln!("Expected number after operator at position {}", next_tok.pos);
                        process::exit(1);
                    }
                };
                match op {
                    '+' => println!("    addi a0, a0, {}", val),
                    '-' => println!("    addi a0, a0, -{}", val),
                    _ => unreachable!(),
                }
            }
            TokenKind::Eof => break,
            _ => {
                eprintln!("Unexpected token at position {}", tok.pos);
                process::exit(1);
            }
        }
    }

    println!("    ret");
}

