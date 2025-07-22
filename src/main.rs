use axum::{
    extract::Json,
    http::{Method, StatusCode},
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use compiler_core::lexer::lexer::{Lexer, TokenType};
use compiler_core::parser::Parser;
use compiler_core::ast::ASTNode;
use compiler_core::graphviz::GraphvizRenderer;
use serde::{Deserialize, Serialize};

use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct CompileRequest {
    code: String,
}

#[derive(Serialize)]
struct TokenizeResponse {
    tokens: Vec<TokenInfo>,
    success: bool,
    error: Option<String>,
}

#[derive(Serialize)]
struct TokenInfo {
    token_type: String,
    raw_value: String,
    position: usize,
}

#[derive(Serialize)]
struct ParseResponse {
    ast: Option<ASTNode>,
    success: bool,
    error: Option<String>,
}

#[derive(Serialize)]
struct SemanticAnalysisResponse {
    steps: Vec<SemanticStep>,
    symbol_table: Vec<SymbolInfo>,
    type_checks: Vec<TypeCheck>,
    success: bool,
    error: Option<String>,
}

#[derive(Serialize)]
struct SemanticStep {
    step_number: usize,
    description: String,
    node_type: String,
    action: String,
    symbol_added: Option<String>,
    type_check: Option<String>,
    error: Option<String>,
}

#[derive(Serialize)]
struct SymbolInfo {
    name: String,
    symbol_type: String,
    data_type: String,
    scope: String,
    line: usize,
}

#[derive(Serialize)]
struct TypeCheck {
    expression: String,
    expected_type: String,
    actual_type: String,
    is_valid: bool,
    error_message: Option<String>,
}

#[derive(Serialize)]
struct VisualizationResponse {
    dot_content: String,
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
    success: bool,
    error: Option<String>,
}

#[derive(Serialize)]
struct NodeData {
    id: String,
    label: String,
    node_type: String,
    color: String,
}

#[derive(Serialize)]
struct EdgeData {
    from: String,
    to: String,
}

#[derive(Serialize)]
struct ExampleResponse {
    examples: Vec<Example>,
}

#[derive(Serialize)]
struct Example {
    name: String,
    code: String,
    description: String,
    category: String,
}

async fn health_check() -> &'static str {
    "Compiler Backend is running!"
}

async fn tokenize(Json(request): Json<CompileRequest>) -> Result<ResponseJson<TokenizeResponse>, StatusCode> {
    let mut lexer = Lexer::new(&request.code);
    let mut tokens = Vec::new();
    let mut position = 0;
    
    loop {
        match lexer.next_token() {
            Ok(TokenType::EOF) => {
                tokens.push(TokenInfo {
                    token_type: "EOF".to_string(),
                    raw_value: "".to_string(),
                    position,
                });
                break;
            },
            Ok(token) => {
                tokens.push(TokenInfo {
                    token_type: format!("{:?}", token).split('{').next().unwrap_or("Unknown").to_string(),
                    raw_value: format!("{:?}", token),
                    position,
                });
                position += 1;
            },
            Err(e) => {
                return Ok(ResponseJson(TokenizeResponse {
                    tokens,
                    success: false,
                    error: Some(format!("{:?}", e)),
                }));
            }
        }
    }
    
    Ok(ResponseJson(TokenizeResponse {
        tokens,
        success: true,
        error: None,
    }))
}

async fn parse(Json(request): Json<CompileRequest>) -> Result<ResponseJson<ParseResponse>, StatusCode> {
    match Parser::new(&request.code) {
        Ok(mut parser) => {
            match parser.parse() {
                Ok(ast) => Ok(ResponseJson(ParseResponse {
                    ast: Some(ast),
                    success: true,
                    error: None,
                })),
                Err(e) => Ok(ResponseJson(ParseResponse {
                    ast: None,
                    success: false,
                    error: Some(format!("{:?}", e)),
                })),
            }
        },
        Err(e) => Ok(ResponseJson(ParseResponse {
            ast: None,
            success: false,
            error: Some(format!("{:?}", e)),
        })),
    }
}

