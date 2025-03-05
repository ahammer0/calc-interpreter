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

// TODO: add precedence for mult & div
fn parser(mut lx: Vec<Token>) -> ExpOrValue {
    println!("appel de parser avec lx: {:?}", &lx);
    //handle end of lexer
    if lx.len() == 1 {
        match lx.pop().unwrap() {
            Token::Value(v) => {
                return ExpOrValue::Val(v);
            }
            _ => panic!("lexex with only one item should contain a value"),
        }
    }

    //searching for plus or minus operators
    println!("lx avant de chercher +-: {:?}", &lx);
    let pm_pos = lx
        .iter()
        .position(|k| *k == Token::Op(OpKind::Add) || *k == Token::Op(OpKind::Sub));

    println!("lx apres de chercher +-: {:?}", &lx);
    println!("pm_pos:{:?}", &pm_pos);
    // if found +- operators then compute left&right vectors
    if let Some(pos) = pm_pos {
        let right = lx.split_off(pos + 1);
        println!("right apres split: {:?}", &right);

        if let Token::Op(op) = lx.get(pos).unwrap().clone() {
            lx.pop();
            let left = lx;
            println!("left apres split et pop: {:?}", &left);

            ExpOrValue::Exp(Box::new(Expression {
                left: (parser(left)),
                right: (parser(right)),
                op,
            }))
        } else {
            //this could not be before of previous serach
            panic!("should be operators, opKind should be Add or Sub")
        }
    } else {
        // +- not found, processing */ operators
        //
        dbg!(&lx);
        let md_pos = lx
            .iter()
            .position(|k| *k == Token::Op(OpKind::Mul) || *k == Token::Op(OpKind::Div));

        println!("md_pos:{:?}", &md_pos);
        // if found */ operators then compute left&right vectors
        if let Some(pos) = md_pos {
            let right = lx.split_off(pos + 1);

            if let Token::Op(op) = lx.get(pos).unwrap().clone() {
                lx.pop();
                let left = lx;

                ExpOrValue::Exp(Box::new(Expression {
                    left: (parser(left)),
                    right: (parser(right)),
                    op,
                }))
            } else {
                //this could not be before of previous serach
                panic!("should be operators, opKind should be Div or Mul")
            }
        } else {
            // +- not found, normal procesing
            panic!("expression should contain at least one operator");
        }
    }
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
}
