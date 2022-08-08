use crate::{
    error::Error,
    number::{Number, NumberStack},
};

/// Struct represents mathematical expression.
#[derive(Debug)]
pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    /// Creates empty expression.
    fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    /// Tries to parse input string into tokens.
    pub fn try_from(input: &str) -> Result<Self, Error> {
        let mut num_stack = NumberStack::new();
        let mut expressions_stack = ExpressionsStack::new();
        expressions_stack.push_expression();
        for ch in input.chars() {
            // Check if char is a digit: 0..9.
            if ch.is_ascii_digit() {
                num_stack.push(ch);
                continue;
            }

            // If char is not a digit and num_stack is not empty, finalize number.
            if let Some(num) = num_stack.finalize()? {
                expressions_stack.add_token(Token::Number(num))?;
            }

            // If char is an operation, add it to stack.
            if let Some(op) = Operator::try_from(ch) {
                expressions_stack.add_token(Token::Operator(op))?;
                continue;
            }

            // If char is left bracket, start new nested expression.
            // If it is right bracket, evaluate top expression on the stack.
            if let Some(br) = Bracket::try_from(ch) {
                match br {
                    Bracket::Left => expressions_stack.push_expression(),
                    Bracket::Right => {
                        if let Some(num) = num_stack.finalize()? {
                            expressions_stack.add_token(Token::Number(num))?;
                        }
                        let exp = expressions_stack.pop_expression();

                        expressions_stack.add_token(Token::Expression(exp))?;
                    }
                }
                continue;
            }

            // Invalid input, not 0..9 | a..f.
            return Err(Error::InvalidInput(ch.into()));
        }

        // Finalize last number in expression.
        if let Some(num) = num_stack.finalize()? {
            expressions_stack.add_token(Token::Number(num))?;
        }

        let res_exp = expressions_stack.pop_expression();

        // println!("{res_exp:?}");
        Ok(res_exp)
    }

    fn into_iter(self) -> impl IntoIterator<Item = Token> {
        self.tokens.into_iter()
    }

    /// Evaluate expression.
    /// Returns error if there are any wrong syntax:
    /// - number after right bracket
    /// - two operations in a row
    /// - number followed by bracket
    /// - trailing operations
    /// - no numbers in input
    /// - unmatched right/left bracket
    pub fn evaluate(self) -> Result<Number, Error> {
        let mut lhs = None;
        let mut cur_op: Option<Operator> = None;

        for token in self.into_iter() {
            match token {
                Token::Number(num) => {
                    if let Some(left_num) = lhs {
                        let op = cur_op.take().ok_or_else(|| {
                            Error::InvalidInput("number after right bracket".into())
                        })?;
                        let res_num = op.apply(left_num, num);
                        lhs = Some(res_num);
                    } else {
                        lhs = Some(num);
                    }
                }
                Token::Operator(op) => {
                    if let Some(cur_op) = cur_op.as_ref() {
                        let msg = format!("two ops in a row: {cur_op:?}, {op:?}");
                        return Err(Error::InvalidInput(msg));
                    }
                    if cur_op.is_none() {
                        cur_op = Some(op);
                    } else {
                    }
                }
                Token::Expression(exp) => {
                    let exp_res = exp.evaluate()?;
                    if let Some(left_num) = lhs {
                        let op = cur_op.take().ok_or_else(|| {
                            Error::InvalidInput("number followed by bracket".into())
                        })?;
                        let res_num = op.apply(left_num, exp_res);
                        lhs = Some(res_num);
                    } else {
                        lhs = Some(exp_res);
                    }
                }
            }
        }
        if let Some(op) = cur_op {
            Err(Error::InvalidInput(format!(
                "trailing operation (maybe before closing bracket): {op:?}"
            )))
        } else {
            lhs.ok_or_else(|| {
                Error::InvalidInput("no numbers in input/unxpected left bracket".into())
            })
        }
    }
}

/// Expressions stack.
struct ExpressionsStack {
    inner: Vec<Expression>,
}

impl ExpressionsStack {
    /// Create new stack.
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Pushes expression into stack.
    fn push_expression(&mut self) {
        self.inner.push(Expression::new())
    }

    /// Pops expression from stack.
    fn pop_expression(&mut self) -> Expression {
        self.inner.pop().unwrap()
    }

    /// Adds token to upper expression.
    /// Errors:
    /// - unexpected right bracket
    fn add_token(&mut self, token: Token) -> Result<(), Error> {
        self.inner
            .last_mut()
            .ok_or_else(|| Error::InvalidInput("unexpected right bracket".into()))
            .map(|exp| exp.tokens.push(token))
    }
}

/// Represents different kinds of tokens.
#[derive(Debug)]
enum Token {
    Number(Number),
    Operator(Operator),
    Expression(Expression),
}

/// Brackets increase priority of inner operations.
#[derive(Debug)]
enum Bracket {
    Left,
    Right,
}

impl Bracket {
    /// 'e' is a '('.
    /// 'f' is a ')'.
    fn try_from(ch: char) -> Option<Self> {
        match ch {
            'e' => Some(Self::Left),
            'f' => Some(Self::Right),
            _ => None,
        }
    }
}

/// Supported operators.
#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    /// 'a' is a '+'.
    /// 'b' is a '-'.
    /// 'c' is a '*'.
    /// 'd' is a '/'.
    pub fn try_from(ch: char) -> Option<Self> {
        match ch {
            'a' => Some(Self::Add),
            'b' => Some(Self::Sub),
            'c' => Some(Self::Mul),
            'd' => Some(Self::Div),
            _ => None,
        }
    }

    /// Applies operation on given operands.
    fn apply(self, lhs: Number, rhs: Number) -> Number {
        let res = match self {
            Operator::Add => lhs.0 + rhs.0,
            Operator::Sub => lhs.0 - rhs.0,
            Operator::Mul => lhs.0 * rhs.0,
            Operator::Div => lhs.0 / rhs.0,
        };
        Number(res)
    }
}
