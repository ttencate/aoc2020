type Num = u64;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Op {
    Plus,
    Star,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Op::Plus => "+",
            Op::Star => "*",
        })
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Token {
    Num(Num),
    Op(Op),
    ParenOpen,
    ParenClose,
    Eol,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Num(num) => write!(f, "{}", num),
            Token::Op(op) => write!(f, "{}", op),
            Token::ParenOpen => write!(f, "("),
            Token::ParenClose => write!(f, ")"),
            Token::Eol => write!(f, "<eol>"),
        }
    }
}

#[derive(Debug)]
struct ParseError { idx: usize, msg: String }

struct Tokenizer<'a> {
    bytes: &'a [u8],
    idx: usize,
}

impl<'a> Tokenizer<'a> {
    fn curr(&self) -> Option<char> {
        if self.idx < self.bytes.len() { Some(self.bytes[self.idx] as char) } else { None }
    }

    fn peek(&self) -> Option<char> {
        if self.idx + 1 < self.bytes.len() { Some(self.bytes[self.idx + 1] as char) } else { None }
    }

    fn advance(&mut self) {
        self.idx += 1;
    }

    fn err(&self, msg: String) -> ParseError {
        ParseError { idx: self.idx, msg }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, ParseError>;

    fn next(&mut self) -> Option<Result<Token, ParseError>> {
        while self.curr().map(char::is_whitespace).unwrap_or(false) {
            self.advance();
        }
        let radix = 10;
        Some(if let Some(c) = self.curr() {
            let token = match c {
                '+' => Token::Op(Op::Plus),
                '*' => Token::Op(Op::Star),
                '(' => Token::ParenOpen,
                ')' => Token::ParenClose,
                d if d.is_digit(radix) => {
                    let mut num = 0;
                    loop {
                        num = (num * radix as Num) + self.curr().unwrap().to_digit(radix).unwrap() as Num;
                        if self.peek().map(|d| d.is_digit(radix)).unwrap_or(false) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    Token::Num(num)
                },
                _ => {
                    return Some(Err(self.err(format!("unexpected character: '{}'", c))));
                },
            };
            self.advance();
            Ok(token)
        } else {
            Ok(Token::Eol)
        })
    }
}

fn tokenize(expr: &str) -> Tokenizer {
    Tokenizer { bytes: expr.as_bytes(), idx: 0 }
}

struct Lexer<'a> {
    tok: Tokenizer<'a>,
    curr: Token,
}

impl<'a> Lexer<'a> {
    fn new(line: &str) -> Result<Lexer, ParseError> {
        let mut tok = tokenize(line);
        let curr = tok.next().unwrap()?;
        Ok(Lexer { tok, curr })
    }

    fn eol(&mut self) -> Result<(), ParseError> {
        match self.curr {
            Token::Eol => Ok(()),
            _ => Err(self.expected("end of line")),
        }
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.curr = self.tok.next().unwrap()?;
        Ok(())
    }

    fn expected(&self, what: &str) -> ParseError {
        self.tok.err(format!("expected {}, found '{}'", what, self.curr))
    }
}

struct Eval1<'a> {
    lexer: Lexer<'a>
}

impl<'a> Eval1<'a> {
    fn eval(&mut self) -> Result<Num, ParseError> {
        let num = self.expr()?;
        self.lexer.eol()?;
        Ok(num)
    }

    fn expr(&mut self) -> Result<Num, ParseError> {
        let mut num = self.term()?;
        while let Token::Op(op) = self.lexer.curr {
            self.lexer.advance()?;
            let term = self.term()?;
            match op {
                Op::Plus => num += term,
                Op::Star => num *= term,
            }
        }
        Ok(num)
    }

    fn term(&mut self) -> Result<Num, ParseError> {
        Ok(match self.lexer.curr {
            Token::Num(num) => {
                self.lexer.advance()?;
                num
            },
            Token::ParenOpen => {
                self.lexer.advance()?;
                let num = self.expr()?;
                if self.lexer.curr != Token::ParenClose {
                    return Err(self.lexer.expected("')'"));
                }
                self.lexer.advance()?;
                num
            },
            _ => {
                return Err(self.lexer.expected("number or '('"));
            }
        })
    }
}

fn part1(input: &str) -> Num {
    input
        .lines()
        .map(|line| Eval1 { lexer: Lexer::new(line).unwrap() }.eval().unwrap())
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
    assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
    assert_eq!(part1(&aoc::input()), 6811433855019);
}

struct Eval2<'a> {
    lexer: Lexer<'a>
}

impl<'a> Eval2<'a> {
    fn eval(&mut self) -> Result<Num, ParseError> {
        let num = self.product()?;
        self.lexer.eol()?;
        Ok(num)
    }

    fn product(&mut self) -> Result<Num, ParseError> {
        let mut num = self.sum()?;
        while let Token::Op(Op::Star) = self.lexer.curr {
            self.lexer.advance()?;
            let factor = self.sum()?;
            num *= factor;
        }
        Ok(num)
    }

    fn sum(&mut self) -> Result<Num, ParseError> {
        let mut num = self.term()?;
        while let Token::Op(Op::Plus) = self.lexer.curr {
            self.lexer.advance()?;
            let term = self.term()?;
            num += term;
        }
        Ok(num)
    }

    fn term(&mut self) -> Result<Num, ParseError> {
        Ok(match self.lexer.curr {
            Token::Num(num) => {
                self.lexer.advance()?;
                num
            },
            Token::ParenOpen => {
                self.lexer.advance()?;
                let num = self.product()?;
                if self.lexer.curr != Token::ParenClose {
                    return Err(self.lexer.expected("')'"));
                }
                self.lexer.advance()?;
                num
            },
            _ => {
                return Err(self.lexer.expected("number or '('"));
            }
        })
    }
}

fn part2(input: &str) -> Num {
    input
        .lines()
        .map(|line| Eval2 { lexer: Lexer::new(line).unwrap() }.eval().unwrap())
        .sum()
}

#[test]
fn test_part2() {
    assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
    assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    assert_eq!(part2(&aoc::input()), 129770152447927);
}

fn main() {
    aoc::main(part1, part2);
}