async fn visualize(Json(request): Json<CompileRequest>) -> Result<ResponseJson<VisualizationResponse>, StatusCode> {
    match Parser::new(&request.code) {
        Ok(mut parser) => {
            match parser.parse() {
                Ok(ast) => {
                    let mut renderer = GraphvizRenderer::new();
                    let dot_content = renderer.render_to_dot(&ast);
                    
                    // Generate simplified node/edge data for frontend
                    let (nodes, edges) = generate_visualization_data(&ast);
                    
                    Ok(ResponseJson(VisualizationResponse {
                        dot_content,
                        nodes,
                        edges,
                        success: true,
                        error: None,
                    }))
                },
                Err(e) => Ok(ResponseJson(VisualizationResponse {
                    dot_content: String::new(),
                    nodes: Vec::new(),
                    edges: Vec::new(),
                    success: false,
                    error: Some(format!("{:?}", e)),
                })),
            }
        },
        Err(e) => Ok(ResponseJson(VisualizationResponse {
            dot_content: String::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
            success: false,
            error: Some(format!("{:?}", e)),
        })),
    }
}

async fn get_examples() -> ResponseJson<ExampleResponse> {
    let examples = vec![
        Example {
            name: "String Assignment".to_string(),
            code: r#"id2 = "Mi nombre es Levi""#.to_string(),
            description: "Simple string assignment".to_string(),
            category: "basic".to_string(),
        },
        Example {
            name: "Arithmetic Expression".to_string(),
            code: "x = 5 + 3 * 2".to_string(),
            description: "Arithmetic with operator precedence".to_string(),
            category: "arithmetic".to_string(),
        },
        Example {
            name: "Parenthesized Expression".to_string(),
            code: "result = (a + b) * c".to_string(),
            description: "Parenthesized expressions".to_string(),
            category: "arithmetic".to_string(),
        },
        Example {
            name: "Function Call".to_string(),
            code: "func(x, y + 1)".to_string(),
            description: "Function calls with arguments".to_string(),
            category: "functions".to_string(),
        },
        Example {
            name: "Logical Operations".to_string(),
            code: "a > b && c <= d".to_string(),
            description: "Logical operations".to_string(),
            category: "logical".to_string(),
        },
        Example {
            name: "Multiple Statements".to_string(),
            code: "x = 10; y = 20; result = x + y".to_string(),
            description: "Multiple statements".to_string(),
            category: "advanced".to_string(),
        },
    ];
    
    ResponseJson(ExampleResponse { examples })
}

