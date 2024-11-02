#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    OpenParen,
    CloseParen,
    Num(f64),
    Var(String),
    Sin,
    Cos,
    Tan,
    Cot,
    Sec,
    Csc,
    Arcsin,
    Arccos,
    Arctan,
    Arccot,
    Arcsec,
    Arccsc,
    Sinh,
    Cosh,
    Tanh,
    Coth,
    Sech,
    Csch,
    Arsinh,
    Arcosh,
    Artanh,
    Arcoth,
    Arsech,
    Arcsch,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(&c) = chars.peek() {
        match c {
            ' ' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Star);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                chars.next();
            }
            '^' => {
            tokens.push(Token::Caret);
            chars.next();
            }
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            }
            '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let num = num_str.parse::<f64>().unwrap();
                tokens.push(Token::Num(num));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut var_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphabetic() {
                        var_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match var_str.as_str() {
                    "sin" => tokens.push(Token::Sin),
                    "cos" => tokens.push(Token::Cos),
                    "tan" => tokens.push(Token::Tan),
                    "cot" => tokens.push(Token::Cot),
                    "sec" => tokens.push(Token::Sec),
                    "csc" => tokens.push(Token::Csc),
                    "arsin" => tokens.push(Token::Arcsin),
                    "arcos" => tokens.push(Token::Arccos),
                    "artan" => tokens.push(Token::Arctan),
                    "arcot" => tokens.push(Token::Arccot),
                    "arcsec" => tokens.push(Token::Arcsec),
                    "arccsc" => tokens.push(Token::Arccsc),
                    "sinh" => tokens.push(Token::Sinh),
                    "cosh" => tokens.push(Token::Cosh),
                    "tanh" => tokens.push(Token::Tanh),
                    "coth" => tokens.push(Token::Coth),
                    "sech" => tokens.push(Token::Sech),
                    "csch" => tokens.push(Token::Csch),
                    "arsinh" => tokens.push(Token::Arsinh),
                    "arcosh" => tokens.push(Token::Arcosh),
                    "artanh" => tokens.push(Token::Artanh),
                    "arcoth" => tokens.push(Token::Arcoth),
                    "arsech" => tokens.push(Token::Arsech),
                    "arcsch" => tokens.push(Token::Arcsch),
                    _ => tokens.push(Token::Var(var_str)),
                }
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }
    
    tokens
}

pub fn print_tokens(tokens: &[Token]) {
    for token in tokens {
        match token {
            Token::Sin => println!("Token: FUNC \"Sin\""),
            Token::Cos => println!("Token: FUNC \"Cos\""),
            Token::Tan => println!("Token: FUNC \"Tan\""),
            Token::Cot => println!("Token: FUNC \"Cot\""),
            Token::Sec => println!("Token: FUNC \"Sec\""),
            Token::Csc => println!("Token: FUNC \"Csc\""),
            Token::Arcsin => println!("Token: FUNC \"Arcsin\""),
            Token::Arccos => println!("Token: FUNC \"Arccos\""),
            Token::Arctan => println!("Token: FUNC \"Arctan\""),
            Token::Arccot => println!("Token: FUNC \"Arccot\""),
            Token::Arcsec => println!("Token: FUNC \"Arcsec\""),
            Token::Arccsc => println!("Token: FUNC \"Arccsc\""),
            Token::Num(n) => println!("Token: NUM {}", n),
            Token::Var(v) => println!("Token: VAR \"{}\"", v),
            Token::Plus => println!("Token: PLUS \"+\""),
            Token::Minus => println!("Token: MINUS \"-\""),
            Token::Star => println!("Token: STAR \"*\""),
            Token::Slash => println!("Token: SLASH \"/\""),
            Token::Caret => println!("Token: CARET \"^\""),
            Token::OpenParen => println!("Token: LParen \"(\""),
            Token::CloseParen => println!("Token: RParen \")\""),
            _ => println!("Token: {:?}", token),
        }
    }
}

