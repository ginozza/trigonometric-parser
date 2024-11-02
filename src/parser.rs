#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Var(String),
    Func(String, Box<Expr>),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Parser<'a> {
    tokens: &'a [super::Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [super::Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse_expression(&mut self) -> Expr {
        let mut left = self.parse_term();

        while self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                super::Token::Plus => {
                    self.current += 1;
                    let right = self.parse_term();
                    left = Expr::BinOp(Box::new(left), BinOp::Add, Box::new(right));
                }
                super::Token::Minus => {
                    self.current += 1;
                    let right = self.parse_term();
                    left = Expr::BinOp(Box::new(left), BinOp::Sub, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while self.current < self.tokens.len() {
            match &self.tokens[self.current] {
                super::Token::Star => {
                    self.current += 1;
                    let right = self.parse_factor();
                    left = Expr::BinOp(Box::new(left), BinOp::Mul, Box::new(right));
                }
                super::Token::Slash => {
                    self.current += 1;
                    let right = self.parse_factor();
                    left = Expr::BinOp(Box::new(left), BinOp::Div, Box::new(right));
                }
                _ => break,
            }
        }
        left
    }

    fn parse_factor(&mut self) -> Expr {
        if self.current >= self.tokens.len() {
            panic!("Unexpected end of input");
        }

        match &self.tokens[self.current] {
            super::Token::OpenParen => {
                self.current += 1;
                let expr = self.parse_expression();
                self.expect(super::Token::CloseParen);
                expr
            }
            super::Token::Num(n) => {
                self.current += 1;
                Expr::Num(*n)
            }
            super::Token::Var(v) => {
                self.current += 1;
                Expr::Var(v.clone())
            }
            super::Token::Sin | super::Token::Cos | super::Token::Tan => {
                let func = match &self.tokens[self.current] {
                    super::Token::Sin => "sin".to_string(),
                    super::Token::Cos => "cos".to_string(),
                    super::Token::Tan => "tan".to_string(),
                    _ => unreachable!(),
                };
                self.current += 1;
                self.expect(super::Token::OpenParen);
                let arg = self.parse_expression();
                self.expect(super::Token::CloseParen);
                Expr::Func(func, Box::new(arg))
            }
            _ => panic!("Unexpected token: {:?}", self.tokens[self.current]),
        }
    }

    fn expect(&mut self, expected: super::Token) {
        if self.current < self.tokens.len() && &self.tokens[self.current] == &expected {
            self.current += 1;
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.tokens[self.current]);
        }
    }
}

