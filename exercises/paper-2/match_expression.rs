use std::fmt::{Display, Formatter};

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Expression {
    Number(i32),
    Operation {
        left: Box<Expression>,
        op: Operation,
        right: Box<Expression>,
    },
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Sub => write!(f, "-"),
            Operation::Mul => write!(f, "*"),
            Operation::Div => write!(f, "/"),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Operation {left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expression::Number(n) => write!(f, "{}", n),
        }
    }
}

fn evaluate_expression(expression: &Expression) -> Result<i32, &str> {
    match expression {
        Expression::Operation { left, op, right } => {
            let val_left = evaluate_expression(left)?;
            let val_right = evaluate_expression(right)?;

            match op {
                Operation::Add => {
                    let r = val_left.checked_add(val_right);
                    match r {
                        Option::None => Result::Err("Overflow"),
                        Option::Some(v) => Result::Ok(v),
                    }
                }
                Operation::Sub => {
                    let r = val_left.checked_sub(val_right);
                    match r {
                        Option::None => Result::Err("Overflow"),
                        Option::Some(v) => Result::Ok(v),
                    }
                }
                Operation::Mul => {
                    let r = val_left.checked_mul(val_right);
                    match r {
                        Option::None => Result::Err("Overflow"),
                        Option::Some(v) => Result::Ok(v),
                    }
                }
                Operation::Div => {
                    let r = val_left.checked_div(val_right);
                    match r {
                        Option::None => Result::Err("Overflow"),
                        Option::Some(v) => Result::Ok(v),
                    }
                }
            }
        }

        Expression::Number(n) => Result::Ok(*n),
    }
}
