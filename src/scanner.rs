use super::err;
use std::cmp::PartialEq;
use std::str;
use strum_macros::AsRefStr;

#[derive(AsRefStr, PartialEq)]
pub enum TokenKind
{
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    String,
    Number,

    // keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

pub struct Token<'a>
{
    pub kind: TokenKind,
    pub str: &'a [u8],
    pub line: u32,
}

impl Token<'_>
{
    pub fn as_str(&self) -> &str
    {
        unsafe { str::from_utf8_unchecked(self.str) }
    }
}

pub struct Scanner<'a>
{
    start: u32,
    current: u32,
    line: u32,
    source: &'a [u8],
}

macro_rules! two_char_token {
    ($enum:expr, $enum_equal:expr, $self:expr) => {
        if $self.match_next('=')
        {
            Ok($self.make_token($enum_equal))
        }
        else
        {
            Ok($self.make_token($enum))
        }
    };
}

impl Scanner<'_>
{
    pub fn new(source: &str) -> Scanner
    {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            source: source.as_bytes(),
        }
    }

    pub fn scan_token(&mut self) -> Result<Token, err::Error>
    {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_end()
        {
            return Ok(self.make_token(TokenKind::Eof));
        }

        let c = self.advance();
        return match c
        {
            '(' => Ok(self.make_token(TokenKind::LeftParen)),
            ')' => Ok(self.make_token(TokenKind::RightParen)),
            '{' => Ok(self.make_token(TokenKind::LeftBrace)),
            '}' => Ok(self.make_token(TokenKind::RightBrace)),
            ';' => Ok(self.make_token(TokenKind::Semicolon)),
            ',' => Ok(self.make_token(TokenKind::Comma)),
            '.' => Ok(self.make_token(TokenKind::Dot)),
            '-' => Ok(self.make_token(TokenKind::Minus)),
            '+' => Ok(self.make_token(TokenKind::Plus)),
            '/' => Ok(self.make_token(TokenKind::Slash)),
            '*' => Ok(self.make_token(TokenKind::Star)),
            '!' =>
            {
                two_char_token!(TokenKind::BangEqual, TokenKind::Bang, self)
            }
            '=' =>
            {
                two_char_token!(TokenKind::EqualEqual, TokenKind::Equal, self)
            }
            '<' =>
            {
                two_char_token!(TokenKind::LessEqual, TokenKind::Less, self)
            }
            '>' =>
            {
                two_char_token!(TokenKind::GreaterEqual, TokenKind::Greater, self)
            }
            '"' => self.string(),
            '0'..'9' => Ok(self.number()),
            'a'..'z' | 'A'..'Z' | '_' => Ok(self.identifier()),
            _ => Err(err::Error::CompileError("Unexpected character".to_string())),
        };
    }

    pub fn get_slice(&self, start: u32, end: u32) -> &[u8]
    {
        &self.source[start as usize..end as usize]
    }

    fn is_end(&self) -> bool
    {
        self.get_char(self.current) as char == '\0'
    }

    fn make_token(&self, kind: TokenKind) -> Token
    {
        Token {
            kind,
            str: self.get_slice(self.start, self.current),
            line: self.line,
        }
    }

    fn string(&mut self) -> Result<Token, err::Error>
    {
        while self.peek() != '"' && self.is_end()
        {
            if self.peek() == '\n'
            {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end()
        {
            return Err(err::Error::CompileError("Unterminated string.".to_string()));
        }

        // Skip closing quote
        self.advance();

        Ok(self.make_token(TokenKind::String))
    }

    fn number(&mut self) -> Token
    {
        while self.peek().is_digit(10)
        {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10)
        {
            // Consume the "."
            self.advance();

            while self.peek().is_digit(10)
            {
                self.advance();
            }
        }

        self.make_token(TokenKind::Number)
    }

    fn identifier(&mut self) -> Token
    {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) || self.peek() == '_'
        {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenKind
    {
        // A simple trie that recognizes keywords
        match self.get_char(self.start)
        {
            'a' => self.check_keyword(1, 2, "nd", TokenKind::And),
            'c' => self.check_keyword(1, 4, "lass", TokenKind::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenKind::Else),
            'f' =>
            {
                if self.current - self.start > 1
                {
                    match self.get_char(self.start + 1)
                    {
                        'a' => self.check_keyword(2, 3, "lse", TokenKind::False),
                        'o' => self.check_keyword(2, 1, "r", TokenKind::For),
                        'u' => self.check_keyword(2, 1, "n", TokenKind::Fun),
                        _ => TokenKind::Identifier,
                    }
                }
                else
                {
                    TokenKind::Identifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenKind::If),
            'n' => self.check_keyword(1, 2, "il", TokenKind::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenKind::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenKind::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenKind::Return),
            's' => self.check_keyword(1, 4, "uper", TokenKind::Super),
            't' =>
            {
                if self.current - self.start > 1
                {
                    match self.get_char(self.start + 1)
                    {
                        'h' => self.check_keyword(2, 2, "is", TokenKind::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenKind::True),
                        _ => TokenKind::Identifier,
                    }
                }
                else
                {
                    TokenKind::Identifier
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenKind::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenKind::While),
            _ => TokenKind::Identifier,
        }
    }

    fn check_keyword(&self, start: u32, length: u32, rest: &str, kind: TokenKind) -> TokenKind
    {
        // We check for two conditions:
        // 1. The lexeme is as long as the keyword
        // 2. All the characters match
        if self.current - self.start == start + length
            && self.get_slice(self.start + start, self.start + length) == rest.as_bytes()
        {
            kind
        }
        else
        {
            TokenKind::Identifier
        }
    }

    fn get_char(&self, i: u32) -> char
    {
        match self.source.get(i as usize)
        {
            Some(value) => *value as char,
            None => '\0',
        }
    }

    fn advance(&mut self) -> char
    {
        self.current += 1;
        self.get_char(self.current - 1)
    }

    fn match_next(&mut self, expected: char) -> bool
    {
        if self.is_end()
        {
            return false;
        }

        if self.get_char(self.current) != expected
        {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char
    {
        self.get_char(self.current)
    }

    fn peek_next(&self) -> char
    {
        self.get_char(self.current + 1)
    }

    fn skip_whitespace(&mut self)
    {
        loop
        {
            let c = self.peek();
            match c
            {
                ' ' | '\r' | '\t' =>
                {
                    self.advance();
                }
                '\n' =>
                {
                    self.line += 1;
                    self.advance();
                }
                '/' =>
                {
                    if self.peek_next() == '/'
                    {
                        while self.peek() != '\n' && !self.is_end()
                        {
                            self.advance();
                        }
                    }
                    else
                    {
                        return;
                    }
                }
                _ => return,
            };
        }
    }
}
