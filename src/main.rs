use std::io;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
enum DKind {
    Open,
    Close,
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Value(usize),
    Op(OpKind),
    Delimiter(DKind),
}

fn to_lexer(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut input_iter = input.chars();
    let mut char = input_iter.next();

    while let Some(c) = char {
        match c {
            '+' | '-' | '*' | '/' => {
                // add last token
                tokens.push(Token::Op(OpKind::from_char(c)));
                println!("ici");
                char = input_iter.next();
            }
            '(' | ')' => {
                match c {
                    '(' => tokens.push(Token::Delimiter(DKind::Open)),
                    ')' => tokens.push(Token::Delimiter(DKind::Close)),
                    _ => panic!("should be open or closed parenthesis"),
                };
                char = input_iter.next();
            }
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut tta = String::from(c);
                dbg!(&tta);
                loop {
                    char = input_iter.next();
                    match char {
                        Some(nc) => {
                            if nc.is_numeric() {
                                tta.push(nc)
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                tokens.push(Token::Value(tta.parse().unwrap()));
            }
            _ => {
                if c.is_alphabetic() {
                    todo!("letters handling")
                }
                char = input_iter.next();
            }
        };
    }

    tokens
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

fn parser(mut lx: Vec<Token>) -> ExpOrValue {
    println!("appel de parser avec lx: {:?}", &lx);
    //handle end of lexer
    if lx.len() == 1 {
        //unwrap should work since we checked that lx contains one value
        match lx.pop().unwrap() {
            Token::Value(v) => {
                return ExpOrValue::Val(v);
            }
            _ => panic!("lexex with only one item should contain a value"),
        }
    }
    // Hanldling all cases:
    // Search for +- since they are less priority operators
    let mut lx_iter = lx.iter();
    let mut next_token = lx_iter.next();
    let mut left = Vec::new();
    // if found +- operators then compute left&right vectors
    let mut par_count: usize = 0;
    while let Some(token) = next_token {
        // iter through all tokens until we find + or - not in parenthesis and store the left vector
        match token {
            Token::Op(op) => {
                // we have found a +- operator, we store the left vector and continue
                match op {
                    OpKind::Add | OpKind::Sub => {
                        // when found, we store the operand and the remaining items in a vector
                        if par_count == 0 {
                            let right: Vec<Token> = lx_iter.map(|x| x.to_owned()).collect();
                            //     return Expression
                            return ExpOrValue::Exp(Box::new(Expression {
                                left: (parser(left)),
                                right: (parser(right)),
                                op: *op,
                            }));
                        }
                        left.push(token.clone());
                        next_token = lx_iter.next();
                    }
                    _ => {
                        left.push(token.clone());
                        next_token = lx_iter.next();
                    }
                }
            }
            Token::Delimiter(DKind::Open) => {
                par_count += 1;
                left.push(token.clone());
                next_token = lx_iter.next();
            }
            Token::Delimiter(DKind::Close) => {
                par_count -= 1;
                left.push(token.clone());
                next_token = lx_iter.next();
            }
            _ => {
                left.push(token.clone());
                next_token = lx_iter.next();
            }
        }
        // TODO: if iterator not emptied, returning the Expression with the left, right vector and the operand
        //
    }
    // At this point we should have no more +- at base level
    //
    let old_left = left;
    lx_iter = old_left.iter();
    next_token = lx_iter.next();
    left = Vec::new();

    // Iter through previous left vector until emptied
    if let Some(token) = next_token {
        match token {
            Token::Value(val) => {
                //if value, expect operand then set right to the remaining tokens
                //    return Expression
                next_token = lx_iter.next();
                let right: Vec<Token> = lx_iter.map(|x| x.to_owned()).collect();
                if let Some(Token::Op(op)) = next_token {
                    return ExpOrValue::Exp(Box::new(Expression {
                        left: (ExpOrValue::Val(*val)),
                        right: (parser(right)),
                        op: *op,
                    }));
                } else if next_token.is_none() {
                    return ExpOrValue::Val(*val);
                } else {
                    panic!("token after value should be operand");
                }
            }
            Token::Delimiter(DKind::Open) => {
                //if parenthesis, store content until matching,
                let mut par_count: usize = 1;
                next_token = lx_iter.next();
                while par_count != 0 {
                    if let Some(token) = next_token {
                        match token {
                            Token::Delimiter(DKind::Open) => {
                                par_count += 1;
                            }
                            Token::Delimiter(DKind::Close) => {
                                par_count -= 1;
                            }
                            _ => {}
                        }
                        left.push(token.clone());
                        next_token = lx_iter.next();
                    } else {
                        panic!("unmatching parenthesis")
                    }
                }
                //strip off last parenthesis
                left.pop();

                //check next value for operand
                if let Some(token) = next_token {
                    match token {
                        //if operand, store it and set right to remaining items and return Expression
                        Token::Op(op) => {
                            let right: Vec<Token> = lx_iter.map(|x| x.to_owned()).collect();
                            return ExpOrValue::Exp(Box::new(Expression {
                                left: (parser(left)),
                                right: (parser(right)),
                                op: *op,
                            }));
                        }
                        _ => panic!("should be operand after parenthesis"),
                    }
                } else {
                    return parser(left);
                }
            }
            _ => panic!("should be an operand"),
        }
    }

    parser(left)
    //              else, iterator is emptied, return parser(stripped from parenthesis)
}

fn compute(ex: ExpOrValue) -> usize {
    match ex {
        ExpOrValue::Val(val) => val,
        ExpOrValue::Exp(exp) => {
            let left = match exp.left {
                ExpOrValue::Val(val) => val,
                ExpOrValue::Exp(exp) => compute(ExpOrValue::Exp(exp)),
            };
            let right = match exp.right {
                ExpOrValue::Val(val) => val,
                ExpOrValue::Exp(exp) => compute(ExpOrValue::Exp(exp)),
            };
            match exp.op {
                OpKind::Add => left + right,
                OpKind::Sub => left - right,
                OpKind::Mul => left * right,
                OpKind::Div => left / right,
            }
        }
    }
}
fn compute_string(st: &str) -> usize {
    let lx = to_lexer(st.to_string());
    let exp = parser(lx);
    dbg!(&exp);
    compute(exp)
}
fn main() {
    let mut input = String::new();

    println!("entrez une ligne");
    io::stdin().read_line(&mut input).unwrap();
    let lx = dbg!(to_lexer(input));
    let exp = dbg!(parser(lx));
    let res = dbg!(compute(exp));

    // println!("Exp: {:?}", exp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let result = compute_string("1+1");
        assert_eq!(result, 2);
    }
    #[test]
    fn mult_prio() {
        let result = compute_string("1+2*3");
        assert_eq!(result, 7);
    }
    #[test]
    fn mult_prio_rev() {
        let result = compute_string("1*2+3");
        assert_eq!(result, 5);
    }
    #[test]
    fn parenthesis_prio() {
        let result = compute_string("(1+2)*3+4");
        assert_eq!(result, 13);
    }
    #[test]
    fn parenthesis_prio_rev() {
        let result = compute_string("1*2+(3+4)");
        assert_eq!(result, 9);
    }
    #[test]
    fn par_unwrap() {
        let result = compute_string("(1+2)");
        assert_eq!(result, 3);
    }
    #[test]
    #[should_panic]
    fn par_folowwed_by_value() {
        compute_string("(1+2)5");
    }
    #[test]
    #[should_panic]
    fn par_unmatched() {
        compute_string("(1+2");
    }
}
