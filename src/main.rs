use std::io;

#[derive(Clone, Debug, PartialEq)]
enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}
impl OpKind {
    fn from_char(c: char) -> OpKind {
        match c {
            '+' => OpKind::Add,
            '-' => OpKind::Sub,
            '*' => OpKind::Mul,
            '/' => OpKind::Div,
            _ => panic!("invalid operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Value,
    Op(OpKind),
    Delimiter,
}
#[derive(Debug)]
struct Token {
    kind: TokenKind,
    value: String,
}
#[derive(Debug)]
struct Lexer {
    tokens: Vec<Token>,
    pos: usize,
}

fn to_lexer(input: String) -> Lexer {
    let mut tokens: Vec<Token> = Vec::new();
    let mut tta = String::new();
    for char in input.chars() {
        match char {
            '+' | '-' | '*' | '/' => {
                // add last token
                if !tta.is_empty() {
                    tokens.push(Token {
                        kind: TokenKind::Value,
                        value: tta,
                    });
                    tta = String::new();
                }
                tokens.push(Token {
                    kind: TokenKind::Op(OpKind::from_char(char)),
                    value: String::from(char),
                });
            }
            '(' | ')' => {
                if !tta.is_empty() {
                    tokens.push(Token {
                        kind: TokenKind::Value,
                        value: tta,
                    });
                    tta = String::new();
                    tokens.push(Token {
                        kind: TokenKind::Delimiter,
                        value: String::from(char),
                    });
                }
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                tta.push(char);
            }
            _ => {
                if char.is_alphabetic() {
                    todo!("letters handling")
                }
            }
        };
    }

    // empty the tta buffer before exiting
    if !tta.is_empty() {
        tokens.push(Token {
            kind: TokenKind::Value,
            value: tta,
        });
    };
    Lexer { tokens, pos: 0 }
}

#[derive(Debug, Clone)]
enum ExpOrValue {
    Exp(Box<Expression>),
    Val(usize),
}
#[derive(Debug, Clone)]
struct Expression {
    left: ExpOrValue,
    right: ExpOrValue,
    op: OpKind,
}

// TODO: add precedence for mult & div
fn parser(mut lx: Lexer) -> Expression {
    let mut left: Option<ExpOrValue> = None;

    while lx.pos < lx.tokens.len() {
        let t = lx.tokens.get(lx.pos).unwrap();

        match &t.kind {
            TokenKind::Value => {
                if left.is_some() {
                    panic!("cannot have two values in a row");
                } else {
                    left = Some(ExpOrValue::Val(t.value.parse().unwrap()));
                    lx.pos += 1;
                }
            }
            TokenKind::Op(kind) => {
                if let Some(left) = &left {
                    lx.pos += 1;
                    let op = kind.clone();
                    if lx.pos == lx.tokens.len() - 1
                        && lx.tokens.get(lx.pos).unwrap().kind == TokenKind::Value
                    {
                        let next = lx.tokens.get(lx.pos).unwrap();

                        if op == OpKind::Div || op == OpKind::Mul {
                            return Expression {
                                left: ExpOrValue::Val(next.value.parse().unwrap()),
                                right: left.clone(),
                                op,
                            };
                        } else {
                            return Expression {
                                left: left.clone(),
                                right: ExpOrValue::Val(next.value.parse().unwrap()),
                                op,
                            };
                        }
                    } else {
                        let rest = Box::new(parser(lx));
                        return Expression {
                            left: left.clone(),
                            right: ExpOrValue::Exp(rest),
                            op,
                        };
                    }
                } else {
                    panic!("Parser failed: cannot have operator without left value");
                }
            }
            TokenKind::Delimiter => {
                println!("token is Delimiter");
                if let Some(_) = left {
                    todo!()
                } else {
                    todo!()
                }
            }
        }
    }
    panic!("Parser failed");
}

fn compute(ex: Expression) -> usize {
    let left = match ex.left {
        ExpOrValue::Val(val) => val,
        ExpOrValue::Exp(exp) => compute(*exp),
    };
    let right = match ex.right {
        ExpOrValue::Val(val) => val,
        ExpOrValue::Exp(exp) => compute(*exp),
    };
    match ex.op {
        OpKind::Add => left + right,
        OpKind::Sub => left - right,
        OpKind::Mul => left * right,
        OpKind::Div => left / right,
    }
}
fn main() {
    let mut input = String::new();

    println!("entrez une ligne");
    io::stdin().read_line(&mut input).unwrap();
    let lx = to_lexer(input);
    let exp = dbg!(parser(lx));
    let res = dbg!(compute(exp));

    // println!("Exp: {:?}", exp);
}
