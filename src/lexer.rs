use std::{
    fmt::{self, Display},
    fs::read_to_string,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Dot,
    Comma,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    QuestionMark,
    Semicolon,
    Bang,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Caret,

    If,
    While,
    Do,
    Else,
    For,
    In,
    Let,
    Function,
    Return,
    Use,

    Identifier,
    Number,
    String,
    Boolean,

    Equal,
    EqualEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    BangEqual,

    And,
    Or,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Dot => write!(f, "."),
            Self::Comma => write!(f, ","),
            Self::LeftParenthesis => write!(f, "("),
            Self::RightParenthesis => write!(f, ")"),
            Self::LeftBracket => write!(f, "["),
            Self::RightBracket => write!(f, "]"),
            Self::LeftBrace => write!(f, "{{"),
            Self::RightBrace => write!(f, "}}"),
            Self::QuestionMark => write!(f, "?"),
            Self::Semicolon => write!(f, ";"),
            Self::Bang => write!(f, "!"),

            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Asterisk => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Percent => write!(f, "%"),
            Self::Caret => write!(f, "^"),

            Self::If => write!(f, "if"),
            Self::While => write!(f, "while"),
            Self::Do => write!(f, "do"),
            Self::Else => write!(f, "else"),
            Self::For => write!(f, "for"),
            Self::In => write!(f, "in"),
            Self::Let => write!(f, "let"),
            Self::Function => write!(f, "function"),
            Self::Return => write!(f, "return"),
            Self::Use => write!(f, "use"),

            Self::Identifier => write!(f, "identifier"),
            Self::Number => write!(f, "number"),
            Self::String => write!(f, "string"),
            Self::Boolean => write!(f, "boolean"),
            Self::Equal => write!(f, "="),
            Self::EqualEqual => write!(f, "=="),
            Self::Greater => write!(f, ">"),
            Self::Less => write!(f, "<"),
            Self::GreaterEqual => write!(f, ">="),
            Self::LessEqual => write!(f, "<="),
            Self::BangEqual => write!(f, "!="),

            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),

            Self::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub token_name: Option<String>,
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    source: String,
    row: usize,
    col: usize,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            tokens: Default::default(),
            source: Default::default(),
            row: 1,
            col: 1,
        }
    }
}

impl Lexer {
    pub fn new(source_path: &str) -> Self {
        let src = read_to_string(source_path).expect("Error reading file.");

        Lexer {
            source: src,
            ..Default::default()
        }
    }

