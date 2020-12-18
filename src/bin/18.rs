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
    curr: Token,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Result<Tokenizer, ParseError> {
        let mut tok = Tokenizer {
            bytes: input.as_bytes(),
            idx: 0,
            curr: Token::Eol,
        };
        tok.advance()?;
        Ok(tok)
    }

    fn curr_char(&self) -> Option<char> {
        if self.idx < self.bytes.len() { Some(self.bytes[self.idx] as char) } else { None }
    }

    fn next_char(&self) -> Option<char> {
        if self.idx + 1 < self.bytes.len() { Some(self.bytes[self.idx + 1] as char) } else { None }
    }

    pub fn err(&self, msg: String) -> ParseError {
        ParseError { idx: self.idx, msg }
    }

    pub fn expected(&self, what: &str) -> ParseError {
        self.err(format!("expected {}, found '{}'", what, self.curr))
    }

    pub fn curr(&self) -> Token {
        self.curr
    }

    pub fn advance(&mut self) -> Result<(), ParseError> {
        while self.curr_char().map(char::is_whitespace).unwrap_or(false) {
            self.idx += 1;
        }
        let radix = 10;
        self.curr = if let Some(c) = self.curr_char() {
            let token = match c {
                '+' => Token::Op(Op::Plus),
                '*' => Token::Op(Op::Star),
                '(' => Token::ParenOpen,
                ')' => Token::ParenClose,
                d if d.is_digit(radix) => {
                    let mut num = 0;
                    loop {
                        num = (num * radix as Num) + self.curr_char().unwrap().to_digit(radix).unwrap() as Num;
                        if self.next_char().map(|d| d.is_digit(radix)).unwrap_or(false) {
                            self.idx += 1;
                        } else {
                            break;
                        }
                    }
                    Token::Num(num)
                },
                _ => {
                    return Err(self.err(format!("unexpected character: '{}'", c)));
                },
            };
            self.idx += 1;
            token
        } else {
            Token::Eol
        };
        Ok(())
    }

    pub fn expect_eol(&mut self) -> Result<(), ParseError> {
        match self.curr {
            Token::Eol => Ok(()),
            _ => Err(self.expected("end of line")),
        }
    }
}

struct Parser1<'a> {
    tok: Tokenizer<'a>
}

impl<'a> Parser1<'a> {
    fn eval(&mut self) -> Result<Num, ParseError> {
        let num = self.expr()?;
        self.tok.expect_eol()?;
        Ok(num)
    }

    fn expr(&mut self) -> Result<Num, ParseError> {
        let mut num = self.term()?;
        while let Token::Op(op) = self.tok.curr() {
            self.tok.advance()?;
            let term = self.term()?;
            match op {
                Op::Plus => num += term,
                Op::Star => num *= term,
            }
        }
        Ok(num)
    }

    fn term(&mut self) -> Result<Num, ParseError> {
        Ok(match self.tok.curr() {
            Token::Num(num) => {
                self.tok.advance()?;
                num
            },
            Token::ParenOpen => {
                self.tok.advance()?;
                let num = self.expr()?;
                if self.tok.curr() != Token::ParenClose {
                    return Err(self.tok.expected("')'"));
                }
                self.tok.advance()?;
                num
            },
            _ => {
                return Err(self.tok.expected("number or '('"));
            }
        })
    }
}

fn part1(input: &str) -> Num {
    input
        .lines()
        .map(|line| Parser1 { tok: Tokenizer::new(line).unwrap() }.eval().unwrap())
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

struct Parser2<'a> {
    tok: Tokenizer<'a>
}

impl<'a> Parser2<'a> {
    fn eval(&mut self) -> Result<Num, ParseError> {
        let num = self.product()?;
        self.tok.expect_eol()?;
        Ok(num)
    }

    fn product(&mut self) -> Result<Num, ParseError> {
        let mut num = self.sum()?;
        while let Token::Op(Op::Star) = self.tok.curr() {
            self.tok.advance()?;
            let factor = self.sum()?;
            num *= factor;
        }
        Ok(num)
    }

    fn sum(&mut self) -> Result<Num, ParseError> {
        let mut num = self.term()?;
        while let Token::Op(Op::Plus) = self.tok.curr() {
            self.tok.advance()?;
            let term = self.term()?;
            num += term;
        }
        Ok(num)
    }

    fn term(&mut self) -> Result<Num, ParseError> {
        Ok(match self.tok.curr() {
            Token::Num(num) => {
                self.tok.advance()?;
                num
            },
            Token::ParenOpen => {
                self.tok.advance()?;
                let num = self.product()?;
                if self.tok.curr() != Token::ParenClose {
                    return Err(self.tok.expected("')'"));
                }
                self.tok.advance()?;
                num
            },
            _ => {
                return Err(self.tok.expected("number or '('"));
            }
        })
    }
}

fn part2(input: &str) -> Num {
    input
        .lines()
        .map(|line| Parser2 { tok: Tokenizer::new(line).unwrap() }.eval().unwrap())
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
