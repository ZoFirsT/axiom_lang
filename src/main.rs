// ==========================================
// Phase 1: Lexical Analyzer (Lexer)
// ==========================================
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let, Function, Identifier(String), String(String),
    Assign, Number(i64), Plus, Minus, Asterisk, Slash,
    LParen, RParen, LBrace, RBrace, Comma, Colon, Semicolon,
    Illegal, EOF,
}

pub struct Lexer {
    input: String, position: usize, read_position: usize, ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer { input, position: 0, read_position: 0, ch: '\0' };
        lexer.read_char();
        lexer
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() { self.ch = '\0'; } 
        else { self.ch = self.input.chars().nth(self.read_position).unwrap(); }
        self.position = self.read_position;
        self.read_position += 1;
    }
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' { self.read_char(); }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '=' => Token::Assign, ';' => Token::Semicolon,
            '+' => Token::Plus, '-' => Token::Minus,
            '*' => Token::Asterisk, '/' => Token::Slash,
            '"' => Token::String(self.read_string()),
            '\0' => Token::EOF,
            _ => {
                if self.is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return match ident.as_str() { "let" => Token::Let, "fn" => Token::Function, _ => Token::Identifier(ident) };
                } else if self.is_digit(self.ch) {
                    return Token::Number(self.read_number());
                } else { Token::Illegal }
            }
        };
        self.read_char();
        tok
    }
    fn is_letter(&self, ch: char) -> bool { ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_' }
    fn is_digit(&self, ch: char) -> bool { '0' <= ch && ch <= '9' }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) { self.read_char(); }
        self.input[position..self.position].to_string()
    }
    fn read_number(&mut self) -> i64 {
        let position = self.position;
        while self.is_digit(self.ch) { self.read_char(); }
        self.input[position..self.position].parse::<i64>().unwrap()
    }
    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\0' { break; }
        }
        self.input[position..self.position].to_string()
    }
}

// ==========================================
// Phase 2: Abstract Syntax Tree (AST) Structures
// ==========================================
#[derive(Debug)]
pub enum Statement {
    LetStatement { name: String, value: Expression },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    String(String),
    Identifier(String),
    InfixExpression {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    }
}

#[derive(Debug)]
pub struct Program { pub statements: Vec<Statement> }

// ==========================================
// Phase 3: Syntactic Analyzer (Pratt Parser)
// ==========================================
#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest = 1,
    Sum = 2,      // + หรือ -
    Product = 3,  // * หรือ /
}

pub struct Parser { lexer: Lexer, current_token: Token, peek_token: Token }

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser { lexer, current_token, peek_token }
    }
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    
    fn peek_precedence(&self) -> Precedence {
        match self.peek_token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
    
    fn current_precedence(&self) -> Precedence {
        match self.current_token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };
        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() { program.statements.push(stmt); }
            self.next_token();
        }
        program
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token { Token::Let => self.parse_let_statement(), _ => None }
    }
    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let name = match &self.current_token { Token::Identifier(ident) => ident.clone(), _ => return None };
        self.next_token();
        if self.current_token != Token::Assign { return None; }
        self.next_token();
        
        let value = self.parse_expression(Precedence::Lowest);

        while self.current_token != Token::Semicolon && self.current_token != Token::EOF {
            self.next_token();
        }
        
        value.map(|val| Statement::LetStatement { name, value: val })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left_exp = match &self.current_token {
            Token::Number(num) => Expression::Integer(*num),
            Token::String(str_val) => Expression::String(str_val.clone()),
            Token::Identifier(ident) => Expression::Identifier(ident.clone()),
            _ => return None,
        };

        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            match self.peek_token {
                Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                _ => return Some(left_exp),
            }
        }
        Some(left_exp)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = match self.current_token {
            Token::Plus => "+", Token::Minus => "-",
            Token::Asterisk => "*", Token::Slash => "/",
            _ => "",
        }.to_string();

        let precedence = self.current_precedence();
        self.next_token();
        
        let right = self.parse_expression(precedence)?;

        Some(Expression::InfixExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

// ==========================================
// Phase 4: Execution & Verification
// ==========================================
fn main() {
    println!("=== Axiom Compiler: Pratt Parser Engine ===");
    
    let axiom_code = String::from(
        "
        let result = 10 + 5 * 2;
        "
    );
    
    println!("Target Source Code:\n{}", axiom_code);
    println!("------------------------------------");
    println!("Generated AST Configuration:");

    let lexer = Lexer::new(axiom_code);
    let mut parser = Parser::new(lexer);
    
    let program = parser.parse_program();
    for stmt in program.statements { println!("{:#?}", stmt); }
}