fn generate_visualization_data(ast: &ASTNode) -> (Vec<NodeData>, Vec<EdgeData>) {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut node_counter = 0;
    
    fn traverse_ast(
        node: &ASTNode,
        parent_id: Option<String>,
        nodes: &mut Vec<NodeData>,
        edges: &mut Vec<EdgeData>,
        counter: &mut usize,
    ) -> String {
        let node_id = format!("node_{}", counter);
        *counter += 1;
        
        let (label, color) = match node {
            ASTNode::Number { value, .. } => (value.clone(), "#FFE4B5".to_string()),
            ASTNode::String { value } => (format!("\"{}\"", value), "#E6E6FA".to_string()),
            ASTNode::Boolean { value } => (value.to_string(), "#90EE90".to_string()),
            ASTNode::Identifier { name } => (name.clone(), "#B0E0E6".to_string()),
            ASTNode::BinaryOp { operator, .. } => (operator.clone(), "#FFB6C1".to_string()),
            ASTNode::UnaryOp { operator, .. } => (operator.clone(), "#DDA0DD".to_string()),
            ASTNode::Assignment { .. } => ("=".to_string(), "#98FB98".to_string()),
            ASTNode::FunctionCall { name, .. } => (format!("{}()", name), "#F0E68C".to_string()),
            ASTNode::Parenthesized { .. } => ("( )".to_string(), "#D3D3D3".to_string()),
            ASTNode::Program { .. } => ("Program".to_string(), "#FFA07A".to_string()),
            ASTNode::ExpressionStatement { .. } => ("Statement".to_string(), "#20B2AA".to_string()),
        };
        
        nodes.push(NodeData {
            id: node_id.clone(),
            label,
            node_type: node.node_type().to_string(),
            color,
        });
        
        if let Some(parent) = parent_id {
            edges.push(EdgeData {
                from: parent,
                to: node_id.clone(),
            });
        }
        
        // Traverse children
        match node {
            ASTNode::BinaryOp { left, right, .. } => {
                traverse_ast(left, Some(node_id.clone()), nodes, edges, counter);
                traverse_ast(right, Some(node_id.clone()), nodes, edges, counter);
            },
            ASTNode::UnaryOp { operand, .. } => {
                traverse_ast(operand, Some(node_id.clone()), nodes, edges, counter);
            },
            ASTNode::Assignment { left, right } => {
                traverse_ast(left, Some(node_id.clone()), nodes, edges, counter);
                traverse_ast(right, Some(node_id.clone()), nodes, edges, counter);
            },
            ASTNode::FunctionCall { arguments, .. } => {
                for arg in arguments {
                    traverse_ast(arg, Some(node_id.clone()), nodes, edges, counter);
                }
            },
            ASTNode::Parenthesized { expression } => {
                traverse_ast(expression, Some(node_id.clone()), nodes, edges, counter);
            },
            ASTNode::Program { statements } => {
                for stmt in statements {
                    traverse_ast(stmt, Some(node_id.clone()), nodes, edges, counter);
                }
            },
            ASTNode::ExpressionStatement { expression } => {
                traverse_ast(expression, Some(node_id.clone()), nodes, edges, counter);
            },
            _ => {} // Leaf nodes
        }
        
        node_id
    }
    
    traverse_ast(ast, None, &mut nodes, &mut edges, &mut node_counter);
    (nodes, edges)
}

// Helper function to infer type from AST node
fn infer_type_from_node(node: &ASTNode, symbol_table: &[SymbolInfo]) -> String {
    match node {
        ASTNode::Number { is_float, .. } => {
            if *is_float {
                "float64".to_string() // Go's default float type
            } else {
                "int".to_string() // Go's default int type
            }
        },
        ASTNode::String { .. } => "string".to_string(),
        ASTNode::Boolean { .. } => "bool".to_string(),
        ASTNode::Identifier { name } => {
            // Try to find the symbol in the symbol table
            if let Some(symbol) = symbol_table.iter().find(|sym| sym.name == *name) {
                symbol.data_type.clone()
            } else {
                "unknown".to_string()
            }
        },
        ASTNode::BinaryOp { operator, left, right } => {
            let left_type = infer_type_from_node(left, symbol_table);
            let right_type = infer_type_from_node(right, symbol_table);
            
            match operator.as_str() {
                "+" | "-" | "*" | "/" | "%" => {
                    if left_type == "float64" || right_type == "float64" {
                        "float64".to_string()
                    } else {
                        "int".to_string()
                    }
                },
                "==" | "!=" | "<" | ">" | "<=" | ">=" => "bool".to_string(), // Comparison operations
                "&&" | "||" => "bool".to_string(), // Logical operations
                _ => "unknown".to_string(),
            }
        },
        ASTNode::UnaryOp { operator, operand } => {
            let operand_type = infer_type_from_node(operand, symbol_table);
            
            match operator.as_str() {
                "!" => {
                    if operand_type == "bool" {
                        "bool".to_string()
                    } else {
                        "unknown".to_string()
                    }
                },
                "-" | "+" => {
                    if operand_type == "int" || operand_type == "float64" {
                        operand_type
                    } else {
                        "unknown".to_string()
                    }
                },
                _ => "unknown".to_string(),
            }
        },
        ASTNode::FunctionCall { .. } => "unknown".to_string(), // Function return type unknown
        ASTNode::Parenthesized { expression } => infer_type_from_node(expression, symbol_table),
        _ => "unknown".to_string(),
    }
}

