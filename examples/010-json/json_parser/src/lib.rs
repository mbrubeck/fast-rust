use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
    Bool(bool),
    Null
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseError;
pub type ParseResult<T> = Result<T, ParseError>;

impl Value {
    pub fn from_str(s: &str) -> ParseResult<Self> {
        let mut parser = Parser::new(s);
        let value = parser.parse_value()?;
        parser.skip_whitespace();
        if parser.done() {
            Ok(value)
        } else {
            Err(ParseError)
        }
    }
}

struct Parser {
    input: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(s: &str) -> Self {
        Parser {
            input: s.chars().collect(),
            pos: 0
        }
    }

    fn peek(&self) -> ParseResult<char> {
        if self.pos < self.input.len() {
            Ok(self.input[self.pos])
        } else {
            Err(ParseError)
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn next(&mut self) -> ParseResult<char> {
        let c = self.peek()?;
        self.advance();
        Ok(c)
    }

    fn done(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.done() && self.input[self.pos].is_whitespace() {
            self.advance();
        }
    }

    fn expect_char(&mut self, c: char) -> ParseResult<()> {
        if self.next()? == c {
            Ok(())
        } else {
            Err(ParseError)
        }
    }

    fn expect_str(&mut self, s: &str) -> ParseResult<()> {
        for c in s.chars() {
            self.expect_char(c)?;
        }
        Ok(())
    }

    fn parse_value(&mut self) -> ParseResult<Value> {
        self.skip_whitespace();
        match self.peek()? {
            '"' => self.parse_string(),
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            't' => self.parse_true(),
            'f' => self.parse_false(),
            'n' => self.parse_null(),
            '-' | '0'...'9' => self.parse_number(),
            _ => Err(ParseError)
        }
    }

    fn parse_string(&mut self) -> ParseResult<Value> {
        self.expect_char('"')?;

        let mut s = String::new();
        loop {
            match self.next()? {
                '"' => break,
                '\\' => match self.next()? {
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    '/' => s.push('/'),
                    'n' => s.push('\n'),
                    'r' => s.push('\r'),
                    't' => s.push('\t'),
                    'b' => s.push('\u{10}'),
                    'f' => s.push('\u{14}'),
                    'u' => s.push(std::char::from_u32(self.parse_hex_digit()? << 12 |
                                                      self.parse_hex_digit()? << 8 |
                                                      self.parse_hex_digit()? << 4 |
                                                      self.parse_hex_digit()?).ok_or(ParseError)?),
                    _ => return Err(ParseError)
                },
                x => s.push(x)
            }
        }
        Ok(Value::String(s))
    }

    fn parse_hex_digit(&mut self) -> ParseResult<u32> {
        self.next()?.to_digit(16).ok_or(ParseError)
    }

    fn parse_number(&mut self) -> ParseResult<Value> {
        let mut s = String::new();

        // Optional negative sign.
        if self.peek()? == '-' {
            s.push('-');
            self.advance();
        }

        // A single '0' or one or more digits.
        match self.peek()? {
            '0' => {
                s.push('0');
                self.advance();
            }
            _ => s += &self.parse_digits()?
        }

        // Optional fractional part.
        if self.peek() == Ok('.') {
            s.push('.');
            self.advance();
            s += &self.parse_digits()?;
        }

        // Optional exponent.
        match self.peek() {
            Ok('e') | Ok('E') => {
                s.push('e');
                self.advance();
                // Optional sign.
                match self.peek()? {
                    sign @ '+' | sign @ '-' => {
                        s.push(sign);
                        self.advance();
                    }
                    _ => {}
                }
                s += &self.parse_digits()?;
            }
            _ => {}
        }

        // Use Rust's f64 parser to parse the number
        let num = f64::from_str(&s).or(Err(ParseError))?;
        Ok(Value::Number(num))
    }

    /// Consume one or more digits '0'...'9' and return them as a string.
    fn parse_digits(&mut self) -> ParseResult<String> {
        let mut s = String::new();
        match self.peek()? {
            '0'...'9' => while let Ok(digit @ '0'...'9') = self.peek() {
                s.push(digit);
                self.advance();
            },
            _ => return Err(ParseError)
        }
        Ok(s)
    }

    fn parse_object(&mut self) -> ParseResult<Value> {
        self.expect_char('{')?;

        let mut map = HashMap::new();
        loop {
            self.skip_whitespace();
            let key = match self.parse_string()? {
                Value::String(s) => s,
                _ => unreachable!()
            };

            self.skip_whitespace();
            self.expect_char(':')?;

            self.skip_whitespace();
            let value = self.parse_value()?;

            map.insert(key, value);

            self.skip_whitespace();
            match self.next()? {
                '}' => break,
                ',' => continue,
                _ => return Err(ParseError)
            }
        }
        Ok(Value::Object(map))
    }

    fn parse_array(&mut self) -> ParseResult<Value> {
        self.expect_char('[')?;

        let mut values = vec![];
        loop {
            self.skip_whitespace();
            match self.peek()? {
                ']' => { self.advance(); break }
                _ => values.push(self.parse_value()?)
            }
            self.skip_whitespace();
            match self.next()? {
                ']' => break,
                ',' => continue,
                _ => return Err(ParseError)
            }
        }
        Ok(Value::Array(values))
    }

    fn parse_true(&mut self) -> ParseResult<Value> {
        self.expect_str("true")?;
        Ok(Value::Bool(true))
    }

    fn parse_false(&mut self) -> ParseResult<Value> {
        self.expect_str("false")?;
        Ok(Value::Bool(false))
    }

    fn parse_null(&mut self) -> ParseResult<Value> {
        self.expect_str("null")?;
        Ok(Value::Null)
    }
}
