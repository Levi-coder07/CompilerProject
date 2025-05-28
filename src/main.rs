extern crate compiler_core;
use compiler_core::lexer::lexer::*;
fn main(){
    let mut lexer = Lexer::new(r#"((152..2))(()[]] id2 = "Mi nombre es Levi" "#);
    loop {
    
        match lexer.next_token() {
            Ok(TokenType::EOF)=> break,
            Ok(token) => 
                println!("{:?}", token),
            
            Err(e) => 
                println!("Error: {:?}", e),
        }
    }
   
}