async fn semantic_analysis(Json(request): Json<CompileRequest>) -> Result<ResponseJson<SemanticAnalysisResponse>, StatusCode> {
    let mut steps = Vec::new();
    let mut symbol_table = Vec::new();
    let mut type_checks = Vec::new();
    let mut step_number = 1;
    
    // First, parse the AST
    let ast = match Parser::new(&request.code) {
        Ok(mut parser) => {
            match parser.parse() {
                Ok(ast) => ast,
                Err(e) => {
                    return Ok(ResponseJson(SemanticAnalysisResponse {
                        steps,
                        symbol_table,
                        type_checks,
                        success: false,
                        error: Some(format!("Error parsing: {:?}", e)),
                    }));
                }
            }
        },
        Err(e) => {
            return Ok(ResponseJson(SemanticAnalysisResponse {
                steps,
                symbol_table,
                type_checks,
                success: false,
                error: Some(format!("Error creating parser: {:?}", e)),
            }));
        }
    };
    
    // Step 1: Initialize semantic analysis
    steps.push(SemanticStep {
        step_number,
        description: "Iniciando análisis semántico".to_string(),
        node_type: "Program".to_string(),
        action: "Crear tabla de símbolos global".to_string(),
        symbol_added: None,
        type_check: None,
        error: None,
    });
    step_number += 1;
    
    // Step 2: Analyze AST nodes
    fn analyze_node(node: &ASTNode, steps: &mut Vec<SemanticStep>, symbol_table: &mut Vec<SymbolInfo>, 
                   type_checks: &mut Vec<TypeCheck>, step_number: &mut usize) {
        match node {
            ASTNode::Identifier { name } => {
                // Check if identifier is declared
                let is_declared = symbol_table.iter().any(|sym| sym.name == *name);
                if !is_declared {
                    steps.push(SemanticStep {
                        step_number: *step_number,
                        description: format!("Variable '{}' no declarada", name),
                        node_type: "Identifier".to_string(),
                        action: "Verificar declaración".to_string(),
                        symbol_added: None,
                        type_check: None,
                        error: Some(format!("Variable '{}' no está declarada", name)),
                    });
                } else {
                    // Find the symbol to get its type
                    let symbol = symbol_table.iter().find(|sym| sym.name == *name);
                    let data_type = symbol.map(|s| s.data_type.clone()).unwrap_or_else(|| "unknown".to_string());
                    
                    steps.push(SemanticStep {
                        step_number: *step_number,
                        description: format!("Variable '{}' encontrada en tabla de símbolos", name),
                        node_type: "Identifier".to_string(),
                        action: "Verificar declaración".to_string(),
                        symbol_added: None,
                        type_check: Some(data_type),
                        error: None,
                    });
                }
                *step_number += 1;
            },
            
            ASTNode::Assignment { left, right } => {
                steps.push(SemanticStep {
                    step_number: *step_number,
                    description: "Analizando asignación".to_string(),
                    node_type: "Assignment".to_string(),
                    action: "Verificar tipos de asignación".to_string(),
                    symbol_added: None,
                    type_check: Some("Assignment check".to_string()),
                    error: None,
                });
                *step_number += 1;
                
                // Analyze left side (should be identifier)
                if let ASTNode::Identifier { name } = &**left {
                    // Determine type from right side
                    let right_type = infer_type_from_node(right, symbol_table);
                    
                    // Add to symbol table if not exists
                    if !symbol_table.iter().any(|sym| sym.name == *name) {
                        symbol_table.push(SymbolInfo {
                            name: name.clone(),
                            symbol_type: "Variable".to_string(),
                            data_type: right_type.clone(),
                            scope: "Global".to_string(),
                            line: 1,
                        });
                        
                        steps.push(SemanticStep {
                            step_number: *step_number,
                            description: format!("Variable '{}' agregada a tabla de símbolos con tipo {}", name, right_type),
                            node_type: "Identifier".to_string(),
                            action: "Agregar a tabla de símbolos".to_string(),
                            symbol_added: Some(name.clone()),
                            type_check: Some(right_type),
                            error: None,
                        });
                        *step_number += 1;
                    } else {
                        // Update existing symbol type if needed
                        if let Some(symbol) = symbol_table.iter_mut().find(|sym| sym.name == *name) {
                            if symbol.data_type == "Unknown" {
                                symbol.data_type = right_type.clone();
                                steps.push(SemanticStep {
                                    step_number: *step_number,
                                    description: format!("Tipo de variable '{}' actualizado a {}", name, right_type),
                                    node_type: "Identifier".to_string(),
                                    action: "Actualizar tipo en tabla de símbolos".to_string(),
                                    symbol_added: None,
                                    type_check: Some(right_type),
                                    error: None,
                                });
                                *step_number += 1;
                            }
                        }
                    }
                }
                
                // Analyze right side
                analyze_node(right, steps, symbol_table, type_checks, step_number);
            },
            
            ASTNode::UnaryOp { operator, operand } => {
                let operand_type = infer_type_from_node(operand, symbol_table);
                let result_type = match operator.as_str() {
                    "!" => {
                        if operand_type == "bool" {
                            "bool".to_string()
                        } else {
                            "unknown".to_string()
                        }
                    },
                    "-" | "+" => {
                        if operand_type == "int" || operand_type == "float64" {
                            operand_type.clone()
                        } else {
                            "unknown".to_string()
                        }
                    },
                    _ => "unknown".to_string(),
                };
                
                let is_valid = match operator.as_str() {
                    "!" => operand_type == "bool",
                    "-" | "+" => operand_type == "int" || operand_type == "float64",
                    _ => true,
                };
                
                steps.push(SemanticStep {
                    step_number: *step_number,
                    description: format!("Analizando operación unaria: {} (operando: {})", operator, operand_type),
                    node_type: "UnaryOp".to_string(),
                    action: "Verificar tipo de operando".to_string(),
                    symbol_added: None,
                    type_check: Some(format!("Resultado: {}", result_type)),
                    error: None,
                });
                *step_number += 1;
                
                // Add type check
                type_checks.push(TypeCheck {
                    expression: format!("{}{}", 
                        operator,
                        match &**operand {
                            ASTNode::Identifier { name } => name.clone(),
                            ASTNode::Number { value, .. } => value.clone(),
                            ASTNode::Boolean { value } => value.to_string(),
                            _ => "expr".to_string(),
                        }
                    ),
                    expected_type: result_type.clone(),
                    actual_type: result_type,
                    is_valid,
                    error_message: if !is_valid {
                        Some(format!("Operación {} no válida para tipo {}", operator, operand_type))
                    } else {
                        None
                    },
                });
                
                analyze_node(operand, steps, symbol_table, type_checks, step_number);
            },
            
            ASTNode::BinaryOp { left, operator, right } => {
                let left_type = infer_type_from_node(left, symbol_table);
                let right_type = infer_type_from_node(right, symbol_table);
                let result_type = match operator.as_str() {
                    "+" | "-" | "*" | "/" | "%" => {
                        if left_type == "float64" || right_type == "float64" {
                            "float64".to_string()
                        } else {
                            "int".to_string()
                        }
                    },
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => "bool".to_string(),
                    "&&" | "||" => "bool".to_string(),
                    _ => "unknown".to_string(),
                };
                
                steps.push(SemanticStep {
                    step_number: *step_number,
                    description: format!("Analizando operación binaria: {} ({} {} {})", operator, left_type, operator, right_type),
                    node_type: "BinaryOp".to_string(),
                    action: "Verificar tipos de operandos".to_string(),
                    symbol_added: None,
                    type_check: Some(format!("Resultado: {}", result_type)),
                    error: None,
                });
                *step_number += 1;
                
                // Add type check
                let is_valid = match operator.as_str() {
                    "+" | "-" | "*" | "/" | "%" => {
                        left_type == "int" || left_type == "float64" || left_type == "float32" ||
                        right_type == "int" || right_type == "float64" || right_type == "float32"
                    },
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => {
                        left_type == "int" || left_type == "float64" || left_type == "float32" || left_type == "string" || left_type == "bool" ||
                        right_type == "int" || right_type == "float64" || right_type == "float32" || right_type == "string" || right_type == "bool"
                    },
                    "&&" | "||" => {
                        left_type == "bool" && right_type == "bool"
                    },
                    _ => true,
                };
                
                type_checks.push(TypeCheck {
                    expression: format!("{} {} {}", 
                        match &**left {
                            ASTNode::Identifier { name } => name.clone(),
                            ASTNode::Number { value, .. } => value.clone(),
                            _ => "expr".to_string(),
                        },
                        operator,
                        match &**right {
                            ASTNode::Identifier { name } => name.clone(),
                            ASTNode::Number { value, .. } => value.clone(),
                            _ => "expr".to_string(),
                        }
                    ),
                    expected_type: result_type.clone(),
                    actual_type: result_type,
                    is_valid,
                    error_message: if !is_valid {
                        Some(format!("Tipos incompatibles: {} {} {}", left_type, operator, right_type))
                    } else {
                        None
                    },
                });
                
                analyze_node(left, steps, symbol_table, type_checks, step_number);
                analyze_node(right, steps, symbol_table, type_checks, step_number);
            },
            
            ASTNode::Number { value, is_float } => {
                let go_type = if *is_float { "float64" } else { "int" };
                steps.push(SemanticStep {
                    step_number: *step_number,
                    description: format!("Literal numérico: {} (tipo: {})", value, go_type),
                    node_type: "Number".to_string(),
                    action: "Verificar tipo numérico".to_string(),
                    symbol_added: None,
                    type_check: Some(go_type.to_string()),
                    error: None,
                });
                *step_number += 1;
            },
            
            ASTNode::String { value } => {
                steps.push(SemanticStep {
                    step_number: *step_number,
                    description: format!("Literal de cadena: \"{}\" (tipo: string)", value),
                    node_type: "String".to_string(),
                    action: "Verificar tipo string".to_string(),
                    symbol_added: None,
                    type_check: Some("string".to_string()),
                    error: None,
                });
                *step_number += 1;
            },
            
            ASTNode::Boolean { value } => {
                steps.push(SemanticStep {
                    step_number: *step_number,
                    description: format!("Literal booleano: {} (tipo: bool)", value),
                    node_type: "Boolean".to_string(),
                    action: "Verificar tipo booleano".to_string(),
                    symbol_added: None,
                    type_check: Some("bool".to_string()),
                    error: None,
                });
                *step_number += 1;
            },
            
            ASTNode::Program { statements } => {
                for stmt in statements {
                    analyze_node(stmt, steps, symbol_table, type_checks, step_number);
                }
            },
            
            ASTNode::ExpressionStatement { expression } => {
                analyze_node(expression, steps, symbol_table, type_checks, step_number);
            },
            
            _ => {
                steps.push(SemanticStep {
                    step_number: *step_number,
                    description: "Analizando nodo".to_string(),
                    node_type: node.node_type().to_string(),
                    action: "Procesar nodo".to_string(),
                    symbol_added: None,
                    type_check: None,
                    error: None,
                });
                *step_number += 1;
            }
        }
    }
    
    // Analyze the AST
    analyze_node(&ast, &mut steps, &mut symbol_table, &mut type_checks, &mut step_number);
    
    // Final step
    steps.push(SemanticStep {
        step_number,
        description: "Análisis semántico completado".to_string(),
        node_type: "Program".to_string(),
        action: "Validación final".to_string(),
        symbol_added: None,
        type_check: None,
        error: None,
    });
    
    Ok(ResponseJson(SemanticAnalysisResponse {
        steps,
        symbol_table,
        type_checks,
        success: true,
        error: None,
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/tokenize", post(tokenize))
        .route("/api/parse", post(parse))
        .route("/api/visualize", post(visualize))
        .route("/api/semantic-analysis", post(semantic_analysis))
        .route("/api/examples", get(get_examples))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    
    println!("🚀 Compiler Backend running on http://localhost:3000");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}