    pub fn tokenize(mut self) -> Self {
        macro_rules! push_token {
            ($type:expr, $name:expr) => {
                self.tokens.push(Token {
                    token_type: $type,
                    token_name: $name,
                })
            };
        }

        let mut iter = self.source.chars().peekable();

        while let Some(current_char) = iter.next() {
            match current_char {
                '.' => push_token!(TokenType::Dot, None),
                ',' => push_token!(TokenType::Comma, None),
                '(' => push_token!(TokenType::LeftParenthesis, None),
                ')' => push_token!(TokenType::RightParenthesis, None),
                '[' => push_token!(TokenType::LeftBracket, None),
                ']' => push_token!(TokenType::RightBracket, None),
                '{' => push_token!(TokenType::LeftBrace, None),
                '}' => push_token!(TokenType::RightBrace, None),
                '?' => push_token!(TokenType::QuestionMark, None),
                ';' => push_token!(TokenType::Semicolon, None),
                '+' => push_token!(TokenType::Plus, None),
                '-' => push_token!(TokenType::Minus, None),
                '*' => push_token!(TokenType::Asterisk, None),
                '/' => push_token!(TokenType::Slash, None),
                '%' => push_token!(TokenType::Percent, None),
                '^' => push_token!(TokenType::Caret, None),

                c if c == '"' => {
                    let mut buffer = String::new();

                    while let Some(cc) = iter.next() {
                        self.col += 1;
                        if cc != '"' {
                            buffer.push(cc);
                        } else {
                            break;
                        }
                    }

                    push_token!(TokenType::String, Some(buffer));
                }

                c if c.is_alphabetic() || c == '_' => {
                    let mut buffer = String::new();
                    buffer.push(c);

                    while let Some(cc) = iter.peek() {
                        if cc.is_alphanumeric() || *cc == '_' {
                            buffer.push(*cc);
                            iter.next();
                            self.col += 1;
                        } else {
                            break;
                        }
                    }

                    match buffer.as_str() {
                        "let" => push_token!(TokenType::Let, None),
                        "if" => push_token!(TokenType::If, None),
                        "else" => push_token!(TokenType::Else, None),
                        "while" => push_token!(TokenType::While, None),
                        "do" => push_token!(TokenType::Do, None),
                        "in" => push_token!(TokenType::In, None),
                        "for" => push_token!(TokenType::For, None),
                        "fn" => push_token!(TokenType::Function, None),
                        "return" => push_token!(TokenType::Return, None),
                        "use" => push_token!(TokenType::Use, None),
                        "true" | "false" => push_token!(TokenType::Boolean, Some(buffer)),
                        _ => push_token!(TokenType::Identifier, Some(buffer)),
                    }
                }

                c if c.is_numeric() => {
                    let mut buffer = String::new();
                    let mut seen_dot = false;

                    buffer.push(c);

                    while let Some(&cc) = iter.peek() {
                        if cc.is_numeric() {
                            buffer.push(cc);
                            iter.next();
                            self.col += 1;
                        } else if cc == '.' && !seen_dot {
                            seen_dot = true;
                            buffer.push(cc);
                            iter.next();
                            self.col += 1;
                        } else {
                            break;
                        }
                    }

                    push_token!(TokenType::Number, Some(buffer));
                }

                c if c == '!' => {
                    if matches!(iter.peek(), Some(&'=')) {
                        push_token!(TokenType::BangEqual, None);
                        iter.next();
                        self.col += 1;
                    } else {
                        push_token!(TokenType::Bang, None);
                    }
                }

                c if c == '=' => {
                    if matches!(iter.peek(), Some(&'=')) {
                        push_token!(TokenType::EqualEqual, None);
                        iter.next();
                        self.col += 1;
                    } else {
                        push_token!(TokenType::Equal, None);
                    }
                }

                c if c == '>' => {
                    if matches!(iter.peek(), Some(&'=')) {
                        push_token!(TokenType::GreaterEqual, None);
                        iter.next();
                        self.col += 1;
                    } else {
                        push_token!(TokenType::Greater, None);
                    }
                }

                c if c == '<' => {
                    if matches!(iter.peek(), Some(&'=')) {
                        push_token!(TokenType::LessEqual, None);
                        iter.next();
                        self.col += 1;
                    } else {
                        push_token!(TokenType::Less, None);
                    }
                }

                c if c.is_whitespace() => {
                    if c == '\n' {
                        self.row += 1;
                        self.col = 0;
                    }
                }

                c if c == '&' => {
                    if matches!(iter.peek(), Some(&'&')) {
                        iter.next();
                        iter.next();
                        self.col += 2;
                        push_token!(TokenType::And, None);
                    }
                }

                c if c == '|' => {
                    if matches!(iter.peek(), Some(&'|')) {
                        iter.next();
                        iter.next();
                        self.col += 2;
                        push_token!(TokenType::Or, None);
                    }
                }

                '#' => {
                    while let Some(c) = iter.next() {
                        self.col += 1;
                        if c == '\n' {
                            break;
                        };
                    }

                    self.row += 1;
                    self.col = 0;
                }

                _ => panic!(
                    "  ::LEXER::  unknown token '{}' at [{}, {}]",
                    current_char, self.row, self.col
                ),
            }

            self.col += 1;
        }

        push_token!(TokenType::EOF, None);

        return self;
    }

    pub fn _d(&self) {
        dbg!(&self.tokens);
    }
}
