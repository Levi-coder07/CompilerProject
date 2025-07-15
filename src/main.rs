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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/tokenize", post(tokenize))
        .route("/api/parse", post(parse))
        .route("/api/visualize", post(visualize))
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