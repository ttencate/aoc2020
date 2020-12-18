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

#[derive(Debug, PartialEq, Eq)]
enum Term {
    Num(Num),
    Paren(Box<Expr>),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Term::Num(num) => write!(f, "{}", num),
            Term::Paren(expr) => write!(f, "({})", expr),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Chain(Term, Vec<(Op, Term)>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Chain(head, tail) => {
                write!(f, "({}", head)?;
                for (op, expr) in tail {
                    write!(f, " {} {}", op, expr)?;
                }
                write!(f, ")")?;
                Ok(())
            }
        }
    }
}

struct Parser<'a> {
    tok: Tokenizer<'a>,
    curr: Token,
    next: Token,
}

impl<'a> Parser<'a> {
    fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expr()?;
        self.eol()?;
        Ok(expr)
    }

    fn expr(&mut self) -> Result<Expr, ParseError> {
        let head = self.term()?;
        let mut tail = Vec::new();
        while let Token::Op(op) = self.curr {
            self.advance()?;
            let term = self.term()?;
            tail.push((op, term));
        }
        Ok(Expr::Chain(head, tail))
    }

    fn term(&mut self) -> Result<Term, ParseError> {
        Ok(match self.curr {
            Token::Num(num) => {
                self.advance()?;
                Term::Num(num)
            },
            Token::ParenOpen => {
                self.advance()?;
                let expr = self.expr()?;
                if self.curr != Token::ParenClose {
                    return Err(self.expected("')'"));
                }
                self.advance()?;
                Term::Paren(Box::new(expr))
            },
            _ => {
                return Err(self.expected("number or '('"));
            }
        })
    }

    fn eol(&mut self) -> Result<(), ParseError> {
        match self.curr {
            Token::Eol => Ok(()),
            _ => Err(self.expected("end of line")),
        }
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.curr = self.next;
        self.next = self.tok.next().unwrap()?;
        Ok(())
    }

    fn expected(&self, what: &str) -> ParseError {
        self.tok.err(format!("expected {}, found '{}'", what, self.curr))
    }
}

fn parse(line: &str) -> Result<Expr, ParseError> {
    let mut tok = tokenize(line);
    let curr = tok.next().unwrap()?;
    let next = tok.next().unwrap()?;
    let mut parser = Parser { tok, curr, next };
    parser.parse()
}

fn eval_term(term: &Term) -> Num {
    match term {
        Term::Num(num) => *num,
        Term::Paren(expr) => eval(expr),
    }
}

fn eval(expr: &Expr) -> Num {
    match expr {
        Expr::Chain(head, tail) => {
            let mut num = eval_term(head);
            for (op, term) in tail {
                match op {
                    Op::Plus => num += eval_term(term),
                    Op::Star => num *= eval_term(term),
                }
            }
            num
        },
    }
}

fn part1(input: &str) -> Num {
    input
        .lines()
        .map(|line| eval(&parse(line).unwrap()))
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(eval(&parse("1 + 2 * 3 + 4 * 5 + 6").unwrap()), 71);
    assert_eq!(eval(&parse("2 * 3 + (4 * 5)").unwrap()), 26);
    assert_eq!(eval(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap()), 437);
    assert_eq!(eval(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap()), 12240);
    assert_eq!(eval(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap()), 13632);
    assert_eq!(part1(&aoc::input()), 6811433855019);
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

#[test]
fn test_part2() {
    // assert_eq!(part2(&aoc::example(0)), );
    // assert_eq!(part2(&aoc::input()), );
}

fn main() {
    aoc::main(part1, part2);
}
