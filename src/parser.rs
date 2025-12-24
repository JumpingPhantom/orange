#![allow(dead_code)]
use core::panic;

use crate::lexer::{Token, TokenType};

#[derive(Debug)]
enum Expression {
    Literal(Literal),
    Variable(String),
    Grouping(Box<Expression>),

    Unary {
        operator: TokenType,
        rhs: Box<Expression>,
    },

    Binary {
        lhs: Box<Expression>,
        operator: TokenType,
        rhs: Box<Expression>,
    },
}

#[derive(Debug)]
enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
}

pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

#[derive(Debug)]
enum Statement {
    Declaration {
        variable_name: String,
        expression: Expression,
    },
    Assignment {
        variable_name: String,
        expression: Expression,
    },
    Expression(Expression),

    Loop {
        loop_type: TokenType,
        condition: Option<Box<Expression>>,
        body: Vec<Box<Statement>>,
    },
}

/*
 * =======================GRAMMAR=============================
 *  program     ::= { statement }
 *  statement   ::= (declaration ';') | (assignment ';') | (expression ';') | loop | function
 *
 *  declaration ::= let assignment
 *  assignment  ::= identifier '=' expression
 *
 *  loop        ::= conditioned | ranged
 *  conditioned ::= while expression '{' {statement} '}'
 *  ranged      ::= for identifier in range '{' {statement} '}'
 *  range       ::= expression ',' expression
 *
 *  expression  ::= equality
 *  equality    ::= comparison {(bangequal | equalequal) comparison}
 *  comparison  ::= term {(greater | greaterequal | less | lessequal) term}
 *  term        ::= factor {(plus | minus) factor}
 *  factor      ::= unary {(asterisk | slash | percent) unary}
 *  unary       ::= {'-' | '!'} primary
 *  primary     ::= number | identifier | string | boolean | '(' expression ')'
 * ===========================================================
 */

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current_index: 0,
        }
    }

    pub fn parse(mut self) -> Self {
        self.program();
        return self;
    }

    fn current(&self) -> &Token {
        &self.tokens[self.current_index]
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current_index + 1)
    }

    /*
     * compares the current token to the supplied token_type
     * advances if they match and panics if they don't
     */
    fn expect(&mut self, token_type: TokenType) {
        if self.current().token_type == token_type {
            self.advance();
        } else {
            panic!(
                "error: parser; expected '{}', got '{}'",
                token_type,
                self.current().token_type
            );
        }
    }

    // advances to the next token
    fn advance(&mut self) {
        if self.peek().is_some() {
            self.current_index += 1;
        } else {
            eprintln!("reached the end of tokens vector")
        }
    }

    fn program(&mut self) {
        let mut stmts = Vec::<Statement>::new();

        while !matches!(self.current().token_type, TokenType::EOF) {
            stmts.push(self.statement());
        }

        dbg!(stmts);
    }

    fn _x(&mut self) -> Vec<Box<Statement>> {
        Vec::default()
    }

    fn statement(&mut self) -> Statement {
        match self.current().token_type.clone() {
            TokenType::Let => {
                let stmt = self.declaration();
                self.expect(TokenType::Semicolon);
                stmt
            }

            TokenType::Identifier => {
                let expr = self.assignment();
                self.expect(TokenType::Semicolon);
                Statement::Assignment {
                    variable_name: self.current().token_name.clone().unwrap(),
                    expression: expr,
                }
            }

            TokenType::For => {
                let mut stmts: Vec<Statement> = Vec::new();

                self.advance();
                let ident = self.expression();
                self.expect(TokenType::In);
                let begin = self.expression();
                self.expect(TokenType::Comma);
                let end = self.expression();
                self.expect(TokenType::LeftBrace);

                while self.current().token_type != TokenType::RightBrace {
                    stmts.push(self.statement());
                }

                dbg!(stmts);

                panic!();
            }

            TokenType::While => {
                self.advance();
                self.expression();

                panic!();
            }

            _ => {
                let expression = self.expression();
                self.expect(TokenType::Semicolon);
                Statement::Expression(expression)
            }
        }
    }

    fn declaration(&mut self) -> Statement {
        self.advance();
        let name = self.current().token_name.clone().unwrap();
        let value = self.assignment();
        Statement::Declaration {
            variable_name: name,
            expression: value,
        }
    }

    fn assignment(&mut self) -> Expression {
        self.expect(TokenType::Identifier);
        self.expect(TokenType::Equal);
        self.expression()
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expression = self.comparison();

        while matches!(
            self.current().token_type,
            TokenType::BangEqual | TokenType::EqualEqual
        ) {
            let operator = self.current().token_type.clone();
            self.advance();
            let rhs = self.comparison();

            expression = Expression::Binary {
                lhs: Box::new(expression),
                operator: operator,
                rhs: Box::new(rhs),
            }
        }

        expression
    }

    fn comparison(&mut self) -> Expression {
        let mut expression = self.term();

        while matches!(
            self.current().token_type,
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual
        ) {
            let operator = self.current().token_type.clone();
            self.advance();
            let rhs = self.term();

            expression = Expression::Binary {
                lhs: Box::new(expression),
                operator: operator,
                rhs: Box::new(rhs),
            }
        }

        expression
    }

    fn term(&mut self) -> Expression {
        let mut expression = self.factor();

        while matches!(
            self.current().token_type,
            TokenType::Plus | TokenType::Minus
        ) {
            let operator = self.current().token_type.clone();
            self.advance();
            let rhs = self.factor();

            expression = Expression::Binary {
                lhs: Box::new(expression),
                operator: operator,
                rhs: Box::new(rhs),
            }
        }

        expression
    }

    fn factor(&mut self) -> Expression {
        let mut expression = self.unary();

        while matches!(
            self.current().token_type,
            TokenType::Asterisk | TokenType::Slash | TokenType::Percent
        ) {
            let operator = self.current().token_type.clone();
            self.advance();
            let rhs = self.unary();

            expression = Expression::Binary {
                lhs: Box::new(expression),
                operator: operator,
                rhs: Box::new(rhs),
            }
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        if matches!(
            self.current().token_type,
            TokenType::Minus | TokenType::Bang
        ) {
            let operator = self.current().token_type.clone();
            self.advance();
            let rhs = self.unary();

            Expression::Unary {
                operator: operator,
                rhs: Box::new(rhs),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expression {
        match self.current().token_type {
            TokenType::Number => {
                let value = self
                    .current()
                    .token_name
                    .clone()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap();
                self.advance();
                Expression::Literal(Literal::Number(value))
            }

            TokenType::Identifier => {
                let name = self.current().token_name.clone().unwrap();
                self.advance();
                Expression::Variable(name)
            }

            TokenType::LeftParenthesis => {
                self.advance();
                let expression = self.expression();
                self.expect(TokenType::RightParenthesis);
                Expression::Grouping(Box::new(expression))
            }

            _ => panic!(
                "error: parser; expected expression, got {:?}",
                self.current().token_type
            ),
        }
    }
}
