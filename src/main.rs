extern crate compiler_core;
use compiler_core::lexer::lexer::*;
use compiler_core::parser::Parser;
use compiler_core::graphviz::render_ast_to_png;

fn main() {
    // Test input with various expressions
    let test_inputs = vec![
        // Single expressions
        r#"id2 = "Mi nombre es Levi""#,
        r#"x = 5 + 3 * 2"#,
        r#"result = (a + b) * c"#,
        r#"func(x, y + 1)"#,
        r#"a > b && c <= d"#,
        
        // Multi-statement programs
        r#"x = 10; y = 20; result = x + y"#,
        r#"name = "John"; age = 25; print(name, age)"#,
        r#"a = 1; b = 2; c = a * b + 3; result = c > 5"#,
    ];

    for (i, input) in test_inputs.iter().enumerate() {
        println!("\n========== Test {} ==========", i + 1);
        println!("Input: {}", input);
        
        // Test lexer
        println!("\n--- Lexer Output ---");
        let mut lexer = Lexer::new(input);
        loop {
            match lexer.next_token() {
                Ok(TokenType::EOF) => {
                    println!("EOF");
                    break;
                },
                Ok(token) => println!("{:?}", token),
                Err(e) => {
                    println!("Lexer Error: {:?}", e);
                    break;
                }
            }
        }
        
        // Test parser
        println!("\n--- Parser Output ---");
        match Parser::new(input) {
            Ok(mut parser) => {
                match parser.parse() {
                    Ok(ast) => {
                        println!("AST: {:#?}", ast);
                        
                        // Generate visualization
                        let filename = format!("ast_test_{}", i + 1);
                        match render_ast_to_png(&ast, &filename) {
                            Ok(_) => println!("✓ AST visualization generated successfully"),
                            Err(e) => println!("Warning: Could not generate visualization: {}", e),
                        }
                    },
                    Err(e) => println!("Parse Error: {:?}", e),
                }
            },
            Err(e) => println!("Parser Creation Error: {:?}", e),
        }
        
        println!("========================================");
    }
    
    println!("\n🎉 All tests completed!");
}