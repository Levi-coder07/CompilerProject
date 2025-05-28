extern crate compiler_core;
use compiler_core::lexer::lexer::*;
fn main(){
    let mut lexer = Lexer::new("(())(()[]]");
    loop {
    
        match lexer.next_token() {
            Ok(TokenType::EOF)=> break,
            Ok(token) => 
                println!("{:?}", token),
            
            Err(e) => 
                println!("Error: {:?}", e),
        }
    }
    println!("Hello, world!");
}