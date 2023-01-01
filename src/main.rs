use std::env;
use std::fs;

#[derive(Debug)]
enum TokenKind {
    Identifier,
    Assign,
    Let,
    String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Token {
    kind: TokenKind,
    literal: String,
}

struct Lexer {
    source: Vec<char>,
    counter: usize,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            counter: 0,
        }
    }

    pub fn lex(&mut self) {
        let mut tokens: Vec<Token> = Vec::new();
        while self.counter < self.source.len() {
            match self.current_char() {
                ' ' => self.advance(),
                'a'..='z' => {
                    let mut literal = String::new();
                    while self.current_char().is_alphanumeric() {
                        literal.push(self.current_char());
                        self.advance();
                    }
                    tokens.push(Token::new(TokenKind::Identifier, literal));
                }
                '-' => {
                    self.advance();
                    if self.current_char() == '>' {
                        self.advance();
                        tokens.push(Token::new(TokenKind::Assign, "->".to_string()));
                    }
                }
                _ if self.current_char() == 'l' => {
                    let mut literal = String::new();
                    while self.current_char().is_alphanumeric() {
                        literal.push(self.current_char());
                        self.advance();
                    }
                    tokens.push(Token::new(TokenKind::Let, literal));
                }
                '\'' => {
                    let mut literal = String::new();
                    self.advance();
                    while self.current_char() != '\'' {
                        literal.push(self.current_char());
                        self.advance();
                    }
                    self.advance();
                    tokens.push(Token::new(TokenKind::String, literal));
                }
                _ => panic!("Unexpected character: {}", self.current_char()),
            }
        }
        print!("{:?}", tokens);
    }

    pub fn current_char(&self) -> char {
        self.source[self.counter]
    }

    pub fn advance(&mut self) {
        self.counter += 1;
    }
}

fn main() {
    let file = env::args().nth(1).unwrap();
    let source = fs::read_to_string(file).unwrap();

    let mut lexer = Lexer::new(source);
    lexer.lex();
}
