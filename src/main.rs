// ==========================================
// ส่วนที่ 1: Lexer (ตัวหั่นคำ ที่เราทำเสร็จแล้ว)
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
}

// ==========================================
// ส่วนที่ 2: AST (เพิ่มการรองรับ String)
// ==========================================
#[derive(Debug)]
pub enum Statement {
    LetStatement {
        name: String,
        value: Expression,
    },
}

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    String(String),       // เพิ่มการรองรับข้อมูลประเภทข้อความ
    Identifier(String),   // เพิ่มการรองรับการอ้างอิงชื่อตัวแปรอื่น
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// ==========================================
// ส่วนที่ 3: Parser (อัปเกรดให้ฉลาดขึ้น)
// ==========================================
// ... (โครงสร้าง Parser เหมือนเดิม) ...
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

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

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };
        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        self.next_token();
        let name = match &self.current_token {
            Token::Identifier(ident) => ident.clone(),
            _ => return None,
        };

        self.next_token();
        if self.current_token != Token::Assign { return None; }

        self.next_token();
        
        // อัปเกรดตรงนี้: ให้มันวิเคราะห์ว่าค่าที่อยู่หลัง '=' คืออะไร?
        let value = match &self.current_token {
            Token::Number(num) => Expression::Integer(*num),
            Token::String(str_val) => Expression::String(str_val.clone()), // ถ้าเป็น String ให้เก็บเป็น String
            Token::Identifier(ident) => Expression::Identifier(ident.clone()), // ถ้าเป็นการดึงตัวแปรอื่นมาใส่
            _ => return None,
        };

        self.next_token();
        Some(Statement::LetStatement { name, value })
    }
}

// ==========================================
// ส่วนที่ 4: ทดสอบการทำงาน (Main)
// ==========================================
fn main() {
    println!("=== Axiom Compiler: Parser Engine v2 ===");
    
    // โค้ดที่เราต้องการทดสอบจัดโครงสร้างรอบนี้ ซับซ้อนขึ้น!
    let axiom_code = String::from(
        "
        let port = 8080;
        let host = \"127.0.0.1\";
        let target_port = port;
        "
    );
    
    println!("Source Code: {}", axiom_code);
    println!("------------------------------------");

    let lexer = Lexer::new(axiom_code);
    let mut parser = Parser::new(lexer);
    
    let program = parser.parse_program();
    
    for stmt in program.statements {
        println!("{:#?}", stmt);
    }
}