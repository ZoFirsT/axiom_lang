
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,                 
    Identifier(String),  
    Assign,              
    Number(i64),         
    Semicolon,           
    Illegal,             
    EOF,                 
}

pub struct Lexer {
    input: String,       
    position: usize,     
    read_position: usize,
    ch: char,            
}

impl Lexer {

    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }


    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

fn main() {
    println!("Welcome to the Axiom Compiler!");

}